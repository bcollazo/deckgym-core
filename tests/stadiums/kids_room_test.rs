use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, PlayedCard},
    test_support::get_initialized_game,
};

fn trainer_from_id(card_id: CardId) -> deckgym::models::TrainerCard {
    match get_card_by_enum(card_id) {
        Card::Trainer(trainer_card) => trainer_card,
        _ => panic!("Expected trainer card"),
    }
}

/// Sets up a game with Kid's Room active, a non-tool card in player 0's hand, and (optionally)
/// a Pokemon Tool card in player 0's deck.
fn setup_game_with_kids_room(tool_in_deck: bool) -> deckgym::Game<'static> {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 2;

    state.hands[0] = vec![
        get_card_by_enum(CardId::B3b069KidsRoom),
        get_card_by_enum(CardId::PA001Potion),
    ];

    if tool_in_deck {
        state.decks[0]
            .cards
            .push(get_card_by_enum(CardId::A2148RockyHelmet));
    }

    game.set_state(state);

    let trainer_card = trainer_from_id(CardId::B3b069KidsRoom);
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    });

    game
}

#[test]
fn test_kids_room_use_stadium_available_with_tool_in_deck() {
    let game = setup_game_with_kids_room(true);
    let state = game.get_state_clone();

    assert!(
        state.active_stadium.is_some(),
        "Kid's Room should be active"
    );

    let (_actor, actions) = state.generate_possible_actions();
    let has_use_stadium = actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::UseStadium));

    assert!(
        has_use_stadium,
        "UseStadium should be available when Kid's Room is active, hand has a card, and deck has a Tool"
    );
}

#[test]
fn test_kids_room_use_stadium_not_available_without_tool_in_deck() {
    let game = setup_game_with_kids_room(false);
    let state = game.get_state_clone();

    let (_actor, actions) = state.generate_possible_actions();
    let has_use_stadium = actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::UseStadium));

    assert!(
        !has_use_stadium,
        "UseStadium should NOT be available when deck has no Pokemon Tool card"
    );
}

#[test]
fn test_kids_room_switches_hand_card_with_random_tool_from_deck() {
    let mut game = setup_game_with_kids_room(true);

    // Use the stadium effect
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseStadium,
        is_stack: false,
    });

    // The game queues a choice of which hand card to switch — pick the Potion.
    let state = game.get_state_clone();
    let stack_top = state.move_generation_stack.last().cloned();
    let (_actor, choices) = stack_top.expect("Kid's Room should push a choice of hand cards");
    let choice = choices
        .iter()
        .find(|action| {
            matches!(
                action,
                SimpleAction::SwitchHandCardForRandomTool { hand_card }
                    if hand_card.get_name() == "Potion"
            )
        })
        .expect("Potion should be an available choice")
        .clone();
    game.apply_action(&Action {
        actor: 0,
        action: choice,
        is_stack: true,
    });

    let state = game.get_state_clone();

    // Potion should have left the hand and Rocky Helmet should now be in hand.
    assert!(
        !state.hands[0]
            .iter()
            .any(|card| card.get_name() == "Potion"),
        "Potion should have been switched out of hand"
    );
    assert!(
        state.hands[0]
            .iter()
            .any(|card| card.get_name() == "Rocky Helmet"),
        "Rocky Helmet should have been switched into hand"
    );

    // Potion should now be in the deck, and Rocky Helmet should be gone from it.
    assert!(
        state.decks[0]
            .cards
            .iter()
            .any(|card| card.get_name() == "Potion"),
        "Potion should have been placed into the deck"
    );
    assert!(
        !state.decks[0]
            .cards
            .iter()
            .any(|card| card.get_name() == "Rocky Helmet"),
        "Rocky Helmet should have left the deck"
    );

    assert!(
        state.has_used_stadium[0],
        "has_used_stadium[0] should be true after using Kid's Room"
    );
}

#[test]
fn test_kids_room_cannot_use_twice_per_turn() {
    let mut game = setup_game_with_kids_room(true);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseStadium,
        is_stack: false,
    });

    let state = game.get_state_clone();
    if let Some((_actor, choices)) = state.move_generation_stack.last().cloned() {
        game.apply_action(&Action {
            actor: 0,
            action: choices[0].clone(),
            is_stack: true,
        });
    }

    let state = game.get_state_clone();
    let (_actor, actions) = state.generate_possible_actions();
    let has_use_stadium = actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::UseStadium));

    assert!(
        !has_use_stadium,
        "UseStadium should NOT be available after using Kid's Room once this turn"
    );
}
