use std::collections::HashMap;
use std::sync::LazyLock;

use strum::IntoEnumIterator;

use crate::{
    card_ids::CardId,
    database::get_card_by_enum,
    hooks::get_stage,
    models::{Card, PlayedCard},
};

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
                        lookup.insert(
                            (basic_name.clone(), stage2_pokemon.name.clone()),
                            true,
                        );
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
}
