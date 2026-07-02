use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, PlayedCard},
    test_support::get_initialized_game,
};

fn make_trainer_card(card_id: CardId) -> deckgym::models::TrainerCard {
    get_card_by_enum(card_id).as_trainer()
}

#[test]
fn test_elesa_returns_all_tools_to_owners_hand() {
    // Elesa: "Return all Pokémon Tools attached to each Pokémon (both yours and your
    // opponent's) to their owner's hand."
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_tool(get_card_by_enum(CardId::A2148RockyHelmet)),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)
            .with_tool(get_card_by_enum(CardId::A2148RockyHelmet))],
    );

    let elesa = make_trainer_card(CardId::B3b066Elesa);
    state.hands[0] = vec![Card::Trainer(elesa.clone())];
    game.set_state(state);

    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: elesa,
        },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let state = game.get_state_clone();

    let player_active = state.in_play_pokemon[0][0]
        .as_ref()
        .expect("Player active should remain");
    assert!(
        player_active.attached_tool.is_none(),
        "Rocky Helmet should have been detached from player's Bulbasaur"
    );
    let opponent_active = state.in_play_pokemon[1][0]
        .as_ref()
        .expect("Opponent active should remain");
    assert!(
        opponent_active.attached_tool.is_none(),
        "Rocky Helmet should have been detached from opponent's Charmander"
    );

    let player_hand_tool_count = state.hands[0]
        .iter()
        .filter(|c| matches!(c, Card::Trainer(tc) if tc.id == "A2 148"))
        .count();
    assert_eq!(
        player_hand_tool_count, 1,
        "Player should get their Rocky Helmet back in hand"
    );

    let opponent_hand_tool_count = state.hands[1]
        .iter()
        .filter(|c| matches!(c, Card::Trainer(tc) if tc.id == "A2 148"))
        .count();
    assert_eq!(
        opponent_hand_tool_count, 1,
        "Opponent should get their Rocky Helmet back in hand"
    );
}

#[test]
fn test_elesa_no_tools_attached_does_nothing() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );

    let elesa = make_trainer_card(CardId::B3b066Elesa);
    state.hands[0] = vec![Card::Trainer(elesa.clone())];
    game.set_state(state);

    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: elesa,
        },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let state = game.get_state_clone();
    assert!(state.hands[0].is_empty(), "No tools should return to hand");
    assert!(
        !state.hands[1].iter().any(|c| matches!(c, Card::Trainer(tc) if tc.trainer_card_type == deckgym::models::TrainerType::Tool)),
        "No tools should return to hand"
    );
}
