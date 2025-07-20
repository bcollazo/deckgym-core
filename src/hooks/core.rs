use core::panic;
use std::vec;

use log::debug;

use crate::{
    actions::SimpleAction,
    effects::{CardEffect, TurnEffect},
    tool_ids::ToolId,
    types::{Card, EnergyType, PlayedCard, TrainerCard, BASIC_STAGE},
    AbilityId, State,
};

// Fossils
const FOSSIL_CARD_NAMES: [&str; 5] = [
    "Helix Fossil",
    "Dome Fossil",
    "Old Amber",
    "Skull Fossil",
    "Armor Fossil",
];

fn is_fossil(trainer_card: &TrainerCard) -> bool {
    FOSSIL_CARD_NAMES.contains(&trainer_card.name.as_str())
}

pub(crate) fn to_playable_card(card: &crate::types::Card, played_this_turn: bool) -> PlayedCard {
    let total_hp = match card {
        Card::Pokemon(pokemon_card) => pokemon_card.hp,
        Card::Trainer(trainer_card) => {
            if is_fossil(trainer_card) {
                40
            } else {
                panic!("Unplayable Trainer Card: {:?}", trainer_card);
            }
        }
    };
    PlayedCard::new(
        card.clone(),
        total_hp,
        total_hp,
        vec![],
        played_this_turn,
        vec![],
    )
}

pub(crate) fn get_stage(played_card: &PlayedCard) -> u8 {
    match &played_card.card {
        Card::Pokemon(pokemon_card) => pokemon_card.stage,
        Card::Trainer(trainer_card) => {
            if is_fossil(trainer_card) {
                BASIC_STAGE // Fossils are considered basic for stage purposes
            } else {
                panic!("Trainer cards do not have a stage")
            }
        }
    }
}

pub(crate) fn on_attach_tool(state: &mut State, actor: usize, in_play_idx: usize, tool_id: ToolId) {
    match tool_id {
        ToolId::A2147GiantCape => {
            // Add +20 to remaining_hp and total_hp
            let card = state.in_play_pokemon[actor][in_play_idx]
                .as_mut()
                .expect("Active Pokemon should be there");
            card.remaining_hp += 20;
            card.total_hp += 20;
        }
        // Many tools do nothing on attach
        ToolId::A2148RockyHelmet => {}
    }
}

/// Called when a Pokémon evolves
pub(crate) fn on_evolve(actor: usize, state: &mut State, to_card: &Card) {
    if let Some(ability_id) = AbilityId::from_pokemon_id(&to_card.get_id()[..]) {
        if ability_id == AbilityId::A3b034SylveonExHappyRibbon {
            // Give the user the option to draw 2 cards
            state.move_generation_stack.push((
                actor,
                vec![SimpleAction::DrawCard { amount: 2 }, SimpleAction::Noop],
            ));
        }
    }
}

// TODO: Implement Gengars ability that disallow playing support cards.
pub(crate) fn can_play_support(state: &State) -> bool {
    let has_modifiers = state
        .get_current_turn_effects()
        .iter()
        .any(|x| matches!(x, TurnEffect::NoSupportCards));

    !state.has_played_support && !has_modifiers
}

pub(crate) fn get_damage_from_attack(
    state: &State,
    player: usize,
    index: usize,
    receiving_index: usize,
) -> u32 {
    let active = state.get_active(player);
    let attack = active.card.get_attacks()[index].clone();

    // If attack is 0, not even Giovanni takes it to 10.
    if attack.fixed_damage == 0 {
        debug!("Attack is 0, returning 0");
        return attack.fixed_damage;
    }

    // If its bench attack, don't apply multipliers
    if receiving_index != 0 {
        debug!("Bench attack, returning fixed {}", attack.fixed_damage);
        return attack.fixed_damage;
    }

    // Modifiers by effect (like Giovanni)
    let increased_turn_effect_modifiers = state
        .get_current_turn_effects()
        .iter()
        .filter(|x| matches!(x, TurnEffect::IncreasedDamage { .. }))
        .map(|x| match x {
            TurnEffect::IncreasedDamage { amount } => *amount,
            _ => 0,
        })
        .sum::<u32>();

    // Modifiers by receiving card effects
    let reduced_card_effect_modifiers = state
        .get_active((player + 1) % 2)
        .get_active_effects()
        .iter()
        .filter(|effect| matches!(effect, CardEffect::ReducedDamage { .. }))
        .map(|effect| match effect {
            CardEffect::ReducedDamage { amount } => *amount,
            _ => 0,
        })
        .sum::<u32>();

    // Weakness Modifier
    let opponent = (player + 1) % 2;
    let mut weakness_modifier = 0;
    let receiving = state.get_active(opponent);
    if let Card::Pokemon(pokemon_card) = &receiving.card {
        if pokemon_card.weakness == active.card.get_type() {
            debug!(
                "Weakness! {:?} is weak to {:?}",
                pokemon_card,
                active.card.get_type()
            );
            weakness_modifier = 20;
        }
    }

    debug!(
        "Attack: {:?}, Weakness: {}, IncreasedDamage: {}, ReducedDamage: {}",
        attack.fixed_damage,
        weakness_modifier,
        increased_turn_effect_modifiers,
        reduced_card_effect_modifiers
    );
    attack.fixed_damage + weakness_modifier + increased_turn_effect_modifiers
        - reduced_card_effect_modifiers
}

// Check if attached satisfies cost (considering Colorless)
pub(crate) fn contains_energy(attached: &[EnergyType], cost: &[EnergyType]) -> bool {
    // First try to match the non-colorless energy
    let non_colorless_cost = cost.iter().filter(|x| **x != EnergyType::Colorless);
    let colorless_cost = cost.iter().filter(|x| **x == EnergyType::Colorless);

    let mut attached_copy: Vec<EnergyType> = attached.to_vec();
    for energy in non_colorless_cost {
        let index = attached_copy.iter().position(|x| *x == *energy);
        if let Some(i) = index {
            attached_copy.remove(i);
        } else {
            return false;
        }
    }

    // If all non-colorless energy is satisfied, check if there are enough colorless energy
    attached_copy.len() >= colorless_cost.count()
}

// Test Colorless is wildcard when counting energy
#[cfg(test)]
mod tests {
    use crate::{card_ids::CardId, database::get_card_by_enum};

    use super::*;

    #[test]
    fn test_contains_energy() {
        let slice_a = vec![EnergyType::Fire, EnergyType::Fire, EnergyType::Fire];
        let slice_b = vec![EnergyType::Colorless, EnergyType::Fire];
        assert!(contains_energy(&slice_a, &slice_b));
    }

    #[test]
    fn test_contains_energy_colorless() {
        let slice_a = vec![EnergyType::Fire, EnergyType::Fire, EnergyType::Water];
        let slice_b = vec![EnergyType::Colorless, EnergyType::Fire, EnergyType::Fire];
        assert!(contains_energy(&slice_a, &slice_b));
    }

    #[test]
    fn test_contains_energy_false_missing() {
        let slice_a = vec![EnergyType::Grass, EnergyType::Grass, EnergyType::Fire];
        let slice_b = vec![EnergyType::Colorless, EnergyType::Fire, EnergyType::Water];
        assert!(!contains_energy(&slice_a, &slice_b));
    }

    #[test]
    fn test_contains_energy_double_colorless() {
        let slice_a = vec![EnergyType::Water, EnergyType::Water, EnergyType::Fire];
        let slice_b = vec![EnergyType::Colorless, EnergyType::Colorless];
        assert!(contains_energy(&slice_a, &slice_b));
    }

    #[test]
    fn test_can_play_support() {
        // Normal state should allow support cards
        let mut state = State::default();
        assert!(can_play_support(&state));

        // After playing a support, it should disallow
        state.has_played_support = true;
        assert!(!can_play_support(&state));

        // Reset state
        state.has_played_support = false;
        assert!(can_play_support(&state));

        // With Psyduck headache effect, it should disallow
        state.add_turn_effect(TurnEffect::NoSupportCards, 1);
        assert!(!can_play_support(&state));
    }

    #[test]
    fn test_giovanni_modifier() {
        // Create a basic state with attacking and defending Pokémon
        let mut state = State::default();

        // Set up attacker with a fixed damage attack
        let attacker = get_card_by_enum(CardId::A1001Bulbasaur);
        let played_attacker = to_playable_card(&attacker, false);
        state.in_play_pokemon[0][0] = Some(played_attacker);

        // Set up defender
        let defender = get_card_by_enum(CardId::A1033Charmander);
        let played_defender = to_playable_card(&defender, false);
        state.in_play_pokemon[1][0] = Some(played_defender);

        // Get base damage without Giovanni effect
        let base_damage = get_damage_from_attack(&state, 0, 0, 0);

        // Add Giovanni effect
        state.add_turn_effect(TurnEffect::IncreasedDamage { amount: 10 }, 0);

        // Get damage with Giovanni effect
        let damage_with_giovanni = get_damage_from_attack(&state, 0, 0, 0);

        // Verify Giovanni adds exactly 10 damage
        assert_eq!(
            damage_with_giovanni,
            base_damage + 10,
            "Giovanni should add exactly 10 damage to attacks"
        );
    }

    #[test]
    fn test_cosmoem_reduced_damage() {
        // Arrange
        let mut state = State::default();
        let attacker = get_card_by_enum(CardId::A3122SolgaleoEx);
        let played_attacker = to_playable_card(&attacker, false);
        state.in_play_pokemon[0][0] = Some(played_attacker);
        let defender = get_card_by_enum(CardId::A3086Cosmoem);
        let played_defender = to_playable_card(&defender, false);
        state.in_play_pokemon[1][0] = Some(played_defender);
        state.in_play_pokemon[1][0]
            .as_mut()
            .unwrap()
            .add_effect(crate::effects::CardEffect::ReducedDamage { amount: 50 }, 1);

        // Act
        let damage_with_stiffen = get_damage_from_attack(&state, 0, 0, 0);

        // Assert
        assert_eq!(
            damage_with_stiffen, 70,
            "Cosmoem's Stiffen should reduce damage by exactly 50"
        );
    }
}
