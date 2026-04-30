use crate::deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::PlayedCard,
    test_support::get_test_game_with_board,
};

#[test]
fn test_vaporeon_ex_frozen_flow_forces_opponent_switch() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B3037VaporeonEx)],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1002Ivysaur),
        ],
    );

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
    let game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::B3037VaporeonEx),
        ],
        vec![
            PlayedCard::from_id(CardId::A1002Ivysaur),
            PlayedCard::from_id(CardId::A1003Venusaur),
        ],
    );

    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    assert!(!actions
        .iter()
        .any(|action| { matches!(action.action, SimpleAction::UseAbility { in_play_idx: 1 }) }));
}
