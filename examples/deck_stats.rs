/// Example demonstrating how to use DeckStatsCollector to analyze deck performance
///
/// Run with: `cargo run --example deck_stats`
///
/// This example runs R vs R games and collects statistics about:
/// - Average turn to deck out (empty the deck)
/// - Average turn when each Pokemon appears on the mat
/// - Average damage dealt per game
/// - Most common first Pokemon played
/// - And more deck-building relevant statistics
use deckgym::{
    players::PlayerCode,
    simulate::{initialize_logger, Simulation},
    DeckStatsCollector,
};

fn main() {
    // Configure logging to show warnings and errors
    initialize_logger(1);

    // Simulation parameters
    let num_simulations = 1_000;
    let deck_a_path = "example_decks/mewtwoex.txt";
    let deck_b_path = "example_decks/mewtwoex.txt"; // Same deck (mirror match)
    let player_codes = vec![PlayerCode::E { max_depth: 2 }, PlayerCode::E { max_depth: 2 }]; // Expectimax players

    println!("========================================");
    println!("Deck Statistics Analysis");
    println!("========================================");
    println!("Running {} games of E2 vs E2", num_simulations);
    println!("Deck: {}", deck_a_path);
    println!("========================================\n");

    // Create simulation with deck statistics collector
    let mut simulation = Simulation::new(
        deck_a_path,
        deck_b_path,
        player_codes,
        num_simulations,
        None,  // No fixed seed
        true,  // Parallel execution
        None,  // Use default number of threads
    )
    .expect("Failed to create simulation");

    // Register the DeckStatsCollector event handler
    simulation = simulation.register::<DeckStatsCollector>();

    // Run the simulation
    // Statistics will be printed to console when complete
    simulation.run();

    println!("\n========================================");
    println!("Analysis complete!");
    println!("========================================");
}
