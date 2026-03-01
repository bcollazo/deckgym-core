use core::panic;
use std::vec;

use log::debug;

use crate::{
    actions::{abilities::AbilityMechanic, ability_mechanic_from_effect, SimpleAction},
    card_ids::CardId,
    effects::{CardEffect, TurnEffect},
    models::{Card, EnergyType, PlayedCard, TrainerCard, TrainerType, BASIC_STAGE},
    stadiums::get_training_area_damage_bonus,
    tools::has_tool,
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

pub fn to_playable_card(card: &crate::models::Card, played_this_turn: bool) -> PlayedCard {
    let base_hp = match card {
        Card::Pokemon(pokemon_card) => pokemon_card.hp,
        Card::Trainer(trainer_card) => {
            if is_fossil(trainer_card) {
                40
            } else {
                panic!("Unplayable Trainer Card: {:?}", trainer_card);
            }
        }
    };
    PlayedCard::new(card.clone(), 0, base_hp, vec![], played_this_turn, vec![])
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

// TODO: Deprecated. Use PokemonCard::can_evolve_into instead.
pub(crate) fn can_evolve_into(evolution_card: &Card, base_pokemon: &PlayedCard) -> bool {
    base_pokemon.card.can_evolve_into(evolution_card)
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
        if ability_id == AbilityId::A4a022MiloticHealingRipples {
            // Healing Ripples: heal 60 damage from 1 of your [W] Pokémon
            let possible_moves: Vec<SimpleAction> = state
                .enumerate_in_play_pokemon(actor)
                .filter(|(_, pokemon)| {
                    pokemon.is_damaged() && pokemon.get_energy_type() == Some(EnergyType::Water)
                })
                .map(|(in_play_idx, _)| SimpleAction::Heal {
                    in_play_idx,
                    amount: 60,
                    cure_status: false,
                })
                .chain(std::iter::once(SimpleAction::Noop))
                .collect();

            if possible_moves.len() > 1 {
                state.move_generation_stack.push((actor, possible_moves));
            }
        }
        if ability_id == AbilityId::B1a012CharmeleonIgnition {
            // Ignition: When you play this Pokémon from your hand to evolve 1 of your Pokémon during your turn,
            // you may take 1 [R] Energy from your Energy Zone and attach it to this Pokémon.
            // Find the active Pokémon (where evolution just happened) and attach energy
            state.move_generation_stack.push((
                actor,
                vec![
                    SimpleAction::Attach {
                        attachments: vec![(1, EnergyType::Fire, 0)], // Attach to active (index 0)
                        is_turn_energy: false, // From ability, not turn energy
                    },
                    SimpleAction::Noop,
                ],
            ));
        }
    }
}

/// Called when a basic Pokémon is played to the bench from hand
pub(crate) fn on_end_turn(player_ending_turn: usize, state: &mut State) {
    // Check if active Pokémon has an end-of-turn ability
    let active = state.get_active(player_ending_turn);
    if let Some(ability_id) = AbilityId::from_pokemon_id(&active.card.get_id()[..]) {
        if ability_id == AbilityId::A4a010EnteiExLegendaryPulse
            || ability_id == AbilityId::A4a020SuicuneExLegendaryPulse
            || ability_id == AbilityId::A4a025RaikouExLegendaryPulse
        {
            // At the end of your turn, if this Pokémon is in the Active Spot, draw a card.
            debug!("Legendary Pulse: Drawing a card");
            state.move_generation_stack.push((
                player_ending_turn,
                vec![SimpleAction::DrawCard { amount: 1 }],
            ));
        }
        if ability_id == AbilityId::A3b057SnorlaxExFullMouthManner {
            // At the end of your turn, if this Pokémon is in the Active Spot, heal 20 damage from it.
            debug!("Full-Mouth Manner: Healing 20 damage from active");
            let active = state.get_active_mut(player_ending_turn);
            active.heal(20);
        }
    }

    // Process delayed damage effects on active Pokemon
    // Delayed damage triggers at the end of the opponent's turn (when their turn ends, the effect expires)
    let total_delayed_damage: u32 = state
        .get_active(player_ending_turn)
        .get_effects()
        .iter()
        .filter_map(|(effect, _)| {
            if let CardEffect::DelayedDamage { amount } = effect {
                Some(*amount)
            } else {
                None
            }
        })
        .sum();

    if total_delayed_damage > 0 {
        debug!(
            "Delayed damage: Applying {} damage to active Pokemon",
            total_delayed_damage
        );
        // The opponent is the source of the delayed damage (they used the attack that caused it)
        let opponent = (player_ending_turn + 1) % 2;
        crate::actions::handle_damage(
            state,
            (opponent, 0), // Opponent's active Pokemon as the source
            &[(total_delayed_damage, player_ending_turn, 0)], // Target is current player's active
            false,         // Not from an active attack (it's a delayed effect)
            None,          // No attack name
        );
    }

    // Discard Metal Core Barrier from the opponent's Pokémon at the end of this player's turn.
    // ("discard it at the end of your opponent's turn" — the tool owner is the other player)
    let tool_owner = (player_ending_turn + 1) % 2;
    let barrier_indices: Vec<usize> = state.in_play_pokemon[tool_owner]
        .iter()
        .enumerate()
        .filter(|(_, slot)| {
            slot.as_ref()
                .is_some_and(|p| has_tool(p, CardId::B2148MetalCoreBarrier))
        })
        .map(|(i, _)| i)
        .collect();
    for idx in barrier_indices {
        debug!("Metal Core Barrier: Discarding at end of opponent's turn");
        state.discard_tool(tool_owner, idx);
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
            state.attach_energy_from_zone(
                player_ending_turn,
                in_play_idx,
                EnergyType::Lightning,
                1,
                false,
            );
        }
    }
}

pub(crate) fn can_play_support(state: &State) -> bool {
    let has_modifiers = state
        .get_current_turn_effects()
        .iter()
        .any(|x| matches!(x, TurnEffect::NoSupportCards));

    // Check if opponent has Gengar ex with Shadowy Spellbind in active spot
    let opponent = (state.current_player + 1) % 2;
    let blocked_by_gengar = state.in_play_pokemon[opponent][0]
        .as_ref()
        .and_then(|opponent_active| AbilityId::from_pokemon_id(&opponent_active.get_id()))
        .map(|id| id == AbilityId::A1123GengarExShadowySpellbind)
        .unwrap_or(false);

    !state.has_played_support && !has_modifiers && !blocked_by_gengar
}

pub(crate) fn can_play_item(state: &State) -> bool {
    let has_modifiers = state
        .get_current_turn_effects()
        .iter()
        .any(|x| matches!(x, TurnEffect::NoItemCards));

    !has_modifiers
}

fn get_heavy_helmet_reduction(state: &State, (target_player, target_idx): (usize, usize)) -> u32 {
    let defending_pokemon = &state.in_play_pokemon[target_player][target_idx]
        .as_ref()
        .expect("Defending Pokemon should be there when checking Heavy Helmet");
    if has_tool(defending_pokemon, CardId::B1219HeavyHelmet) {
        if let Card::Pokemon(pokemon_card) = &defending_pokemon.card {
            if pokemon_card.retreat_cost.len() >= 3 {
                debug!("Heavy Helmet: Reducing damage by 20");
                return 20;
            }
        }
    }
    0
}

fn get_metal_core_barrier_reduction(
    state: &State,
    (target_player, target_idx): (usize, usize),
    is_from_active_attack: bool,
) -> u32 {
    if !is_from_active_attack {
        return 0;
    }

    let defending_pokemon = &state.in_play_pokemon[target_player][target_idx]
        .as_ref()
        .expect("Defending Pokemon should be there when checking Metal Core Barrier");
    if has_tool(defending_pokemon, CardId::B2148MetalCoreBarrier) {
        debug!("Metal Core Barrier: Reducing damage by 50");
        return 50;
    }
    0
}

fn get_intimidating_fang_reduction(
    state: &State,
    attacking_ref: (usize, usize),
    target_ref: (u32, usize, usize),
    is_from_active_attack: bool,
) -> u32 {
    let (attacking_player, attacking_idx) = attacking_ref;
    let (_, target_player, _) = target_ref;
    if attacking_player == target_player || attacking_idx != 0 || !is_from_active_attack {
        return 0;
    }

    let defenders_active = &state.in_play_pokemon[target_player][0]
        .as_ref()
        .expect("Defending Pokemon should be there when checking Intimidating Fang");
    if let Some(ability_id) = AbilityId::from_pokemon_id(&defenders_active.card.get_id()[..]) {
        if ability_id == AbilityId::A3a015LuxrayIntimidatingFang {
            debug!("Intimidating Fang: Reducing opponent's attack damage by 20");
            return 20;
        }
    }
    0
}

fn get_ability_damage_reduction(
    receiving_pokemon: &crate::models::PlayedCard,
    is_from_active_attack: bool,
) -> u32 {
    if let Some(ability) = receiving_pokemon.card.get_ability() {
        if let Some(AbilityMechanic::ReduceDamageFromAttacks { amount }) =
            ability_mechanic_from_effect(&ability.effect)
        {
            if is_from_active_attack {
                debug!("ReduceDamageFromAttacks: Reducing damage by {}", amount);
                return *amount;
            }
        }
    }
    0
}

fn get_increased_turn_effect_modifiers(
    state: &State,
    is_active_to_active: bool,
    target_is_ex: bool,
    attacker_is_eevee_evolution: bool,
    attacking_pokemon: &crate::models::PlayedCard,
) -> u32 {
    if !is_active_to_active {
        return 0;
    }
    state
        .get_current_turn_effects()
        .iter()
        .map(|effect| match effect {
            TurnEffect::IncreasedDamage { amount } => *amount,
            TurnEffect::IncreasedDamageForType {
                amount,
                energy_type,
            } if attacking_pokemon.get_energy_type() == Some(*energy_type) => *amount,
            TurnEffect::IncreasedDamageAgainstEx { amount } if target_is_ex => *amount,
            TurnEffect::IncreasedDamageForEeveeEvolutions { amount }
                if attacker_is_eevee_evolution =>
            {
                *amount
            }
            TurnEffect::IncreasedDamageForSpecificPokemon {
                amount,
                pokemon_names,
            } => {
                let attacker_name = attacking_pokemon.get_name();
                if pokemon_names
                    .iter()
                    .any(|name| name.as_str() == attacker_name)
                {
                    *amount
                } else {
                    0
                }
            }
            _ => 0,
        })
        .sum::<u32>()
}

fn get_increased_attack_specific_modifiers(
    attacking_pokemon: &crate::models::PlayedCard,
    is_active_to_active: bool,
    attack_name: Option<&str>,
) -> u32 {
    if !is_active_to_active {
        return 0;
    }
    attacking_pokemon
        .get_active_effects()
        .iter()
        .filter_map(|effect| match effect {
            CardEffect::IncreasedDamageForAttack {
                attack_name: effect_attack_name,
                amount,
            } => {
                if let Some(current_attack_name) = attack_name {
                    if current_attack_name == effect_attack_name {
                        Some(*amount)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        })
        .sum::<u32>()
}

fn get_reduced_card_effect_modifiers(
    state: &State,
    is_active_to_active: bool,
    target_player: usize,
) -> u32 {
    if !is_active_to_active {
        return 0;
    }
    state
        .get_active(target_player)
        .get_active_effects()
        .iter()
        .filter(|effect| matches!(effect, CardEffect::ReducedDamage { .. }))
        .map(|effect| match effect {
            CardEffect::ReducedDamage { amount } => *amount,
            _ => 0,
        })
        .sum::<u32>()
}

fn get_turn_effect_damage_reduction(
    state: &State,
    target_player: usize,
    target_pokemon: &crate::models::PlayedCard,
    attacking_player: usize,
    is_from_active_attack: bool,
) -> u32 {
    if !is_from_active_attack || attacking_player == target_player {
        return 0;
    }
    let target_energy_type = target_pokemon.get_energy_type();
    state
        .get_current_turn_effects()
        .iter()
        .filter_map(|effect| match effect {
            TurnEffect::ReducedDamageForType {
                amount,
                energy_type,
                player,
            } if *player == target_player && target_energy_type == Some(*energy_type) => {
                Some(*amount)
            }
            _ => None,
        })
        .sum::<u32>()
}

fn get_weakness_modifier(
    state: &State,
    is_active_to_active: bool,
    target_player: usize,
    attacking_pokemon: &crate::models::PlayedCard,
) -> u32 {
    if !is_active_to_active {
        return 0;
    }
    let receiving = state.get_active(target_player);

    // Check if defender has NoWeakness effect active
    if receiving
        .get_active_effects()
        .iter()
        .any(|effect| matches!(effect, CardEffect::NoWeakness))
    {
        debug!("NoWeakness: Ignoring weakness damage");
        return 0;
    }

    if let Card::Pokemon(pokemon_card) = &receiving.card {
        if pokemon_card.weakness == attacking_pokemon.card.get_type() {
            debug!(
                "Weakness! {:?} is weak to {:?}",
                pokemon_card,
                attacking_pokemon.card.get_type()
            );
            return 20;
        }
    }
    0
}

// TODO: Confirm is_from_attack and goes to enemy active
pub(crate) fn modify_damage(
    state: &State,
    attacking_ref: (usize, usize),
    target_ref: (u32, usize, usize),
    is_from_active_attack: bool,
    attack_name: Option<&str>,
) -> u32 {
    // If attack is 0, not even Giovanni takes it to 10.
    let (attacking_player, attacking_idx) = attacking_ref;
    let (base_damage, target_player, target_idx) = target_ref;
    if base_damage == 0 {
        debug!("Attack is 0, returning 0");
        return base_damage;
    }

    let attacking_pokemon = state.in_play_pokemon[attacking_player][attacking_idx]
        .as_ref()
        .expect("Attacking Pokemon should be there when modifying damage");
    let receiving_pokemon = state.in_play_pokemon[target_player][target_idx]
        .as_ref()
        .expect("Receiving Pokemon should be there when modifying damage");

    // Check for Safeguard ability (prevents all damage from opponent's Pokémon ex)
    if let Some(ability_id) = AbilityId::from_pokemon_id(&receiving_pokemon.card.get_id()[..]) {
        if ability_id == AbilityId::A3066OricoricSafeguard
            && is_from_active_attack
            && attacking_pokemon.card.is_ex()
        {
            debug!("Safeguard: Preventing all damage from opponent's Pokémon ex");
            return 0;
        }
        // Wartortle Shell Shield: prevent all damage when on bench
        if ability_id == AbilityId::B1a018WartortleShellShield
            && is_from_active_attack
            && target_idx != 0
        {
            debug!("Shell Shield: Preventing all damage to benched Wartortle");
            return 0;
        }
    }

    // Protective Poncho: prevent all damage to benched Pokémon with this tool attached
    if target_idx != 0 && has_tool(receiving_pokemon, CardId::B2147ProtectivePoncho) {
        debug!("Protective Poncho: Preventing all damage to benched Pokémon");
        return 0;
    }

    // Check for PreventAllDamageAndEffects (Shinx's Hide)
    if receiving_pokemon
        .get_active_effects()
        .iter()
        .any(|effect| matches!(effect, CardEffect::PreventAllDamageAndEffects))
    {
        debug!("PreventAllDamageAndEffects: Preventing all damage and effects");
        return 0;
    }

    // Calculate all modifiers
    let is_active_to_active = target_idx == 0 && attacking_idx == 0 && is_from_active_attack;
    let target_is_ex = receiving_pokemon.card.is_ex();
    let attacker_is_eevee_evolution = attacking_pokemon.evolved_from("Eevee");

    let intimidating_fang_reduction =
        get_intimidating_fang_reduction(state, attacking_ref, target_ref, is_from_active_attack);
    let heavy_helmet_reduction = get_heavy_helmet_reduction(state, (target_player, target_idx));
    let metal_core_barrier_reduction =
        get_metal_core_barrier_reduction(state, (target_player, target_idx), is_from_active_attack);
    let ability_damage_reduction =
        get_ability_damage_reduction(receiving_pokemon, is_from_active_attack);
    let increased_turn_effect_modifiers = get_increased_turn_effect_modifiers(
        state,
        is_active_to_active,
        target_is_ex,
        attacker_is_eevee_evolution,
        attacking_pokemon,
    );
    let increased_attack_specific_modifiers = get_increased_attack_specific_modifiers(
        attacking_pokemon,
        is_active_to_active,
        attack_name,
    );
    let reduced_card_effect_modifiers =
        get_reduced_card_effect_modifiers(state, is_active_to_active, target_player);
    let reduced_turn_effect_modifiers = get_turn_effect_damage_reduction(
        state,
        target_player,
        receiving_pokemon,
        attacking_player,
        is_from_active_attack,
    );
    let weakness_modifier =
        get_weakness_modifier(state, is_active_to_active, target_player, attacking_pokemon);

    // Type-specific damage boost abilities (e.g., Lucario's Fighting Coach, Aegislash's Royal Command)
    // These check if certain ability-holders are in play and boost damage for specific energy types
    // Only applies to active-to-active attacks (not damage moves like Dusknoir's Shadow Void)
    let type_boost_bonus = if is_active_to_active {
        calculate_type_boost_bonus(state, attacking_player, attacking_pokemon)
    } else {
        0
    };

    // Stadium damage bonus (e.g., Training Area for Stage 1 Pokemon)
    // Only applies to attacks against the opponent's Active Pokemon
    let stadium_damage_bonus = if is_active_to_active {
        get_training_area_damage_bonus(state, get_stage(attacking_pokemon))
    } else {
        0
    };

    debug!(
        "Attack: {:?}, Weakness: {}, IncreasedDamage: {}, IncreasedAttackSpecific: {}, ReducedDamage: {}, TurnEffectReduction: {}, HeavyHelmet: {}, MetalCoreBarrier: {}, IntimidatingFang: {}, AbilityReduction: {}, TypeBoost: {}, StadiumBonus: {}",
        base_damage,
        weakness_modifier,
        increased_turn_effect_modifiers,
        increased_attack_specific_modifiers,
        reduced_card_effect_modifiers,
        reduced_turn_effect_modifiers,
        heavy_helmet_reduction,
        metal_core_barrier_reduction,
        intimidating_fang_reduction,
        ability_damage_reduction,
        type_boost_bonus,
        stadium_damage_bonus
    );
    (base_damage
        + weakness_modifier
        + increased_turn_effect_modifiers
        + increased_attack_specific_modifiers
        + type_boost_bonus
        + stadium_damage_bonus)
        .saturating_sub(
            reduced_card_effect_modifiers
                + reduced_turn_effect_modifiers
                + heavy_helmet_reduction
                + metal_core_barrier_reduction
                + intimidating_fang_reduction
                + ability_damage_reduction,
        )
}

/// Calculate type-specific damage boost from abilities like Lucario's Fighting Coach or Aegislash's Royal Command
/// Returns the bonus damage amount based on attacking Pokemon's energy type and abilities in play
fn calculate_type_boost_bonus(
    state: &State,
    attacking_player: usize,
    attacking_pokemon: &PlayedCard,
) -> u32 {
    let attacker_energy_type = match attacking_pokemon.get_energy_type() {
        Some(energy_type) => energy_type,
        None => return 0,
    };

    let mut bonus = 0;

    // Check each Pokemon in play for type-boosting abilities
    for (_, pokemon) in state.enumerate_in_play_pokemon(attacking_player) {
        if let Some(ability_id) = AbilityId::from_pokemon_id(&pokemon.get_id()) {
            match ability_id {
                // Lucario's Fighting Coach: +20 damage to Fighting-type attacks
                AbilityId::A2092LucarioFightingCoach => {
                    if attacker_energy_type == EnergyType::Fighting {
                        debug!("Fighting Coach (Lucario): Increasing damage by 20");
                        bonus += 20;
                    }
                }
                // Aegislash's Cursed Metal: +30 damage to Psychic and Metal-type attacks
                AbilityId::B1172AegislashCursedMetal => {
                    if attacker_energy_type == EnergyType::Psychic
                        || attacker_energy_type == EnergyType::Metal
                    {
                        debug!("Cursed Metal (Aegislash): Increasing damage by 30");
                        bonus += 30;
                    }
                }
                _ => {}
            }
        }
    }

    bonus
}

// Get the attack cost, considering abilities and active card effects that modify attack costs.
pub(crate) fn get_attack_cost(
    base_cost: &[EnergyType],
    state: &State,
    attacking_player: usize,
) -> Vec<EnergyType> {
    use crate::ability_ids::AbilityId;
    let mut modified_cost = base_cost.to_vec();

    // Check if opponent has Goomy with Sticky Membrane in the active spot
    let opponent = (attacking_player + 1) % 2;
    if let Some(opponent_active) = &state.in_play_pokemon[opponent][0] {
        if let Some(ability_id) = AbilityId::from_pokemon_id(&opponent_active.get_id()[..]) {
            if ability_id == AbilityId::B1177GoomyStickyMembrane {
                // Add 1 Colorless energy to the attack cost
                modified_cost.push(EnergyType::Colorless);
            }
        }
    }

    // Check if attacking active has an effect that increases attack cost
    let extra_colorless = state.in_play_pokemon[attacking_player][0]
        .as_ref()
        .map(|active| {
            active
                .get_active_effects()
                .into_iter()
                .map(|effect| match effect {
                    CardEffect::IncreasedAttackCost { amount } => amount as usize,
                    _ => 0,
                })
                .sum::<usize>()
        })
        .unwrap_or_default();
    modified_cost.extend(vec![EnergyType::Colorless; extra_colorless]);

    modified_cost
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

/// Called when a Pokémon is knocked out
/// This is called before the Pokémon is discarded from play
pub(crate) fn on_knockout(
    state: &mut State,
    knocked_out_player: usize,
    knocked_out_idx: usize,
    is_from_active_attack: bool,
) {
    let knocked_out_pokemon = state.in_play_pokemon[knocked_out_player][knocked_out_idx]
        .as_ref()
        .expect("Pokemon should be there if knocked out");

    // Handle Electrical Cord
    if has_tool(knocked_out_pokemon, CardId::A3a065ElectricalCord) {
        // Only triggers if knocked out in active spot from an active attack
        if knocked_out_idx != 0 || !is_from_active_attack {
            return;
        }

        // Collect up to 2 Lightning energies from the knocked out Pokemon
        let mut lightning_energies = vec![];
        let knocked_out_pokemon_mut = state.in_play_pokemon[knocked_out_player][knocked_out_idx]
            .as_mut()
            .expect("Pokemon should be there if knocked out");
        for _ in 0..2 {
            if let Some(pos) = knocked_out_pokemon_mut
                .attached_energy
                .iter()
                .position(|e| *e == EnergyType::Lightning)
            {
                // Remove from pokemon so it doesn't end up in discard pile
                lightning_energies.push(knocked_out_pokemon_mut.attached_energy.swap_remove(pos));
            }
        }
        if lightning_energies.is_empty() {
            return;
        }

        // Distribute energies to benched Pokemon (1 each to up to 2 Pokemon)
        debug!(
            "Electrical Cord: Moving {} Lightning Energy from knocked out Pokemon",
            lightning_energies.len()
        );
        // Collect just the indices to avoid borrow checker issues
        let bench_indices: Vec<_> = state
            .enumerate_bench_pokemon(knocked_out_player)
            .map(|(idx, _)| idx)
            .collect();
        for (i, energy) in lightning_energies.into_iter().enumerate() {
            if i < bench_indices.len() {
                let bench_idx = bench_indices[i];
                if let Some(pokemon) = state.in_play_pokemon[knocked_out_player][bench_idx].as_mut()
                {
                    pokemon.attached_energy.push(energy);
                    debug!(
                        "Electrical Cord: Attached Lightning Energy to benched Pokemon at position {}",
                        bench_idx
                    );
                }
            }
        }
    }
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
    fn test_get_attack_cost_with_increased_attack_cost_effect() {
        let mut state = State::default();
        let mut attacker = to_playable_card(&get_card_by_enum(CardId::A1001Bulbasaur), false);
        attacker.add_effect(CardEffect::IncreasedAttackCost { amount: 2 }, 1);
        state.in_play_pokemon[0][0] = Some(attacker);
        state.in_play_pokemon[1][0] = Some(to_playable_card(
            &get_card_by_enum(CardId::A1005Caterpie),
            false,
        ));

        let base_cost = vec![EnergyType::Grass];
        let modified = get_attack_cost(&base_cost, &state, 0);
        assert_eq!(
            modified,
            vec![
                EnergyType::Grass,
                EnergyType::Colorless,
                EnergyType::Colorless
            ]
        );
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
        let base_damage = modify_damage(&state, (0, 0), (attack.fixed_damage, 1, 0), true, None);

        // Add Giovanni effect
        state.add_turn_effect(TurnEffect::IncreasedDamage { amount: 10 }, 0);

        // Get damage with Giovanni effect
        let damage_with_giovanni =
            modify_damage(&state, (0, 0), (attack.fixed_damage, 1, 0), true, None);

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
        let base_damage_non_ex = modify_damage(&non_ex_state, (0, 0), (40, 1, 0), true, None);
        non_ex_state.add_turn_effect(TurnEffect::IncreasedDamageAgainstEx { amount: 20 }, 0);
        let damage_with_red_vs_non_ex =
            modify_damage(&non_ex_state, (0, 0), (40, 1, 0), true, None);
        assert_eq!(
            damage_with_red_vs_non_ex, base_damage_non_ex,
            "Red should not increase damage against non-EX Pokémon"
        );

        // EX opponent should receive the bonus damage
        let mut ex_state = State::default();
        ex_state.in_play_pokemon[0][0] = Some(to_playable_card(&attacker_card, false));
        let ex_defender = get_card_by_enum(CardId::A3122SolgaleoEx);
        ex_state.in_play_pokemon[1][0] = Some(to_playable_card(&ex_defender, false));
        let base_damage_ex = modify_damage(&ex_state, (0, 0), (40, 1, 0), true, None);
        ex_state.add_turn_effect(TurnEffect::IncreasedDamageAgainstEx { amount: 20 }, 0);
        let damage_with_red_vs_ex = modify_damage(&ex_state, (0, 0), (40, 1, 0), true, None);
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
        let damage_with_stiffen = modify_damage(&state, (0, 0), (120, 1, 0), true, None);

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

    #[test]
    fn test_aerodactyl_can_evolve_from_old_amber() {
        // Aerodactyl (regular) should be able to evolve from Old Amber fossil
        let aerodactyl = get_card_by_enum(CardId::A1210Aerodactyl);
        let old_amber = to_playable_card(&get_card_by_enum(CardId::A1218OldAmber), false);

        assert!(
            can_evolve_into(&aerodactyl, &old_amber),
            "Aerodactyl should be able to evolve from Old Amber fossil"
        );
    }

    #[test]
    fn test_aerodactyl_ex_can_evolve_from_old_amber() {
        // Aerodactyl ex should be able to evolve from Old Amber fossil
        let aerodactyl_ex = get_card_by_enum(CardId::A1a046AerodactylEx);
        let old_amber = to_playable_card(&get_card_by_enum(CardId::A1218OldAmber), false);

        assert!(
            can_evolve_into(&aerodactyl_ex, &old_amber),
            "Aerodactyl ex should be able to evolve from Old Amber fossil"
        );
    }

    #[test]
    fn test_omanyte_can_evolve_from_helix_fossil() {
        // Omanyte should be able to evolve from Helix Fossil
        let omanyte = get_card_by_enum(CardId::A1081Omanyte);
        let helix_fossil = to_playable_card(&get_card_by_enum(CardId::A1216HelixFossil), false);

        assert!(
            can_evolve_into(&omanyte, &helix_fossil),
            "Omanyte should be able to evolve from Helix Fossil"
        );
    }

    #[test]
    fn test_kabuto_can_evolve_from_dome_fossil() {
        // Kabuto should be able to evolve from Dome Fossil
        let kabuto = get_card_by_enum(CardId::A1158Kabuto);
        let dome_fossil = to_playable_card(&get_card_by_enum(CardId::A1217DomeFossil), false);

        assert!(
            can_evolve_into(&kabuto, &dome_fossil),
            "Kabuto should be able to evolve from Dome Fossil"
        );
    }

    #[test]
    fn test_aerodactyl_cannot_evolve_from_wrong_fossil() {
        // Aerodactyl should NOT be able to evolve from Helix Fossil (only Old Amber)
        let aerodactyl = get_card_by_enum(CardId::A1210Aerodactyl);
        let helix_fossil = to_playable_card(&get_card_by_enum(CardId::A1216HelixFossil), false);

        assert!(
            !can_evolve_into(&aerodactyl, &helix_fossil),
            "Aerodactyl should not be able to evolve from Helix Fossil"
        );
    }
}
