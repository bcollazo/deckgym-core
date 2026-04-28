use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, PlayedCard, TrainerCard},
    test_support::get_initialized_game,
};

fn make_field_blower_trainer_card() -> TrainerCard {
    match get_card_by_enum(CardId::B3147FieldBlower) {
        Card::Trainer(tc) => tc,
        _ => panic!("Expected trainer card"),
    }
}

fn make_rocky_helmet_card() -> Card {
    get_card_by_enum(CardId::A2148RockyHelmet)
}

#[test]
fn test_field_blower_discards_opponents_tool() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1033Charmander).with_tool(make_rocky_helmet_card())],
    );
    state.hands[0].clear();
    let trainer_card = make_field_blower_trainer_card();
    state.hands[0].push(Card::Trainer(trainer_card.clone()));
    game.set_state(state);

    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    };
    game.apply_action(&play_action);

    // Player must choose what to discard
    let state = game.get_state_clone();
    let (actor, choices) = state.generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(
        choices.iter().any(|a| matches!(
            a.action,
            SimpleAction::DiscardToolFromPokemon {
                player: 1,
                in_play_idx: 0
            }
        )),
        "Should offer discarding opponent's tool"
    );

    // Apply the discard
    let discard_action = choices
        .iter()
        .find(|a| {
            matches!(
                a.action,
                SimpleAction::DiscardToolFromPokemon {
                    player: 1,
                    in_play_idx: 0
                }
            )
        })
        .unwrap()
        .clone();
    game.apply_action(&discard_action);

    let state = game.get_state_clone();
    assert!(
        state.in_play_pokemon[1][0]
            .as_ref()
            .unwrap()
            .attached_tool
            .is_none(),
        "Opponent's tool should be discarded"
    );
    assert!(
        state.discard_piles[1].contains(&make_rocky_helmet_card()),
        "Tool should be in opponent's discard pile"
    );
}

#[test]
fn test_field_blower_discards_active_stadium() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    // Place a stadium
    let stadium_card = get_card_by_enum(CardId::B2155PeculiarPlaza);
    state.active_stadium = Some(stadium_card.clone());

    state.hands[0].clear();
    let trainer_card = make_field_blower_trainer_card();
    state.hands[0].push(Card::Trainer(trainer_card.clone()));
    game.set_state(state);

    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let state = game.get_state_clone();
    let (actor, choices) = state.generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(
        choices
            .iter()
            .any(|a| matches!(a.action, SimpleAction::DiscardActiveStadium)),
        "Should offer discarding the active stadium"
    );

    let discard_action = choices
        .iter()
        .find(|a| matches!(a.action, SimpleAction::DiscardActiveStadium))
        .unwrap()
        .clone();
    game.apply_action(&discard_action);

    let state = game.get_state_clone();
    assert!(state.active_stadium.is_none(), "Stadium should be gone");
    assert!(
        state.discard_piles[0].contains(&stadium_card),
        "Stadium should be in player's discard pile"
    );
}
