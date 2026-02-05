use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    tool_ids::ToolId,
};

mod common;

// ============================================================================
// Protective Poncho Tests
// ============================================================================

/// Test that Protective Poncho prevents damage to a benched Pokémon from an active attack
#[test]
fn test_protective_poncho_prevents_bench_damage_from_attack() {
    let opponent_card = get_card_by_enum(CardId::A1001Bulbasaur);

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    let attacker = state.current_player;
    let defender = (attacker + 1) % 2;

    // Set up attacker's active Pokémon with energy
    let attacker_pokemon = PlayedCard::new(
        get_card_by_enum(CardId::A1001Bulbasaur),
        70,
        70,
        vec![EnergyType::Grass, EnergyType::Colorless],
        false,
        vec![],
    );
    state.in_play_pokemon[attacker][0] = Some(attacker_pokemon);

    // Set up defender's active Pokémon
    let defender_active = PlayedCard::new(opponent_card.clone(), 70, 70, vec![], false, vec![]);
    state.in_play_pokemon[defender][0] = Some(defender_active);

    // Set up defender's benched Pokémon WITH Protective Poncho
    let mut benched_with_poncho =
        PlayedCard::new(opponent_card.clone(), 70, 70, vec![], false, vec![]);
    benched_with_poncho.attached_tool = Some(ToolId::B2147ProtectivePoncho);
    state.in_play_pokemon[defender][1] = Some(benched_with_poncho);

    // Set up defender's benched Pokémon WITHOUT Protective Poncho
    let benched_without_poncho =
        PlayedCard::new(opponent_card.clone(), 70, 70, vec![], false, vec![]);
    state.in_play_pokemon[defender][2] = Some(benched_without_poncho);

    state.move_generation_stack.clear();
    game.set_state(state);

    // Apply damage to the benched Pokémon with poncho using ApplyDamage
    let damage_action = Action {
        actor: attacker,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (attacker, 0),
            targets: vec![(30, defender, 1)],
            is_from_active_attack: true,
        },
        is_stack: false,
    };
    game.apply_action(&damage_action);

    let state = game.get_state_clone();

    // Benched Pokémon with Protective Poncho should NOT have taken any damage
    let poncho_pokemon_hp = state.in_play_pokemon[defender][1]
        .as_ref()
        .unwrap()
        .remaining_hp;
    assert_eq!(
        poncho_pokemon_hp, 70,
        "Benched Pokémon with Protective Poncho should take 0 damage from attacks"
    );
}

/// Test that Protective Poncho prevents damage from Greninja's Water Shuriken ability
#[test]
fn test_protective_poncho_prevents_greninja_water_shuriken() {
    let greninja_card = get_card_by_enum(CardId::A1089Greninja);
    let bulbasaur_card = get_card_by_enum(CardId::A1001Bulbasaur);

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    let greninja_player = state.current_player;
    let defender = (greninja_player + 1) % 2;

    // Set up Greninja on the bench (position 1) — ability can be used from bench or active
    let greninja = PlayedCard::new(greninja_card.clone(), 120, 120, vec![], false, vec![]);
    state.in_play_pokemon[greninja_player][1] = Some(greninja);

    // Set up a basic active for the Greninja player
    let active = PlayedCard::new(bulbasaur_card.clone(), 70, 70, vec![], false, vec![]);
    state.in_play_pokemon[greninja_player][0] = Some(active);

    // Set up defender's active Pokémon
    let defender_active = PlayedCard::new(bulbasaur_card.clone(), 70, 70, vec![], false, vec![]);
    state.in_play_pokemon[defender][0] = Some(defender_active);

    // Set up defender's benched Pokémon WITH Protective Poncho
    let mut benched_with_poncho =
        PlayedCard::new(bulbasaur_card.clone(), 70, 70, vec![], false, vec![]);
    benched_with_poncho.attached_tool = Some(ToolId::B2147ProtectivePoncho);
    state.in_play_pokemon[defender][1] = Some(benched_with_poncho);

    state.move_generation_stack.clear();
    game.set_state(state);

    // Use Greninja's Water Shuriken ability (from bench position 1)
    let ability_action = Action {
        actor: greninja_player,
        action: SimpleAction::UseAbility { in_play_idx: 1 },
        is_stack: false,
    };
    game.apply_action(&ability_action);

    // The ability queues a move generation stack for choosing the target.
    // Choose to target the benched Pokémon with Protective Poncho (defender, position 1).
    let target_action = Action {
        actor: greninja_player,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (greninja_player, 1),
            targets: vec![(20, defender, 1)],
            is_from_active_attack: false,
        },
        is_stack: false,
    };
    game.apply_action(&target_action);

    let final_state = game.get_state_clone();

    // Benched Pokémon with Protective Poncho should NOT have taken any damage
    let poncho_pokemon_hp = final_state.in_play_pokemon[defender][1]
        .as_ref()
        .unwrap()
        .remaining_hp;
    assert_eq!(
        poncho_pokemon_hp, 70,
        "Benched Pokémon with Protective Poncho should take 0 damage from Water Shuriken ability"
    );
}

/// Test that Protective Poncho does NOT prevent damage when the Pokémon is in the active spot
#[test]
fn test_protective_poncho_no_protection_when_active() {
    let bulbasaur_card = get_card_by_enum(CardId::A1001Bulbasaur);

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    let attacker = state.current_player;
    let defender = (attacker + 1) % 2;

    // Set up attacker's active Pokémon with energy for Vine Whip (40 damage)
    let attacker_pokemon = PlayedCard::new(
        bulbasaur_card.clone(),
        70,
        70,
        vec![EnergyType::Grass, EnergyType::Colorless],
        false,
        vec![],
    );
    state.in_play_pokemon[attacker][0] = Some(attacker_pokemon);

    // Set up defender's ACTIVE Pokémon with Protective Poncho (should NOT protect in active)
    let mut active_with_poncho =
        PlayedCard::new(bulbasaur_card.clone(), 70, 70, vec![], false, vec![]);
    active_with_poncho.attached_tool = Some(ToolId::B2147ProtectivePoncho);
    state.in_play_pokemon[defender][0] = Some(active_with_poncho);

    // Add a bench Pokémon so game doesn't end if KO
    let bench = PlayedCard::new(bulbasaur_card.clone(), 70, 70, vec![], false, vec![]);
    state.in_play_pokemon[defender][1] = Some(bench);

    state.move_generation_stack.clear();
    game.set_state(state);

    // Attack with Vine Whip (40 damage)
    let attack_action = Action {
        actor: attacker,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // Active Pokémon with Protective Poncho SHOULD take damage (poncho only protects bench)
    let active_hp = final_state.in_play_pokemon[defender][0]
        .as_ref()
        .unwrap()
        .remaining_hp;
    assert_eq!(
        active_hp, 30,
        "Active Pokémon with Protective Poncho should still take damage (70 - 40 = 30)"
    );
}
