use std::collections::HashSet;

use crate::card_ids::CardId;
use crate::database::get_card_by_enum;
use crate::models::{Card, EnergyType, PokemonCard, TrainerType};
use strum::IntoEnumIterator;

/// Find a CardId by its ID string (e.g., "A1 003")
pub fn find_card_id(id: &str) -> Option<CardId> {
    for card_id in CardId::iter() {
        let card = get_card_by_enum(card_id);
        let card_id_str = card.get_id();
        if card_id_str == id {
            return Some(card_id);
        }
    }
    None
}

/// Generate a temporary deck for testing based on the card type.
pub fn generate_temp_deck(card: &Card) -> String {
    match card {
        Card::Pokemon(pokemon) => {
            let (basic, stage1, stage2) = get_evolution_line(card);
            let evolution_line = vec![basic, stage1, stage2];
            let fallback = get_valid_fallback_energy(pokemon.energy_type);
            generate_evolution_line_deck(&evolution_line, fallback)
        }
        Card::Trainer(trainer) => {
            if trainer.trainer_card_type == TrainerType::Fossil {
                generate_fossil_deck(card)
            } else {
                generate_trainer_deck(card, &trainer.trainer_card_type)
            }
        }
    }
}

/// Get the full evolution line for a given card.
/// Returns (Basic, Stage1, Stage2) where Stage1 and Stage2 may be None.
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

/// Find a Stage 1 evolution for a given Basic Pokemon name.
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

/// Find a Stage 2 evolution for a given Stage 1 Pokemon name.
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

/// Find a card by exact name match.
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

/// Get a valid fallback energy type (Water if Colorless or Dragon).
fn get_valid_fallback_energy(energy_type: EnergyType) -> EnergyType {
    if energy_type == EnergyType::Colorless || energy_type == EnergyType::Dragon {
        EnergyType::Water
    } else {
        energy_type
    }
}

/// Generate a deck for a Fossil card.
fn generate_fossil_deck(fossil_card: &Card) -> String {
    let fossil_name = fossil_card.get_name();

    // Find Pokemon that evolve from this fossil and get the highest evolution
    let highest_evolution = find_highest_fossil_evolution(&fossil_name);

    match highest_evolution {
        Some(pokemon_card) => {
            if let Card::Pokemon(pokemon) = &pokemon_card {
                // Get the evolution line (this will include the fossil as "basic")
                let (_, stage1, stage2) = get_evolution_line(&pokemon_card);

                // Build evolution line with fossil instead of basic
                let evolution_line = vec![Some(fossil_card.clone()), stage1, stage2];
                let fallback = get_valid_fallback_energy(pokemon.energy_type);
                generate_evolution_line_deck(&evolution_line, fallback)
            } else {
                "Error: Unexpected card type for fossil evolution".to_string()
            }
        }
        None => format!(
            "Error: No Pokemon found that evolves from '{}' fossil",
            fossil_name
        ),
    }
}

/// Find the highest evolution Pokemon that can evolve from a fossil.
fn find_highest_fossil_evolution(fossil_name: &str) -> Option<Card> {
    // Find all Pokemon that directly evolve from this fossil
    let direct_evolutions: Vec<Card> = CardId::iter()
        .map(get_card_by_enum)
        .filter(|card| {
            if let Card::Pokemon(pokemon) = card {
                pokemon
                    .evolves_from
                    .as_ref()
                    .is_some_and(|name| name == fossil_name)
            } else {
                false
            }
        })
        .collect();

    // Find the highest stage evolution from the direct evolutions
    direct_evolutions
        .iter()
        .filter_map(find_highest_evolution)
        .max_by_key(|card| {
            if let Card::Pokemon(pokemon) = card {
                pokemon.stage
            } else {
                0
            }
        })
}

/// Find the highest evolution stage for a given Pokemon.
fn find_highest_evolution(card: &Card) -> Option<Card> {
    let Card::Pokemon(pokemon) = card else {
        return None;
    };

    // Check if there's a Stage 2 evolution
    if let Some(stage2) = find_stage2_evolution(&pokemon.name) {
        return Some(stage2);
    }

    // Check if there's a Stage 1 evolution
    if let Some(stage1) = find_stage1_evolution(&pokemon.name) {
        // Recursively check if that Stage 1 has a Stage 2
        return find_highest_evolution(&stage1);
    }

    // This is the highest evolution
    Some(card.clone())
}

/// Calculate energy types from all attacks in the Pokemon list.
/// Returns a comma-separated string of energy types.
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

/// Format a card ID by removing leading zeros from the number part.
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

/// Unified deck generation for evolution lines (works for both Pokemon and Fossils).
/// Takes a list of cards in evolution order and generates an appropriate deck.
fn generate_evolution_line_deck(
    evolution_line: &[Option<Card>],
    fallback_energy: EnergyType,
) -> String {
    // Collect all Pokemon cards from the evolution line
    let pokemon_cards: Vec<&Card> = evolution_line.iter().filter_map(|c| c.as_ref()).collect();

    // Check if there are any basic Pokemon in the evolution line
    let has_basic = pokemon_cards.iter().any(|card| card.is_basic());

    // Extract PokemonCard refs for energy calculation
    let pokemon_list: Vec<&PokemonCard> = pokemon_cards
        .iter()
        .filter_map(|c| {
            if let Card::Pokemon(p) = c {
                Some(p)
            } else {
                None
            }
        })
        .collect();

    // Calculate energy types
    let energy_type = calculate_energy_types(&pokemon_list, fallback_energy);

    // Generate card lines for all cards in the evolution line
    let mut card_lines: Vec<String> = pokemon_cards
        .iter()
        .map(|card| format_card_line(card, 2))
        .collect();

    // If no basic Pokemon, add Meowth as a basic
    if !has_basic {
        let meowth = get_card_by_enum(CardId::A1196Meowth);
        card_lines.push(format_card_line(&meowth, 2));
    }

    // Count how many cards we have to determine what trainers to include
    let num_cards = card_lines.len();

    // Build the deck output
    let mut deck = format!("Energy: {energy_type}\n");

    // Add all evolution line cards
    for line in card_lines {
        deck.push_str(&line);
        deck.push('\n');
    }

    // Add 10 trainer cards
    deck.push_str("2 Giovanni A1 223\n");
    deck.push_str("2 Poké Ball A2b 111\n");
    deck.push_str("2 Professor's Research A4b 373\n");
    deck.push_str("2 Copycat B1 270\n");
    deck.push_str("2 Giant Cape A2 147\n");

    // Adjust trainers based on number of Pokemon cards
    match num_cards {
        4 => {
            // 4 Pokemon cards (fossil + 2 evolutions + Meowth) = 8 cards
            // Need 2 more trainer cards
            deck.push_str("1 Sabrina A1 225\n");
            deck.push_str("1 Cyrus A2 150\n");
        }
        3 => {
            // 3 Pokemon cards = 6 cards
            // Need 4 more trainer cards
            deck.push_str("2 Sabrina A1 225\n");
            deck.push_str("2 Cyrus A2 150\n");
        }
        2 => {
            // 2 Pokemon cards = 4 cards
            // Need 6 more trainer cards
            deck.push_str("2 Sabrina A1 225\n");
            deck.push_str("2 Cyrus A2 150\n");
            deck.push_str("2 Potion P-A 1\n");
        }
        _ => {
            // 1 Pokemon card = 2 cards
            // Need 8 more trainer cards
            deck.push_str("2 Sabrina A1 225\n");
            deck.push_str("2 Cyrus A2 150\n");
            deck.push_str("2 Potion P-A 1\n");
            deck.push_str("2 X Speed P-A 2\n");
        }
    }

    deck
}

/// Generate a deck for a Trainer card (18T template).
fn generate_trainer_deck(card: &Card, trainer_type: &TrainerType) -> String {
    let card_line = format_card_line(card, 2);

    match trainer_type {
        TrainerType::Fossil => {
            panic!("Fossils should be handled by generate_fossil_deck")
        }
        TrainerType::Tool => {
            // Replace Giant Cape
            format!(
                r#"Energy: Lightning
2 Tapu Koko ex A3a 19
2 Giovanni A1 223
2 Sabrina A1 225
2 Cyrus A2 150
2 Poké Ball A2b 111
2 Professor's Research A4b 373
2 Copycat B1 270
2 Potion P-A 1
{card_line}
"#
            )
        }
        TrainerType::Item => {
            // Replace X Speed with the requested item/tool/fossil
            format!(
                r#"Energy: Lightning
2 Tapu Koko ex A3a 19
2 Giovanni A1 223
2 Sabrina A1 225
2 Giant Cape A2 147
2 Cyrus A2 150
2 Poké Ball A2b 111
2 Professor's Research A4b 373
2 Copycat B1 270
2 Potion P-A 1
{card_line}
"#
            )
        }
        TrainerType::Supporter => {
            // Replace Sabrina with the requested supporter
            format!(
                r#"Energy: Lightning
2 Tapu Koko ex A3a 19
2 Giovanni A1 223
{card_line}
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
        TrainerType::Stadium => {
            // Replace X Speed with the requested stadium
            format!(
                r#"Energy: Lightning
2 Tapu Koko ex A3a 19
2 Giovanni A1 223
2 Sabrina A1 225
2 Giant Cape A2 147
2 Cyrus A2 150
2 Poké Ball A2b 111
2 Professor's Research A4b 373
2 Copycat B1 270
2 Potion P-A 1
{card_line}
"#
            )
        }
    }
}
