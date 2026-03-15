use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
};

mod common;

#[test]
fn test_darkrai_ex_nightmare_aura() {
    // Darkrai ex's Nightmare Aura: Whenever you attach a Darkness Energy from your Energy Zone to this Pokémon, do 20 damage to your opponent's Active Pokémon.

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up Darkrai ex vs Bulbasaur
    state.set_board(
        vec![PlayedCard::from_id(CardId::A2110DarkraiEx)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    game.set_state(state);

    // Attach Darkness energy from Energy Zone to Darkrai ex
    let attach_action = Action {
        actor: 0,
        action: SimpleAction::Attach {
            attachments: vec![(1, EnergyType::Darkness, 0)],
            is_turn_energy: true,
        },
        is_stack: false,
    };

    game.apply_action(&attach_action);
    let state = game.get_state_clone();

    // Check that Darkrai ex has the energy attached
    assert_eq!(
        state.get_active(0).attached_energy.len(),
        1,
        "Darkrai ex should have 1 energy attached"
    );

    // Check that opponent's active took 20 damage
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        50,
        "Opponent's active should have taken 20 damage (70 - 20 = 50)"
    );
}

#[test]
fn test_darkrai_ex_nightmare_aura_only_darkness() {
    // Test that non-Darkness energy doesn't trigger the ability

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A2110DarkraiEx)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    game.set_state(state);

    // Attach Fire energy from Energy Zone to Darkrai ex
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

    // Check that opponent's active did NOT take damage
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        70,
        "Opponent's active should not have taken damage from non-Darkness energy"
    );
}

#[test]
fn test_darkrai_ex_nightmare_aura_only_turn_energy() {
    // Test that the ability only triggers for energy from Energy Zone (is_turn_energy = true)

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A2110DarkraiEx)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    game.set_state(state);

    // Attach Darkness energy NOT from Energy Zone (is_turn_energy = false, e.g., from an ability)
    let attach_action = Action {
        actor: 0,
        action: SimpleAction::Attach {
            attachments: vec![(1, EnergyType::Darkness, 0)],
            is_turn_energy: false,
        },
        is_stack: false,
    };

    game.apply_action(&attach_action);
    let state = game.get_state_clone();

    // Check that opponent's active did NOT take damage
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        70,
        "Opponent's active should not have taken damage when energy is not from Energy Zone"
    );
}

#[test]
fn test_darkrai_ex_nightmare_aura_ko_triggers_promotion() {
    // If Nightmare Aura KOs the opponent's Active and they have a bench,
    // promotion should be queued.

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Darkrai ex vs Bulbasaur at 20 HP with a benched Pokemon
    state.set_board(
        vec![PlayedCard::from_id(CardId::A2110DarkraiEx)],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_remaining_hp(20),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );
    state.current_player = 0;
    game.set_state(state);

    // Attach Darkness energy from Energy Zone to Darkrai ex
    let attach_action = Action {
        actor: 0,
        action: SimpleAction::Attach {
            attachments: vec![(1, EnergyType::Darkness, 0)],
            is_turn_energy: true,
        },
        is_stack: false,
    };

    game.apply_action(&attach_action);
    let state = game.get_state_clone();

    // Promotion should be queued for the opponent (player 1)
    let (actor, actions) = state.generate_possible_actions();
    let has_promotion = actor == 1
        && actions
            .iter()
            .any(|a| matches!(a.action, SimpleAction::Activate { .. }));

    assert!(
        has_promotion,
        "Expected promotion to be queued after Darkrai Aura KOs active"
    );
}
