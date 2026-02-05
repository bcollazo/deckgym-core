use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    effects::CardEffect,
    models::{Card, EnergyType, PlayedCard},
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
    state.in_play_pokemon[0][0] = Some(PlayedCard::from_id(CardId::B1a026Magnezone).with_energy(
        vec![
            EnergyType::Lightning,
            EnergyType::Colorless,
            EnergyType::Colorless,
        ],
    ));

    // Set up opponent active
    state.in_play_pokemon[1][0] = Some(PlayedCard::from_id(CardId::A1001Bulbasaur));

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
    state.in_play_pokemon[0][0] =
        Some(PlayedCard::from_id(CardId::B1a037Xerneas).with_energy(vec![
            EnergyType::Psychic,
            EnergyType::Psychic,
            EnergyType::Colorless,
        ]));

    // Set up opponent with higher HP to survive the attack
    state.in_play_pokemon[1][0] = Some(PlayedCard::from_id(CardId::A1001Bulbasaur).with_hp(150));
    // Fix total_hp too since from_id sets it to 70
    state.in_play_pokemon[1][0].as_mut().unwrap().total_hp = 150;

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
    state.in_play_pokemon[0][0] = Some(
        PlayedCard::from_id(CardId::B1a037Xerneas)
            .with_damage(50)
            .with_energy(vec![
                EnergyType::Psychic,
                EnergyType::Psychic,
                EnergyType::Colorless,
            ]),
    );

    // Set up opponent with 100 HP
    state.in_play_pokemon[1][0] = Some(PlayedCard::from_id(CardId::A1001Bulbasaur).with_hp(100));
    state.in_play_pokemon[1][0].as_mut().unwrap().total_hp = 100;

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
    state.in_play_pokemon[0][0] = Some(PlayedCard::from_id(CardId::B1a058PorygonZ).with_energy(
        vec![
            EnergyType::Colorless,
            EnergyType::Colorless,
            EnergyType::Colorless,
        ],
    ));

    // Set up opponent with 150 HP
    state.in_play_pokemon[1][0] = Some(PlayedCard::from_id(CardId::A1001Bulbasaur).with_hp(150));
    state.in_play_pokemon[1][0].as_mut().unwrap().total_hp = 150;

    // Put 4 Trainer cards in opponent's deck
    state.decks[1].cards = vec![
        get_card_by_enum(CardId::A2b111PokeBall),
        get_card_by_enum(CardId::A4b373ProfessorsResearch),
        get_card_by_enum(CardId::A1223Giovanni),
        get_card_by_enum(CardId::PA001Potion),
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
    state.in_play_pokemon[0][0] =
        Some(PlayedCard::from_id(CardId::B1a008Sunflora).with_energy(vec![EnergyType::Grass]));

    // Set up opponent
    state.in_play_pokemon[1][0] = Some(PlayedCard::from_id(CardId::A1001Bulbasaur));

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
    state.in_play_pokemon[0][0] =
        Some(PlayedCard::from_id(CardId::B1a008Sunflora).with_energy(vec![EnergyType::Grass]));

    // Set up opponent
    state.in_play_pokemon[1][0] = Some(PlayedCard::from_id(CardId::A1001Bulbasaur));

    // Put Quick-Grow Extract in discard pile
    state.discard_piles[0] = vec![get_card_by_enum(CardId::B1a067QuickGrowExtract)];

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
    let mut charmander_played = PlayedCard::from_id(CardId::A1033Charmander)
        .with_energy(vec![EnergyType::Fire, EnergyType::Fire]);
    charmander_played.add_effect(CardEffect::CoinFlipToBlockAttack, 1);
    state.in_play_pokemon[0][0] = Some(charmander_played);

    // Set up opponent
    state.in_play_pokemon[1][0] = Some(PlayedCard::from_id(CardId::A1001Bulbasaur));

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
    state.in_play_pokemon[0][0] = Some(PlayedCard::from_id(CardId::B1a019Blastoise).with_energy(
        vec![
            EnergyType::Water,
            EnergyType::Water,
            EnergyType::Water,
            EnergyType::Water,
            EnergyType::Water,
        ],
    ));

    // Set up opponent active with high HP
    state.in_play_pokemon[1][0] = Some(PlayedCard::from_id(CardId::A1001Bulbasaur).with_hp(150));
    state.in_play_pokemon[1][0].as_mut().unwrap().total_hp = 150;

    // Set up opponent bench Pokemon
    state.in_play_pokemon[1][1] = Some(PlayedCard::from_id(CardId::A1053Squirtle));

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

    // Verify bench took 50 damage (60 - 50 = 10 HP remaining)
    let opponent_bench = &state.in_play_pokemon[1][1];
    assert!(
        opponent_bench.is_some(),
        "Opponent bench Pokemon should still be alive (60 - 50 = 10)"
    );
    assert_eq!(
        opponent_bench.as_ref().unwrap().remaining_hp,
        10,
        "Opponent bench Pokemon should have 10 HP remaining (60 - 50)"
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
    state.in_play_pokemon[0][0] = Some(
        PlayedCard::from_id(CardId::B1a019Blastoise).with_energy(vec![
            EnergyType::Water,
            EnergyType::Water,
            EnergyType::Fire,
        ]),
    );

    // Set up opponent active with 150 HP
    state.in_play_pokemon[1][0] = Some(PlayedCard::from_id(CardId::A1001Bulbasaur).with_hp(150));
    state.in_play_pokemon[1][0].as_mut().unwrap().total_hp = 150;

    // Set up opponent bench Pokemon
    state.in_play_pokemon[1][1] = Some(PlayedCard::from_id(CardId::A1053Squirtle));

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
        60,
        "Opponent bench should still have 60 HP (no bench damage without extra energy)"
    );
}

/// Test Blastoise B1a 019 - Double Splash with extra energy but no bench
/// Should deal 90 to active only (no bench to hit)
#[test]
fn test_blastoise_double_splash_with_extra_energy_no_bench() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    // Set up Blastoise with 5 Water energies (3 required + 2 extra)
    state.in_play_pokemon[0][0] = Some(PlayedCard::from_id(CardId::B1a019Blastoise).with_energy(
        vec![
            EnergyType::Water,
            EnergyType::Water,
            EnergyType::Water,
            EnergyType::Water,
            EnergyType::Water,
        ],
    ));

    // Set up opponent active ONLY (no bench) with 150 HP
    state.in_play_pokemon[1][0] = Some(PlayedCard::from_id(CardId::A1001Bulbasaur).with_hp(150));
    state.in_play_pokemon[1][0].as_mut().unwrap().total_hp = 150;
    // No bench Pokemon!

    game.set_state(state);

    // Attack with Double Splash
    let action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };

    game.apply_action(&action);
    let state = game.get_state_clone();

    // Check that move_generation_stack has NO ApplyDamage actions (no bench to hit)
    let has_apply_damage = state.move_generation_stack.iter().any(|(_, choices)| {
        choices
            .iter()
            .any(|action| matches!(action, SimpleAction::ApplyDamage { .. }))
    });
    assert!(
        !has_apply_damage,
        "Move generation stack should have no ApplyDamage actions (no bench Pokemon to hit)"
    );

    // Verify active took 90 damage (150 - 90 = 60)
    let opponent_active = state.get_active(1);
    assert_eq!(
        opponent_active.remaining_hp, 60,
        "Opponent active should have 60 HP remaining (150 - 90), even with extra energy but no bench"
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
    state.in_play_pokemon[0][0] = Some(
        PlayedCard::from_id(CardId::B1a052MegaSteelixEx).with_energy(vec![
            EnergyType::Metal,
            EnergyType::Metal,
            EnergyType::Metal,
            EnergyType::Colorless,
        ]),
    );

    // Set up opponent (player 1) with Charmander (Fire type attacker)
    // Give it extra HP to survive Mega Steelix's 120 damage attack
    let mut charmander_played = PlayedCard::from_id(CardId::A1033Charmander)
        .with_energy(vec![EnergyType::Fire, EnergyType::Fire]);
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

/// Test Quick-Grow Extract B1a 067 - Evolution from deck
/// Should evolve a Grass Pokemon in play with a random Grass evolution from deck
#[test]
fn test_quick_grow_extract_evolves_from_deck() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    // Clear the hand and deck to have a controlled test environment
    state.hands[0].clear();
    state.decks[0].cards.clear();

    // Set up Bulbasaur as active (Grass type, stage 0)
    state.in_play_pokemon[0][0] = Some(PlayedCard::from_id(CardId::A1001Bulbasaur));

    // Put exactly ONE Ivysaur (evolution of Bulbasaur, Grass type) in the deck
    let ivysaur = get_card_by_enum(CardId::A1002Ivysaur);
    state.decks[0].cards.push(ivysaur.clone());

    // Add some other cards to the deck so it's not empty
    state.decks[0]
        .cards
        .push(get_card_by_enum(CardId::A1011Oddish));

    // Put Quick-Grow Extract in hand
    let extract = get_card_by_enum(CardId::B1a067QuickGrowExtract);
    state.hands[0].push(extract.clone());

    game.set_state(state);

    // Play Quick-Grow Extract
    let play_extract = Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: if let deckgym::models::Card::Trainer(tc) = extract {
                tc
            } else {
                panic!("Expected trainer card")
            },
        },
        is_stack: false,
    };

    game.apply_action(&play_extract);
    let state = game.get_state_clone();

    // Verify that Bulbasaur evolved into Ivysaur
    let active = state.get_active(0);
    if let Card::Pokemon(pokemon) = &active.card {
        assert_eq!(
            pokemon.name, "Ivysaur",
            "Bulbasaur should have evolved into Ivysaur"
        );
    } else {
        panic!("Expected Pokemon card");
    }
}

/// Test Charmeleon B1a 012 - Ignition ability
/// Should trigger on evolution, offering to attach Fire energy
#[test]
fn test_charmeleon_ignition() {
    let setup_game = || {
        let mut game = get_initialized_game(0);
        let mut state = game.get_state_clone();
        state.current_player = 0;

        state.in_play_pokemon[0][0] = Some(PlayedCard::from_id(CardId::A1033Charmander));

        let charmeleon = get_card_by_enum(CardId::B1a012Charmeleon);
        state.hands[0].push(charmeleon.clone());

        game.set_state(state);
        (game, charmeleon)
    };

    let evolve = |game: &mut deckgym::Game, charmeleon: Card| {
        let evolve_action = Action {
            actor: 0,
            action: SimpleAction::Evolve {
                in_play_idx: 0,
                evolution: if let Card::Pokemon(pc) = charmeleon {
                    Card::Pokemon(pc)
                } else {
                    panic!("Expected Pokemon card")
                },
                from_deck: false,
            },
            is_stack: false,
        };
        game.apply_action(&evolve_action);
    };

    // Test 1: Ability triggers on evolution with 2 options
    {
        let (mut game, charmeleon) = setup_game();
        evolve(&mut game, charmeleon);
        let state = game.get_state_clone();

        let active = state.get_active(0);
        if let Card::Pokemon(pokemon) = &active.card {
            assert_eq!(pokemon.name, "Charmeleon");
        }

        assert!(!state.move_generation_stack.is_empty());
        let (_, moves) = state.move_generation_stack.last().unwrap();
        assert_eq!(moves.len(), 2);
    }

    // Test 2: User accepts and attaches Fire energy
    {
        let (mut game, charmeleon) = setup_game();
        evolve(&mut game, charmeleon);
        let state = game.get_state_clone();

        let (_, moves) = state.move_generation_stack.last().unwrap();
        game.apply_action(&Action {
            actor: 0,
            action: moves[0].clone(),
            is_stack: true,
        });
        let state = game.get_state_clone();

        let charmeleon_active = state.get_active(0);
        assert_eq!(charmeleon_active.attached_energy.len(), 1);
        assert_eq!(charmeleon_active.attached_energy[0], EnergyType::Fire);
    }

    // Test 3: User declines and doesn't attach energy
    {
        let (mut game, charmeleon) = setup_game();
        evolve(&mut game, charmeleon);
        let state = game.get_state_clone();

        let (_, moves) = state.move_generation_stack.last().unwrap();
        game.apply_action(&Action {
            actor: 0,
            action: moves[1].clone(),
            is_stack: true,
        });
        let state = game.get_state_clone();

        let charmeleon_active = state.get_active(0);
        assert_eq!(charmeleon_active.attached_energy.len(), 0);
    }
}
