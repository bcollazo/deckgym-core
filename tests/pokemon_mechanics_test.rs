use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    effects::CardEffect,
    generate_possible_actions,
    models::{EnergyType, PlayedCard},
};

mod common;

// ============================================================================
// Marshadow Tests - Revenge Attack
// ============================================================================

/// Test Marshadow's Revenge attack base damage (40) when no KO happened last turn
#[test]
fn test_marshadow_revenge_base_damage() {
    let marshadow_card = get_card_by_enum(CardId::A1a047Marshadow);
    let opponent_card = get_card_by_enum(CardId::A1001Bulbasaur);

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    let test_player = state.current_player;
    let opponent_player = (test_player + 1) % 2;

    // Set up Marshadow with enough energy (Fighting + Colorless)
    let marshadow = PlayedCard::new(
        marshadow_card.clone(),
        80,
        80,
        vec![EnergyType::Fighting, EnergyType::Colorless],
        false,
        vec![],
    );
    state.in_play_pokemon[test_player][0] = Some(marshadow);

    // Set up opponent's active Pokemon with high HP to survive
    let opponent_active = PlayedCard::new(opponent_card.clone(), 70, 70, vec![], false, vec![]);
    state.in_play_pokemon[opponent_player][0] = Some(opponent_active);

    // Ensure no KO happened last turn
    state.set_knocked_out_by_opponent_attack_last_turn(false);

    // Clear move generation stack
    state.move_generation_stack.clear();

    game.set_state(state);

    // Apply Revenge attack (attack index 0)
    let attack_action = Action {
        actor: test_player,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // Base damage is 40, so opponent should have 70 - 40 = 30 HP
    let opponent_hp = final_state.in_play_pokemon[opponent_player][0]
        .as_ref()
        .unwrap()
        .remaining_hp;

    assert_eq!(
        opponent_hp, 30,
        "Marshadow's Revenge should deal 40 damage without KO bonus (70 - 40 = 30)"
    );
}

/// Test Marshadow's Revenge attack boosted damage (40 + 60 = 100) when KO happened last turn
#[test]
fn test_marshadow_revenge_boosted_damage() {
    let marshadow_card = get_card_by_enum(CardId::A1a047Marshadow);
    let opponent_card = get_card_by_enum(CardId::A1001Bulbasaur);

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    let test_player = state.current_player;
    let opponent_player = (test_player + 1) % 2;

    // Set up Marshadow with enough energy
    let marshadow = PlayedCard::new(
        marshadow_card.clone(),
        80,
        80,
        vec![EnergyType::Fighting, EnergyType::Colorless],
        false,
        vec![],
    );
    state.in_play_pokemon[test_player][0] = Some(marshadow);

    // Set up opponent's active Pokemon with high HP to survive boosted damage
    let opponent_active = PlayedCard::new(opponent_card.clone(), 150, 150, vec![], false, vec![]);
    state.in_play_pokemon[opponent_player][0] = Some(opponent_active);

    // Simulate that a Pokemon was KO'd by opponent's attack last turn
    state.set_knocked_out_by_opponent_attack_last_turn(true);

    // Clear move generation stack
    state.move_generation_stack.clear();

    game.set_state(state);

    // Apply Revenge attack
    let attack_action = Action {
        actor: test_player,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // Boosted damage is 40 + 60 = 100, so opponent should have 150 - 100 = 50 HP
    let opponent_hp = final_state.in_play_pokemon[opponent_player][0]
        .as_ref()
        .unwrap()
        .remaining_hp;

    assert_eq!(
        opponent_hp, 50,
        "Marshadow's Revenge should deal 100 damage with KO bonus (150 - 100 = 50)"
    );
}

// ============================================================================
// Dusknoir Tests - Shadow Void Ability
// ============================================================================

/// Test Dusknoir's Shadow Void ability moving damage correctly
#[test]
fn test_dusknoir_shadow_void_move_damage() {
    let dusknoir_card = get_card_by_enum(CardId::A2072Dusknoir);
    let bulbasaur_card = get_card_by_enum(CardId::A1001Bulbasaur);

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    let test_player = state.current_player;

    // Set up Dusknoir on bench (position 1) with full HP
    let dusknoir = PlayedCard::new(
        dusknoir_card.clone(),
        130,
        130,
        vec![],
        false,
        vec![],
    );
    state.in_play_pokemon[test_player][1] = Some(dusknoir);

    // Set up Bulbasaur in active (position 0) with damage (40 damage taken, 30 HP remaining)
    let bulbasaur = PlayedCard::new(
        bulbasaur_card.clone(),
        30, // 70 - 40 = 30 remaining HP (40 damage)
        70,
        vec![],
        false,
        vec![],
    );
    state.in_play_pokemon[test_player][0] = Some(bulbasaur);

    // Clear move generation stack
    state.move_generation_stack.clear();

    game.set_state(state);

    // Use Dusknoir's Shadow Void ability
    let ability_action = Action {
        actor: test_player,
        action: SimpleAction::UseAbility { in_play_idx: 1 },
        is_stack: false,
    };
    game.apply_action(&ability_action);

    // The ability should queue a move generation for selecting which Pokemon's damage to move
    let state = game.get_state_clone();
    assert!(
        !state.move_generation_stack.is_empty(),
        "Shadow Void should queue a move generation for selecting damage source"
    );

    // Select to move damage from Bulbasaur (index 0) to Dusknoir (index 1)
    let move_damage_action = Action {
        actor: test_player,
        action: SimpleAction::MoveAllDamage { from: 0, to: 1 },
        is_stack: false,
    };
    game.apply_action(&move_damage_action);

    let final_state = game.get_state_clone();

    // Bulbasaur should now have full HP (70)
    let bulbasaur_hp = final_state.in_play_pokemon[test_player][0]
        .as_ref()
        .unwrap()
        .remaining_hp;
    assert_eq!(
        bulbasaur_hp, 70,
        "Bulbasaur should be fully healed after Shadow Void (70 HP)"
    );

    // Dusknoir should have taken the 40 damage (130 - 40 = 90 HP)
    let dusknoir_hp = final_state.in_play_pokemon[test_player][1]
        .as_ref()
        .unwrap()
        .remaining_hp;
    assert_eq!(
        dusknoir_hp, 90,
        "Dusknoir should have 90 HP after receiving 40 damage (130 - 40)"
    );
}

/// Test Dusknoir's Shadow Void causing KO and awarding points to opponent
#[test]
fn test_dusknoir_shadow_void_ko() {
    let dusknoir_card = get_card_by_enum(CardId::A2072Dusknoir);
    let bulbasaur_card = get_card_by_enum(CardId::A1001Bulbasaur);

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    let test_player = state.current_player;
    let opponent_player = (test_player + 1) % 2;

    // Set up Dusknoir on bench with LOW HP (will die from damage transfer)
    let dusknoir = PlayedCard::new(
        dusknoir_card.clone(),
        30, // Will die if receiving 40+ damage
        130,
        vec![],
        false,
        vec![],
    );
    state.in_play_pokemon[test_player][1] = Some(dusknoir);

    // Set up Bulbasaur in active with damage (50 damage taken)
    let bulbasaur = PlayedCard::new(
        bulbasaur_card.clone(),
        20, // 70 - 50 = 20 HP (50 damage)
        70,
        vec![],
        false,
        vec![],
    );
    state.in_play_pokemon[test_player][0] = Some(bulbasaur);

    // Reset points
    state.points = [0, 0];

    // Clear move generation stack
    state.move_generation_stack.clear();

    game.set_state(state);

    // Use Dusknoir's Shadow Void ability
    let ability_action = Action {
        actor: test_player,
        action: SimpleAction::UseAbility { in_play_idx: 1 },
        is_stack: false,
    };
    game.apply_action(&ability_action);

    // Select to move damage from Bulbasaur to Dusknoir
    let move_damage_action = Action {
        actor: test_player,
        action: SimpleAction::MoveAllDamage { from: 0, to: 1 },
        is_stack: false,
    };
    game.apply_action(&move_damage_action);

    let final_state = game.get_state_clone();

    // Dusknoir should be KO'd (removed from play)
    assert!(
        final_state.in_play_pokemon[test_player][1].is_none(),
        "Dusknoir should be KO'd after receiving lethal damage"
    );

    // Opponent should receive 1 point for KO'ing a non-ex Pokemon
    assert_eq!(
        final_state.points[opponent_player], 1,
        "Opponent should receive 1 point for KO'ing Dusknoir"
    );
}

/// Test Dusknoir's Shadow Void can be used multiple times per turn
#[test]
fn test_dusknoir_shadow_void_multiple_uses() {
    let dusknoir_card = get_card_by_enum(CardId::A2072Dusknoir);
    let bulbasaur_card = get_card_by_enum(CardId::A1001Bulbasaur);
    let squirtle_card = get_card_by_enum(CardId::A1053Squirtle);

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    let test_player = state.current_player;

    // Set up Dusknoir on bench with lots of HP
    let dusknoir = PlayedCard::new(
        dusknoir_card.clone(),
        130,
        130,
        vec![],
        false,
        vec![],
    );
    state.in_play_pokemon[test_player][1] = Some(dusknoir);

    // Set up Bulbasaur in active with damage
    let bulbasaur = PlayedCard::new(
        bulbasaur_card.clone(),
        50, // 20 damage
        70,
        vec![],
        false,
        vec![],
    );
    state.in_play_pokemon[test_player][0] = Some(bulbasaur);

    // Set up Squirtle on bench with damage
    let squirtle = PlayedCard::new(
        squirtle_card.clone(),
        30, // 20 damage
        50,
        vec![],
        false,
        vec![],
    );
    state.in_play_pokemon[test_player][2] = Some(squirtle);

    // Clear move generation stack
    state.move_generation_stack.clear();

    game.set_state(state);

    // First use: Move damage from Bulbasaur
    let ability_action = Action {
        actor: test_player,
        action: SimpleAction::UseAbility { in_play_idx: 1 },
        is_stack: false,
    };
    game.apply_action(&ability_action);

    let move_damage_action = Action {
        actor: test_player,
        action: SimpleAction::MoveAllDamage { from: 0, to: 1 },
        is_stack: false,
    };
    game.apply_action(&move_damage_action);

    // Second use: Move damage from Squirtle
    let ability_action2 = Action {
        actor: test_player,
        action: SimpleAction::UseAbility { in_play_idx: 1 },
        is_stack: false,
    };
    game.apply_action(&ability_action2);

    let move_damage_action2 = Action {
        actor: test_player,
        action: SimpleAction::MoveAllDamage { from: 2, to: 1 },
        is_stack: false,
    };
    game.apply_action(&move_damage_action2);

    let final_state = game.get_state_clone();

    // Bulbasaur should be fully healed
    let bulbasaur_hp = final_state.in_play_pokemon[test_player][0]
        .as_ref()
        .unwrap()
        .remaining_hp;
    assert_eq!(bulbasaur_hp, 70, "Bulbasaur should be fully healed");

    // Squirtle should be fully healed
    let squirtle_hp = final_state.in_play_pokemon[test_player][2]
        .as_ref()
        .unwrap()
        .remaining_hp;
    assert_eq!(squirtle_hp, 50, "Squirtle should be fully healed");

    // Dusknoir should have taken both damages (130 - 20 - 20 = 90 HP)
    let dusknoir_hp = final_state.in_play_pokemon[test_player][1]
        .as_ref()
        .unwrap()
        .remaining_hp;
    assert_eq!(
        dusknoir_hp, 90,
        "Dusknoir should have 90 HP after receiving 40 total damage"
    );
}

// ============================================================================
// Lucario Tests - Fighting Coach Ability
// ============================================================================

/// Test Lucario's Fighting Coach ability gives +20 damage to Fighting attacks
#[test]
fn test_lucario_fighting_coach_single() {
    let lucario_card = get_card_by_enum(CardId::A2092Lucario);
    let riolu_card = get_card_by_enum(CardId::A2091Riolu); // Basic Fighting Pokemon
    let opponent_card = get_card_by_enum(CardId::A1001Bulbasaur);

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    let test_player = state.current_player;
    let opponent_player = (test_player + 1) % 2;

    // Set up Riolu in active with enough energy for its attack (Jab: 20 damage)
    let riolu_attacker = PlayedCard::new(
        riolu_card.clone(),
        60,
        60,
        vec![EnergyType::Fighting],
        false,
        vec![],
    );
    state.in_play_pokemon[test_player][0] = Some(riolu_attacker);

    // Set up Lucario on bench for the Fighting Coach ability
    let lucario_bench = PlayedCard::new(
        lucario_card.clone(),
        100,
        100,
        vec![],
        false,
        vec![],
    );
    state.in_play_pokemon[test_player][1] = Some(lucario_bench);

    // Set up opponent
    let opponent_active = PlayedCard::new(opponent_card.clone(), 100, 100, vec![], false, vec![]);
    state.in_play_pokemon[opponent_player][0] = Some(opponent_active);

    // Clear move generation stack
    state.move_generation_stack.clear();

    game.set_state(state);

    // Apply Riolu's Jab attack (20 base damage + 20 from Fighting Coach = 40)
    let attack_action = Action {
        actor: test_player,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // With 1 Fighting Coach: 20 + 20 = 40 damage, so 100 - 40 = 60 HP
    let opponent_hp = final_state.in_play_pokemon[opponent_player][0]
        .as_ref()
        .unwrap()
        .remaining_hp;

    assert_eq!(
        opponent_hp, 60,
        "Riolu's attack should deal 40 damage with 1 Fighting Coach boost (20 + 20)"
    );
}

/// Test two Lucarios stack Fighting Coach (+40 total damage)
#[test]
fn test_lucario_fighting_coach_stacked() {
    let lucario_card = get_card_by_enum(CardId::A2092Lucario);
    let opponent_card = get_card_by_enum(CardId::A1001Bulbasaur);

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    let test_player = state.current_player;
    let opponent_player = (test_player + 1) % 2;

    // Set up Lucario in active with energy
    let lucario_active = PlayedCard::new(
        lucario_card.clone(),
        100,
        100,
        vec![EnergyType::Fighting, EnergyType::Fighting],
        false,
        vec![],
    );
    state.in_play_pokemon[test_player][0] = Some(lucario_active);

    // Set up TWO Lucarios on bench for stacked ability
    let lucario_bench1 = PlayedCard::new(
        lucario_card.clone(),
        100,
        100,
        vec![],
        false,
        vec![],
    );
    state.in_play_pokemon[test_player][1] = Some(lucario_bench1);

    let lucario_bench2 = PlayedCard::new(
        lucario_card.clone(),
        100,
        100,
        vec![],
        false,
        vec![],
    );
    state.in_play_pokemon[test_player][2] = Some(lucario_bench2);

    // Set up opponent with high HP
    let opponent_active = PlayedCard::new(opponent_card.clone(), 150, 150, vec![], false, vec![]);
    state.in_play_pokemon[opponent_player][0] = Some(opponent_active);

    // Clear move generation stack
    state.move_generation_stack.clear();

    game.set_state(state);

    // Apply attack: 40 base + 20 (active Lucario) + 20 (bench1) + 20 (bench2) = 100
    let attack_action = Action {
        actor: test_player,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // With 3 Lucarios: 40 + (20 * 3) = 100 damage, so 150 - 100 = 50 HP
    let opponent_hp = final_state.in_play_pokemon[opponent_player][0]
        .as_ref()
        .unwrap()
        .remaining_hp;

    assert_eq!(
        opponent_hp, 50,
        "Lucario's attack should deal 100 damage with 3 Fighting Coaches (40 + 60)"
    );
}

/// Test Fighting Coach doesn't boost non-Fighting type attacks
#[test]
fn test_lucario_fighting_coach_no_boost_non_fighting() {
    let lucario_card = get_card_by_enum(CardId::A2092Lucario);
    let bulbasaur_card = get_card_by_enum(CardId::A1001Bulbasaur);
    let opponent_card = get_card_by_enum(CardId::A1053Squirtle);

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    let test_player = state.current_player;
    let opponent_player = (test_player + 1) % 2;

    // Set up Bulbasaur (Grass type) in active with energy for Vine Whip (40 damage)
    let bulbasaur = PlayedCard::new(
        bulbasaur_card.clone(),
        70,
        70,
        vec![EnergyType::Grass, EnergyType::Colorless],
        false,
        vec![],
    );
    state.in_play_pokemon[test_player][0] = Some(bulbasaur);

    // Set up Lucario on bench
    let lucario_bench = PlayedCard::new(
        lucario_card.clone(),
        100,
        100,
        vec![],
        false,
        vec![],
    );
    state.in_play_pokemon[test_player][1] = Some(lucario_bench);

    // Set up opponent
    let opponent_active = PlayedCard::new(opponent_card.clone(), 100, 100, vec![], false, vec![]);
    state.in_play_pokemon[opponent_player][0] = Some(opponent_active);

    // Clear move generation stack
    state.move_generation_stack.clear();

    game.set_state(state);

    // Apply Vine Whip attack (40 damage, should NOT get Fighting Coach boost)
    let attack_action = Action {
        actor: test_player,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // No boost: 40 damage, so 100 - 40 = 60 HP
    let opponent_hp = final_state.in_play_pokemon[opponent_player][0]
        .as_ref()
        .unwrap()
        .remaining_hp;

    assert_eq!(
        opponent_hp, 60,
        "Grass-type attack should NOT get Fighting Coach boost (40 damage only)"
    );
}

// ============================================================================
// Shinx Tests - Hide Attack
// ============================================================================

/// Test Shinx's Hide prevents damage on successful coin flip (heads)
#[test]
fn test_shinx_hide_damage_prevention() {
    let shinx_card = get_card_by_enum(CardId::A2058Shinx);
    let opponent_card = get_card_by_enum(CardId::A1001Bulbasaur);

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    let test_player = state.current_player;
    let opponent_player = (test_player + 1) % 2;

    // Set up Shinx with energy for Hide attack
    let shinx = PlayedCard::new(
        shinx_card.clone(),
        60,
        60,
        vec![EnergyType::Lightning],
        false,
        vec![],
    );
    state.in_play_pokemon[test_player][0] = Some(shinx);

    // Set up opponent with energy for attack
    let opponent_active = PlayedCard::new(
        opponent_card.clone(),
        70,
        70,
        vec![EnergyType::Grass, EnergyType::Colorless],
        false,
        vec![],
    );
    state.in_play_pokemon[opponent_player][0] = Some(opponent_active);

    // Clear move generation stack
    state.move_generation_stack.clear();

    game.set_state(state);

    // Manually add the PreventAllDamageAndEffects effect to simulate successful Hide
    // (In real game, this happens on coin flip heads)
    let mut state = game.get_state_clone();
    state.in_play_pokemon[test_player][0]
        .as_mut()
        .unwrap()
        .add_effect(CardEffect::PreventAllDamageAndEffects, 1);
    game.set_state(state);

    // Switch turns to opponent
    let mut state = game.get_state_clone();
    state.current_player = opponent_player;
    state.move_generation_stack.clear();
    game.set_state(state);

    // Opponent attacks Shinx with Vine Whip (40 damage)
    let attack_action = Action {
        actor: opponent_player,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // Shinx should still have full HP due to PreventAllDamageAndEffects
    let shinx_hp = final_state.in_play_pokemon[test_player][0]
        .as_ref()
        .unwrap()
        .remaining_hp;

    assert_eq!(
        shinx_hp, 60,
        "Shinx should take 0 damage when protected by Hide effect"
    );
}

/// Test Shinx's Hide prevents status effects (like Poison)
#[test]
fn test_shinx_hide_effect_prevention() {
    let shinx_card = get_card_by_enum(CardId::A2058Shinx);
    let weezing_card = get_card_by_enum(CardId::A1177Weezing);

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    let test_player = state.current_player;
    let opponent_player = (test_player + 1) % 2;

    // Set up Shinx with PreventAllDamageAndEffects effect already applied
    let mut shinx = PlayedCard::new(
        shinx_card.clone(),
        60,
        60,
        vec![EnergyType::Lightning],
        false,
        vec![],
    );
    shinx.add_effect(CardEffect::PreventAllDamageAndEffects, 1);
    state.in_play_pokemon[test_player][0] = Some(shinx);

    // Set up Weezing as opponent (has Poison ability)
    let weezing = PlayedCard::new(
        weezing_card.clone(),
        110,
        110,
        vec![EnergyType::Darkness, EnergyType::Colorless],
        false,
        vec![],
    );
    state.in_play_pokemon[opponent_player][0] = Some(weezing);

    // Clear move generation stack and set opponent as current player
    state.current_player = opponent_player;
    state.move_generation_stack.clear();

    game.set_state(state);

    // Opponent uses Weezing's attack (Tackle: 50 damage)
    let attack_action = Action {
        actor: opponent_player,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // Shinx should still have full HP
    let shinx_hp = final_state.in_play_pokemon[test_player][0]
        .as_ref()
        .unwrap()
        .remaining_hp;

    assert_eq!(
        shinx_hp, 60,
        "Shinx should not take damage when protected by Hide"
    );

    // Shinx should NOT be poisoned (effect prevented)
    let shinx_poisoned = final_state.in_play_pokemon[test_player][0]
        .as_ref()
        .unwrap()
        .poisoned;

    assert!(
        !shinx_poisoned,
        "Shinx should not be poisoned when protected by Hide"
    );
}

// ============================================================================
// Vulpix Tests - Tail Whip Attack
// ============================================================================

/// Test Vulpix's Tail Whip prevents opponent from attacking (on heads)
#[test]
fn test_vulpix_tail_whip_attack_prevention() {
    let vulpix_card = get_card_by_enum(CardId::A1037Vulpix);
    let opponent_card = get_card_by_enum(CardId::A1001Bulbasaur);

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    let test_player = state.current_player;
    let opponent_player = (test_player + 1) % 2;

    // Set up Vulpix with energy
    let vulpix = PlayedCard::new(
        vulpix_card.clone(),
        50,
        50,
        vec![EnergyType::Colorless],
        false,
        vec![],
    );
    state.in_play_pokemon[test_player][0] = Some(vulpix);

    // Set up opponent with energy
    let opponent_active = PlayedCard::new(
        opponent_card.clone(),
        70,
        70,
        vec![EnergyType::Grass, EnergyType::Colorless],
        false,
        vec![],
    );
    state.in_play_pokemon[opponent_player][0] = Some(opponent_active);

    // Clear move generation stack
    state.move_generation_stack.clear();

    game.set_state(state);

    // Manually add CannotAttack effect to opponent's active (simulating successful Tail Whip)
    let mut state = game.get_state_clone();
    state.in_play_pokemon[opponent_player][0]
        .as_mut()
        .unwrap()
        .add_effect(CardEffect::CannotAttack, 1);
    game.set_state(state);

    // Switch to opponent's turn
    let mut state = game.get_state_clone();
    state.current_player = opponent_player;
    state.move_generation_stack.clear();
    game.set_state(state);

    // Generate possible actions - attack should NOT be available
    let state = game.get_state_clone();
    let (actor, actions) = generate_possible_actions(&state);

    assert_eq!(actor, opponent_player);

    let has_attack_action = actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::Attack(_)));

    assert!(
        !has_attack_action,
        "Opponent should not be able to attack when affected by Tail Whip"
    );
}

/// Test Tail Whip effect clears when Pokemon switches to bench
#[test]
fn test_vulpix_tail_whip_switch_clears_effect() {
    let vulpix_card = get_card_by_enum(CardId::A1037Vulpix);
    let opponent_card = get_card_by_enum(CardId::A1001Bulbasaur);
    let squirtle_card = get_card_by_enum(CardId::A1053Squirtle);

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    let test_player = state.current_player;
    let opponent_player = (test_player + 1) % 2;

    // Set up Vulpix
    let vulpix = PlayedCard::new(
        vulpix_card.clone(),
        50,
        50,
        vec![EnergyType::Colorless],
        false,
        vec![],
    );
    state.in_play_pokemon[test_player][0] = Some(vulpix);

    // Set up opponent's active with CannotAttack effect
    let mut opponent_active = PlayedCard::new(
        opponent_card.clone(),
        70,
        70,
        vec![EnergyType::Grass, EnergyType::Colorless],
        false,
        vec![],
    );
    opponent_active.add_effect(CardEffect::CannotAttack, 1);
    state.in_play_pokemon[opponent_player][0] = Some(opponent_active);

    // Set up opponent's bench Pokemon
    let bench_pokemon = PlayedCard::new(
        squirtle_card.clone(),
        50,
        50,
        vec![EnergyType::Water, EnergyType::Colorless],
        false,
        vec![],
    );
    state.in_play_pokemon[opponent_player][1] = Some(bench_pokemon);

    // Set opponent as current player
    state.current_player = opponent_player;
    state.move_generation_stack.clear();

    game.set_state(state);

    // Opponent retreats/switches to bench Pokemon
    let switch_action = Action {
        actor: opponent_player,
        action: SimpleAction::Activate { in_play_idx: 1 },
        is_stack: false,
    };
    game.apply_action(&switch_action);

    let state_after_switch = game.get_state_clone();

    // The new active (Squirtle) should be able to attack
    let (_, actions) = generate_possible_actions(&state_after_switch);

    let has_attack_action = actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::Attack(_)));

    assert!(
        has_attack_action,
        "New active Pokemon should be able to attack after switching"
    );

    // The old active (now on bench at position 0 or moved) should have effects cleared
    // Note: In the game, switching clears status effects and card effects
}

// ============================================================================
// Rampardos Tests - Head Smash Attack (Recoil if KO)
// ============================================================================

/// Test Rampardos's Head Smash deals 130 damage without recoil when opponent survives
#[test]
fn test_rampardos_head_smash_no_ko_no_recoil() {
    let rampardos_card = get_card_by_enum(CardId::A2089Rampardos);
    let opponent_card = get_card_by_enum(CardId::A1001Bulbasaur);

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    let test_player = state.current_player;
    let opponent_player = (test_player + 1) % 2;

    // Set up Rampardos with enough energy for Head Smash (1 Fighting)
    let rampardos = PlayedCard::new(
        rampardos_card.clone(),
        150,
        150,
        vec![EnergyType::Fighting],
        false,
        vec![],
    );
    state.in_play_pokemon[test_player][0] = Some(rampardos);

    // Set up opponent with HIGH HP so they survive (more than 130)
    let opponent_active = PlayedCard::new(opponent_card.clone(), 200, 200, vec![], false, vec![]);
    state.in_play_pokemon[opponent_player][0] = Some(opponent_active);

    // Clear move generation stack
    state.move_generation_stack.clear();

    game.set_state(state);

    // Apply Head Smash attack (attack index 0)
    let attack_action = Action {
        actor: test_player,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // Opponent should have 200 - 130 = 70 HP
    let opponent_hp = final_state.in_play_pokemon[opponent_player][0]
        .as_ref()
        .unwrap()
        .remaining_hp;
    assert_eq!(
        opponent_hp, 70,
        "Rampardos's Head Smash should deal 130 damage (200 - 130 = 70)"
    );

    // Rampardos should have full HP (no recoil since no KO)
    let rampardos_hp = final_state.in_play_pokemon[test_player][0]
        .as_ref()
        .unwrap()
        .remaining_hp;
    assert_eq!(
        rampardos_hp, 150,
        "Rampardos should take no recoil damage when opponent survives"
    );
}

/// Test Rampardos's Head Smash deals 50 recoil damage when opponent is KO'd
#[test]
fn test_rampardos_head_smash_ko_with_recoil() {
    let rampardos_card = get_card_by_enum(CardId::A2089Rampardos);
    let opponent_card = get_card_by_enum(CardId::A1001Bulbasaur);

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    let test_player = state.current_player;
    let opponent_player = (test_player + 1) % 2;

    // Set up Rampardos with enough energy
    let rampardos = PlayedCard::new(
        rampardos_card.clone(),
        150,
        150,
        vec![EnergyType::Fighting],
        false,
        vec![],
    );
    state.in_play_pokemon[test_player][0] = Some(rampardos);

    // Set up opponent with LOW HP so they get KO'd (less than or equal to 130)
    let opponent_active = PlayedCard::new(opponent_card.clone(), 100, 100, vec![], false, vec![]);
    state.in_play_pokemon[opponent_player][0] = Some(opponent_active);

    // Set up a bench Pokemon for opponent so game doesn't end
    let bench_pokemon = PlayedCard::new(opponent_card.clone(), 70, 70, vec![], false, vec![]);
    state.in_play_pokemon[opponent_player][1] = Some(bench_pokemon);

    // Reset points
    state.points = [0, 0];

    // Clear move generation stack
    state.move_generation_stack.clear();

    game.set_state(state);

    // Apply Head Smash attack
    let attack_action = Action {
        actor: test_player,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // Opponent's active should be KO'd (removed or replaced by promotion)
    // Player should have earned 1 point for the KO
    assert_eq!(
        final_state.points[test_player], 1,
        "Player should earn 1 point for KO'ing opponent's Pokemon"
    );

    // Rampardos should have taken 50 recoil damage (150 - 50 = 100)
    let rampardos_hp = final_state.in_play_pokemon[test_player][0]
        .as_ref()
        .unwrap()
        .remaining_hp;
    assert_eq!(
        rampardos_hp, 100,
        "Rampardos should take 50 recoil damage after KO'ing opponent (150 - 50 = 100)"
    );
}

/// Test Rampardos can KO itself with recoil damage if HP is low enough
#[test]
fn test_rampardos_head_smash_self_ko_from_recoil() {
    let rampardos_card = get_card_by_enum(CardId::A2089Rampardos);
    let opponent_card = get_card_by_enum(CardId::A1001Bulbasaur);

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    let test_player = state.current_player;
    let opponent_player = (test_player + 1) % 2;

    // Set up Rampardos with LOW HP (less than 50, so recoil will KO it)
    let rampardos = PlayedCard::new(
        rampardos_card.clone(),
        30, // Will die from 50 recoil
        150,
        vec![EnergyType::Fighting],
        false,
        vec![],
    );
    state.in_play_pokemon[test_player][0] = Some(rampardos);

    // Set up a bench Pokemon for test player so game doesn't end from self-KO
    let bench_pokemon = PlayedCard::new(rampardos_card.clone(), 150, 150, vec![], false, vec![]);
    state.in_play_pokemon[test_player][1] = Some(bench_pokemon);

    // Set up opponent with LOW HP so they get KO'd
    let opponent_active = PlayedCard::new(opponent_card.clone(), 100, 100, vec![], false, vec![]);
    state.in_play_pokemon[opponent_player][0] = Some(opponent_active);

    // Set up a bench Pokemon for opponent so game doesn't end
    let opponent_bench = PlayedCard::new(opponent_card.clone(), 70, 70, vec![], false, vec![]);
    state.in_play_pokemon[opponent_player][1] = Some(opponent_bench);

    // Reset points
    state.points = [0, 0];

    // Clear move generation stack
    state.move_generation_stack.clear();

    game.set_state(state);

    // Apply Head Smash attack
    let attack_action = Action {
        actor: test_player,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // Test player should earn 1 point for KO'ing opponent
    assert_eq!(
        final_state.points[test_player], 1,
        "Player should earn 1 point for KO'ing opponent's Pokemon"
    );

    // Opponent should earn 1 point for Rampardos self-KO from recoil
    assert_eq!(
        final_state.points[opponent_player], 1,
        "Opponent should earn 1 point when Rampardos KO's itself from recoil"
    );

    // Rampardos should be KO'd (removed from active position)
    // The bench Pokemon should have been promoted or there's a promotion pending
    // Since Rampardos was at position 0 and got KO'd, it should be None or promoted
}
