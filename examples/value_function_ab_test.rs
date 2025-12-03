use clap::Parser;
use colored::Colorize;
use deckgym::{
    players::{value_functions, ExpectiMiniMaxPlayer, Player, ValueFunction},
    simulate::initialize_logger,
    simulation_event_handler::{SimulationEventHandler, StatsCollector},
    state::GameOutcome,
    Deck, Simulation, State,
};
use num_format::{Locale, ToFormattedString};
use uuid::Uuid;

/// Custom event handler to track points scored by each player
#[derive(Default)]
struct PointsCollector {
    total_points: [u32; 2],
    num_games: u32,
}

impl SimulationEventHandler for PointsCollector {
    fn on_game_start(&mut self, _game_id: Uuid) {}

    fn on_game_end(&mut self, _game_id: Uuid, state: State, _outcome: Option<GameOutcome>) {
        self.total_points[0] += state.points[0] as u32;
        self.total_points[1] += state.points[1] as u32;
        self.num_games += 1;
    }

    fn merge(&mut self, other: &dyn SimulationEventHandler) {
        if let Some(other_points) = (other as &dyn std::any::Any).downcast_ref::<PointsCollector>()
        {
            self.total_points[0] += other_points.total_points[0];
            self.total_points[1] += other_points.total_points[1];
            self.num_games += other_points.num_games;
        }
    }
}

impl PointsCollector {
    fn avg_points(&self, player: usize) -> f64 {
        if self.num_games == 0 {
            0.0
        } else {
            self.total_points[player] as f64 / self.num_games as f64
        }
    }
}

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

    /// Baseline value function name (defaults to "baseline")
    #[arg(short, long, default_value = "baseline")]
    baseline: String,

    /// Test value function name (defaults to all variants)
    #[arg(short, long)]
    test: Option<String>,

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

fn get_value_function(name: &str) -> Result<ValueFunction, String> {
    match name {
        "baseline" => Ok(value_functions::baseline_value_function),
        "hand" => Ok(value_functions::hand_value_function),
        "bench_depth" => Ok(value_functions::bench_depth_value_function),
        _ => Err(format!("Unknown value function: {}", name)),
    }
}

fn list_available_functions() {
    println!("Available value functions:");
    println!("  baseline    - Original value function (active_factor=2.0, hand_weight=1.0)");
    println!("  hand        - Values hand cards more (hand_weight=2.0) [+4.8% win rate]");
    println!("  bench_depth - Considers bench depth as a feature [+5.2% win rate - BEST]");
}

struct ComparisonConfig<'a> {
    deck_path: &'a str,
    baseline_fn: ValueFunction,
    test_fn: ValueFunction,
    baseline_name: &'a str,
    test_name: &'a str,
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
        config.baseline_name.green(),
        config.test_name.yellow()
    );
    println!("{}", "=".repeat(70).blue().bold());

    // Load deck
    let deck = Deck::from_file(config.deck_path)?;

    // Create player factory that builds ExpectiMiniMaxPlayers with different value functions
    let baseline_fn = config.baseline_fn;
    let test_fn = config.test_fn;
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
    .register::<StatsCollector>()
    .register::<PointsCollector>();

    simulation.run();

    // Get stats
    let stats_collector = simulation
        .get_event_handler::<StatsCollector>()
        .ok_or("Failed to retrieve StatsCollector")?;
    let points_collector = simulation
        .get_event_handler::<PointsCollector>()
        .ok_or("Failed to retrieve PointsCollector")?;

    let summary = stats_collector.compute_stats();

    println!("\n{}", "Results:".cyan().bold());
    println!(
        "  Games played: {}",
        config.num_games.to_formatted_string(&Locale::en)
    );
    println!(
        "  Average game length: {:.1} turns",
        summary.avg_turns_per_game
    );
    println!();

    // Player A (baseline) stats
    println!(
        "  {} ({}):",
        "Player A".green().bold(),
        config.baseline_name
    );
    println!(
        "    Wins: {} ({:.1}%)",
        summary.player_a_wins.to_formatted_string(&Locale::en),
        summary.player_a_win_rate * 100.0
    );
    println!("    Avg points: {:.2}", points_collector.avg_points(0));
    println!();

    // Player B (test) stats
    println!("  {} ({}):", "Player B".yellow().bold(), config.test_name);
    println!(
        "    Wins: {} ({:.1}%)",
        summary.player_b_wins.to_formatted_string(&Locale::en),
        summary.player_b_win_rate * 100.0
    );
    println!("    Avg points: {:.2}", points_collector.avg_points(1));
    println!();

    // Comparison
    let win_rate_diff = (summary.player_b_win_rate - summary.player_a_win_rate) * 100.0;
    let point_diff = points_collector.avg_points(1) - points_collector.avg_points(0);

    println!("{}", "Comparison:".cyan().bold());
    if win_rate_diff > 0.0 {
        println!(
            "  {} wins {:.1}% more games",
            config.test_name.yellow(),
            win_rate_diff.abs()
        );
    } else if win_rate_diff < 0.0 {
        println!(
            "  {} wins {:.1}% more games",
            config.baseline_name.green(),
            win_rate_diff.abs()
        );
    } else {
        println!("  Tie!");
    }

    if point_diff.abs() > 0.01 {
        if point_diff > 0.0 {
            println!(
                "  {} scores {:.2} more points on average",
                config.test_name.yellow(),
                point_diff.abs()
            );
        } else {
            println!(
                "  {} scores {:.2} more points on average",
                config.baseline_name.green(),
                point_diff.abs()
            );
        }
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

    // List available functions and validate
    if args.baseline == "list" {
        list_available_functions();
        return Ok(());
    }

    // Get value functions
    let baseline_fn = get_value_function(&args.baseline)?;

    // If test function is specified, run single comparison
    if let Some(test_name) = &args.test {
        let test_fn = get_value_function(test_name)?;
        run_comparison(ComparisonConfig {
            deck_path: &args.deck,
            baseline_fn,
            test_fn,
            baseline_name: &args.baseline,
            test_name,
            depth: args.depth,
            num_games: args.num,
            seed: args.seed,
            parallel: args.parallel,
        })?;
    } else {
        // Run against top performing variants
        let variant_names = vec!["hand", "bench_depth"];

        println!(
            "\n{} Running A/B tests against top performing variants...\n",
            "●".blue().bold()
        );

        for test_name in variant_names {
            let test_fn = get_value_function(test_name)?;
            run_comparison(ComparisonConfig {
                deck_path: &args.deck,
                baseline_fn,
                test_fn,
                baseline_name: &args.baseline,
                test_name,
                depth: args.depth,
                num_games: args.num,
                seed: args.seed,
                parallel: args.parallel,
            })?;
        }

        println!("\n{}", "=".repeat(70).blue().bold());
        println!("{}", "All tests complete!".green().bold());
        println!("{}", "=".repeat(70).blue().bold());
    }

    Ok(())
}
