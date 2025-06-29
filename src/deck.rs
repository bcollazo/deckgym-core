use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashSet;
use std::fs;
use std::hash::{Hash, Hasher};

use crate::card_ids::CardId;
use crate::database::get_card_by_enum;
use crate::types::{Card, EnergyType};

/// Represents a deck of cards.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Deck {
    pub cards: Vec<Card>,
    pub(crate) energy_types: Vec<EnergyType>,
}

impl Hash for Deck {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cards.hash(state);
        // order energy types alphabetically to ensure consistent hash
        let mut energy_types: Vec<_> = self.energy_types.iter().collect();
        energy_types.sort();
        energy_types.hash(state);
    }
}

impl Deck {
    /// Parses a deck file and returns a `Deck` struct with cards flattened based on their counts.
    pub fn from_file(file_path: &str) -> Result<Self, String> {
        let contents = fs::read_to_string(file_path)
            .map_err(|err| format!("Failed to read file {file_path}: {err}"))?;

        Self::from_string(&contents)
    }

    pub fn from_string(contents: &str) -> Result<Self, String> {
        let mut energy_types = HashSet::new();
        let mut cards = Vec::new();
        for line in contents.lines() {
            // if line is empty or starts with "Pokemon:" or "Trainer:, skip it
            let trimmed = line.trim();
            if trimmed.is_empty()
                || trimmed.starts_with("Pokémon:")
                || trimmed.starts_with("Trainer:")
            {
                continue;
            }
            if trimmed.starts_with("Energy:") {
                let energy_type: &str = trimmed
                    .split_whitespace()
                    .last()
                    .expect("Energy: line should have an energy type");
                energy_types
                    .insert(EnergyType::from_str(energy_type).expect("Invalid energy type"));
                continue;
            }

            let (count, card) = Card::from_str_with_count(trimmed)?;
            cards.extend(vec![card; count as usize]);
        }

        // If empty energy types set, populate it with the energy types from the cards
        if energy_types.is_empty() {
            cards.iter().for_each(|x| {
                if let Card::Pokemon(pokemon_card) = x {
                    energy_types.insert(pokemon_card.energy_type);
                }
            });
        }

        Ok(Self {
            cards,
            energy_types: energy_types.into_iter().collect(),
        })
    }

    pub fn is_valid(&self) -> bool {
        let basic = self.cards.iter().filter(|x| x.is_basic()).count();
        self.cards.len() == 20 && basic >= 1
    }

    /// Draws a card from the deck.
    /// Returns `Some(Card)` if the deck is not empty, otherwise returns `None`.
    pub fn draw(&mut self) -> Option<Card> {
        if self.cards.is_empty() {
            None
        } else {
            Some(self.cards.remove(0))
        }
    }

    /// Shuffles the deck of cards.
    pub fn shuffle(&mut self, initial_shuffle: bool, rng: &mut impl Rng) {
        if initial_shuffle {
            // Ensure there is at least 1 basic pokemon in the initial 5 cards
            let (mut matching, mut non_matching): (Vec<_>, Vec<_>) =
                self.cards.clone().into_iter().partition(is_basic);
            matching.shuffle(rng);
            non_matching.shuffle(rng);

            let shuffled_cards: Vec<Card> =
                vec![matching.pop().expect("Decks must have at least 1 basic")];

            let mut remaining = [matching, non_matching].concat();
            remaining.shuffle(rng);

            self.cards = [shuffled_cards, remaining].concat();
        } else {
            self.cards.shuffle(rng);
        }
    }
}

impl Card {
    /// Parses a line and returns a tuple of count and a `Card`.
    pub fn from_str_with_count(line: &str) -> Result<(u32, Card), String> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            return Err(format!("Invalid card format: {line}"));
        }

        let count = parts[0]
            .parse::<u32>()
            .map_err(|_| format!("Invalid count: {}", parts[0]))?;
        let set = parts[parts.len() - 2];
        // maybe pad number with 0 on the left if missing 0s
        let number = parts[parts.len() - 1];
        let padded_number = format!("{number:0>3}");
        let id = format!("{set} {padded_number}");

        let card_id =
            CardId::from_card_id(&id).ok_or_else(|| format!("Card ID not found for id: {id}"))?;
        let card = get_card_by_enum(card_id);

        Ok((count, card.clone()))
    }
}

pub fn is_basic(card: &Card) -> bool {
    card.is_basic()
}

#[cfg(test)]
mod tests {
    use rand::thread_rng;

    use super::*;

    #[test]
    fn test_deck_from_file() {
        let file_path = "example_decks/venusaur-exeggutor.txt";
        let deck = Deck::from_file(file_path).expect("Failed to parse deck from file");

        // Add assertions to verify the deck content
        assert!(!deck.cards.is_empty(), "Deck should not be empty");

        // Example assertion: Check if the first card is Bulbasaur
        let first_card = &deck.cards[0];
        if let Card::Pokemon(card) = first_card {
            assert_eq!(card.name, "Bulbasaur");
        } else {
            panic!("Expected first card to be Bulbasaur");
        }
    }

    #[test]
    fn test_draw_card() {
        let file_path = "example_decks/venusaur-exeggutor.txt";
        let mut deck = Deck::from_file(file_path).expect("Failed to parse deck from file");

        let initial_count = deck.cards.len();
        let drawn_card = deck.draw();
        assert!(drawn_card.is_some(), "Should draw a card");
        assert_eq!(
            deck.cards.len(),
            initial_count - 1,
            "Deck size should decrease by 1"
        );
    }

    #[test]
    fn test_initial_shuffle_deck() {
        let file_path = "example_decks/venusaur-exeggutor.txt";
        let mut deck = Deck::from_file(file_path).expect("Failed to parse deck from file");

        let mut rng = thread_rng();
        deck.shuffle(true, &mut rng);

        // Ensure there is at least 1 basic pokemon in the initial 5 cards
        let initial_five_cards = &deck.cards[..5];
        assert!(
            initial_five_cards.iter().any(is_basic),
            "There should be at least 1 basic pokemon in the initial 5 cards"
        );
    }

    #[test]
    fn test_from_string() {
        let string = r#"Pokémon: 8
2 Ekans A1 164
2 Arbok A1 165
2 Koffing A1 176
2 Weezing A1 177

Trainer: 12
2 Professor's Research P-A 007
2 Koga A1 222
2 Poké Ball P-A 005
2 Sabrina A1 225
2 Potion P-A 001
1 X Speed P-A 002
1 Giovanni A1 223"#;
        let deck = Deck::from_string(string).expect("Failed to parse deck from string");

        assert_eq!(deck.cards.len(), 20);
    }

    #[test]
    fn test_from_string_with_energy() {
        let string = r#"Energy: Grass
Pokémon: 8
2 Ekans A1 164
2 Arbok A1 165
2 Koffing A1 176
2 Weezing A1 177

Trainer: 12
2 Professor's Research P-A 007
2 Koga A1 222
2 Poké Ball P-A 005
2 Sabrina A1 225
2 Potion P-A 001
1 X Speed P-A 002
1 Giovanni A1 223"#;
        let deck = Deck::from_string(string).expect("Failed to parse deck from string");

        assert_eq!(deck.cards.len(), 20);
        assert_eq!(deck.energy_types.len(), 1);
        assert_eq!(deck.energy_types[0], EnergyType::Grass);
    }

    #[test]
    fn test_from_string_without_leading_zeros() {
        let string = r#"Energy: Grass
2 Weedle A1 8
2 Kakuna A1 9
2 Beedrill A1 10
2 Cottonee A1 27
2 Whimsicott A1 28
1 Giovanni A1 270
1 Sabrina A1 272
2 Potion P-A 1
2 X Speed P-A 2
2 Poke Ball P-A 5
2 Professor's Research P-A 7"#;
        let deck = Deck::from_string(string).expect("Failed to parse deck from string");
        assert_eq!(deck.cards.len(), 20);
    }
}
