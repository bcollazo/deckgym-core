use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
};

mod common;

// ============================================================================
// Protective Poncho Tests
// ============================================================================

/// Test that Protective Poncho prevents damage to a benched Pokémon from an active attack
#[test]
fn test_protective_poncho_prevents_bench_damage_from_attack() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up attacker vs defender with poncho on bench
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_tool(get_card_by_enum(CardId::B2147ProtectivePoncho)),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );
    state.current_player = 0;
    game.set_state(state);

    // Apply damage to the benched Pokémon with poncho using ApplyDamage
    let damage_action = Action {
        actor: 0,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (0, 0),
            targets: vec![(30, 1, 1)],
            is_from_active_attack: true,
        },
        is_stack: false,
    };
    game.apply_action(&damage_action);

    let state = game.get_state_clone();

    // Benched Pokémon with Protective Poncho should NOT have taken any damage
    let poncho_pokemon_hp = state
        .enumerate_bench_pokemon(1)
        .next()
        .unwrap()
        .1
        .remaining_hp;
    assert_eq!(
        poncho_pokemon_hp, 70,
        "Benched Pokémon with Protective Poncho should take 0 damage from attacks"
    );
}

/// Test that Protective Poncho prevents damage from Greninja's Water Shuriken ability
#[test]
fn test_protective_poncho_prevents_greninja_water_shuriken() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Greninja player vs defender with poncho on bench
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1089Greninja),
        ],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_tool(get_card_by_enum(CardId::B2147ProtectivePoncho)),
        ],
    );
    state.current_player = 0;
    game.set_state(state);

    // Use Greninja's Water Shuriken ability (from bench position 1)
    let ability_action = Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 1 },
        is_stack: false,
    };
    game.apply_action(&ability_action);

    // The ability queues a move generation stack for choosing the target.
    // Choose to target the benched Pokémon with Protective Poncho (player 1, position 1).
    let target_action = Action {
        actor: 0,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (0, 1),
            targets: vec![(20, 1, 1)],
            is_from_active_attack: false,
        },
        is_stack: false,
    };
    game.apply_action(&target_action);

    let final_state = game.get_state_clone();

    // Benched Pokémon with Protective Poncho should NOT have taken any damage
    let poncho_pokemon_hp = final_state
        .enumerate_bench_pokemon(1)
        .next()
        .unwrap()
        .1
        .remaining_hp;
    assert_eq!(
        poncho_pokemon_hp, 70,
        "Benched Pokémon with Protective Poncho should take 0 damage from Water Shuriken ability"
    );
}

/// Test that Protective Poncho does NOT prevent damage when the Pokémon is in the active spot
#[test]
fn test_protective_poncho_no_protection_when_active() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Attacker vs defender with poncho on ACTIVE (should NOT protect)
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_tool(get_card_by_enum(CardId::B2147ProtectivePoncho)),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );
    state.current_player = 0;
    game.set_state(state);

    // Attack with Vine Whip (40 damage)
    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // Active Pokémon with Protective Poncho SHOULD take damage (poncho only protects bench)
    let active_hp = final_state.get_active(1).remaining_hp;
    assert_eq!(
        active_hp, 30,
        "Active Pokémon with Protective Poncho should still take damage (70 - 40 = 30)"
    );
}
