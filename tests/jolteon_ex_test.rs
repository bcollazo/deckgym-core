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

    let actor = state.current_player;
    let opponent = (actor + 1) % 2;

    // Opponent has Jolteon ex active
    state.in_play_pokemon[opponent][0] = Some(PlayedCard::from_id(CardId::B1081JolteonEx));

    // Actor's active at 20 HP so Electromagnetic Wall KOs it
    state.in_play_pokemon[actor][0] = Some(PlayedCard::from_id(CardId::A1001Bulbasaur).with_hp(20));
    // Actor has a benched Pokemon to promote
    state.in_play_pokemon[actor][1] = Some(PlayedCard::from_id(CardId::A1001Bulbasaur));

    state.move_generation_stack.clear();
    game.set_state(state);

    // Actor attaches energy from Energy Zone to their active
    let attach_action = Action {
        actor,
        action: SimpleAction::Attach {
            attachments: vec![(1, EnergyType::Fire, 0)],
            is_turn_energy: true,
        },
        is_stack: false,
    };

    game.apply_action(&attach_action);

    let state = game.get_state_clone();

    // Promotion should be queued for the actor
    let has_promotion = state.move_generation_stack.iter().any(|(player, actions)| {
        *player == actor
            && actions
                .iter()
                .any(|a| matches!(a, SimpleAction::Activate { .. }))
    });

    assert!(
        has_promotion,
        "Expected promotion to be queued after Electromagnetic Wall KOs active"
    );
}
