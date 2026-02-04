use clap::Parser;
use colored::Colorize;
use deckgym::database::get_card_by_enum;
use deckgym::simulate::initialize_logger;
use deckgym::temp_deck::{find_card_id, generate_temp_deck};
use deckgym::{simulate, Deck};
use log::warn;
use num_format::{Locale, ToFormattedString};
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(name = "card_test")]
#[command(
    about = "Generate a temp deck for a card and simulate games against example decks",
    long_about = None
)]
struct Args {
    /// Card ID (e.g., "A1 003")
    card_id: String,
}

fn main() {
    let args = Args::parse();

    let card_id = match find_card_id(&args.card_id) {
        Some(id) => id,
        None => {
            eprintln!("Error: Card ID '{}' not found", args.card_id);
            eprintln!("Please provide a valid card ID (e.g., 'A1 003')");
            std::process::exit(1);
        }
    };

    let card = get_card_by_enum(card_id);
    let deck_string = generate_temp_deck(&card);

    // Validate the generated deck
    match Deck::from_string(&deck_string) {
        Ok(deck) => {
            if !deck.is_valid() {
                eprintln!("Error: Generated deck is invalid");
                eprintln!("Deck must have exactly 20 cards, at least 1 basic Pokemon,");
                eprintln!("and no card name can appear more than twice.");
                eprintln!("\nGenerated deck:");
                eprintln!("{deck_string}");
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to parse generated deck: {e}");
            eprintln!("\nGenerated deck:");
            eprintln!("{deck_string}");
            std::process::exit(1);
        }
    }

    let mut deck_path = std::env::temp_dir();
    let sanitized_id = args.card_id.replace(' ', "_");
    deck_path.push(format!("deckgym_card_test_{sanitized_id}.txt"));
    if let Err(e) = fs::write(&deck_path, deck_string) {
        eprintln!("Error: Failed to write temp deck file: {e}");
        std::process::exit(1);
    }

    initialize_logger(1);
    warn!("Welcome to {} card test!", "deckgym".blue().bold());
    warn!("Temp deck: {}", deck_path.display());

    let example_decks = Path::new("example_decks");
    if !example_decks.is_dir() {
        eprintln!(
            "Error: example_decks folder not found at {}",
            example_decks.display()
        );
        std::process::exit(1);
    }

    simulate_against_folder(
        deck_path.to_str().expect("Temp deck path should be valid"),
        example_decks
            .to_str()
            .expect("Example decks path should be valid"),
        10_000,
    );
}

/// Simulate games between one deck and multiple decks in a folder.
fn simulate_against_folder(deck_a_path: &str, decks_folder: &str, total_num_simulations: u32) {
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
            None,
            games_for_this_deck,
            None,
            false,
            None,
        );
    }

    warn!("\n{}", "=".repeat(60));
    warn!("All simulations complete!");
    warn!("{}", "=".repeat(60));
}
