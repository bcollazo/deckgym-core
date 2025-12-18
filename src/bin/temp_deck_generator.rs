use std::collections::HashSet;

use clap::Parser;
use deckgym::card_ids::CardId;
use deckgym::database::get_card_by_enum;
use deckgym::models::{Card, EnergyType, PokemonCard};
use strum::IntoEnumIterator;

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
    let deck = generate_temp_deck(&card);
    println!("{}", deck);
}

/// Find a CardId by its ID string (e.g., "A1 003")
fn find_card_id(id: &str) -> Option<CardId> {
    for card_id in CardId::iter() {
        let card = get_card_by_enum(card_id);
        let card_id_str = card.get_id();
        if card_id_str == id {
            return Some(card_id);
        }
    }
    None
}

/// Get the full evolution line for a given card
/// Returns (Basic, Stage1, Stage2) where Stage1 and Stage2 may be None
fn get_evolution_line(card: &Card) -> (Option<Card>, Option<Card>, Option<Card>) {
    let Card::Pokemon(pokemon) = card else {
        return (None, None, None);
    };

    match pokemon.stage {
        0 => {
            // Basic Pokemon - find Stage 1 and Stage 2
            let stage1 = find_stage1_evolution(&pokemon.name);
            let stage2 = if let Some(Card::Pokemon(s1_pokemon)) = &stage1 {
                find_stage2_evolution(&s1_pokemon.name)
            } else {
                None
            };
            (Some(card.clone()), stage1, stage2)
        }
        1 => {
            // Stage 1 - find Basic and Stage 2
            let basic = pokemon
                .evolves_from
                .as_ref()
                .and_then(|name| find_card_by_name(name));
            let stage2 = find_stage2_evolution(&pokemon.name);
            (basic, Some(card.clone()), stage2)
        }
        2 => {
            // Stage 2 - find Stage 1 and Basic
            let stage1 = pokemon
                .evolves_from
                .as_ref()
                .and_then(|name| find_card_by_name(name));
            let basic = if let Some(Card::Pokemon(s1_pokemon)) = &stage1 {
                s1_pokemon
                    .evolves_from
                    .as_ref()
                    .and_then(|name| find_card_by_name(name))
            } else {
                None
            };
            (basic, stage1, Some(card.clone()))
        }
        _ => (None, None, None),
    }
}

/// Find a Stage 1 evolution for a given Basic Pokemon name
fn find_stage1_evolution(basic_name: &str) -> Option<Card> {
    for id in CardId::iter() {
        let card = get_card_by_enum(id);
        if let Card::Pokemon(pokemon) = &card {
            if pokemon.stage == 1 {
                if let Some(ref evolves_from) = pokemon.evolves_from {
                    if evolves_from == basic_name {
                        return Some(card);
                    }
                }
            }
        }
    }
    None
}

/// Find a Stage 2 evolution for a given Stage 1 Pokemon name
fn find_stage2_evolution(stage1_name: &str) -> Option<Card> {
    for id in CardId::iter() {
        let card = get_card_by_enum(id);
        if let Card::Pokemon(pokemon) = &card {
            if pokemon.stage == 2 {
                if let Some(ref evolves_from) = pokemon.evolves_from {
                    if evolves_from == stage1_name {
                        return Some(card);
                    }
                }
            }
        }
    }
    None
}

/// Find a card by exact name match
fn find_card_by_name(name: &str) -> Option<Card> {
    for id in CardId::iter() {
        let card = get_card_by_enum(id);
        let card_name = match &card {
            Card::Pokemon(pokemon) => &pokemon.name,
            Card::Trainer(trainer) => &trainer.name,
        };
        if card_name == name {
            return Some(card);
        }
    }
    None
}

/// Generate a temporary deck for testing based on the card's evolution stage
fn generate_temp_deck(card: &Card) -> String {
    let Card::Pokemon(pokemon) = card else {
        return format!(
            "Error: Only Pokemon cards are supported for deck generation. '{}' is a Trainer card.",
            card.get_name()
        );
    };

    let (basic, stage1, stage2) = get_evolution_line(card);

    // Collect all Pokemon in the evolution line
    let mut pokemon_list = Vec::new();
    if let Some(Card::Pokemon(p)) = &basic {
        pokemon_list.push(p);
    }
    if let Some(Card::Pokemon(p)) = &stage1 {
        pokemon_list.push(p);
    }
    if let Some(Card::Pokemon(p)) = &stage2 {
        pokemon_list.push(p);
    }

    // Calculate energy types from all attacks in the evolution line
    let energy_types = calculate_energy_types(&pokemon_list, pokemon.energy_type);

    match pokemon.stage {
        2 => {
            // Stage 2 deck template
            generate_stage2_deck(basic, stage1, stage2, &energy_types)
        }
        1 => {
            // Stage 1 deck template
            generate_stage1_deck(basic, stage1, &energy_types)
        }
        0 => {
            // Basic deck template
            generate_basic_deck(basic, &energy_types)
        }
        _ => String::from("Error: Unknown Pokemon stage"),
    }
}

/// Calculate energy types from all attacks in the Pokemon list
/// Returns a comma-separated string of energy types
fn calculate_energy_types(pokemon_list: &[&PokemonCard], fallback_type: EnergyType) -> String {
    let mut energy_set: HashSet<EnergyType> = HashSet::new();

    // Collect energy types from all Pokemon in the list
    for pokemon in pokemon_list {
        for attack in &pokemon.attacks {
            for energy in &attack.energy_required {
                energy_set.insert(*energy);
            }
        }
    }

    // Remove Colorless from the set
    energy_set.remove(&EnergyType::Colorless);

    // If empty (only had Colorless or no attacks), use fallback
    if energy_set.is_empty() {
        return fallback_type.as_str().to_string();
    }

    // Sort for consistent output
    let mut energy_vec: Vec<EnergyType> = energy_set.into_iter().collect();
    energy_vec.sort();

    // Format as comma-separated string
    energy_vec
        .iter()
        .map(|e| e.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}

fn format_card_line(card: &Card, count: u8) -> String {
    let id = card.get_id();
    let formatted_id = format_card_id(&id);

    match card {
        Card::Pokemon(pokemon) => format!("{} {} {}", count, pokemon.name, formatted_id),
        Card::Trainer(trainer) => format!("{} {} {}", count, trainer.name, formatted_id),
    }
}

/// Format a card ID by removing leading zeros from the number part
/// e.g., "A1 001" -> "A1 1", "A1 014" -> "A1 14", "P-A 007" -> "P-A 7"
fn format_card_id(id: &str) -> String {
    let parts: Vec<&str> = id.split_whitespace().collect();
    if parts.len() == 2 {
        let set = parts[0];
        let number = parts[1].parse::<u32>().unwrap_or(0);
        format!("{} {}", set, number)
    } else {
        id.to_string()
    }
}

fn generate_stage2_deck(
    basic: Option<Card>,
    stage1: Option<Card>,
    stage2: Option<Card>,
    energy_type: &str,
) -> String {
    let basic_line = if let Some(basic) = basic {
        format_card_line(&basic, 2)
    } else {
        "# ERROR: Could not find Basic Pokemon".to_string()
    };

    let stage1_line = if let Some(stage1) = stage1 {
        format_card_line(&stage1, 2)
    } else {
        "# ERROR: Could not find Stage 1 Pokemon".to_string()
    };

    let stage2_line = if let Some(stage2) = stage2 {
        format_card_line(&stage2, 2)
    } else {
        "# ERROR: Could not find Stage 2 Pokemon".to_string()
    };

    format!(
        r#"Energy: {energy_type}
{basic_line}
{stage1_line}
{stage2_line}
2 Giovanni A1 223
1 Sabrina A1 225
2 Giant Cape A2 147
1 Cyrus A2 150
2 Poké Ball A2b 111
2 Professor's Research A4b 373
2 Copycat B1 270
2 Potion P-A 1
"#
    )
}

fn generate_stage1_deck(basic: Option<Card>, stage1: Option<Card>, energy_type: &str) -> String {
    let basic_line = if let Some(basic) = basic {
        format_card_line(&basic, 2)
    } else {
        "# ERROR: Could not find Basic Pokemon".to_string()
    };

    let stage1_line = if let Some(stage1) = stage1 {
        format_card_line(&stage1, 2)
    } else {
        "# ERROR: Could not find Stage 1 Pokemon".to_string()
    };

    format!(
        r#"Energy: {energy_type}
{basic_line}
{stage1_line}
2 Giovanni A1 223
1 Sabrina A1 225
2 Giant Cape A2 147
1 Cyrus A2 150
2 Poké Ball A2b 111
2 Professor's Research A4b 373
2 Copycat B1 270
2 Potion P-A 1
2 X Speed P-A 2
"#
    )
}

fn generate_basic_deck(basic: Option<Card>, energy_type: &str) -> String {
    let basic_line = if let Some(basic) = basic {
        format_card_line(&basic, 2)
    } else {
        "# ERROR: Could not find Basic Pokemon".to_string()
    };

    format!(
        r#"Energy: {energy_type}
{basic_line}
2 Giovanni A1 223
2 Sabrina A1 225
2 Giant Cape A2 147
2 Cyrus A2 150
2 Poké Ball A2b 111
2 Professor's Research A4b 373
2 Copycat B1 270
2 Potion P-A 1
2 X Speed P-A 2
"#
    )
}
