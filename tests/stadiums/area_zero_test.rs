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

/// Sets up a game with Area Zero active and a Basic Pokemon in player 0's hand.
fn setup_game_with_area_zero(basic_in_hand: bool) -> deckgym::Game<'static> {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 2;

    let mut hand = vec![get_card_by_enum(CardId::B3a074AreaZero)];
    if basic_in_hand {
        hand.push(get_card_by_enum(CardId::A1033Charmander));
    } else {
        // Non-basic Trainer card only
        hand.push(get_card_by_enum(CardId::PA001Potion));
    }
    state.hands[0] = hand;

    game.set_state(state);

    let trainer_card = trainer_from_id(CardId::B3a074AreaZero);
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    });

    game
}

#[test]
fn test_area_zero_use_stadium_available_with_basic_in_hand() {
    let game = setup_game_with_area_zero(true);
    let state = game.get_state_clone();

    assert!(state.active_stadium.is_some(), "Area Zero should be active");

    let (_actor, actions) = state.generate_possible_actions();
    let has_use_stadium = actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::UseStadium));

    assert!(
        has_use_stadium,
        "UseStadium should be available when Area Zero is active and hand has a Basic Pokemon"
    );
}

#[test]
fn test_area_zero_use_stadium_not_available_without_basic_in_hand() {
    let game = setup_game_with_area_zero(false);
    let state = game.get_state_clone();

    let (_actor, actions) = state.generate_possible_actions();
    let has_use_stadium = actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::UseStadium));

    assert!(
        !has_use_stadium,
        "UseStadium should NOT be available when hand has no Basic Pokemon"
    );
}

#[test]
fn test_area_zero_shuffles_basic_into_deck_and_draws_card() {
    let mut game = setup_game_with_area_zero(true);
    let state = game.get_state_clone();

    let hand_size_before = state.hands[0].len();
    let deck_size_before = state.decks[0].cards.len();

    // Use the stadium effect
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseStadium,
        is_stack: false,
    });

    // The game queues a choice — pick the first option (shuffle the Basic Pokemon)
    let state = game.get_state_clone();
    let stack_top = state.move_generation_stack.last().cloned();
    if let Some((_actor, choices)) = stack_top {
        let choice = choices[0].clone();
        game.apply_action(&Action {
            actor: 0,
            action: choice,
            is_stack: true,
        });
    }

    let state = game.get_state_clone();

    // Hand size should be unchanged: shuffled one Pokemon in, drew one card
    assert_eq!(
        state.hands[0].len(),
        hand_size_before,
        "Hand size should be unchanged after shuffling a Pokemon in and drawing a card"
    );

    // Deck should have one more card (Basic Pokemon shuffled in) and one less (drawn card), net 0
    assert_eq!(
        state.decks[0].cards.len(),
        deck_size_before,
        "Deck size should be unchanged (one shuffled in, one drawn)"
    );

    // has_used_stadium should be true
    assert!(
        state.has_used_stadium[0],
        "has_used_stadium[0] should be true after using Area Zero"
    );
}

#[test]
fn test_area_zero_cannot_use_twice_per_turn() {
    let mut game = setup_game_with_area_zero(true);

    // Use the stadium once
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseStadium,
        is_stack: false,
    });

    // Resolve the queued choice
    let state = game.get_state_clone();
    if let Some((_actor, choices)) = state.move_generation_stack.last().cloned() {
        game.apply_action(&Action {
            actor: 0,
            action: choices[0].clone(),
            is_stack: true,
        });
    }

    // Now check UseStadium is no longer available
    let state = game.get_state_clone();
    let (_actor, actions) = state.generate_possible_actions();
    let has_use_stadium = actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::UseStadium));

    assert!(
        !has_use_stadium,
        "UseStadium should NOT be available after using Area Zero once this turn"
    );
}
