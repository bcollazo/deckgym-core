use clap::{ArgAction, Parser, Subcommand};
use colored::Colorize;
use deckgym::optimize::{ParallelConfig, SimulationConfig};
use deckgym::players::{parse_player_code, PlayerCode};
use deckgym::simulate::initialize_logger;
use deckgym::{cli_optimize, simulate};
use log::warn;
use num_format::{Locale, ToFormattedString};
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Simulate games between two decks (or one deck against multiple decks in a folder)
    Simulate {
        /// Path to the first deck file
        deck_a: String,

        /// Path to the second deck file or folder containing multiple deck files
        deck_b_or_folder: String,

        /// Players' strategies as a comma-separated list (e.g., "e2,e4" or "r,e5")
        /// Available codes: aa, et, r, h, w, m, v, e<depth>, er
        /// Example: e2 = ExpectiMiniMax with depth 2
        #[arg(long, value_delimiter = ',', value_parser = parse_player_code)]
        players: Option<Vec<PlayerCode>>,

        /// Number of simulations to run
        #[arg(short, long)]
        num: u32,

        /// Seed for random number generation
        #[arg(short, long)]
        seed: Option<u64>,

        /// Run simulations in parallel
        #[arg(short, long, default_value_t = false)]
        parallel: bool,

        /// Number of threads to use (defaults to number of CPU cores if not specified)
        #[arg(short = 'j', long)]
        threads: Option<usize>,

        /// Increase verbosity (-v, -vv, -vvv, etc.)
        #[arg(short, long, action = ArgAction::Count, default_value_t = 1)]
        verbose: u8,
    },
    /// Optimize an incomplete deck against enemy decks
    Optimize {
        /// Path to the incomplete deck file (missing up to 4 cards)
        incomplete_deck: String,

        /// Comma-separated list of candidate card IDs for completion
        candidate_cards: String,

        /// Folder containing enemy deck files
        enemy_decks_folder: String,

        /// Number of simulations to run per enemy deck for each combination
        #[arg(short, long)]
        num: u32,

        /// Players' strategies as a comma-separated list (e.g., "e2,e4" or "r,e5")
        /// Available codes: aa, et, r, h, w, m, v, e<depth>, er
        /// Example: e2 = ExpectiMiniMax with depth 2
        #[arg(long, value_delimiter = ',', value_parser = parse_player_code)]
        players: Option<Vec<PlayerCode>>,

        /// Seed for random number generation
        #[arg(short, long)]
        seed: Option<u64>,

        /// Run simulations in parallel
        #[arg(short, long, default_value_t = false)]
        parallel: bool,

        /// Number of threads to use (defaults to number of CPU cores if not specified)
        #[arg(short = 'j', long)]
        threads: Option<usize>,

        /// Increase verbosity (-v, -vv, -vvv, etc.)
        #[arg(short, long, action = ArgAction::Count, default_value_t = 1)]
        verbose: u8,
    },
}

/// Simulate games between one deck and multiple decks in a folder
fn simulate_against_folder(
    deck_a_path: &str,
    decks_folder: &str,
    players: Option<Vec<PlayerCode>>,
    total_num_simulations: u32,
    seed: Option<u64>,
    parallel: bool,
    num_threads: Option<usize>,
) {
    use deckgym::Deck;

    // Read all deck files from the folder
    let deck_paths: Vec<String> = fs::read_dir(decks_folder)
        .expect("Failed to read decks folder")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.path().is_file() {
                Some(entry.path().to_str()?.to_string())
            } else {
                None
            }
        })
        .collect();

    // Load and validate decks
    let valid_decks: Vec<(String, Deck)> = deck_paths
        .iter()
        .filter_map(|path| {
            let deck = Deck::from_file(path).ok()?;
            if deck.cards.len() == 20 {
                Some((path.clone(), deck))
            } else {
                warn!("Skipping deck {} (invalid)", path);
                None
            }
        })
        .collect();

    if valid_decks.is_empty() {
        warn!("No valid decks found in folder: {}", decks_folder);
        return;
    }

    warn!(
        "Found {} valid deck files in folder",
        valid_decks.len().to_formatted_string(&Locale::en)
    );

    // Calculate games per deck (distribute evenly)
    let num_decks = valid_decks.len() as u32;
    let games_per_deck = total_num_simulations / num_decks;
    let remainder = total_num_simulations % num_decks;

    warn!(
        "Running {} total games ({} per deck)",
        total_num_simulations.to_formatted_string(&Locale::en),
        games_per_deck.to_formatted_string(&Locale::en)
    );

    // Run simulations against each deck
    for (i, (deck_path, _)) in valid_decks.iter().enumerate() {
        let deck_name = deck_path.split('/').last().unwrap_or(deck_path);
        let games_for_this_deck = if i < remainder as usize {
            games_per_deck + 1
        } else {
            games_per_deck
        };

        if games_for_this_deck == 0 {
            continue;
        }

        warn!("\n{}", "=".repeat(60));
        warn!(
            "Simulating against deck {}/{}: {}",
            i + 1,
            num_decks,
            deck_name
        );
        warn!("{}", "=".repeat(60));

        simulate(
            deck_a_path,
            deck_path,
            players.clone(),
            games_for_this_deck,
            seed,
            parallel,
            num_threads,
        );
    }

    warn!("\n{}", "=".repeat(60));
    warn!("All simulations complete!");
    warn!("{}", "=".repeat(60));
}

fn main() {
    let cli = Cli::parse();

    // Branch depending on the chosen subcommand.
    match cli.command {
        Commands::Simulate {
            deck_a,
            deck_b_or_folder,
            players,
            num,
            seed,
            parallel,
            threads,
            verbose,
        } => {
            initialize_logger(verbose);

            warn!("Welcome to {} simulation!", "deckgym".blue().bold());

            // Check if deck_b_or_folder is a directory
            let path = std::path::Path::new(&deck_b_or_folder);
            if path.is_dir() {
                simulate_against_folder(&deck_a, &deck_b_or_folder, players, num, seed, parallel, threads);
            } else {
                simulate(&deck_a, &deck_b_or_folder, players, num, seed, parallel, threads);
            }
        }
        Commands::Optimize {
            incomplete_deck,
            candidate_cards,
            enemy_decks_folder,
            num,
            players,
            seed,
            parallel,
            threads,
            verbose,
        } => {
            initialize_logger(verbose);

            warn!("Welcome to {} optimizer!", "deckgym".blue().bold());

            let sim_config = SimulationConfig {
                num_games: num,
                players,
                seed,
            };
            let parallel_config = ParallelConfig {
                enabled: parallel,
                num_threads: threads,
            };

            cli_optimize(
                &incomplete_deck,
                &candidate_cards,
                &enemy_decks_folder,
                sim_config,
                parallel_config,
            );
        }
    }
}
