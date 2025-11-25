use deckgym::{
    gameplay_stats_collector::GameplayStatsCollector, players::PlayerCode,
    simulate::initialize_logger, simulation_event_handler::StatsCollector, Simulation,
};

/// Example showing how to use the GameplayStatsCollector to track detailed gameplay statistics
///
/// This collector tracks:
/// - When players run out of deck cards
/// - First time each card appears on the mat
/// - Hand sizes at the end of each turn
/// - First time each card uses an attack
///
/// Run with: cargo run --example gameplay_stats
fn main() {
    let num_simulations = 100;
    let deck_a_path = "example_decks/venusaur-exeggutor.txt";
    let deck_b_path = "example_decks/weezing-arbok.txt";
    let player_codes = vec![PlayerCode::ER, PlayerCode::ER];

    // Initialize logger with verbosity level 1
    initialize_logger(1);

    println!(
        "Running {} simulations to collect detailed gameplay statistics...",
        num_simulations
    );
    println!("Deck A: {}", deck_a_path);
    println!("Deck B: {}", deck_b_path);
    println!();

    // Create simulation and register event handlers using builder pattern
    let mut simulation = Simulation::new(
        deck_a_path,
        deck_b_path,
        player_codes,
        num_simulations,
        None,
        true, // parallel
        None, // use default number of threads
    )
    .expect("Failed to create simulation")
    .register::<StatsCollector>()
    .register::<GameplayStatsCollector>();

    // Run the simulation - statistics will be printed at the end
    simulation.run();
}
