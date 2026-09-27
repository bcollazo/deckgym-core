use crate::{
    actions::abilities::AbilityMechanic,
    card_ids::CardId,
    effects::{CardEffect, TurnEffect},
    models::{Card, EnergyType, PlayedCard},
    stadiums::get_peculiar_plaza_retreat_reduction,
    tools::has_tool,
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
        if matches!(
            card.ability_mechanic(),
            Some(AbilityMechanic::NoRetreatIfHasEnergy)
        ) && !card.attached_energy.is_empty()
        {
            return vec![];
        }
        if has_no_retreat_cost_from_abilities(state, card) {
            return vec![];
        }
        let mut normal_cost = pokemon_card.retreat_cost.clone();
        let retreat_cost_increase: u8 = card
            .get_effective_card_effects()
            .iter()
            .map(|effect| match effect {
                CardEffect::IncreasedRetreatCost { amount } => *amount,
                _ => 0,
            })
            .sum();
        for _ in 0..retreat_cost_increase {
            normal_cost.push(EnergyType::Colorless);
        }
        if has_tool(card, CardId::A4a067InflatableBoat) && card.is_type(EnergyType::Water) {
            normal_cost.pop();
        }
        if has_tool(card, CardId::B2a087BigAirBalloon) && pokemon_card.stage == 2 {
            return vec![];
        }
        if has_tool(card, CardId::B3b064SmallBalloon) && pokemon_card.stage == 0 {
            normal_cost.pop();
        }
        // Implement Retreat Cost Modifiers here
        let mut to_subtract = state
            .get_current_turn_effects()
            .iter()
            .filter(|x| matches!(x, TurnEffect::ReducedRetreatCost { .. }))
            .map(|x| match x {
                TurnEffect::ReducedRetreatCost { amount } => *amount,
                _ => 0,
            })
            .sum::<u8>();

        // Shaymin's Sky Support: As long as this Pokémon is on your Bench, your Active Basic Pokémon's Retreat Cost is 1 less.
        if pokemon_card.stage == 0 {
            // Only affects Basic Pokemon
            let current_player = state.current_player;
            for (_idx, benched_pokemon) in state.enumerate_bench_pokemon(current_player) {
                if matches!(
                    benched_pokemon.ability_mechanic(),
                    Some(
                        AbilityMechanic::ReduceRetreatCostOfYourActiveBasicFromBench { amount: 1 }
                    )
                ) {
                    to_subtract += 1;
                }
            }
        }
        // Each benched source applies its reduction once, even if the Active Pokémon happens to
        // count as the required type through more than one of its in-play types.
        let current_player = state.current_player;
        for (_idx, benched_pokemon) in state.enumerate_bench_pokemon(current_player) {
            if let Some(AbilityMechanic::ReduceRetreatCostOfYourActiveTypedFromBench {
                energy_type,
                amount,
            }) = benched_pokemon.ability_mechanic()
            {
                if card.is_type(*energy_type) {
                    to_subtract += *amount as u8;
                }
            }
        }

        // Beldum's Conductive Body: "If you have another Beldum in play, this Pokémon's Retreat
        // Cost is 2 less." ("another" = a different in-play slot holding a Pokémon of the same
        // name.)
        if let Some(AbilityMechanic::ReduceRetreatCostIfAnotherSameNameInPlay { amount }) =
            card.ability_mechanic()
        {
            if count_in_play_by_name(state, state.current_player, &card.get_name()) > 1 {
                to_subtract += *amount;
            }
        }

        // Peculiar Plaza: Psychic Pokemon retreat cost is 2 less
        to_subtract +=
            get_peculiar_plaza_retreat_reduction(state, card.is_type(EnergyType::Psychic));

        // Retreat Effects accumulate so we add them.
        for _ in 0..to_subtract {
            normal_cost.pop(); // Remove one colorless energy from retreat cost
        }

        // Ariados Trap Territory: Your opponent's Active Pokémon's Retreat Cost is 1 more.
        // This check needs to look at if the OPPONENT has Ariados in play
        let opponent = (state.current_player + 1) % 2;
        for (_idx, pokemon) in state.enumerate_in_play_pokemon(opponent) {
            if matches!(
                pokemon.ability_mechanic(),
                Some(AbilityMechanic::IncreaseRetreatCostForOpponentActive { amount: 1 })
            ) {
                normal_cost.push(EnergyType::Colorless);
                break;
            }
        }

        normal_cost
    } else {
        vec![]
    }
}

/// Passive abilities that zero out a Pokémon's Retreat Cost outright. Two shapes are covered:
/// self-scoped ones read off `card`'s own ability (Speed Link, Fantastical Floating, Wimp Out,
/// Surge Surfer), and auras granted by any of the owner's in-play Pokémon (Jumpluff's Fluffy
/// Flight, Tatsugiri's Retreat Directive).
fn has_no_retreat_cost_from_abilities(state: &State, card: &PlayedCard) -> bool {
    let player = state.current_player;
    let self_scoped = match card.ability_mechanic() {
        Some(AbilityMechanic::NoRetreatCostIfNamedPokemonInPlay { names }) => {
            has_named_pokemon_in_play(state, player, names)
        }
        Some(AbilityMechanic::NoRetreatCostDuringFirstTurn) => state.is_users_first_turn(),
        Some(AbilityMechanic::NoRetreatCostIfStadiumInPlay) => state.active_stadium.is_some(),
        _ => false,
    };
    if self_scoped {
        return true;
    }

    // Auras only free the *Active* Pokémon, so they never apply to a Benched retreat-cost query.
    let card_name = card.get_name();
    state
        .enumerate_in_play_pokemon(player)
        .any(|(_, pokemon)| match pokemon.ability_mechanic() {
            Some(AbilityMechanic::NoRetreatCostForYourActive { pokemon_name }) => {
                pokemon_name.as_ref().is_none_or(|name| *name == card_name)
            }
            _ => false,
        })
}

/// Whether `player` has any in-play Pokémon whose name is in `names`.
fn has_named_pokemon_in_play(state: &State, player: usize, names: &[String]) -> bool {
    state
        .enumerate_in_play_pokemon(player)
        .any(|(_, pokemon)| names.contains(&pokemon.get_name()))
}

/// How many of `player`'s in-play Pokémon share `name`.
fn count_in_play_by_name(state: &State, player: usize, name: &str) -> usize {
    state
        .enumerate_in_play_pokemon(player)
        .filter(|(_, pokemon)| pokemon.get_name() == name)
        .count()
}

// Test Colorless is wildcard when counting energy
#[cfg(test)]
mod tests {
    use crate::{
        card_ids::CardId, database::get_card_by_enum, effects::TurnEffect,
        hooks::core::to_playable_card,
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
        playable_card.attached_tool = Some(crate::database::get_card_by_enum(
            CardId::A4a067InflatableBoat,
        ));
        let retreat_cost = get_retreat_cost(&state, &playable_card);
        assert_eq!(
            retreat_cost,
            vec![EnergyType::Colorless, EnergyType::Colorless]
        );
    }
}
