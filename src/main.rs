use clap::{ArgAction, Parser, Subcommand};
use colored::Colorize;
use deckgym::optimize::{ParallelConfig, SimulationConfig};
use deckgym::players::{parse_player_code, PlayerCode};
use deckgym::simulate::initialize_logger;
use deckgym::{cli_optimize, simulate};
use log::warn;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Simulate games between two decks
    Simulate {
        /// Path to the first deck file
        deck_a: String,

        /// Path to the second deck file
        deck_b: String,

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

        /// Output folder for exporting (state, action) pairs in JSON format
        #[arg(long)]
        data_output: Option<String>,
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

fn main() {
    let cli = Cli::parse();

    // Branch depending on the chosen subcommand.
    match cli.command {
        Commands::Simulate {
            deck_a,
            deck_b,
            players,
            num,
            seed,
            parallel,
            threads,
            verbose,
            data_output,
        } => {
            initialize_logger(verbose);

            warn!("Welcome to {} simulation!", "deckgym".blue().bold());

            let sim_config = SimulationConfig {
                num_games: num,
                players,
                seed,
                data_output,
            };

            let parallel_config = ParallelConfig {
                enabled: parallel,
                num_threads: threads,
            };

            simulate(&deck_a, &deck_b, sim_config, parallel_config);
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
                data_output: None,
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
