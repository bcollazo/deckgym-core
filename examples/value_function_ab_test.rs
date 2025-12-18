use clap::Parser;
use colored::Colorize;
use deckgym::{
    example_utils::discover_deck_files,
    players::{value_functions, ExpectiMiniMaxPlayer, Player},
    simulate::initialize_logger,
    simulation_event_handler::{ComputedStats, StatsCollector},
    Deck, Simulation,
};
use std::path::PathBuf;

/// A/B testing tool for comparing different value functions in ExpectiMiniMaxPlayer
///
/// This tool runs mirror matches where both players use the same deck but different
/// value functions, allowing you to directly compare their performance across multiple decks.
///
/// Example usage:
///   cargo run --example value_function_ab_test -- example_decks/ --num 1000
///   cargo run --example value_function_ab_test -- example_decks/ --num 1000 --depth 2
#[derive(Parser, Debug)]
#[command(name = "Value Function A/B Test")]
#[command(about = "Compare two value functions in mirror matches across multiple decks", long_about = None)]
struct Args {
    /// Path to the folder containing deck files (both players will use each deck)
    deck_folder: String,

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

struct ComparisonConfig {
    deck_paths: Vec<PathBuf>,
    depth: usize,
    num_games: u32,
    seed: Option<u64>,
    parallel: bool,
}

struct DeckStats {
    deck_name: String,
    baseline_wins: u32,
    variant_wins: u32,
    ties: u32,
    total_games: u32,
}

fn run_comparison(config: ComparisonConfig) -> Result<Vec<DeckStats>, Box<dyn std::error::Error>> {
    println!("\n{}", "=".repeat(70).blue().bold());
    println!(
        "{} {} vs {} across {} decks",
        "Testing:".blue().bold(),
        "baseline".green(),
        "variant".yellow(),
        config.deck_paths.len()
    );
    println!("{}", "=".repeat(70).blue().bold());

    let mut all_deck_stats = Vec::new();

    // Run simulation for each deck
    for (idx, deck_path) in config.deck_paths.iter().enumerate() {
        let deck_name = deck_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        println!(
            "\n{} [{}/{}] Testing with deck: {}",
            "→".cyan(),
            idx + 1,
            config.deck_paths.len(),
            deck_name.yellow()
        );

        // Load deck
        let deck = Deck::from_file(deck_path.to_str().ok_or("Invalid deck path")?)?;

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
                    value_function: Box::new(baseline_fn),
                }),
                Box::new(ExpectiMiniMaxPlayer {
                    deck: deck_b,
                    max_depth: depth,
                    write_debug_trees: false,
                    value_function: Box::new(test_fn),
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

        // Store per-deck stats
        let deck_stats = DeckStats {
            deck_name: deck_name.clone(),
            baseline_wins: summary.player_a_wins,
            variant_wins: summary.player_b_wins,
            ties: summary.ties,
            total_games: config.num_games,
        };

        all_deck_stats.push(deck_stats);

        // Print summary for this deck
        print_deck_summary(&deck_name, &summary);
    }

    Ok(all_deck_stats)
}

fn print_deck_summary(_deck_name: &str, summary: &ComputedStats) {
    let win_rate_diff = (summary.player_b_win_rate - summary.player_a_win_rate) * 100.0;

    println!(
        "  {} vs {} vs {}",
        format!("{:.1}%", summary.player_a_win_rate * 100.0).green(),
        format!("{:.1}%", summary.tie_rate * 100.0).white(),
        format!("{:.1}%", summary.player_b_win_rate * 100.0).yellow(),
    );

    if win_rate_diff > 0.0 {
        println!(
            "  {} {} wins {:.1}% more",
            "→".cyan(),
            "variant".yellow(),
            win_rate_diff.abs()
        );
    } else if win_rate_diff < 0.0 {
        println!(
            "  {} {} wins {:.1}% more",
            "→".cyan(),
            "baseline".green(),
            win_rate_diff.abs()
        );
    } else {
        println!("  {} Tie!", "→".cyan());
    }
}

/// Combine stats across all decks and print summary
fn print_combined_stats(all_stats: &[DeckStats]) {
    println!("\n{}", "=".repeat(70).blue().bold());
    println!("{}", "OVERALL RESULTS ACROSS ALL DECKS".blue().bold());
    println!("{}", "=".repeat(70).blue().bold());

    let total_baseline_wins: u32 = all_stats.iter().map(|s| s.baseline_wins).sum();
    let total_variant_wins: u32 = all_stats.iter().map(|s| s.variant_wins).sum();
    let total_ties: u32 = all_stats.iter().map(|s| s.ties).sum();
    let total_games: u32 = all_stats.iter().map(|s| s.total_games).sum();

    let baseline_win_rate = total_baseline_wins as f64 / total_games as f64;
    let variant_win_rate = total_variant_wins as f64 / total_games as f64;
    let tie_rate = total_ties as f64 / total_games as f64;

    println!("\n{}", "Combined Statistics:".cyan().bold());
    println!("  Total games: {}", total_games);
    println!("  Decks tested: {}", all_stats.len());
    println!(
        "  {} wins: {} ({:.1}%)",
        "baseline".green(),
        total_baseline_wins,
        baseline_win_rate * 100.0
    );
    println!(
        "  {} wins: {} ({:.1}%)",
        "variant".yellow(),
        total_variant_wins,
        variant_win_rate * 100.0
    );
    println!("  Ties: {} ({:.1}%)", total_ties, tie_rate * 100.0);

    let win_rate_diff = (variant_win_rate - baseline_win_rate) * 100.0;

    println!("\n{}", "Overall Winner:".cyan().bold());
    if win_rate_diff > 0.0 {
        println!(
            "  {} {} wins {:.1}% more games overall",
            "✓".green(),
            "variant".yellow(),
            win_rate_diff.abs()
        );
    } else if win_rate_diff < 0.0 {
        println!(
            "  {} {} wins {:.1}% more games overall",
            "✓".green(),
            "baseline".green(),
            win_rate_diff.abs()
        );
    } else {
        println!("  Tie overall!");
    }

    // Statistical significance (basic check)
    let min_games_for_significance = 100;
    if total_games >= min_games_for_significance && win_rate_diff.abs() > 1.0 {
        println!(
            "  {} Difference appears significant (>{} games, >{:.0}% difference)",
            "✓".green(),
            min_games_for_significance,
            1.0
        );
    } else if total_games < min_games_for_significance {
        println!(
            "  {} Run more games (>={}) for statistical significance",
            "!".yellow(),
            min_games_for_significance
        );
    }
}

/// Print per-deck breakdown table
fn print_per_deck_breakdown(all_stats: &[DeckStats]) {
    println!("\n{}", "=".repeat(70).blue().bold());
    println!("{}", "PER-DECK BREAKDOWN".blue().bold());
    println!("{}", "=".repeat(70).blue().bold());

    println!(
        "\n{:<30} {:>10} {:>10} {:>10} {:>8}",
        "Deck", "Baseline", "Variant", "Ties", "Winner"
    );
    println!("{}", "-".repeat(70));

    for stats in all_stats {
        let baseline_rate = stats.baseline_wins as f64 / stats.total_games as f64 * 100.0;
        let variant_rate = stats.variant_wins as f64 / stats.total_games as f64 * 100.0;
        let tie_rate = stats.ties as f64 / stats.total_games as f64 * 100.0;

        let winner = if stats.baseline_wins > stats.variant_wins {
            "baseline".green()
        } else if stats.variant_wins > stats.baseline_wins {
            "variant".yellow()
        } else {
            "tie".white()
        };

        let deck_display = if stats.deck_name.len() > 28 {
            format!("{}...", &stats.deck_name[..27])
        } else {
            stats.deck_name.clone()
        };

        println!(
            "{:<30} {:>9.1}% {:>9.1}% {:>9.1}% {:>8}",
            deck_display, baseline_rate, variant_rate, tie_rate, winner
        );
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Initialize logger
    initialize_logger(args.verbosity);

    // Discover all deck files in the folder
    let deck_paths = discover_deck_files(&args.deck_folder)?;

    println!(
        "{} Found {} deck files in {}",
        "→".cyan(),
        deck_paths.len(),
        args.deck_folder
    );

    // Run comparisons across all decks
    let all_stats = run_comparison(ComparisonConfig {
        deck_paths,
        depth: args.depth,
        num_games: args.num,
        seed: args.seed,
        parallel: args.parallel,
    })?;

    // Print combined results
    print_combined_stats(&all_stats);
    print_per_deck_breakdown(&all_stats);

    Ok(())
}
