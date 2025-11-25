use deckgym::{
    gameplay_stats_collector::GameplayStatsCollector, players::PlayerCode,
    simulate::initialize_logger, simulation_event_handler::StatsCollector, Simulation,
};
use log::warn;
use num_format::{Locale, ToFormattedString};

/// Example showing how to use the GameplayStatsCollector to track detailed gameplay statistics
///
/// This collector tracks:
/// - When players run out of deck cards
/// - First time each card appears on the mat
/// - Average hand sizes across all turns
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

    // Run the simulation
    simulation.run();

    // Get the gameplay stats collector and compute statistics
    if let Some(collector) = simulation.get_event_handler::<GameplayStatsCollector>() {
        let stats = collector.compute_stats();
        print_stats(&stats);
    } else {
        eprintln!("Failed to retrieve GameplayStatsCollector");
    }
}

fn print_stats(stats: &deckgym::gameplay_stats_collector::AggregatedStats) {
    warn!("=== Gameplay Statistics Summary ===");
    warn!(
        "Total games: {}",
        stats.total_games.to_formatted_string(&Locale::en)
    );
    warn!("");

    // 1. Game Ending Statistics
    warn!("--- Game Statistics ---");
    warn!("Average game length: {:.1} turns", stats.avg_game_length);
    warn!("");

    // 2. Deck Empty Statistics
    warn!("--- Deck Empty Statistics ---");
    for player in 0..2 {
        if let Some(avg_turn) = stats.deck_empty_avg[player] {
            warn!("Player {}: Deck empty avg turn: {:.1}", player, avg_turn);
        } else {
            warn!("Player {}: Deck never empty", player);
        }
    }
    warn!("");

    // 3. Hand Size Statistics (average across all turns)
    warn!("--- Hand Size Statistics ---");
    for player in 0..2 {
        warn!(
            "Player {}: Average hand size: {:.2} cards",
            player, stats.hand_sizes[player]
        );
    }
    warn!("");

    // 4. Cards on Mat Statistics (show first 5 cards)
    warn!("--- Cards on Mat Statistics ---");
    for player in 0..2 {
        let player_cards = &stats.cards_seen[player];
        warn!(
            "Player {}: {} unique cards appeared on mat",
            player,
            player_cards.len()
        );

        for (card_id, avg_turn) in player_cards.iter().take(5) {
            warn!("  {}: avg turn {:.1}", card_id, avg_turn);
        }
    }
    warn!("");

    // 5. Attack Usage Statistics (show first 5 cards)
    warn!("--- Attack Usage Statistics ---");
    for player in 0..2 {
        let player_attacks = &stats.attacks_used[player];
        warn!(
            "Player {}: {} unique cards used attacks",
            player,
            player_attacks.len()
        );

        for (card_id, avg_turn) in player_attacks.iter().take(5) {
            warn!("  {}: avg turn {:.1}", card_id, avg_turn);
        }
    }
}
