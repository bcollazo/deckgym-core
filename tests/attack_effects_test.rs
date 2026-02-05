use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
};

mod common;

#[test]
fn test_weedle_multiply_attack() {
    // Create a custom state with Weedle in active and another in deck
    let weedle_card = get_card_by_enum(CardId::A2b001Weedle);

    // Initialize with basic decks
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    // Set up player 0 with Weedle in active position
    state.in_play_pokemon[0][0] =
        Some(PlayedCard::from_card(&weedle_card).with_energy(vec![EnergyType::Grass]));

    // Add another Weedle to the deck
    state.decks[0].cards.push(weedle_card.clone());

    // Count bench pokemon before attack
    let bench_count_before = state.enumerate_bench_pokemon(0).count();

    game.set_state(state);

    // Apply Multiply attack
    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0), // First attack (Multiply)
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let state = game.get_state_clone();

    // Assert that a Weedle was added to the bench
    let bench_count_after = state.enumerate_bench_pokemon(0).count();
    assert_eq!(
        bench_count_after,
        bench_count_before + 1,
        "Multiply should add one Weedle to the bench"
    );

    // Verify it's actually a Weedle on the bench
    let benched_pokemon: Vec<_> = state.enumerate_bench_pokemon(0).collect();
    let last_benched = benched_pokemon.last();
    assert!(last_benched.is_some(), "Should have a pokemon on bench");
    assert_eq!(
        last_benched.unwrap().1.get_name(),
        "Weedle",
        "The benched pokemon should be Weedle"
    );
}

/// Test Dialga ex's Metallic Turbo attack which attaches 2 Metal energies to bench Pokemon
/// and triggers a Rocky Helmet knockout counterattack. This is an edge case to ensure that
/// attack effects are resolved before handling knockouts and promotions.
#[test]
fn test_dialga_rocky_helmet_knockout_with_energy_attach() {
    // Start with an initialized game to have proper deck structures
    let mut game = get_initialized_game(42);
    let mut state = game.get_state_clone();

    // Set up Player 0 (acting player) with Dialga ex at low HP
    state.in_play_pokemon[0][0] = Some(
        PlayedCard::from_id(CardId::A2119DialgaEx)
            .with_hp(20)
            .with_energy(vec![EnergyType::Metal, EnergyType::Metal]),
    );

    // Add 3 bench Pokémon for Player 0
    state.in_play_pokemon[0][1] = Some(PlayedCard::from_id(CardId::A1001Bulbasaur));
    state.in_play_pokemon[0][2] = Some(PlayedCard::from_id(CardId::A1001Bulbasaur));
    state.in_play_pokemon[0][3] = Some(PlayedCard::from_id(CardId::A1001Bulbasaur));

    // Set up Player 1 (opponent) with Squirtle with Rocky Helmet
    state.in_play_pokemon[1][0] = Some(
        PlayedCard::from_id(CardId::A1053Squirtle)
            .with_tool(get_card_by_enum(CardId::A2148RockyHelmet)),
    );

    // Add 1 bench Pokémon for Player 1
    state.in_play_pokemon[1][1] = Some(PlayedCard::from_id(CardId::A1053Squirtle));

    // Both players start at 0 points
    state.points = [0, 0];

    // Set up proper turn state
    state.turn_count = 3;
    state.current_player = 0;

    // Update the game with our modified state
    game.set_state(state);

    // Apply the Attack action (index 0 = Metallic Turbo)
    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    // The attack should queue up an energy attachment action
    let state = game.get_state_clone();
    assert!(
        !state.move_generation_stack.is_empty(),
        "Move generation stack should have actions queued"
    );

    // Continue with play_tick() until the next turn or game over
    let initial_turn = state.turn_count;
    let mut iterations = 0;
    let max_iterations = 100; // Safety limit

    while iterations < max_iterations {
        let state = game.get_state_clone();

        // Break if game is over or turn has advanced
        if game.is_game_over() || state.turn_count > initial_turn {
            break;
        }

        game.play_tick();
        iterations += 1;
    }

    // Final state assertions
    let final_state = game.get_state_clone();

    // Dialga should be knocked out (not in active spot anymore)
    // Since Dialga takes 20 counterattack damage and has 20 HP, it should be KO'd
    // A promotion should have occurred (originally 3 bench + active, now should have active)
    let active = final_state.get_active(0);
    assert_ne!(
        active.card.get_name(),
        "Dialga ex",
        "Dialga should have been knocked out"
    );
    assert_eq!(
        final_state.points[1], 2,
        "Player 1 should have 2 points from knocking out Dialga ex"
    );

    // At least one bench Pokémon should have Metal energies attached
    // (from Metallic Turbo's effect)
    let total_metal_on_bench: usize = final_state
        .enumerate_bench_pokemon(0)
        .map(|(_, p)| {
            p.attached_energy
                .iter()
                .filter(|e| **e == EnergyType::Metal)
                .count()
        })
        .sum();
    assert!(
        total_metal_on_bench >= 2,
        "Expected at least 2 Metal energies on bench (from Metallic Turbo), found {}",
        total_metal_on_bench
    );
}
