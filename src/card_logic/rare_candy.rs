use strum::IntoEnumIterator;

use crate::{
    card_ids::CardId,
    database::get_card_by_enum,
    hooks::get_stage,
    types::{Card, PlayedCard},
};

/// Check if a Stage 2 Pokemon can evolve from a Basic Pokemon using Rare Candy
pub fn can_rare_candy_evolve(stage2_card: &Card, basic_pokemon: &PlayedCard) -> bool {
    if let Card::Pokemon(stage2_pokemon) = stage2_card {
        if stage2_pokemon.stage != 2
            || get_stage(basic_pokemon) != 0
            || basic_pokemon.played_this_turn
        {
            return false;
        }
        // Look for a Stage 1 that is middle-of-chain
        for id in CardId::iter() {
            let tmp = get_card_by_enum(id);
            if let Card::Pokemon(stage1_card) = tmp {
                if stage1_card.stage == 1
                    && stage1_card.evolves_from == Some(basic_pokemon.get_name())
                {
                    // Check if the Stage 2 evolves from this Stage 1
                    if stage2_pokemon.evolves_from == Some(stage1_card.name.clone()) {
                        return true;
                    }
                }
            }
        }
    }
    false
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
