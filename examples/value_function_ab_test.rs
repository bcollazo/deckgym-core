use clap::Parser;
use colored::Colorize;
use deckgym::{
    players::{value_functions, ExpectiMiniMaxPlayer, Player},
    simulate::{initialize_logger, print_stats},
    simulation_event_handler::StatsCollector,
    Deck, Simulation,
};

/// A/B testing tool for comparing different value functions in ExpectiMiniMaxPlayer
///
/// This tool runs mirror matches where both players use the same deck but different
/// value functions, allowing you to directly compare their performance.
///
/// Example usage:
///   cargo run --example value_function_ab_test -- deck.txt --num 1000
///   cargo run --example value_function_ab_test -- deck.txt --num 1000 --depth 2
///   cargo run --example value_function_ab_test -- deck.txt --num 1000 --test bench_depth
#[derive(Parser, Debug)]
#[command(name = "Value Function A/B Test")]
#[command(about = "Compare two value functions in mirror matches", long_about = None)]
struct Args {
    /// Path to the deck file (both players will use this deck)
    deck: String,

    /// Number of games to simulate
    #[arg(short, long, default_value_t = 1000)]
    num: u32,

    /// Search depth for ExpectiMiniMax player
    #[arg(short, long, default_value_t = 2)]
    depth: usize,

    /// Random seed for reproducibility
    #[arg(short, long)]
    seed: Option<u64>,

    /// Run in parallel
    #[arg(short, long, default_value_t = true)]
    parallel: bool,

    /// Verbosity level (0-4)
    #[arg(short, long, default_value_t = 1)]
    verbosity: u8,
}

struct ComparisonConfig<'a> {
    deck_path: &'a str,
    depth: usize,
    num_games: u32,
    seed: Option<u64>,
    parallel: bool,
}

fn run_comparison(config: ComparisonConfig) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "=".repeat(70).blue().bold());
    println!(
        "{} {} vs {}",
        "Testing:".blue().bold(),
        "baseline".green(),
        "variant".yellow()
    );
    println!("{}", "=".repeat(70).blue().bold());

    // Load deck
    let deck = Deck::from_file(config.deck_path)?;

    // Create player factory that builds ExpectiMiniMaxPlayers with different value functions
    let baseline_fn = value_functions::baseline_value_function;
    let test_fn = value_functions::variant_value_function;
    let depth = config.depth;

    let player_factory = move |deck_a: Deck, deck_b: Deck| -> Vec<Box<dyn Player>> {
        vec![
            Box::new(ExpectiMiniMaxPlayer {
                deck: deck_a,
                max_depth: depth,
                write_debug_trees: false,
                value_function: baseline_fn,
            }),
            Box::new(ExpectiMiniMaxPlayer {
                deck: deck_b,
                max_depth: depth,
                write_debug_trees: false,
                value_function: test_fn,
            }),
        ]
    };

    // Create and run simulation
    let mut simulation = Simulation::new_with_player_factory(
        deck.clone(),
        deck,
        player_factory,
        config.num_games,
        config.seed,
        config.parallel,
        None, // use default threads
    )?
    .register::<StatsCollector>();

    simulation.run();

    // Get stats
    let stats_collector = simulation
        .get_event_handler::<StatsCollector>()
        .ok_or("Failed to retrieve StatsCollector")?;

    let summary = stats_collector.compute_stats();
    print_stats(&summary);

    // Comparison
    let win_rate_diff = (summary.player_b_win_rate - summary.player_a_win_rate) * 100.0;

    println!("{}", "Comparison:".cyan().bold());
    if win_rate_diff > 0.0 {
        println!(
            "  {} wins {:.1}% more games",
            "variant".yellow(),
            win_rate_diff.abs()
        );
    } else if win_rate_diff < 0.0 {
        println!(
            "  {} wins {:.1}% more games",
            "baseline".green(),
            win_rate_diff.abs()
        );
    } else {
        println!("  Tie!");
    }

    // Statistical significance (basic check)
    let min_games_for_significance = 100;
    if config.num_games >= min_games_for_significance && win_rate_diff.abs() > 1.0 {
        println!();
        println!(
            "  {} Difference appears significant (>{:.0} games, >{:.0}% difference)",
            "✓".green(),
            min_games_for_significance,
            1.0
        );
    } else if config.num_games < min_games_for_significance {
        println!();
        println!(
            "  {} Run more games (>={}) for statistical significance",
            "!".yellow(),
            min_games_for_significance
        );
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Initialize logger
    initialize_logger(args.verbosity);

    run_comparison(ComparisonConfig {
        deck_path: &args.deck,
        depth: args.depth,
        num_games: args.num,
        seed: args.seed,
        parallel: args.parallel,
    })?;

    Ok(())
}
