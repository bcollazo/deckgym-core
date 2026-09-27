use clap::{ArgAction, Parser, Subcommand};
use colored::Colorize;
use deckgym::optimize::{ParallelConfig, SimulationConfig};
use deckgym::players::{parse_player_code, PlayerCode};
use deckgym::simulate::initialize_logger;
use deckgym::state::GameOutcome;
use deckgym::{cli_optimize, play, simulate, Deck, PlayConfig};
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
        /// Available codes: aa, et, r, h, w, m, v, e<depth>, er, x (external bot, see --bot-a/-b)
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

        /// Folder to write one JSON replay file per game to, for the web viewer (see viewer/)
        #[arg(long)]
        replay_dir: Option<String>,

        /// Cap how many games get a replay written (requires --replay-dir). When simulating
        /// against a folder of decks, this cap applies per opponent deck rather than to the
        /// whole run.
        #[arg(long)]
        replay_sample: Option<usize>,

        /// Command to run as an external bot for player A (requires player A's code to be `x`).
        /// Spoken to over stdin/stdout — see docs/bot-protocol.md. Example:
        /// --players x,r --bot-a "python3 examples/bots/random_bot.py"
        #[arg(long)]
        bot_a: Option<String>,

        /// Same as --bot-a, for player B.
        #[arg(long)]
        bot_b: Option<String>,

        /// Per-decision timeout for an external bot, in milliseconds
        #[arg(long, default_value_t = deckgym::simulate::DEFAULT_BOT_TIMEOUT_MS)]
        bot_timeout_ms: u64,
    },
    /// Play exactly one game and print a URL to open it in the web viewer (see viewer/). A thin
    /// wrapper around `simulate`'s replay-recording path, for the common "play one game, look at
    /// it" loop: `cargo run -- play a.txt b.txt --players e2,r`, then Ctrl+Click the printed link.
    Play {
        /// Path to the first deck file
        deck_a: String,

        /// Path to the second deck file
        deck_b: String,

        /// Players' strategies as a comma-separated list (e.g., "e2,e4" or "r,e5")
        /// Available codes: aa, et, r, h, w, m, v, e<depth>, er, x (external bot, see --bot-a/-b)
        #[arg(long, value_delimiter = ',', value_parser = parse_player_code)]
        players: Option<Vec<PlayerCode>>,

        /// Seed for random number generation
        #[arg(short, long)]
        seed: Option<u64>,

        /// Command to run as an external bot for player A (requires player A's code to be `x`).
        #[arg(long)]
        bot_a: Option<String>,

        /// Same as --bot-a, for player B.
        #[arg(long)]
        bot_b: Option<String>,

        /// Per-decision timeout for an external bot, in milliseconds
        #[arg(long, default_value_t = deckgym::simulate::DEFAULT_BOT_TIMEOUT_MS)]
        bot_timeout_ms: u64,

        /// Folder to write the game's replay file into. The default matches what `viewer`'s dev
        /// server serves at `/replays/*` (see viewer/vite.config.ts) — the printed viewer URL
        /// always links to `/replays/<game_id>.json`, so a non-default folder here needs the
        /// viewer to also be pointed at it (e.g. `?url=` given manually) for the link to resolve.
        #[arg(long, default_value = "replays/")]
        replay_dir: String,

        /// Base URL of a running `viewer` dev server, used to build the printed link. Defaults to
        /// the `DECKGYM_VIEWER_URL` env var, then "http://localhost:5173".
        #[arg(long)]
        viewer_url: Option<String>,
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
    sim_config: SimulationConfig,
    parallel_config: ParallelConfig,
) {
    let total_num_simulations = sim_config.num_games;
    let players = sim_config.players;
    let seed = sim_config.seed;
    let data_output = sim_config.data_output;
    let replay_dir = sim_config.replay_dir;
    let replay_sample = sim_config.replay_sample;
    let bot_a = sim_config.bot_a;
    let bot_b = sim_config.bot_b;
    let bot_timeout_ms = sim_config.bot_timeout_ms;
    let parallel = parallel_config.enabled;
    let num_threads = parallel_config.num_threads;

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
        let deck_name = deck_path.split('/').next_back().unwrap_or(deck_path);
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
            SimulationConfig {
                num_games: games_for_this_deck,
                players: players.clone(),
                seed,
                data_output: data_output.clone(),
                replay_dir: replay_dir.clone(),
                replay_sample,
                bot_a: bot_a.clone(),
                bot_b: bot_b.clone(),
                bot_timeout_ms,
            },
            ParallelConfig {
                enabled: parallel,
                num_threads,
            },
        );
    }

    warn!("\n{}", "=".repeat(60));
    warn!("All simulations complete!");
    warn!("{}", "=".repeat(60));
}

/// Percent-encodes a string for use as one URL query-parameter *value* (same rule as JavaScript's
/// `encodeURIComponent`: everything except `A-Za-z0-9 - _ . ~` is escaped) — used for `play`'s
/// `--viewer-url` link, which has no other reason to pull in a URL-encoding crate.
fn percent_encode_query_value(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char)
            }
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

/// Builds the `?url=/replays/<game_id>.json` link `play` prints, pointing at a `viewer` dev server
/// serving the repo's `replays/` folder (see `viewer/vite.config.ts`'s replay-serving plugin, which
/// only serves that one folder — hence the literal `/replays/` prefix here regardless of
/// `--replay-dir`; see that flag's doc comment).
fn build_viewer_url(viewer_url: &str, game_id: &uuid::Uuid, cards_pattern: Option<&str>) -> String {
    let base = viewer_url.trim_end_matches('/');
    let mut url = format!("{base}/?url=/replays/{game_id}.json");
    if let Some(pattern) = cards_pattern {
        url.push_str("&cards=");
        url.push_str(&percent_encode_query_value(pattern));
    }
    url
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
            data_output,
            replay_dir,
            replay_sample,
            bot_a,
            bot_b,
            bot_timeout_ms,
        } => {
            initialize_logger(verbose);

            warn!("Welcome to {} simulation!", "deckgym".blue().bold());

            // Check if deck_b_or_folder is a directory
            let path = std::path::Path::new(&deck_b_or_folder);
            if path.is_dir() {
                simulate_against_folder(
                    &deck_a,
                    &deck_b_or_folder,
                    SimulationConfig {
                        num_games: num,
                        players,
                        seed,
                        data_output,
                        replay_dir,
                        replay_sample,
                        bot_a,
                        bot_b,
                        bot_timeout_ms,
                    },
                    ParallelConfig {
                        enabled: parallel,
                        num_threads: threads,
                    },
                );
            } else {
                simulate(
                    &deck_a,
                    &deck_b_or_folder,
                    SimulationConfig {
                        num_games: num,
                        players,
                        seed,
                        data_output,
                        replay_dir,
                        replay_sample,
                        bot_a,
                        bot_b,
                        bot_timeout_ms,
                    },
                    ParallelConfig {
                        enabled: parallel,
                        num_threads: threads,
                    },
                );
            }
        }
        Commands::Play {
            deck_a,
            deck_b,
            players,
            seed,
            bot_a,
            bot_b,
            bot_timeout_ms,
            replay_dir,
            viewer_url,
        } => {
            initialize_logger(1);

            warn!("Welcome to {} play!", "deckgym".blue().bold());

            let result = play(
                &deck_a,
                &deck_b,
                PlayConfig {
                    players,
                    seed,
                    bot_a,
                    bot_b,
                    bot_timeout_ms,
                    replay_dir,
                },
            );

            let winner_text = match result.outcome {
                Some(GameOutcome::Win(0)) => format!("Player 0 ({}) wins!", result.player_names[0]),
                Some(GameOutcome::Win(1)) => format!("Player 1 ({}) wins!", result.player_names[1]),
                Some(GameOutcome::Win(_)) => "Game over.".to_string(),
                Some(GameOutcome::Tie) => "Tie!".to_string(),
                None => "Game did not finish.".to_string(),
            };
            println!("{winner_text}");
            println!(
                "Points: {} ({}) - {} ({})",
                result.points[0], result.player_names[0], result.points[1], result.player_names[1]
            );
            println!("Replay written to: {}", result.replay_path.display());

            let viewer_base = viewer_url
                .or_else(|| std::env::var("DECKGYM_VIEWER_URL").ok())
                .unwrap_or_else(|| "http://localhost:5173".to_string());
            let cards_pattern = std::env::var("DECKGYM_CARD_IMAGE_URL").ok();
            println!(
                "{}",
                build_viewer_url(&viewer_base, &result.game_id, cards_pattern.as_deref())
            );
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
                replay_dir: None,
                replay_sample: None,
                bot_a: None,
                bot_b: None,
                bot_timeout_ms: deckgym::simulate::DEFAULT_BOT_TIMEOUT_MS,
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
