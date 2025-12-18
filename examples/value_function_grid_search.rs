use clap::Parser;
use colored::Colorize;
use deckgym::{
    example_utils::discover_deck_files,
    players::{value_functions, ExpectiMiniMaxPlayer, Player},
    simulate::initialize_logger,
    simulation_event_handler::StatsCollector,
    Deck, Simulation,
};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::path::PathBuf;

/// Hyperparameter optimization tool for value function coefficients
///
/// This tool supports multiple optimization strategies to find the best
/// performing parameters. It compares baseline vs variant (with parameters)
/// across multiple decks to find the configuration with the most total wins.
///
/// Example usage:
///   # Random search (recommended for quick exploration)
///   cargo run --example value_function_grid_search -- example_decks/ --num 100 --strategy random --budget 50
///
///   # Successive halving (adaptive, stops bad configs early)
///   cargo run --example value_function_grid_search -- example_decks/ --num 100 --strategy halving --budget 64
///
///   # Grid search (exhaustive)
///   cargo run --example value_function_grid_search -- example_decks/ --num 100 --strategy grid --grid-values "1,10,100"
#[derive(Parser, Debug)]
#[command(name = "Value Function Hyperparameter Optimization")]
#[command(about = "Optimize value function coefficients using various strategies", long_about = None)]
struct Args {
    /// Path to the folder containing deck files (both players will use each deck)
    deck_folder: String,

    /// Number of games to simulate per configuration
    #[arg(short, long, default_value_t = 100)]
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
    #[arg(short, long, default_value_t = 0)]
    verbosity: u8,

    /// Grid search values to test for each parameter
    #[arg(long, default_value = "0.1,1,10,100,1000,10000")]
    grid_values: String,

    /// Only search over specified parameters (comma-separated).
    /// Available: pokemon_value, hand_size, deck_size, active_retreat_cost,
    /// active_pokemon_online_score, active_safety, active_has_tool, is_winner,
    /// turns_until_opponent_wins, online_pokemon_count, energy_distance_to_online
    #[arg(long)]
    search_params: Option<String>,

    /// Optimization strategy: grid, random, or halving (successive halving)
    #[arg(long, default_value = "random")]
    strategy: String,

    /// Budget (number of configurations to test). For halving, should be a power of 2.
    #[arg(long, default_value_t = 50)]
    budget: usize,

    /// Minimum value for random sampling
    #[arg(long, default_value_t = 0.1)]
    min_value: f64,

    /// Maximum value for random sampling
    #[arg(long, default_value_t = 10000.0)]
    max_value: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum OptimizationStrategy {
    Grid,
    Random,
    SuccessiveHalving,
}

struct OptimizationConfig {
    deck_paths: Vec<PathBuf>,
    depth: usize,
    num_games: u32,
    seed: Option<u64>,
    parallel: bool,
    grid_values: Vec<f64>,
    search_params: Vec<String>,
    strategy: OptimizationStrategy,
    budget: usize,
    min_value: f64,
    max_value: f64,
}

#[derive(Clone)]
struct GridSearchResult {
    params: value_functions::ValueFunctionParams,
    total_variant_wins: u32,
    total_baseline_wins: u32,
    total_games: u32,
}

/// Test a single parameter configuration across all decks
fn test_configuration(
    params: &value_functions::ValueFunctionParams,
    config: &OptimizationConfig,
) -> Result<GridSearchResult, Box<dyn std::error::Error>> {
    let mut total_baseline_wins = 0;
    let mut total_variant_wins = 0;
    let total_games = config.deck_paths.len() as u32 * config.num_games;

    // Run simulation for each deck
    for deck_path in &config.deck_paths {
        // Load deck
        let deck = Deck::from_file(deck_path.to_str().ok_or("Invalid deck path")?)?;

        // Create player factory
        let baseline_fn = value_functions::baseline_value_function;
        let params_copy = *params; // Copy params for closure
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
                    value_function: Box::new(move |state, player| {
                        value_functions::parametric_value_function(state, player, &params_copy)
                    }),
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
            None,
        )?
        .register::<StatsCollector>();

        simulation.run();

        // Get stats
        let stats_collector = simulation
            .get_event_handler::<StatsCollector>()
            .ok_or("Failed to retrieve StatsCollector")?;

        let summary = stats_collector.compute_stats();
        total_baseline_wins += summary.player_a_wins;
        total_variant_wins += summary.player_b_wins;
    }

    Ok(GridSearchResult {
        params: *params,
        total_variant_wins,
        total_baseline_wins,
        total_games,
    })
}

/// Generate parameter combinations based on optimization strategy
fn generate_param_combinations(
    config: &OptimizationConfig,
    rng: &mut StdRng,
) -> Vec<value_functions::ValueFunctionParams> {
    match config.strategy {
        OptimizationStrategy::Grid => generate_grid_combinations(config),
        OptimizationStrategy::Random => generate_random_combinations(config, rng),
        OptimizationStrategy::SuccessiveHalving => generate_random_combinations(config, rng),
    }
}

/// Generate all parameter combinations for grid search
fn generate_grid_combinations(
    config: &OptimizationConfig,
) -> Vec<value_functions::ValueFunctionParams> {
    let baseline = value_functions::ValueFunctionParams::baseline();
    let mut combinations = Vec::new();

    // Determine which parameters to search over
    let all_params = vec![
        "pokemon_value",
        "hand_size",
        "deck_size",
        "active_retreat_cost",
        "active_pokemon_online_score",
        "active_safety",
        "active_has_tool",
        "is_winner",
        "turns_until_opponent_wins",
        "online_pokemon_count",
        "energy_distance_to_online",
    ];

    let search_params: Vec<&str> = if config.search_params.is_empty() {
        all_params.clone()
    } else {
        config.search_params.iter().map(|s| s.as_str()).collect()
    };

    // Generate all combinations recursively
    fn generate_recursive(
        current: value_functions::ValueFunctionParams,
        remaining_params: &[&str],
        grid_values: &[f64],
        baseline: &value_functions::ValueFunctionParams,
        combinations: &mut Vec<value_functions::ValueFunctionParams>,
    ) {
        if remaining_params.is_empty() {
            combinations.push(current);
            return;
        }

        let param_name = remaining_params[0];
        let rest = &remaining_params[1..];

        for &value in grid_values {
            let mut new_params = current;
            match param_name {
                "pokemon_value" => new_params.pokemon_value = value,
                "hand_size" => new_params.hand_size = value,
                "deck_size" => new_params.deck_size = value,
                "active_retreat_cost" => new_params.active_retreat_cost = value,
                "active_pokemon_online_score" => new_params.active_pokemon_online_score = value,
                "active_safety" => new_params.active_safety = value,
                "active_has_tool" => new_params.active_has_tool = value,
                "is_winner" => new_params.is_winner = value,
                "turns_until_opponent_wins" => new_params.turns_until_opponent_wins = value,
                "online_pokemon_count" => new_params.online_pokemon_count = value,
                "energy_distance_to_online" => new_params.energy_distance_to_online = value,
                _ => continue,
            }
            generate_recursive(new_params, rest, grid_values, baseline, combinations);
        }
    }

    generate_recursive(
        baseline,
        &search_params,
        &config.grid_values,
        &baseline,
        &mut combinations,
    );

    combinations
}

/// Generate random parameter combinations
fn generate_random_combinations(
    config: &OptimizationConfig,
    rng: &mut StdRng,
) -> Vec<value_functions::ValueFunctionParams> {
    let baseline = value_functions::ValueFunctionParams::baseline();
    let mut combinations = Vec::new();

    // Determine which parameters to search over
    let all_params = vec![
        "pokemon_value",
        "hand_size",
        "deck_size",
        "active_retreat_cost",
        "active_pokemon_online_score",
        "active_safety",
        "active_has_tool",
        "is_winner",
        "turns_until_opponent_wins",
        "online_pokemon_count",
        "energy_distance_to_online",
    ];

    let search_params: Vec<&str> = if config.search_params.is_empty() {
        all_params.clone()
    } else {
        config.search_params.iter().map(|s| s.as_str()).collect()
    };

    // Generate random combinations
    for _ in 0..config.budget {
        let mut params = baseline;

        for param_name in &search_params {
            // Sample in log space for better coverage
            let log_min = config.min_value.ln();
            let log_max = config.max_value.ln();
            let log_value = rng.gen_range(log_min..log_max);
            let value = log_value.exp();

            match *param_name {
                "pokemon_value" => params.pokemon_value = value,
                "hand_size" => params.hand_size = value,
                "deck_size" => params.deck_size = value,
                "active_retreat_cost" => params.active_retreat_cost = value,
                "active_pokemon_online_score" => params.active_pokemon_online_score = value,
                "active_safety" => params.active_safety = value,
                "active_has_tool" => params.active_has_tool = value,
                "is_winner" => params.is_winner = value,
                "turns_until_opponent_wins" => params.turns_until_opponent_wins = value,
                "online_pokemon_count" => params.online_pokemon_count = value,
                "energy_distance_to_online" => params.energy_distance_to_online = value,
                _ => continue,
            }
        }

        combinations.push(params);
    }

    combinations
}

fn print_result(result: &GridSearchResult) {
    let variant_win_rate = result.total_variant_wins as f64 / result.total_games as f64;
    let baseline_win_rate = result.total_baseline_wins as f64 / result.total_games as f64;
    let win_rate_diff = (variant_win_rate - baseline_win_rate) * 100.0;

    println!(
        "  Variant: {} wins ({:.1}%), Baseline: {} wins ({:.1}%), Diff: {:.1}%",
        result.total_variant_wins,
        variant_win_rate * 100.0,
        result.total_baseline_wins,
        baseline_win_rate * 100.0,
        win_rate_diff
    );
}

/// Run successive halving optimization
/// This adaptively allocates more resources to promising configurations
fn run_successive_halving(
    mut candidates: Vec<value_functions::ValueFunctionParams>,
    config: &OptimizationConfig,
) -> Result<Vec<GridSearchResult>, Box<dyn std::error::Error>> {
    let n = candidates.len();
    let mut results: Vec<GridSearchResult> = Vec::new();
    let mut num_games = (config.num_games as f64 * 0.25) as u32; // Start with 25% of games

    println!("\n{} Starting Successive Halving", "→".cyan());
    println!("{} Initial candidates: {}", "  →".cyan(), n);
    println!(
        "{} Starting with {} games per config",
        "  →".cyan(),
        num_games
    );

    let mut round = 1;
    while candidates.len() > 1 && num_games <= config.num_games {
        println!(
            "\n{} Round {}: Testing {} candidates with {} games each",
            "→".green().bold(),
            round,
            candidates.len(),
            num_games
        );

        // Create a temporary config with reduced num_games
        let mut temp_config = config.clone();
        temp_config.num_games = num_games;

        // Test all candidates in this round
        let mut round_results = Vec::new();
        for (idx, params) in candidates.iter().enumerate() {
            println!(
                "  {} [{}/{}] Testing configuration...",
                "→".cyan(),
                idx + 1,
                candidates.len()
            );

            let result = test_configuration(params, &temp_config)?;
            print_result(&result);
            round_results.push(result);
        }

        // Keep top half of candidates
        round_results.sort_by_key(|r| std::cmp::Reverse(r.total_variant_wins));
        let keep_n = (candidates.len() + 1) / 2; // Round up for odd numbers
        candidates = round_results
            .iter()
            .take(keep_n)
            .map(|r| r.params)
            .collect();
        results = round_results;

        println!(
            "  {} Keeping top {} candidates",
            "✓".green(),
            candidates.len()
        );

        // Double the number of games for next round
        num_games = (num_games * 2).min(config.num_games);
        round += 1;
    }

    // Final evaluation with full num_games if we haven't already
    if num_games < config.num_games {
        println!(
            "\n{} Final round: Evaluating best candidate with {} games",
            "→".green().bold(),
            config.num_games
        );
        let best_params = &candidates[0];
        let final_result = test_configuration(best_params, config)?;
        print_result(&final_result);
        results = vec![final_result];
    }

    Ok(results)
}

impl Clone for OptimizationConfig {
    fn clone(&self) -> Self {
        Self {
            deck_paths: self.deck_paths.clone(),
            depth: self.depth,
            num_games: self.num_games,
            seed: self.seed,
            parallel: self.parallel,
            grid_values: self.grid_values.clone(),
            search_params: self.search_params.clone(),
            strategy: self.strategy,
            budget: self.budget,
            min_value: self.min_value,
            max_value: self.max_value,
        }
    }
}

fn print_params(params: &value_functions::ValueFunctionParams) {
    println!("  pokemon_value: {}", params.pokemon_value);
    println!("  hand_size: {}", params.hand_size);
    println!("  deck_size: {}", params.deck_size);
    println!("  active_retreat_cost: {}", params.active_retreat_cost);
    println!(
        "  active_pokemon_online_score: {}",
        params.active_pokemon_online_score
    );
    println!("  active_safety: {}", params.active_safety);
    println!("  active_has_tool: {}", params.active_has_tool);
    println!("  is_winner: {}", params.is_winner);
    println!(
        "  turns_until_opponent_wins: {}",
        params.turns_until_opponent_wins
    );
    println!("  online_pokemon_count: {}", params.online_pokemon_count);
    println!(
        "  energy_distance_to_online: {}",
        params.energy_distance_to_online
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Initialize logger
    initialize_logger(args.verbosity);

    // Parse strategy
    let strategy = match args.strategy.to_lowercase().as_str() {
        "grid" => OptimizationStrategy::Grid,
        "random" => OptimizationStrategy::Random,
        "halving" | "sh" | "successive" => OptimizationStrategy::SuccessiveHalving,
        _ => {
            return Err(format!(
                "Invalid strategy '{}'. Use: grid, random, or halving",
                args.strategy
            )
            .into())
        }
    };

    // Parse grid values
    let grid_values: Vec<f64> = args
        .grid_values
        .split(',')
        .map(|s| s.trim().parse())
        .collect::<Result<Vec<_>, _>>()?;

    // Parse search parameters if provided
    let search_params: Vec<String> = args
        .search_params
        .map(|s| s.split(',').map(|p| p.trim().to_string()).collect())
        .unwrap_or_default();

    // Discover all deck files in the folder
    let deck_paths = discover_deck_files(&args.deck_folder)?;

    // Create RNG
    let mut rng = match args.seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_entropy(),
    };

    println!("\n{}", "=".repeat(70).blue().bold());
    println!(
        "{}",
        "HYPERPARAMETER OPTIMIZATION FOR VALUE FUNCTIONS"
            .blue()
            .bold()
    );
    println!("{}", "=".repeat(70).blue().bold());
    println!(
        "{} Found {} deck files in {}",
        "→".cyan(),
        deck_paths.len(),
        args.deck_folder
    );
    println!(
        "{} Strategy: {}",
        "→".cyan(),
        match strategy {
            OptimizationStrategy::Grid => "Grid Search",
            OptimizationStrategy::Random => "Random Search",
            OptimizationStrategy::SuccessiveHalving => "Successive Halving",
        }
    );

    let config = OptimizationConfig {
        deck_paths,
        depth: args.depth,
        num_games: args.num,
        seed: args.seed,
        parallel: args.parallel,
        grid_values: grid_values.clone(),
        search_params: search_params.clone(),
        strategy,
        budget: args.budget,
        min_value: args.min_value,
        max_value: args.max_value,
    };

    // Generate all parameter combinations
    let combinations = generate_param_combinations(&config, &mut rng);

    println!(
        "{} Testing {} parameter combinations",
        "→".cyan(),
        combinations.len()
    );

    if strategy == OptimizationStrategy::Grid {
        println!("{} Grid values: {:?}", "→".cyan(), grid_values);
    } else {
        println!(
            "{} Sampling range: [{}, {}] (log-uniform)",
            "→".cyan(),
            args.min_value,
            args.max_value
        );
    }

    if !search_params.is_empty() {
        println!(
            "{} Search parameters: {}",
            "→".cyan(),
            search_params.join(", ")
        );
    } else {
        println!("{} Searching all parameters", "→".cyan());
    }

    // Run optimization based on strategy
    let results = if strategy == OptimizationStrategy::SuccessiveHalving {
        run_successive_halving(combinations, &config)?
    } else {
        let mut results = Vec::new();
        let total_combinations = combinations.len();

        // Test each combination
        for (idx, params) in combinations.iter().enumerate() {
            println!(
                "\n{} [{}/{}] Testing configuration...",
                "→".cyan(),
                idx + 1,
                total_combinations
            );

            let result = test_configuration(params, &config)?;
            print_result(&result);
            results.push(result);
        }

        results
    };

    // Find best configuration
    let best = results
        .iter()
        .max_by_key(|r| r.total_variant_wins)
        .ok_or("No results found")?;

    println!("\n{}", "=".repeat(70).blue().bold());
    println!("{}", "BEST CONFIGURATION".green().bold());
    println!("{}", "=".repeat(70).blue().bold());
    print_params(&best.params);
    println!();
    print_result(best);

    // Print top 5 configurations
    let mut sorted_results = results.clone();
    sorted_results.sort_by_key(|r| std::cmp::Reverse(r.total_variant_wins));

    println!("\n{}", "=".repeat(70).blue().bold());
    println!("{}", "TOP 5 CONFIGURATIONS".blue().bold());
    println!("{}", "=".repeat(70).blue().bold());

    for (idx, result) in sorted_results.iter().take(5).enumerate() {
        println!("\n{} Rank #{}", "→".cyan(), idx + 1);
        print_params(&result.params);
        print_result(result);
    }

    Ok(())
}
