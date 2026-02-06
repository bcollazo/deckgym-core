use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
};

mod common;

#[test]
fn test_jolteon_ex_electromagnetic_wall_ko_triggers_promotion() {
    // If Electromagnetic Wall KOs the opponent's Active and they have a bench,
    // promotion should be queued.

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Player 0 with weak active + bench vs Jolteon ex
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_hp(20),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::B1081JolteonEx)],
    );
    state.current_player = 0;
    game.set_state(state);

    // Actor attaches energy from Energy Zone to their active
    let attach_action = Action {
        actor: 0,
        action: SimpleAction::Attach {
            attachments: vec![(1, EnergyType::Fire, 0)],
            is_turn_energy: true,
        },
        is_stack: false,
    };

    game.apply_action(&attach_action);

    let state = game.get_state_clone();

    // Promotion should be queued for player 0
    let (actor, actions) = state.generate_possible_actions();
    let has_promotion = actor == 0
        && actions
            .iter()
            .any(|a| matches!(a.action, SimpleAction::Activate { .. }));

    assert!(
        has_promotion,
        "Expected promotion to be queued after Electromagnetic Wall KOs active"
    );
}
