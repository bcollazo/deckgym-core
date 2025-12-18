use std::collections::HashMap;
use std::sync::LazyLock;

use strum::IntoEnumIterator;

use crate::{
    card_ids::CardId,
    database::get_card_by_enum,
    hooks::get_stage,
    models::{Card, PlayedCard},
};

/// Pre-computed lookup table: Basic Pokemon Name -> Vec<Stage 1 Pokemon Names>
static STAGE1_LOOKUP: LazyLock<HashMap<String, Vec<String>>> = LazyLock::new(|| {
    let mut lookup: HashMap<String, Vec<String>> = HashMap::new();

    for id in CardId::iter() {
        let card = get_card_by_enum(id);
        if let Card::Pokemon(pokemon_card) = card {
            if pokemon_card.stage == 1 {
                if let Some(evolves_from) = &pokemon_card.evolves_from {
                    lookup
                        .entry(evolves_from.clone())
                        .or_default()
                        .push(pokemon_card.name.clone());
                }
            }
        }
    }

    lookup
});

/// Pre-computed lookup table: Stage 1 Pokemon Name -> Vec<Stage 2 Pokemon Names>
static STAGE2_LOOKUP: LazyLock<HashMap<String, Vec<String>>> = LazyLock::new(|| {
    let mut lookup: HashMap<String, Vec<String>> = HashMap::new();

    for id in CardId::iter() {
        let card = get_card_by_enum(id);
        if let Card::Pokemon(pokemon_card) = card {
            if pokemon_card.stage == 2 {
                if let Some(evolves_from) = &pokemon_card.evolves_from {
                    lookup
                        .entry(evolves_from.clone())
                        .or_default()
                        .push(pokemon_card.name.clone());
                }
            }
        }
    }

    lookup
});

/// Pre-computed lookup table: (Basic Pokemon Name, Stage 2 Pokemon Name) -> is valid Rare Candy evolution
/// This is computed once at startup and cached for O(1) lookups.
static RARE_CANDY_LOOKUP: LazyLock<HashMap<(String, String), bool>> = LazyLock::new(|| {
    let mut lookup = HashMap::new();

    // Build a map of Stage 1 Pokemon: name -> evolves_from
    let mut stage1_map: HashMap<String, String> = HashMap::new();

    for id in CardId::iter() {
        let card = get_card_by_enum(id);
        if let Card::Pokemon(pokemon_card) = card {
            if pokemon_card.stage == 1 {
                if let Some(evolves_from) = pokemon_card.evolves_from {
                    stage1_map.insert(pokemon_card.name.clone(), evolves_from);
                }
            }
        }
    }

    // Now iterate through all Stage 2 Pokemon and build the lookup table
    for id in CardId::iter() {
        let card = get_card_by_enum(id);
        if let Card::Pokemon(stage2_pokemon) = card {
            if stage2_pokemon.stage == 2 {
                if let Some(stage1_name) = &stage2_pokemon.evolves_from {
                    // Check if this Stage 1 exists and what it evolves from
                    if let Some(basic_name) = stage1_map.get(stage1_name) {
                        lookup.insert((basic_name.clone(), stage2_pokemon.name.clone()), true);
                    }
                }
            }
        }
    }

    lookup
});

/// Check if a Stage 2 Pokemon can evolve from a Basic Pokemon using Rare Candy
/// Optimized version using pre-computed lookup table - O(1) instead of O(n)
pub fn can_rare_candy_evolve(stage2_card: &Card, basic_pokemon: &PlayedCard) -> bool {
    if let Card::Pokemon(stage2_pokemon) = stage2_card {
        // Early validation checks
        if stage2_pokemon.stage != 2
            || get_stage(basic_pokemon) != 0
            || basic_pokemon.played_this_turn
        {
            return false;
        }

        // O(1) lookup in pre-computed table
        let key = (basic_pokemon.get_name(), stage2_pokemon.name.clone());
        RARE_CANDY_LOOKUP.get(&key).copied().unwrap_or(false)
    } else {
        false
    }
}

/// Given a card and a collection of available cards, returns the highest evolution
/// cards that can be reached from the given card.
///
/// # Arguments
/// * `card` - The card to check evolutions for
/// * `available_cards` - Cards available in deck + hand
///
/// # Returns
/// Vector of the highest stage Cards that can be evolved into. Returns the card itself if Stage 2.
pub fn get_highest_evolutions(card: &Card, available_cards: &[Card]) -> Vec<Card> {
    let Card::Pokemon(pokemon) = card else {
        return vec![];
    };

    let current_stage = pokemon.stage;
    let card_name = &pokemon.name;

    // If already Stage 2, return the card itself
    if current_stage == 2 {
        return vec![card.clone()];
    }

    let mut highest_evolutions = Vec::new();

    match current_stage {
        0 => {
            // Basic Pokemon - prefer Stage 2, fallback to Stage 1
            if let Some(stage1_names) = STAGE1_LOOKUP.get(card_name) {
                // First try to find Stage 2 evolutions
                for stage1_name in stage1_names {
                    if let Some(stage2_names) = STAGE2_LOOKUP.get(stage1_name) {
                        for available_card in available_cards {
                            if let Card::Pokemon(p) = available_card {
                                if p.stage == 2 && stage2_names.contains(&p.name) {
                                    highest_evolutions.push(available_card.clone());
                                }
                            }
                        }
                    }
                }

                // If no Stage 2 found, look for Stage 1
                if highest_evolutions.is_empty() {
                    for available_card in available_cards {
                        if let Card::Pokemon(p) = available_card {
                            if p.stage == 1 && stage1_names.contains(&p.name) {
                                highest_evolutions.push(available_card.clone());
                            }
                        }
                    }
                }
            }
        }
        1 => {
            // Stage 1 Pokemon - look for Stage 2 evolutions
            if let Some(stage2_names) = STAGE2_LOOKUP.get(card_name) {
                for available_card in available_cards {
                    if let Card::Pokemon(p) = available_card {
                        if p.stage == 2 && stage2_names.contains(&p.name) {
                            highest_evolutions.push(available_card.clone());
                        }
                    }
                }
            }

            // If no Stage 2 found, return the card itself
            if highest_evolutions.is_empty() {
                highest_evolutions.push(card.clone());
            }
        }
        _ => {}
    }

    highest_evolutions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hooks::to_playable_card;

    #[test]
    fn test_venusaur_evolves_from_bulbasaur() {
        let venusaur = get_card_by_enum(CardId::A1003Venusaur);
        let bulbasaur = get_card_by_enum(CardId::A1001Bulbasaur);
        assert!(can_rare_candy_evolve(
            &venusaur,
            &to_playable_card(&bulbasaur, false)
        ));
    }

    #[test]
    fn test_ivysaur_cant_rare_candy_from_bulbasaur() {
        let ivysaur = get_card_by_enum(CardId::A1002Ivysaur);
        let bulbasaur = get_card_by_enum(CardId::A1001Bulbasaur);
        assert!(!can_rare_candy_evolve(
            &ivysaur,
            &to_playable_card(&bulbasaur, false)
        ));
    }

    #[test]
    fn test_charizard_cant_rare_candy_from_bulbasaur() {
        let charizard = get_card_by_enum(CardId::A1035Charizard);
        let bulbasaur = get_card_by_enum(CardId::A1001Bulbasaur);
        assert!(!can_rare_candy_evolve(
            &charizard,
            &to_playable_card(&bulbasaur, false)
        ));
    }

    #[test]
    fn test_rampardos_can_rare_candy_from_skull_fossil() {
        let rampardos = get_card_by_enum(CardId::A2089Rampardos);
        let skull_fossil = get_card_by_enum(CardId::A2144SkullFossil);
        assert!(can_rare_candy_evolve(
            &rampardos,
            &to_playable_card(&skull_fossil, false)
        ));
    }

    #[test]
    fn test_get_highest_evolutions_basic_to_stage2() {
        let bulbasaur = get_card_by_enum(CardId::A1001Bulbasaur);
        let ivysaur = get_card_by_enum(CardId::A1002Ivysaur);
        let venusaur = get_card_by_enum(CardId::A1003Venusaur);

        // When both Stage 1 and Stage 2 are available, should return only Stage 2
        let available = vec![ivysaur.clone(), venusaur.clone()];
        let result = get_highest_evolutions(&bulbasaur, &available);

        // Should return at least one card, and all should be Stage 2
        assert!(!result.is_empty());
        for card in &result {
            if let Card::Pokemon(p) = card {
                assert_eq!(p.stage, 2, "Expected Stage 2, got Stage {}", p.stage);
            } else {
                panic!("Expected Pokemon card");
            }
        }
    }

    #[test]
    fn test_get_highest_evolutions_basic_to_stage1() {
        let bulbasaur = get_card_by_enum(CardId::A1001Bulbasaur);
        let ivysaur = get_card_by_enum(CardId::A1002Ivysaur);

        // When only Stage 1 is available, should return Stage 1
        let available = vec![ivysaur.clone()];
        let result = get_highest_evolutions(&bulbasaur, &available);

        assert_eq!(result.len(), 1);
        if let Card::Pokemon(p) = &result[0] {
            assert_eq!(p.stage, 1);
            assert_eq!(p.name, "Ivysaur");
        } else {
            panic!("Expected Pokemon card");
        }
    }

    #[test]
    fn test_get_highest_evolutions_stage2_returns_itself() {
        let venusaur = get_card_by_enum(CardId::A1003Venusaur);

        // Stage 2 should return itself
        let available = vec![];
        let result = get_highest_evolutions(&venusaur, &available);

        assert_eq!(result.len(), 1);
        if let Card::Pokemon(p) = &result[0] {
            assert_eq!(p.stage, 2);
            assert_eq!(p.name, "Venusaur");
        } else {
            panic!("Expected Pokemon card");
        }
    }
}
