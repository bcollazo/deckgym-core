use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    effects::CardEffect,
    models::{EnergyType, PlayedCard},
};

mod common;

/// Test Shinx's Hide prevents damage on successful coin flip (heads)
#[test]
fn test_shinx_hide_damage_prevention() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up Shinx vs Bulbasaur
    state.set_board(
        vec![PlayedCard::from_id(CardId::A2058Shinx).with_energy(vec![EnergyType::Lightning])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
    );
    state.current_player = 0;

    game.set_state(state);

    // Manually add the PreventAllDamageAndEffects effect to simulate successful Hide
    let mut state = game.get_state_clone();
    state.in_play_pokemon[0][0]
        .as_mut()
        .unwrap()
        .add_effect(CardEffect::PreventAllDamageAndEffects, 1);
    game.set_state(state);

    // Switch turns to opponent
    let mut state = game.get_state_clone();
    state.current_player = 1;
    game.set_state(state);

    // Opponent attacks Shinx with Vine Whip (40 damage)
    let attack_action = Action {
        actor: 1,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // Shinx should still have full HP due to PreventAllDamageAndEffects
    let shinx_hp = final_state.get_active(0).get_remaining_hp();

    assert_eq!(
        shinx_hp, 60,
        "Shinx should take 0 damage when protected by Hide effect"
    );
}

/// Test Shinx's Hide prevents status effects (like Poison)
#[test]
fn test_shinx_hide_effect_prevention() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up Shinx with PreventAllDamageAndEffects vs Weezing
    let mut shinx =
        PlayedCard::from_id(CardId::A2058Shinx).with_energy(vec![EnergyType::Lightning]);
    shinx.add_effect(CardEffect::PreventAllDamageAndEffects, 1);

    state.set_board(
        vec![shinx],
        vec![PlayedCard::from_id(CardId::A1177Weezing)
            .with_energy(vec![EnergyType::Darkness, EnergyType::Colorless])],
    );
    state.current_player = 1;

    game.set_state(state);

    // Opponent uses Weezing's attack (Tackle: 50 damage)
    let attack_action = Action {
        actor: 1,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // Shinx should still have full HP
    let shinx_hp = final_state.get_active(0).get_remaining_hp();
    assert_eq!(
        shinx_hp, 60,
        "Shinx should not take damage when protected by Hide"
    );

    // Shinx should NOT be poisoned (effect prevented)
    let shinx_poisoned = final_state.get_active(0).poisoned;
    assert!(
        !shinx_poisoned,
        "Shinx should not be poisoned when protected by Hide"
    );
}
