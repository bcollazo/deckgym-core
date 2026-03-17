use clap::Parser;
use deckgym::database::get_card_by_enum;
use deckgym::temp_deck::{find_card_id, generate_temp_deck};
use deckgym::Deck;

#[derive(Parser, Debug)]
#[command(name = "temp_deck_generator")]
#[command(about = "Generate a temporary deck for testing given a card", long_about = None)]
struct Args {
    /// Card ID (e.g., "A1 003")
    card_id: String,
}

fn main() {
    let args = Args::parse();

    // Find the CardId enum variant by ID string
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
            if deck.is_valid() {
                println!("{}", deck_string);
            } else {
                eprintln!("Error: Generated deck is invalid");
                eprintln!("Deck must have exactly 20 cards, at least 1 basic Pokemon,");
                eprintln!("and no card name can appear more than twice.");
                eprintln!("\nGenerated deck:");
                eprintln!("{}", deck_string);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to parse generated deck: {}", e);
            eprintln!("\nGenerated deck:");
            eprintln!("{}", deck_string);
            std::process::exit(1);
        }
    }
}
