use clap::{ArgAction, Parser, Subcommand};
use colored::Colorize;
use deckgym::players::{parse_player_code, PlayerCode};
use deckgym::{optimize, simulate};
use env_logger::{Builder, Env};
use log::warn;
use std::io::Write;

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

        /// Players' strategies as a comma-separated list
        #[arg(long, value_delimiter = ',', value_parser = parse_player_code)]
        players: Option<Vec<PlayerCode>>,

        /// Number of simulations to run
        #[arg(short, long)]
        num: u32,

        /// Seed for random number generation
        #[arg(short, long)]
        seed: Option<u64>,

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

        /// Players' strategies as a comma-separated list (e.g. "e,e")
        #[arg(long, value_delimiter = ',', value_parser = parse_player_code)]
        players: Option<Vec<PlayerCode>>,

        /// Seed for random number generation
        #[arg(short, long)]
        seed: Option<u64>,

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
            verbose,
        } => {
            initialize_logger(verbose);

            warn!("Welcome to {} simulation!", "deckgym".blue().bold());

            simulate(&deck_a, &deck_b, players, num, seed);
        }
        Commands::Optimize {
            incomplete_deck,
            candidate_cards,
            enemy_decks_folder,
            num,
            players,
            seed,
            verbose,
        } => {
            initialize_logger(verbose);

            warn!("Welcome to {} optimizer!", "deckgym".blue().bold());

            optimize(
                &incomplete_deck,
                &candidate_cards,
                &enemy_decks_folder,
                num,
                players,
                seed,
            );
        }
    }
}

// Set up the logger according to the given verbosity.
fn initialize_logger(verbose: u8) {
    let level = match verbose {
        1 => "warn",
        2 => "info",
        3 => "debug",
        _ => "trace",
    };
    Builder::from_env(Env::default().default_filter_or(level))
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .init();
}
