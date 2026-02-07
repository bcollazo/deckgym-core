use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    effects::CardEffect,
    models::{EnergyType, PlayedCard},
};

mod common;

fn played_card_with_base_hp(card_id: CardId, base_hp: u32) -> PlayedCard {
    let card = deckgym::database::get_card_by_enum(card_id);
    PlayedCard::new(card, 0, base_hp, vec![], false, vec![])
}

// ============================================================================
// Marshadow Tests - Revenge Attack
// ============================================================================

/// Test Marshadow's Revenge attack base damage (40) when no KO happened last turn
#[test]
fn test_marshadow_revenge_base_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up Marshadow vs Bulbasaur
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1a047Marshadow)
            .with_energy(vec![EnergyType::Fighting, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;

    // Ensure no KO happened last turn
    state.set_knocked_out_by_opponent_attack_last_turn(false);

    game.set_state(state);

    // Apply Revenge attack (attack index 0)
    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // Base damage is 40, so opponent should have 70 - 40 = 30 HP
    let opponent_hp = final_state.get_active(1).get_remaining_hp();

    assert_eq!(
        opponent_hp, 30,
        "Marshadow's Revenge should deal 40 damage without KO bonus (70 - 40 = 30)"
    );
}

/// Test Marshadow's Revenge attack boosted damage (40 + 60 = 100) when KO happened last turn
#[test]
fn test_marshadow_revenge_boosted_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up Marshadow vs high-HP Bulbasaur
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1a047Marshadow)
            .with_energy(vec![EnergyType::Fighting, EnergyType::Colorless])],
        vec![played_card_with_base_hp(CardId::A1001Bulbasaur, 150)],
    );
    state.current_player = 0;

    // Simulate that a Pokemon was KO'd by opponent's attack last turn
    state.set_knocked_out_by_opponent_attack_last_turn(true);

    game.set_state(state);

    // Apply Revenge attack
    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // Boosted damage is 40 + 60 = 100, so opponent should have 150 - 100 = 50 HP
    let opponent_hp = final_state.get_active(1).get_remaining_hp();

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
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up Bulbasaur active with damage + Dusknoir on bench
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_damage(40),
            PlayedCard::from_id(CardId::A2072Dusknoir),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;

    game.set_state(state);

    // Use Dusknoir's Shadow Void ability
    let ability_action = Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 1 },
        is_stack: false,
    };
    game.apply_action(&ability_action);

    // The ability should queue a move generation for selecting which Pokemon's damage to move
    let state = game.get_state_clone();
    let (_actor, actions) = state.generate_possible_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a.action, SimpleAction::MoveAllDamage { .. })),
        "Shadow Void should queue a move generation for selecting damage source"
    );

    // Select to move damage from Bulbasaur (index 0) to Dusknoir (index 1)
    let move_damage_action = Action {
        actor: 0,
        action: SimpleAction::MoveAllDamage { from: 0, to: 1 },
        is_stack: false,
    };
    game.apply_action(&move_damage_action);

    let final_state = game.get_state_clone();

    // Bulbasaur should now have full HP (70)
    let bulbasaur_hp = final_state.get_active(0).get_remaining_hp();
    assert_eq!(
        bulbasaur_hp, 70,
        "Bulbasaur should be fully healed after Shadow Void (70 HP)"
    );

    // Dusknoir should have taken the 40 damage (130 - 40 = 90 HP)
    let dusknoir_hp = final_state
        .enumerate_bench_pokemon(0)
        .next()
        .unwrap()
        .1
        .get_remaining_hp();
    assert_eq!(
        dusknoir_hp, 90,
        "Dusknoir should have 90 HP after receiving 40 damage (130 - 40)"
    );
}

/// Test Dusknoir's Shadow Void causing KO and awarding points to opponent
#[test]
fn test_dusknoir_shadow_void_ko() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up Bulbasaur active with damage + low-HP Dusknoir on bench
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_damage(50),
            PlayedCard::from_id(CardId::A2072Dusknoir).with_remaining_hp(30),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.points = [0, 0];

    game.set_state(state);

    // Use Dusknoir's Shadow Void ability
    let ability_action = Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 1 },
        is_stack: false,
    };
    game.apply_action(&ability_action);

    // Select to move damage from Bulbasaur to Dusknoir
    let move_damage_action = Action {
        actor: 0,
        action: SimpleAction::MoveAllDamage { from: 0, to: 1 },
        is_stack: false,
    };
    game.apply_action(&move_damage_action);

    let final_state = game.get_state_clone();

    // Dusknoir should be KO'd (removed from play)
    assert!(
        final_state.enumerate_bench_pokemon(0).next().is_none(),
        "Dusknoir should be KO'd after receiving lethal damage"
    );

    // Opponent should receive 1 point for KO'ing a non-ex Pokemon
    assert_eq!(
        final_state.points[1], 1,
        "Opponent should receive 1 point for KO'ing Dusknoir"
    );
}

/// Test Dusknoir's Shadow Void can be used multiple times per turn
#[test]
fn test_dusknoir_shadow_void_multiple_uses() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up Bulbasaur active with damage, Dusknoir on bench, Squirtle with damage
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_damage(20),
            PlayedCard::from_id(CardId::A2072Dusknoir),
            PlayedCard::from_id(CardId::A1053Squirtle).with_damage(20),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;

    game.set_state(state);

    // First use: Move damage from Bulbasaur
    let ability_action = Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 1 },
        is_stack: false,
    };
    game.apply_action(&ability_action);

    let move_damage_action = Action {
        actor: 0,
        action: SimpleAction::MoveAllDamage { from: 0, to: 1 },
        is_stack: false,
    };
    game.apply_action(&move_damage_action);

    // Second use: Move damage from Squirtle
    let ability_action2 = Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 1 },
        is_stack: false,
    };
    game.apply_action(&ability_action2);

    let move_damage_action2 = Action {
        actor: 0,
        action: SimpleAction::MoveAllDamage { from: 2, to: 1 },
        is_stack: false,
    };
    game.apply_action(&move_damage_action2);

    let final_state = game.get_state_clone();

    // Bulbasaur should be fully healed
    let bulbasaur_hp = final_state.get_active(0).get_remaining_hp();
    assert_eq!(bulbasaur_hp, 70, "Bulbasaur should be fully healed");

    // Squirtle should be fully healed
    let squirtle_hp = final_state
        .enumerate_in_play_pokemon(0)
        .find(|(i, _)| *i == 2)
        .unwrap()
        .1
        .get_remaining_hp();
    assert_eq!(squirtle_hp, 60, "Squirtle should be fully healed");

    // Dusknoir should have taken both damages (130 - 20 - 20 = 90 HP)
    let dusknoir_hp = final_state
        .enumerate_bench_pokemon(0)
        .find(|(_, p)| p.get_name() == "Dusknoir")
        .unwrap()
        .1
        .get_remaining_hp();
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
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up Riolu active + Lucario on bench vs high-HP opponent
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A2091Riolu).with_energy(vec![EnergyType::Fighting]),
            PlayedCard::from_id(CardId::A2092Lucario),
        ],
        vec![played_card_with_base_hp(CardId::A1001Bulbasaur, 100)],
    );
    state.current_player = 0;

    game.set_state(state);

    // Apply Riolu's Jab attack (20 base damage + 20 from Fighting Coach = 40)
    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // With 1 Fighting Coach: 20 + 20 = 40 damage, so 100 - 40 = 60 HP
    let opponent_hp = final_state.get_active(1).get_remaining_hp();

    assert_eq!(
        opponent_hp, 60,
        "Riolu's attack should deal 40 damage with 1 Fighting Coach boost (20 + 20)"
    );
}

/// Test two Lucarios stack Fighting Coach (+40 total damage)
#[test]
fn test_lucario_fighting_coach_stacked() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up active Lucario + TWO bench Lucarios vs high-HP opponent
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A2092Lucario)
                .with_energy(vec![EnergyType::Fighting, EnergyType::Fighting]),
            PlayedCard::from_id(CardId::A2092Lucario),
            PlayedCard::from_id(CardId::A2092Lucario),
        ],
        vec![played_card_with_base_hp(CardId::A1001Bulbasaur, 150)],
    );
    state.current_player = 0;

    game.set_state(state);

    // Apply attack: 40 base + 20 (active Lucario) + 20 (bench1) + 20 (bench2) = 100
    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // With 3 Lucarios: 40 + (20 * 3) = 100 damage, so 150 - 100 = 50 HP
    let opponent_hp = final_state.get_active(1).get_remaining_hp();

    assert_eq!(
        opponent_hp, 50,
        "Lucario's attack should deal 100 damage with 3 Fighting Coaches (40 + 60)"
    );
}

/// Test Fighting Coach doesn't boost non-Fighting type attacks
#[test]
fn test_lucario_fighting_coach_no_boost_non_fighting() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up Bulbasaur (Grass type) active + Lucario bench vs high-HP opponent
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_energy(vec![EnergyType::Grass, EnergyType::Colorless]),
            PlayedCard::from_id(CardId::A2092Lucario),
        ],
        vec![played_card_with_base_hp(CardId::A1053Squirtle, 100)],
    );
    state.current_player = 0;

    game.set_state(state);

    // Apply Vine Whip attack (40 damage, should NOT get Fighting Coach boost)
    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // No boost: 40 damage, so 100 - 40 = 60 HP
    let opponent_hp = final_state.get_active(1).get_remaining_hp();

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

// ============================================================================
// Vulpix Tests - Tail Whip Attack
// ============================================================================

/// Test Vulpix's Tail Whip prevents opponent from attacking (on heads)
#[test]
fn test_vulpix_tail_whip_attack_prevention() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up Vulpix vs Bulbasaur
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1037Vulpix).with_energy(vec![EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
    );
    state.current_player = 0;

    game.set_state(state);

    // Manually add CannotAttack effect to opponent's active (simulating successful Tail Whip)
    let mut state = game.get_state_clone();
    state.in_play_pokemon[1][0]
        .as_mut()
        .unwrap()
        .add_effect(CardEffect::CannotAttack, 1);
    state.current_player = 1;
    game.set_state(state);

    // Generate possible actions - attack should NOT be available
    let state = game.get_state_clone();
    let (actor, actions) = state.generate_possible_actions();

    assert_eq!(actor, 1);

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
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up Vulpix vs Bulbasaur with CannotAttack + bench Squirtle
    let mut opponent_active = PlayedCard::from_id(CardId::A1001Bulbasaur)
        .with_energy(vec![EnergyType::Grass, EnergyType::Colorless]);
    opponent_active.add_effect(CardEffect::CannotAttack, 1);

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1037Vulpix).with_energy(vec![EnergyType::Colorless])],
        vec![
            opponent_active,
            PlayedCard::from_id(CardId::A1053Squirtle)
                .with_energy(vec![EnergyType::Water, EnergyType::Colorless]),
        ],
    );
    state.current_player = 1;

    game.set_state(state);

    // Opponent retreats/switches to bench Pokemon
    let switch_action = Action {
        actor: 1,
        action: SimpleAction::Activate {
            player: 1,
            in_play_idx: 1,
        },
        is_stack: false,
    };
    game.apply_action(&switch_action);

    let state_after_switch = game.get_state_clone();

    // The new active (Squirtle) should be able to attack
    let (_, actions) = state_after_switch.generate_possible_actions();

    let has_attack_action = actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::Attack(_)));

    assert!(
        has_attack_action,
        "New active Pokemon should be able to attack after switching"
    );
}

// ============================================================================
// Rampardos Tests - Head Smash Attack (Recoil if KO)
// ============================================================================

/// Test Rampardos's Head Smash deals 130 damage without recoil when opponent survives
#[test]
fn test_rampardos_head_smash_no_ko_no_recoil() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up Rampardos vs high-HP Bulbasaur
    state.set_board(
        vec![PlayedCard::from_id(CardId::A2089Rampardos).with_energy(vec![EnergyType::Fighting])],
        vec![played_card_with_base_hp(CardId::A1001Bulbasaur, 200)],
    );
    state.current_player = 0;

    game.set_state(state);

    // Apply Head Smash attack (attack index 0)
    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // Opponent should have 200 - 130 = 70 HP
    let opponent_hp = final_state.get_active(1).get_remaining_hp();
    assert_eq!(
        opponent_hp, 70,
        "Rampardos's Head Smash should deal 130 damage (200 - 130 = 70)"
    );

    // Rampardos should have full HP (no recoil since no KO)
    let rampardos_hp = final_state.get_active(0).get_remaining_hp();
    assert_eq!(
        rampardos_hp, 150,
        "Rampardos should take no recoil damage when opponent survives"
    );
}

/// Test Rampardos's Head Smash deals 50 recoil damage when opponent is KO'd
#[test]
fn test_rampardos_head_smash_ko_with_recoil() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up Rampardos vs low-HP Bulbasaur + bench
    state.set_board(
        vec![PlayedCard::from_id(CardId::A2089Rampardos).with_energy(vec![EnergyType::Fighting])],
        vec![
            played_card_with_base_hp(CardId::A1001Bulbasaur, 100),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );
    state.current_player = 0;
    state.points = [0, 0];

    game.set_state(state);

    // Apply Head Smash attack
    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // Player should have earned 1 point for the KO
    assert_eq!(
        final_state.points[0], 1,
        "Player should earn 1 point for KO'ing opponent's Pokemon"
    );

    // Rampardos should have taken 50 recoil damage (150 - 50 = 100)
    let rampardos_hp = final_state.get_active(0).get_remaining_hp();
    assert_eq!(
        rampardos_hp, 100,
        "Rampardos should take 50 recoil damage after KO'ing opponent (150 - 50 = 100)"
    );
}

/// Test Rampardos can KO itself with recoil damage if HP is low enough
#[test]
fn test_rampardos_head_smash_self_ko_from_recoil() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up low-HP Rampardos + bench vs low-HP Bulbasaur + bench
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A2089Rampardos)
                .with_remaining_hp(30)
                .with_energy(vec![EnergyType::Fighting]),
            PlayedCard::from_id(CardId::A2089Rampardos),
        ],
        vec![
            played_card_with_base_hp(CardId::A1001Bulbasaur, 100),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );
    state.current_player = 0;
    state.points = [0, 0];

    game.set_state(state);

    // Apply Head Smash attack
    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // Test player should earn 1 point for KO'ing opponent
    assert_eq!(
        final_state.points[0], 1,
        "Player should earn 1 point for KO'ing opponent's Pokemon"
    );

    // Opponent should earn 1 point for Rampardos self-KO from recoil
    assert_eq!(
        final_state.points[1], 1,
        "Opponent should earn 1 point when Rampardos KO's itself from recoil"
    );
}
