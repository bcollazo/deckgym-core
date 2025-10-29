use core::panic;
use std::vec;

use log::debug;

use crate::{
    actions::SimpleAction,
    effects::{CardEffect, TurnEffect},
    models::{Card, EnergyType, PlayedCard, TrainerCard, TrainerType, BASIC_STAGE},
    tool_ids::ToolId,
    AbilityId, State,
};

fn is_fossil(trainer_card: &TrainerCard) -> bool {
    trainer_card.trainer_card_type == TrainerType::Fossil
}

// Ultra Beasts
// TODO: Move this to a field in PokemonCard and database in the future
const ULTRA_BEAST_NAMES: [&str; 14] = [
    "Buzzwole ex",
    "Blacephalon",
    "Kartana",
    "Pheromosa",
    "Xurkitree",
    "Nihilego",
    "Guzzlord ex",
    "Poipole",
    "Naganadel",
    "Stakataka",
    "Celesteela",
    "Dawn Wings Necrozma",
    "Dusk Mane Necrozma",
    "Ultra Necrozma",
];

pub fn is_ultra_beast(pokemon_name: &str) -> bool {
    ULTRA_BEAST_NAMES.contains(&pokemon_name)
}

pub(crate) fn to_playable_card(card: &crate::models::Card, played_this_turn: bool) -> PlayedCard {
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

/// Check if a Pokemon in play can evolve into a card from hand
/// This handles special evolution rules like Eevee ex's Veevee 'volve ability
pub(crate) fn can_evolve_into(evolution_card: &Card, base_pokemon: &PlayedCard) -> bool {
    if let Card::Pokemon(evolution_pokemon) = evolution_card {
        if let Some(evolves_from) = &evolution_pokemon.evolves_from {
            // Normal evolution: the card evolves from the base Pokemon's name
            if base_pokemon.get_name() == *evolves_from {
                return true;
            }

            // Special case: Eevee ex's Veevee 'volve ability
            // Allows Eevee ex to evolve into any Pokemon that evolves from "Eevee"
            if let Some(ability_id) = AbilityId::from_pokemon_id(&base_pokemon.card.get_id()[..]) {
                if ability_id == AbilityId::A3b056EeveeExVeeveeVolve && evolves_from == "Eevee" {
                    return true;
                }
            }
        }
    }
    false
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
        ToolId::A3147LeafCape => {
            // Add +30 to remaining_hp and total_hp (only for Grass pokemon)
            let card = state.in_play_pokemon[actor][in_play_idx]
                .as_mut()
                .expect("Active Pokemon should be there");
            card.remaining_hp += 30;
            card.total_hp += 30;
        }
        // Many tools do nothing on attach
        ToolId::A2148RockyHelmet | ToolId::A3146PoisonBarb | ToolId::A4a067InflatableBoat => {}
    }
}

/// Called when energy is attached to a Pokémon
pub(crate) fn on_attach_energy(
    state: &mut State,
    actor: usize,
    in_play_idx: usize,
    energy_type: EnergyType,
    is_turn_energy: bool,
) {
    let pokemon = state.in_play_pokemon[actor][in_play_idx]
        .as_ref()
        .expect("Pokemon should be there if attaching energy to it");

    // Check for Darkrai ex's Nightmare Aura ability
    if let Some(ability_id) = AbilityId::from_pokemon_id(&pokemon.card.get_id()[..]) {
        if ability_id == AbilityId::A2110DarkraiExNightmareAura
            && energy_type == EnergyType::Darkness
            && is_turn_energy
        {
            // Deal 20 damage to opponent's active Pokémon
            debug!("Darkrai ex's Nightmare Aura: Dealing 20 damage to opponent's active Pokemon");
            let opponent = (actor + 1) % 2;
            if let Some(opponent_active) = state.in_play_pokemon[opponent][0].as_mut() {
                opponent_active.apply_damage(20);
            }
        }

        // Check for Komala's Comatose ability
        if ability_id == AbilityId::A3141KomalaComatose && in_play_idx == 0 {
            // As long as this Pokémon is in the Active Spot, whenever you attach an Energy from your Energy Zone to it, it is now Asleep.
            debug!("Komala's Comatose: Putting Komala to sleep");
            let komala = state.get_active_mut(actor);
            komala.asleep = true;
        }
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

pub(crate) fn on_end_turn(player_ending_turn: usize, state: &mut State) {
    // Check if active Pokémon has an end-of-turn ability
    let active = state.get_active(player_ending_turn);
    if let Some(ability_id) = AbilityId::from_pokemon_id(&active.card.get_id()[..]) {
        if ability_id == AbilityId::A4a020SuicuneExLegendaryPulse
            || ability_id == AbilityId::A4a025RaikouExLegendaryPulse
        {
            // At the end of your turn, if this Pokémon is in the Active Spot, draw a card.
            debug!("Legendary Pulse: Drawing a card");
            state.move_generation_stack.push((
                player_ending_turn,
                vec![SimpleAction::DrawCard { amount: 1 }],
            ));
        }
    }

    // Check for Zeraora's Thunderclap Flash ability (on first turn only)
    // Turn 1 is player 0's first turn, turn 2 is player 1's first turn
    if state.turn_count == 1 || state.turn_count == 2 {
        // Collect indices first to avoid borrow checker issues
        let zeraora_indices: Vec<usize> = state
            .enumerate_in_play_pokemon(player_ending_turn)
            .filter_map(|(in_play_idx, pokemon)| {
                if let Some(ability_id) = AbilityId::from_pokemon_id(&pokemon.card.get_id()[..]) {
                    if ability_id == AbilityId::A3a021ZeraoraThunderclapFlash {
                        return Some(in_play_idx);
                    }
                }
                None
            })
            .collect();

        // Now attach energy to all Zeraora pokemon
        for in_play_idx in zeraora_indices {
            // At the end of your first turn, take a Lightning Energy from your Energy Zone and attach it to this Pokémon.
            debug!("Zeraora's Thunderclap Flash: Attaching 1 Lightning Energy");
            let zeraora = state.in_play_pokemon[player_ending_turn][in_play_idx]
                .as_mut()
                .expect("Zeraora should be there");
            zeraora.attach_energy(&EnergyType::Lightning, 1);
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

// TODO: Confirm is_from_attack and goes to enemy active
pub(crate) fn modify_damage(
    state: &State,
    player: usize,
    base_damage: u32,
    receiving_idx: usize,
    is_from_active_attack: bool,
) -> u32 {
    let opponent = (player + 1) % 2;
    let active = state.get_active(player);
    let receiving_pokemon = &state.in_play_pokemon[opponent][0];
    let opponent_is_ex = state.get_active(opponent).card.is_ex();
    let attacker_is_eevee_evolution = active.evolved_from("Eevee");

    // If attack is 0, not even Giovanni takes it to 10.
    if base_damage == 0 {
        debug!("Attack is 0, returning 0");
        return base_damage;
    }

    // Check for Safeguard ability (prevents all damage from opponent's Pokémon ex)
    if let Some(defending_pokemon) = receiving_pokemon {
        if let Some(ability_id) = AbilityId::from_pokemon_id(&defending_pokemon.card.get_id()[..]) {
            if ability_id == AbilityId::A3066OricoricSafeguard
                && is_from_active_attack
                && active.card.is_ex()
            {
                debug!("Safeguard: Preventing all damage from opponent's Pokémon ex");
                return 0;
            }
        }
    }

    if receiving_idx == 0 && is_from_active_attack {
        // Modifiers by effect (like Giovanni, Red, Eevee Bag)
        let increased_turn_effect_modifiers = state
            .get_current_turn_effects()
            .iter()
            .map(|effect| match effect {
                TurnEffect::IncreasedDamage { amount } => *amount,
                TurnEffect::IncreasedDamageAgainstEx { amount } if opponent_is_ex => *amount,
                TurnEffect::IncreasedDamageForEeveeEvolutions { amount }
                    if attacker_is_eevee_evolution =>
                {
                    *amount
                }
                _ => 0,
            })
            .sum::<u32>();

        // Modifiers by receiving card effects
        let reduced_card_effect_modifiers = state
            .get_active(opponent)
            .get_active_effects()
            .iter()
            .filter(|effect| matches!(effect, CardEffect::ReducedDamage { .. }))
            .map(|effect| match effect {
                CardEffect::ReducedDamage { amount } => *amount,
                _ => 0,
            })
            .sum::<u32>();

        // Weakness Modifier
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
            base_damage,
            weakness_modifier,
            increased_turn_effect_modifiers,
            reduced_card_effect_modifiers
        );
        (base_damage + weakness_modifier + increased_turn_effect_modifiers)
            .saturating_sub(reduced_card_effect_modifiers)
    } else {
        debug!("Damage is to benched Pokémon or not from active attack");
        base_damage // modifiers only apply to active Pokémon
    }
}

// Check if attached satisfies cost (considering Colorless and Serperior's ability)
pub(crate) fn contains_energy(
    pokemon: &PlayedCard,
    cost: &[EnergyType],
    state: &State,
    player: usize,
) -> bool {
    energy_missing(pokemon, cost, state, player).is_empty()
}

pub(crate) fn energy_missing(
    pokemon: &PlayedCard,
    cost: &[EnergyType],
    state: &State,
    player: usize,
) -> Vec<EnergyType> {
    let mut energy_missing = vec![];
    let mut effective_attached = pokemon.get_effective_attached_energy(state, player);

    // First try to match the non-colorless energy
    let non_colorless_cost = cost.iter().filter(|x| **x != EnergyType::Colorless);
    for energy in non_colorless_cost {
        let index = effective_attached.iter().position(|x| *x == *energy);
        if let Some(i) = index {
            effective_attached.remove(i);
        } else {
            energy_missing.push(*energy);
        }
    }
    // If all non-colorless energy is satisfied, check if there are enough colorless energy
    // with what is left
    let colorless_cost = cost.iter().filter(|x| **x == EnergyType::Colorless);
    let colorless_missing = colorless_cost
        .count()
        .saturating_sub(effective_attached.len());
    energy_missing.extend(vec![EnergyType::Colorless; colorless_missing]);
    energy_missing
}

// Test Colorless is wildcard when counting energy
#[cfg(test)]
mod tests {
    use crate::{card_ids::CardId, database::get_card_by_enum};

    use super::*;

    #[test]
    fn test_contains_energy() {
        let state = State::default();
        let fire_card = get_card_by_enum(CardId::A1033Charmander);
        let mut pokemon = to_playable_card(&fire_card, false);
        pokemon.attached_energy = vec![EnergyType::Fire, EnergyType::Fire, EnergyType::Fire];
        let cost = vec![EnergyType::Colorless, EnergyType::Fire];
        assert!(contains_energy(&pokemon, &cost, &state, 0));
    }

    #[test]
    fn test_contains_energy_colorless() {
        let state = State::default();
        let fire_card = get_card_by_enum(CardId::A1033Charmander);
        let mut pokemon = to_playable_card(&fire_card, false);
        pokemon.attached_energy = vec![EnergyType::Fire, EnergyType::Fire, EnergyType::Water];
        let cost = vec![EnergyType::Colorless, EnergyType::Fire, EnergyType::Fire];
        assert!(contains_energy(&pokemon, &cost, &state, 0));
    }

    #[test]
    fn test_contains_energy_false_missing() {
        let state = State::default();
        let grass_card = get_card_by_enum(CardId::A1001Bulbasaur);
        let mut pokemon = to_playable_card(&grass_card, false);
        pokemon.attached_energy = vec![EnergyType::Grass, EnergyType::Grass, EnergyType::Fire];
        let cost = vec![EnergyType::Colorless, EnergyType::Fire, EnergyType::Water];
        assert!(!contains_energy(&pokemon, &cost, &state, 0));
    }

    #[test]
    fn test_contains_energy_double_colorless() {
        let state = State::default();
        let water_card = get_card_by_enum(CardId::A1053Squirtle);
        let mut pokemon = to_playable_card(&water_card, false);
        pokemon.attached_energy = vec![EnergyType::Water, EnergyType::Water, EnergyType::Fire];
        let cost = vec![EnergyType::Colorless, EnergyType::Colorless];
        assert!(contains_energy(&pokemon, &cost, &state, 0));
    }

    #[test]
    fn test_baby_pokemon_contain_energy() {
        let state = State::default();
        let baby_card = get_card_by_enum(CardId::A4032Magby);
        let mut pokemon = to_playable_card(&baby_card, false);
        pokemon.attached_energy = vec![];
        let cost = vec![];
        assert!(contains_energy(&pokemon, &cost, &state, 0));
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
        let attack = attacker.get_attacks()[0].clone();
        let base_damage = modify_damage(&state, 0, attack.fixed_damage, 0, true);

        // Add Giovanni effect
        state.add_turn_effect(TurnEffect::IncreasedDamage { amount: 10 }, 0);

        // Get damage with Giovanni effect
        let damage_with_giovanni = modify_damage(&state, 0, attack.fixed_damage, 0, true);

        // Verify Giovanni adds exactly 10 damage
        assert_eq!(
            damage_with_giovanni,
            base_damage + 10,
            "Giovanni should add exactly 10 damage to attacks"
        );
    }

    #[test]
    fn test_red_modifier_only_affects_ex() {
        let attacker_card = get_card_by_enum(CardId::A1001Bulbasaur);

        // Non-EX opponent should not receive extra damage
        let mut non_ex_state = State::default();
        non_ex_state.in_play_pokemon[0][0] = Some(to_playable_card(&attacker_card, false));
        let non_ex_defender = get_card_by_enum(CardId::A1033Charmander);
        non_ex_state.in_play_pokemon[1][0] = Some(to_playable_card(&non_ex_defender, false));
        let base_damage_non_ex = modify_damage(&non_ex_state, 0, 40, 0, true);
        non_ex_state.add_turn_effect(TurnEffect::IncreasedDamageAgainstEx { amount: 20 }, 0);
        let damage_with_red_vs_non_ex = modify_damage(&non_ex_state, 0, 40, 0, true);
        assert_eq!(
            damage_with_red_vs_non_ex, base_damage_non_ex,
            "Red should not increase damage against non-EX Pokémon"
        );

        // EX opponent should receive the bonus damage
        let mut ex_state = State::default();
        ex_state.in_play_pokemon[0][0] = Some(to_playable_card(&attacker_card, false));
        let ex_defender = get_card_by_enum(CardId::A3122SolgaleoEx);
        ex_state.in_play_pokemon[1][0] = Some(to_playable_card(&ex_defender, false));
        let base_damage_ex = modify_damage(&ex_state, 0, 40, 0, true);
        ex_state.add_turn_effect(TurnEffect::IncreasedDamageAgainstEx { amount: 20 }, 0);
        let damage_with_red_vs_ex = modify_damage(&ex_state, 0, 40, 0, true);
        assert_eq!(
            damage_with_red_vs_ex,
            base_damage_ex + 20,
            "Red should add 20 damage against Pokémon ex"
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
        let damage_with_stiffen = modify_damage(&state, 0, 120, 0, true);

        // Assert
        assert_eq!(
            damage_with_stiffen, 70,
            "Cosmoem's Stiffen should reduce damage by exactly 50"
        );
    }

    #[test]
    fn test_normal_evolution_works() {
        // Ivysaur evolves from Bulbasaur
        let ivysaur = get_card_by_enum(CardId::A1002Ivysaur);
        let bulbasaur = to_playable_card(&get_card_by_enum(CardId::A1001Bulbasaur), false);

        assert!(
            can_evolve_into(&ivysaur, &bulbasaur),
            "Ivysaur should be able to evolve from Bulbasaur"
        );
    }

    #[test]
    fn test_normal_evolution_fails_wrong_pokemon() {
        // Charizard cannot evolve from Bulbasaur
        let charizard = get_card_by_enum(CardId::A1035Charizard);
        let bulbasaur = to_playable_card(&get_card_by_enum(CardId::A1001Bulbasaur), false);

        assert!(
            !can_evolve_into(&charizard, &bulbasaur),
            "Charizard should not be able to evolve from Bulbasaur"
        );
    }

    #[test]
    fn test_normal_eevee_can_evolve_into_vaporeon() {
        // Regular Eevee (not Eevee ex) should only evolve normally
        let vaporeon = get_card_by_enum(CardId::A1080Vaporeon);
        let normal_eevee = to_playable_card(&get_card_by_enum(CardId::A1206Eevee), false);

        // Normal Eevee CAN evolve into Vaporeon (normal evolution)
        assert!(
            can_evolve_into(&vaporeon, &normal_eevee),
            "Normal Eevee should be able to evolve into Vaporeon normally"
        );
    }

    #[test]
    fn test_eevee_ex_can_evolve_into_vaporeon() {
        // Eevee ex should be able to evolve into Vaporeon (which evolves from "Eevee")
        let vaporeon = get_card_by_enum(CardId::A1080Vaporeon);
        let eevee_ex = to_playable_card(&get_card_by_enum(CardId::A3b056EeveeEx), false);

        assert!(
            can_evolve_into(&vaporeon, &eevee_ex),
            "Eevee ex should be able to evolve into Vaporeon via Veevee 'volve ability"
        );
    }

    #[test]
    fn test_eevee_ex_cannot_evolve_into_charizard() {
        // Eevee ex should NOT be able to evolve into Charizard (doesn't evolve from "Eevee")
        let charizard = get_card_by_enum(CardId::A1035Charizard);
        let eevee_ex = to_playable_card(&get_card_by_enum(CardId::A3b056EeveeEx), false);

        assert!(
            !can_evolve_into(&charizard, &eevee_ex),
            "Eevee ex should not be able to evolve into Charizard"
        );
    }
}
