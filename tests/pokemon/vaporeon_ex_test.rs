use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::PlayedCard,
    test_support::get_initialized_game,
};

#[test]
fn test_vaporeon_ex_frozen_flow_forces_opponent_switch() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::B3037VaporeonEx)],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1002Ivysaur),
        ],
    );
    state.current_player = 0;
    state.turn_count = 3;
    game.set_state(state);

    let ability_action = Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    };
    game.apply_action(&ability_action);

    let state = game.get_state_clone();
    let (_actor, actions) = state.generate_possible_actions();
    assert!(actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::Activate { player: 1, .. })));
}

#[test]
fn test_vaporeon_ex_frozen_flow_not_available_from_bench() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::B3037VaporeonEx),
        ],
        vec![
            PlayedCard::from_id(CardId::A1002Ivysaur),
            PlayedCard::from_id(CardId::A1003Venusaur),
        ],
    );
    state.current_player = 0;
    state.turn_count = 3;
    game.set_state(state);

    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    assert!(!actions
        .iter()
        .any(|action| { matches!(action.action, SimpleAction::UseAbility { in_play_idx: 1 }) }));
}
