use crate::{
    effects::{CardEffect, TurnEffect},
    models::{Card, EnergyType, PlayedCard},
    tool_ids::ToolId,
    State,
};

pub(crate) fn can_retreat(state: &State) -> bool {
    let active = state.get_active(state.current_player);

    // Check if active card has CardEffect::NoRetreat
    let has_no_retreat_effect = active.get_active_effects().contains(&CardEffect::NoRetreat);

    // Check if active card is a Fossil (Fossils can never retreat)
    let is_fossil = active.is_fossil();

    !state.has_retreated && !has_no_retreat_effect && !is_fossil
}

pub(crate) fn get_retreat_cost(state: &State, card: &PlayedCard) -> Vec<EnergyType> {
    if let Card::Pokemon(pokemon_card) = &card.card {
        let mut normal_cost = pokemon_card.retreat_cost.clone();
        if let Some(tool_id) = card.attached_tool {
            if tool_id == ToolId::A4a067InflatableBoat
                && card.get_energy_type() == Some(EnergyType::Water)
            {
                normal_cost.pop();
            }
        }
        // Implement Retreat Cost Modifiers here
        let to_subtract = state
            .get_current_turn_effects()
            .iter()
            .filter(|x| matches!(x, TurnEffect::ReducedRetreatCost { .. }))
            .map(|x| match x {
                TurnEffect::ReducedRetreatCost { amount } => *amount,
                _ => 0,
            })
            .sum::<u8>();

        // Retreat Effects accumulate so we add them.
        for _ in 0..to_subtract {
            normal_cost.pop(); // Remove one colorless energy from retreat cost
        }
        normal_cost
    } else {
        vec![]
    }
}

// Test Colorless is wildcard when counting energy
#[cfg(test)]
mod tests {
    use crate::{
        card_ids::CardId, database::get_card_by_enum, effects::TurnEffect,
        hooks::core::to_playable_card, tool_ids::ToolId,
    };

    use super::*;

    #[test]
    fn test_retreat_costs() {
        let state = State::default();
        let card = get_card_by_enum(CardId::A1055Blastoise);
        let playable_card = to_playable_card(&card, false);
        let retreat_cost = get_retreat_cost(&state, &playable_card);
        assert_eq!(
            retreat_cost,
            vec![
                EnergyType::Colorless,
                EnergyType::Colorless,
                EnergyType::Colorless
            ]
        );
    }

    #[test]
    fn test_retreat_costs_with_xspeed() {
        let mut state = State::default();
        state.add_turn_effect(TurnEffect::ReducedRetreatCost { amount: 1 }, 0);
        let card = get_card_by_enum(CardId::A1055Blastoise);
        let playable_card = to_playable_card(&card, false);
        let retreat_cost = get_retreat_cost(&state, &playable_card);
        assert_eq!(
            retreat_cost,
            vec![EnergyType::Colorless, EnergyType::Colorless]
        );
    }

    #[test]
    fn test_retreat_costs_with_two_xspeed_and_two_leafs() {
        let mut state = State::default();
        state.add_turn_effect(TurnEffect::ReducedRetreatCost { amount: 1 }, 0);
        state.add_turn_effect(TurnEffect::ReducedRetreatCost { amount: 1 }, 0);
        state.add_turn_effect(TurnEffect::ReducedRetreatCost { amount: 2 }, 0);
        let card = get_card_by_enum(CardId::A1211Snorlax);
        let playable_card = to_playable_card(&card, false);
        let retreat_cost = get_retreat_cost(&state, &playable_card);
        assert_eq!(retreat_cost, vec![]);
    }

    #[test]
    fn test_retreat_costs_with_inflatable_boat() {
        let state = State::default();
        let card = get_card_by_enum(CardId::A1055Blastoise);
        let mut playable_card = to_playable_card(&card, false);
        playable_card.attached_tool = Some(ToolId::A4a067InflatableBoat);
        let retreat_cost = get_retreat_cost(&state, &playable_card);
        assert_eq!(
            retreat_cost,
            vec![EnergyType::Colorless, EnergyType::Colorless]
        );
    }
}
