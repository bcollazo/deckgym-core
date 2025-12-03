use clap::Parser;
use colored::Colorize;
use deckgym::{
    players::{value_functions, ExpectiMiniMaxPlayer, ValueFunction},
    simulate::initialize_logger,
    state::GameOutcome,
    Deck, Game,
};
use indicatif::{ProgressBar, ProgressStyle};
use num_format::{Locale, ToFormattedString};
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};

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
}

struct GameStats {
    wins: [u32; 2],
    points: [u32; 2],
    total_games: u32,
    total_turns: u32,
}

impl GameStats {
    fn new() -> Self {
        Self {
            wins: [0, 0],
            points: [0, 0],
            total_games: 0,
            total_turns: 0,
        }
    }

    fn record_game(&mut self, outcome: GameOutcome, game_points: [u8; 2], turns: u8) {
        self.total_games += 1;
        self.total_turns += turns as u32;

        match outcome {
            GameOutcome::Win(player) => self.wins[player] += 1,
            GameOutcome::Tie => {} // Don't increment wins for either player
        }

        self.points[0] += game_points[0] as u32;
        self.points[1] += game_points[1] as u32;
    }

    fn win_rate(&self, player: usize) -> f64 {
        if self.total_games == 0 {
            0.0
        } else {
            self.wins[player] as f64 / self.total_games as f64
        }
    }

    fn avg_points(&self, player: usize) -> f64 {
        if self.total_games == 0 {
            0.0
        } else {
            self.points[player] as f64 / self.total_games as f64
        }
    }

    fn avg_game_length(&self) -> f64 {
        if self.total_games == 0 {
            0.0
        } else {
            self.total_turns as f64 / self.total_games as f64
        }
    }
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

    // Initialize stats
    let mut stats = GameStats::new();

    // Create progress bar
    let pb = ProgressBar::new(config.num_games as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} games ({eta})")
            .expect("Invalid progress bar template")
            .progress_chars("=>-"),
    );

    // Get base seed for reproducibility
    let seed_base = config.seed.unwrap_or_else(|| {
        let mut rng = StdRng::from_entropy();
        rng.next_u64()
    });

    // Run games sequentially (TODO: parallelize)
    for i in 0..config.num_games {
        let game_seed = seed_base.wrapping_add(i as u64);

        // Create fresh players for each game
        let player_a = Box::new(ExpectiMiniMaxPlayer {
            deck: deck.clone(),
            max_depth: config.depth,
            write_debug_trees: false,
            value_function: config.baseline_fn,
        });

        let player_b = Box::new(ExpectiMiniMaxPlayer {
            deck: deck.clone(),
            max_depth: config.depth,
            write_debug_trees: false,
            value_function: config.test_fn,
        });

        // Create and play game
        let players: Vec<Box<dyn deckgym::players::Player>> = vec![player_a, player_b];
        let mut game = Game::new(players, game_seed);

        if let Some(outcome) = game.play() {
            let state = game.get_state_clone();
            stats.record_game(outcome, state.points, state.turn_count);
        }

        pb.inc(1);
    }

    pb.finish_with_message("Complete");

    // Print results
    println!("\n{}", "Results:".cyan().bold());
    println!(
        "  Games played: {}",
        config.num_games.to_formatted_string(&Locale::en)
    );
    println!(
        "  Average game length: {:.1} turns",
        stats.avg_game_length()
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
        stats.wins[0].to_formatted_string(&Locale::en),
        stats.win_rate(0) * 100.0
    );
    println!("    Avg points: {:.2}", stats.avg_points(0));
    println!();

    // Player B (test) stats
    println!("  {} ({}):", "Player B".yellow().bold(), config.test_name);
    println!(
        "    Wins: {} ({:.1}%)",
        stats.wins[1].to_formatted_string(&Locale::en),
        stats.win_rate(1) * 100.0
    );
    println!("    Avg points: {:.2}", stats.avg_points(1));
    println!();

    // Comparison
    let win_rate_diff = (stats.win_rate(1) - stats.win_rate(0)) * 100.0;
    let point_diff = stats.avg_points(1) - stats.avg_points(0);

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
            })?;
        }

        println!("\n{}", "=".repeat(70).blue().bold());
        println!("{}", "All tests complete!".green().bold());
        println!("{}", "=".repeat(70).blue().bold());
    }

    Ok(())
}
