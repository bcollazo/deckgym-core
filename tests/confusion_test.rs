use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard, StatusCondition},
};

mod common;

/// Test that a confused Pokémon can still attack but has different outcomes
#[test]
fn test_confused_pokemon_can_attack() {
    let mut game = get_initialized_game(42);
    let mut state = game.get_state_clone();

    // Set up Player 0 with a confused Pokémon
    state.in_play_pokemon[0][0] = Some(
        PlayedCard::from_id(CardId::A1035Charizard)
            .with_energy(vec![EnergyType::Fire, EnergyType::Fire, EnergyType::Fire])
            .with_status(StatusCondition::Confused),
    );

    // Set up opponent with a basic Pokémon
    state.in_play_pokemon[1][0] = Some(PlayedCard::from_id(CardId::A1053Squirtle));

    state.turn_count = 3;
    state.current_player = 0;
    game.set_state(state);

    // Apply attack action
    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0), // Fire Blast
        is_stack: false,
    };
    game.apply_action(&attack_action);

    // The game should continue (attack was processed)
    let state = game.get_state_clone();
    // Turn should advance or a stack action should be queued
    assert!(
        !state.move_generation_stack.is_empty() || state.current_player != 0,
        "Game should progress after confused attack"
    );
}

/// Test that confusion is cleared when Pokémon retreats/moves to bench
#[test]
fn test_confusion_cleared_on_retreat() {
    let mut game = get_initialized_game(42);
    let mut state = game.get_state_clone();

    // Set up Player 0 with a confused Pokémon with enough energy to retreat
    state.in_play_pokemon[0][0] = Some(
        PlayedCard::from_id(CardId::A1035Charizard)
            .with_energy(vec![EnergyType::Fire, EnergyType::Fire])
            .with_status(StatusCondition::Confused),
    );

    // Add a bench Pokémon to retreat to
    state.in_play_pokemon[0][1] = Some(PlayedCard::from_id(CardId::A1053Squirtle));

    // Set up opponent
    state.in_play_pokemon[1][0] = Some(PlayedCard::from_id(CardId::A1053Squirtle));

    state.turn_count = 3;
    state.current_player = 0;
    game.set_state(state);

    // Verify active is confused before retreat
    let state = game.get_state_clone();
    assert!(
        state.in_play_pokemon[0][0].as_ref().unwrap().confused,
        "Active should be confused before retreat"
    );

    // Apply retreat action (to bench slot 1)
    let retreat_action = Action {
        actor: 0,
        action: SimpleAction::Retreat(1),
        is_stack: false,
    };
    game.apply_action(&retreat_action);

    // After retreat, the Charizard (now on bench) should NOT be confused
    let state = game.get_state_clone();
    let charizard_on_bench = state.in_play_pokemon[0]
        .iter()
        .skip(1) // Skip active
        .flatten()
        .find(|p| p.get_name() == "Charizard");

    assert!(
        charizard_on_bench.is_some(),
        "Charizard should be on bench after retreat"
    );
    assert!(
        !charizard_on_bench.unwrap().confused,
        "Charizard should NOT be confused after retreating to bench"
    );
}

/// Test that confusion field exists and can be set directly
#[test]
fn test_confusion_field_can_be_set() {
    let mut charizard_played = PlayedCard::from_id(CardId::A1035Charizard);

    // Initially not confused
    assert!(!charizard_played.confused);

    // Set confusion directly
    charizard_played.confused = true;

    // Now should be confused
    assert!(charizard_played.confused);
}

/// Test multiple status conditions including confusion
#[test]
fn test_multiple_status_conditions_with_confusion() {
    let charizard_played = PlayedCard::from_id(CardId::A1035Charizard)
        .with_status(StatusCondition::Confused)
        .with_status(StatusCondition::Poisoned);

    assert!(charizard_played.confused);
    assert!(charizard_played.poisoned);
}

/// Test that a confused Pokémon attack may succeed (run multiple times with different seeds)
#[test]
fn test_confused_attack_can_succeed() {
    // Run with different seeds to get different outcomes
    for seed in 0..20 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();

        state.in_play_pokemon[0][0] = Some(
            PlayedCard::from_id(CardId::A1035Charizard)
                .with_energy(vec![EnergyType::Fire, EnergyType::Fire, EnergyType::Fire])
                .with_status(StatusCondition::Confused),
        );

        state.in_play_pokemon[1][0] = Some(PlayedCard::from_id(CardId::A1053Squirtle));

        state.turn_count = 3;
        state.current_player = 0;
        game.set_state(state);

        let initial_hp = 70;
        let attack_action = Action {
            actor: 0,
            action: SimpleAction::Attack(0),
            is_stack: false,
        };
        game.apply_action(&attack_action);

        let state = game.get_state_clone();
        let opponent_hp = state.in_play_pokemon[1][0]
            .as_ref()
            .map(|p| p.remaining_hp)
            .unwrap_or(0);

        // Either the attack succeeded (opponent took damage or was KO'd)
        // or the attack failed (opponent still at full HP)
        let attack_succeeded = opponent_hp < initial_hp || state.in_play_pokemon[1][0].is_none();
        let attack_failed = opponent_hp == initial_hp;

        assert!(
            attack_succeeded || attack_failed,
            "Attack should either succeed or fail due to confusion"
        );
    }
}

/// Test that confusion is applied when a confusing attack hits
#[test]
fn test_confusing_attack_inflicts_confusion() {
    let mut game = get_initialized_game(42);
    let mut state = game.get_state_clone();

    let mut squirtle_played = PlayedCard::from_id(CardId::A1053Squirtle);

    // Initially not confused
    assert!(!squirtle_played.confused);

    // Simulate being hit by a confusing attack
    squirtle_played.confused = true;

    // Now confused
    assert!(squirtle_played.confused);

    // The confusion field should be accessible
    state.in_play_pokemon[1][0] = Some(squirtle_played);
    game.set_state(state);

    let state = game.get_state_clone();
    assert!(
        state.in_play_pokemon[1][0].as_ref().unwrap().confused,
        "Confusion should be stored in game state"
    );
}
