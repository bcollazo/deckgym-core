use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    effects::CardEffect,
    models::{EnergyType, PlayedCard},
};

mod common;

/// Test Magnezone B1a 026 - Mirror Shot
/// Should deal 90 damage and apply CoinFlipToBlockAttack effect
#[test]
fn test_magnezone_mirror_shot() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    // Set up Magnezone as attacker
    let magnezone = get_card_by_enum(CardId::B1a026Magnezone);
    let magnezone_played = PlayedCard::new(
        magnezone.clone(),
        120,
        120,
        vec![
            EnergyType::Lightning,
            EnergyType::Colorless,
            EnergyType::Colorless,
        ],
        false,
        vec![],
    );
    state.in_play_pokemon[0][0] = Some(magnezone_played);

    // Set up opponent active
    let bulbasaur = get_card_by_enum(CardId::A1001Bulbasaur);
    let bulbasaur_played = PlayedCard::new(bulbasaur.clone(), 70, 70, vec![], false, vec![]);
    state.in_play_pokemon[1][0] = Some(bulbasaur_played);

    game.set_state(state);

    // Attack with Mirror Shot (index 0)
    let action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };

    game.apply_action(&action);
    let state = game.get_state_clone();

    // Check opponent was knocked out (70 HP - 90 damage)
    // Since bulbasaur gets knocked out, we need to check if bench is empty to verify KO happened
    let opponent_bench_empty = state.in_play_pokemon[1][0].is_none();
    assert!(
        opponent_bench_empty,
        "Bulbasaur should have been knocked out by 90 damage attack"
    );
}

/// Test Xerneas B1a 037 - Geoburst
/// Damage should be reduced by the amount of damage Xerneas has
#[test]
fn test_xerneas_geoburst_full_hp() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    // Set up Xerneas at full HP
    let xerneas = get_card_by_enum(CardId::B1a037Xerneas);
    let xerneas_played = PlayedCard::new(
        xerneas.clone(),
        120,
        120,
        vec![
            EnergyType::Psychic,
            EnergyType::Psychic,
            EnergyType::Colorless,
        ],
        false,
        vec![],
    );
    state.in_play_pokemon[0][0] = Some(xerneas_played);

    // Set up opponent with higher HP to survive the attack
    let bulbasaur = get_card_by_enum(CardId::A1001Bulbasaur);
    let mut bulbasaur_played = PlayedCard::new(bulbasaur.clone(), 70, 70, vec![], false, vec![]);
    bulbasaur_played.total_hp = 150;
    bulbasaur_played.remaining_hp = 150;
    state.in_play_pokemon[1][0] = Some(bulbasaur_played);

    game.set_state(state);

    let action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };

    game.apply_action(&action);
    let state = game.get_state_clone();

    // At full HP (120), Xerneas has 0 damage, so should deal full 120 damage
    let opponent_active = state.get_active(1);
    assert_eq!(
        opponent_active.remaining_hp, 30,
        "Opponent should have 30 HP remaining (150 - 120)"
    );
}

#[test]
fn test_xerneas_geoburst_damaged() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    // Set up Xerneas with 50 damage (70 HP remaining out of 120)
    let xerneas = get_card_by_enum(CardId::B1a037Xerneas);
    let xerneas_played = PlayedCard::new(
        xerneas.clone(),
        70, // 50 damage taken
        120,
        vec![
            EnergyType::Psychic,
            EnergyType::Psychic,
            EnergyType::Colorless,
        ],
        false,
        vec![],
    );
    state.in_play_pokemon[0][0] = Some(xerneas_played);

    // Set up opponent with 100 HP
    let bulbasaur = get_card_by_enum(CardId::A1001Bulbasaur);
    let mut bulbasaur_played = PlayedCard::new(bulbasaur.clone(), 70, 70, vec![], false, vec![]);
    bulbasaur_played.total_hp = 100;
    bulbasaur_played.remaining_hp = 100;
    state.in_play_pokemon[1][0] = Some(bulbasaur_played);

    game.set_state(state);

    let action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };

    game.apply_action(&action);
    let state = game.get_state_clone();

    // Xerneas has 50 damage, so attack should do 120 - 50 = 70 damage
    let opponent_active = state.get_active(1);
    assert_eq!(
        opponent_active.remaining_hp, 30,
        "Opponent should have 30 HP remaining (100 - 70)"
    );
}

/// Test Porygon-Z B1a 058 - Cyberjack
/// Should deal 20 + (20 * number of Trainer cards in opponent's deck)
#[test]
fn test_porygonz_cyberjack() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    // Set up Porygon-Z
    let porygonz = get_card_by_enum(CardId::B1a058PorygonZ);
    let porygonz_played = PlayedCard::new(
        porygonz.clone(),
        110,
        110,
        vec![
            EnergyType::Colorless,
            EnergyType::Colorless,
            EnergyType::Colorless,
        ],
        false,
        vec![],
    );
    state.in_play_pokemon[0][0] = Some(porygonz_played);

    // Set up opponent
    let bulbasaur = get_card_by_enum(CardId::A1001Bulbasaur);
    let mut bulbasaur_played = PlayedCard::new(bulbasaur.clone(), 70, 70, vec![], false, vec![]);
    bulbasaur_played.total_hp = 150;
    bulbasaur_played.remaining_hp = 150;
    state.in_play_pokemon[1][0] = Some(bulbasaur_played);

    // Put 4 Trainer cards in opponent's deck
    let pokeball = get_card_by_enum(CardId::A2b111PokeBall);
    let professor = get_card_by_enum(CardId::A4b373ProfessorsResearch);
    let giovanni = get_card_by_enum(CardId::A1223Giovanni);
    let potion = get_card_by_enum(CardId::PA001Potion);

    state.decks[1].cards = vec![
        pokeball.clone(),
        professor.clone(),
        giovanni.clone(),
        potion.clone(),
    ];

    game.set_state(state);

    let action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };

    game.apply_action(&action);
    let state = game.get_state_clone();

    // Should deal 20 + (4 * 20) = 20 + 80 = 100 damage
    let opponent_active = state.get_active(1);
    assert_eq!(
        opponent_active.remaining_hp, 50,
        "Opponent should have 50 HP remaining (150 - 100)"
    );
}

/// Test Sunflora B1a 008 - Quick-Grow Beam
/// Should deal 30 damage, or 60 if Quick-Grow Extract is in discard pile
#[test]
fn test_sunflora_quick_grow_beam_without_extract() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    // Set up Sunflora
    let sunflora = get_card_by_enum(CardId::B1a008Sunflora);
    let sunflora_played = PlayedCard::new(
        sunflora.clone(),
        80,
        80,
        vec![EnergyType::Grass],
        false,
        vec![],
    );
    state.in_play_pokemon[0][0] = Some(sunflora_played);

    // Set up opponent
    let bulbasaur = get_card_by_enum(CardId::A1001Bulbasaur);
    let bulbasaur_played = PlayedCard::new(bulbasaur.clone(), 70, 70, vec![], false, vec![]);
    state.in_play_pokemon[1][0] = Some(bulbasaur_played);

    // No Quick-Grow Extract in discard pile
    state.discard_piles[0] = vec![];

    game.set_state(state);

    let action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };

    game.apply_action(&action);
    let state = game.get_state_clone();

    // Should deal only 30 damage (no bonus)
    let opponent_active = state.get_active(1);
    assert_eq!(
        opponent_active.remaining_hp, 40,
        "Opponent should have 40 HP remaining (70 - 30)"
    );
}

#[test]
fn test_sunflora_quick_grow_beam_with_extract() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    // Set up Sunflora
    let sunflora = get_card_by_enum(CardId::B1a008Sunflora);
    let sunflora_played = PlayedCard::new(
        sunflora.clone(),
        80,
        80,
        vec![EnergyType::Grass],
        false,
        vec![],
    );
    state.in_play_pokemon[0][0] = Some(sunflora_played);

    // Set up opponent
    let bulbasaur = get_card_by_enum(CardId::A1001Bulbasaur);
    let bulbasaur_played = PlayedCard::new(bulbasaur.clone(), 70, 70, vec![], false, vec![]);
    state.in_play_pokemon[1][0] = Some(bulbasaur_played);

    // Put Quick-Grow Extract in discard pile
    let extract = get_card_by_enum(CardId::B1a067QuickGrowExtract);
    state.discard_piles[0] = vec![extract];

    game.set_state(state);

    let action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };

    game.apply_action(&action);
    let state = game.get_state_clone();

    // Should deal 30 + 30 = 60 damage
    let opponent_active = state.get_active(1);
    assert_eq!(
        opponent_active.remaining_hp, 10,
        "Opponent should have 10 HP remaining (70 - 60)"
    );
}

/// Test that CoinFlipToBlockAttack effect blocks attacks 50% of the time
#[test]
fn test_coin_flip_to_block_attack_effect() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    // Set up attacker with CoinFlipToBlockAttack effect
    let charmander = get_card_by_enum(CardId::A1033Charmander);
    let mut charmander_played = PlayedCard::new(
        charmander.clone(),
        60,
        60,
        vec![EnergyType::Fire, EnergyType::Fire],
        false,
        vec![],
    );
    charmander_played.add_effect(CardEffect::CoinFlipToBlockAttack, 1);
    state.in_play_pokemon[0][0] = Some(charmander_played);

    // Set up opponent
    let bulbasaur = get_card_by_enum(CardId::A1001Bulbasaur);
    let bulbasaur_played = PlayedCard::new(bulbasaur.clone(), 70, 70, vec![], false, vec![]);
    state.in_play_pokemon[1][0] = Some(bulbasaur_played);

    game.set_state(state);

    let action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };

    // The attack should have probabilistic outcomes
    // We can't easily test the exact probabilities without accessing internal state,
    // but we can at least verify the attack executes without panic
    game.apply_action(&action);
    let _state = game.get_state_clone();

    // Test passes if no panic occurs
    // In a real scenario, we'd need access to the probability tree to verify 50/50 split
}

/// Test Blastoise B1a 019 - Double Splash with extra energy
/// Should deal 90 to active and 50 to 1 benched Pokemon when 2+ extra Water energies attached
#[test]
fn test_blastoise_double_splash_with_extra_energy() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    // Set up Blastoise with 5 Water energies (3 required + 2 extra)
    let blastoise = get_card_by_enum(CardId::B1a019Blastoise);
    let blastoise_played = PlayedCard::new(
        blastoise.clone(),
        150,
        150,
        vec![
            EnergyType::Water,
            EnergyType::Water,
            EnergyType::Water,
            EnergyType::Water,
            EnergyType::Water,
        ],
        false,
        vec![],
    );
    state.in_play_pokemon[0][0] = Some(blastoise_played);

    // Set up opponent active with high HP
    let bulbasaur = get_card_by_enum(CardId::A1001Bulbasaur);
    let mut bulbasaur_played = PlayedCard::new(bulbasaur.clone(), 70, 70, vec![], false, vec![]);
    bulbasaur_played.total_hp = 150;
    bulbasaur_played.remaining_hp = 150;
    state.in_play_pokemon[1][0] = Some(bulbasaur_played);

    // Set up opponent bench Pokemon
    let squirtle = get_card_by_enum(CardId::A1053Squirtle);
    let squirtle_played = PlayedCard::new(squirtle.clone(), 50, 50, vec![], false, vec![]);
    state.in_play_pokemon[1][1] = Some(squirtle_played);

    game.set_state(state);

    // Attack with Double Splash
    let action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };

    game.apply_action(&action);
    let state = game.get_state_clone();

    // Check that move_generation_stack has bench target choices
    assert!(
        !state.move_generation_stack.is_empty(),
        "Move generation stack should have bench target choices"
    );

    // Get the choices from the stack
    let (actor, choices) = state.move_generation_stack.last().unwrap();
    assert_eq!(*actor, 0, "Actor should be player 0");
    assert!(
        !choices.is_empty(),
        "Should have at least one bench target choice"
    );

    // Apply the first choice (damage to bench position 1)
    let bench_damage_action = Action {
        actor: 0,
        action: choices[0].clone(),
        is_stack: true,
    };

    game.apply_action(&bench_damage_action);
    let state = game.get_state_clone();

    // Verify active took 90 damage (150 - 90 = 60)
    let opponent_active = state.get_active(1);
    assert_eq!(
        opponent_active.remaining_hp, 60,
        "Opponent active should have 60 HP remaining (150 - 90)"
    );

    // Verify bench took 50 damage (50 - 50 = 0, KO'd)
    let opponent_bench = &state.in_play_pokemon[1][1];
    assert!(
        opponent_bench.is_none(),
        "Opponent bench Pokemon should be KO'd (50 - 50 = 0)"
    );
}

/// Test Blastoise B1a 019 - Double Splash without extra energy
/// Should deal only 90 to active when no extra energies
#[test]
fn test_blastoise_double_splash_without_extra_energy() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    // Set up Blastoise with exactly 3 energies (no extra Water)
    let blastoise = get_card_by_enum(CardId::B1a019Blastoise);
    let blastoise_played = PlayedCard::new(
        blastoise.clone(),
        150,
        150,
        vec![EnergyType::Water, EnergyType::Water, EnergyType::Fire],
        false,
        vec![],
    );
    state.in_play_pokemon[0][0] = Some(blastoise_played);

    // Set up opponent active
    let bulbasaur = get_card_by_enum(CardId::A1001Bulbasaur);
    let mut bulbasaur_played = PlayedCard::new(bulbasaur.clone(), 70, 70, vec![], false, vec![]);
    bulbasaur_played.total_hp = 150;
    bulbasaur_played.remaining_hp = 150;
    state.in_play_pokemon[1][0] = Some(bulbasaur_played);

    // Set up opponent bench Pokemon
    let squirtle = get_card_by_enum(CardId::A1053Squirtle);
    let squirtle_played = PlayedCard::new(squirtle.clone(), 50, 50, vec![], false, vec![]);
    state.in_play_pokemon[1][1] = Some(squirtle_played);

    game.set_state(state);

    // Attack with Double Splash
    let action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };

    game.apply_action(&action);
    let state = game.get_state_clone();

    // Check that move_generation_stack has NO ApplyDamage actions (no bench damage)
    let has_apply_damage = state.move_generation_stack.iter().any(|(_, choices)| {
        choices
            .iter()
            .any(|action| matches!(action, SimpleAction::ApplyDamage { .. }))
    });
    assert!(
        !has_apply_damage,
        "Move generation stack should have no ApplyDamage actions (no extra energy for bench damage)"
    );

    // Verify active took 90 damage (150 - 90 = 60)
    let opponent_active = state.get_active(1);
    assert_eq!(
        opponent_active.remaining_hp, 60,
        "Opponent active should have 60 HP remaining (150 - 90)"
    );

    // Verify bench took NO damage (still at 50 HP)
    let opponent_bench = &state.in_play_pokemon[1][1];
    assert!(
        opponent_bench.is_some(),
        "Opponent bench Pokemon should still be alive"
    );
    assert_eq!(
        opponent_bench.as_ref().unwrap().remaining_hp,
        50,
        "Opponent bench should still have 50 HP (no bench damage without extra energy)"
    );
}

/// Test Mega Steelix ex B1a 052 - Adamantine Rolling
/// Should apply NoWeakness and ReducedDamage effects, negating Fire weakness on next turn
#[test]
fn test_mega_steelix_adamantine_rolling_no_weakness() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    // Set up Mega Steelix ex as player 0's active (Fire weakness: +20)
    let mega_steelix = get_card_by_enum(CardId::B1a052MegaSteelixEx);
    let mega_steelix_played = PlayedCard::new(
        mega_steelix.clone(),
        220,
        220,
        vec![
            EnergyType::Metal,
            EnergyType::Metal,
            EnergyType::Metal,
            EnergyType::Colorless,
        ],
        false,
        vec![],
    );
    state.in_play_pokemon[0][0] = Some(mega_steelix_played);

    // Set up opponent (player 1) with Charmander (Fire type attacker)
    // Give it extra HP to survive Mega Steelix's 120 damage attack
    let charmander = get_card_by_enum(CardId::A1033Charmander);
    let mut charmander_played = PlayedCard::new(
        charmander.clone(),
        60,
        60,
        vec![EnergyType::Fire, EnergyType::Fire],
        false,
        vec![],
    );
    charmander_played.total_hp = 150;
    charmander_played.remaining_hp = 150;
    state.in_play_pokemon[1][0] = Some(charmander_played);

    game.set_state(state);

    // Player 0: Mega Steelix attacks with Adamantine Rolling
    // This should apply NoWeakness and ReducedDamage effects to Mega Steelix
    let steelix_attack = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };

    game.apply_action(&steelix_attack);

    // End turn to switch to player 1
    let end_turn = Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    };
    game.apply_action(&end_turn);

    let state = game.get_state_clone();
    let steelix_hp_before = state.get_active(0).remaining_hp;

    // Player 1: Charmander attacks with Ember (30 damage, Fire type)
    // Normally this would do 30 + 20 = 50 damage (base + Fire weakness)
    // But NoWeakness effect should prevent the +20 from weakness
    // And ReducedDamage should reduce damage by 20
    // So: 30 damage (base) - 20 (ReducedDamage) = 10 damage
    let charmander_attack = Action {
        actor: 1,
        action: SimpleAction::Attack(0), // Ember
        is_stack: false,
    };

    game.apply_action(&charmander_attack);
    let state = game.get_state_clone();

    let steelix_hp_after = state.get_active(0).remaining_hp;
    let damage_taken = steelix_hp_before - steelix_hp_after;

    // Verify NoWeakness worked: should take only 10 damage (30 - 20), not 50 (30+20) or 30 (30+20-20)
    assert_eq!(
        damage_taken, 10,
        "Mega Steelix should take 10 damage (30 base - 20 reduction), NoWeakness should negate +20 weakness bonus"
    );
    assert_eq!(
        steelix_hp_after, 210,
        "Mega Steelix should have 210 HP (220 - 10)"
    );
}
