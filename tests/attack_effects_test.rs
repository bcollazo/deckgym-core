use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    types::{EnergyType, PlayedCard},
};

mod common;

#[test]
fn test_weedle_multiply_attack() {
    // Create a custom state with Weedle in active and another in deck
    let weedle_card = get_card_by_enum(CardId::A2b001Weedle);

    // Initialize with basic decks
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up player 0 with Weedle in active position
    let active_weedle = PlayedCard::new(
        weedle_card.clone(),
        50,                      // remaining_hp
        50,                      // total_hp
        vec![EnergyType::Grass], // Has 1 Grass energy to use Multiply
        false,
        vec![],
    );
    state.in_play_pokemon[0][0] = Some(active_weedle);

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
