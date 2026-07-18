use core::panic;
use std::vec;

use log::debug;

use crate::{
    actions::{
        abilities::AbilityMechanic, ability_mechanic_from_effect, get_ability_mechanic,
        SimpleAction,
    },
    card_ids::CardId,
    effects::{CardEffect, TurnEffect},
    models::{Card, EnergyType, PlayedCard, TrainerCard, TrainerType, BASIC_STAGE},
    stadiums::{
        get_arena_of_antiquity_damage_bonus, get_training_area_damage_bonus,
        is_bounded_field_active, is_hiking_trail_active,
    },
    tools::has_tool,
    State,
};

fn is_fossil(trainer_card: &TrainerCard) -> bool {
    trainer_card.trainer_card_type == TrainerType::Fossil
}

const ANCIENT_POKEMON_NAMES: [&str; 11] = [
    "Brute Bonnet",
    "Slither Wing",
    "Scream Tail",
    "Flutter Mane ex",
    "Great Tusk",
    "Sandy Shocks",
    "Koraidon ex",
    "Roaring Moon",
    "Walking Wake",
    "Gouging Fire",
    "Raging Bolt",
];

pub fn is_ancient_pokemon(pokemon_name: &str) -> bool {
    ANCIENT_POKEMON_NAMES.contains(&pokemon_name)
}

const FUTURE_POKEMON_NAMES: [&str; 11] = [
    "Iron Moth",
    "Iron Bundle ex",
    "Iron Hands",
    "Iron Thorns",
    "Miraidon ex",
    "Iron Valiant",
    "Iron Leaves",
    "Iron Boulder",
    "Iron Crown",
    "Iron Jugulis",
    "Iron Treads",
];

pub fn is_future_pokemon(pokemon_name: &str) -> bool {
    FUTURE_POKEMON_NAMES.contains(&pokemon_name)
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
pub(crate) fn on_evolve(actor: usize, state: &mut State, to_card: &Card, from_hand: bool) {
    if !from_hand {
        return;
    }

    match get_ability_mechanic(to_card) {
        Some(AbilityMechanic::DrawCardsOnEvolve { amount }) => {
            state.move_generation_stack.push((
                actor,
                vec![
                    SimpleAction::DrawCard {
                        amount: *amount as u8,
                    },
                    SimpleAction::Noop,
                ],
            ));
        }
        Some(AbilityMechanic::HealTypedPokemonOnEvolve {
            energy_type,
            amount,
        }) => {
            let possible_moves: Vec<SimpleAction> = state
                .enumerate_in_play_pokemon(actor)
                .filter(|(_, pokemon)| {
                    pokemon.is_damaged() && pokemon.get_energy_type() == Some(*energy_type)
                })
                .map(|(in_play_idx, _)| SimpleAction::Heal {
                    in_play_idx,
                    amount: *amount,
                    cure_status: false,
                })
                .chain(std::iter::once(SimpleAction::Noop))
                .collect();

            if possible_moves.len() > 1 {
                state.move_generation_stack.push((actor, possible_moves));
            }
        }
        Some(AbilityMechanic::AttachEnergyFromZoneToActiveTypedOnEvolve { energy_type }) => {
            state.move_generation_stack.push((
                actor,
                vec![
                    SimpleAction::Attach {
                        attachments: vec![(1, *energy_type, 0)],
                        is_turn_energy: false,
                    },
                    SimpleAction::Noop,
                ],
            ));
        }
        Some(AbilityMechanic::DamageOpponentActiveOnEvolve { amount }) => {
            state.move_generation_stack.push((
                actor,
                vec![
                    SimpleAction::ApplyDamage {
                        attacking_ref: (actor, 0),
                        targets: vec![(*amount, (actor + 1) % 2, 0)],
                        is_from_active_attack: false,
                    },
                    SimpleAction::Noop,
                ],
            ));
        }
        Some(AbilityMechanic::DiscardRandomEnergyFromOpponentActiveOnEvolve) => {
            let opponent = (actor + 1) % 2;
            let has_energy = state
                .maybe_get_active(opponent)
                .is_some_and(|active| !active.attached_energy.is_empty());
            if has_energy {
                state.move_generation_stack.push((
                    actor,
                    vec![
                        SimpleAction::DiscardRandomOpponentActiveEnergy,
                        SimpleAction::Noop,
                    ],
                ));
            }
        }
        _ => {}
    }
}

/// Called when a basic Pokémon is placed from hand onto the bench (index > 0).
pub(crate) fn on_bench_from_hand(actor: usize, state: &mut State, card: &Card, bench_idx: usize) {
    match get_ability_mechanic(card) {
        Some(AbilityMechanic::LegendaryDrive) => {
            if state.maybe_get_active(actor).is_none() {
                return;
            }
            debug!("Legendary Drive: offering switch to active");
            state.move_generation_stack.push((
                actor,
                vec![
                    SimpleAction::UseAbility {
                        in_play_idx: bench_idx,
                    },
                    SimpleAction::Noop,
                ],
            ));
        }
        Some(AbilityMechanic::AncientRoar) => {
            let opponent = (actor + 1) % 2;
            if state.enumerate_bench_pokemon(opponent).next().is_none() {
                return;
            }
            debug!("Ancient Roar: offering force-switch of opponent's active");
            state.move_generation_stack.push((
                actor,
                vec![
                    SimpleAction::UseAbility {
                        in_play_idx: bench_idx,
                    },
                    SimpleAction::Noop,
                ],
            ));
        }
        _ => {}
    }
}

/// Called when a basic Pokémon is played to the bench from hand
pub(crate) fn on_end_turn(player_ending_turn: usize, state: &mut State) {
    // Check if active Pokémon has an end-of-turn ability
    let active = state.get_active(player_ending_turn);
    if let Some(mechanic) = get_ability_mechanic(&active.card) {
        if matches!(
            mechanic,
            AbilityMechanic::EndTurnDrawCardIfActive { amount: 1 }
        ) {
            debug!("Legendary Pulse: Drawing a card");
            state.move_generation_stack.push((
                player_ending_turn,
                vec![SimpleAction::DrawCard { amount: 1 }],
            ));
        }
        if let AbilityMechanic::EndTurnHealSelfIfActive { amount } = mechanic {
            debug!("Full-Mouth Manner: Healing 20 damage from active");
            let active = state.get_active_mut(player_ending_turn);
            active.heal(*amount);
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

    // Process delayed spot damage effects from turn effects (e.g. Meowscarada ex's Flower Trick).
    // These target a board position, so they hit whichever Pokémon occupies the spot at trigger time.
    let triggered_spot_damages: Vec<(usize, usize, usize, u32)> = state
        .get_current_turn_effects()
        .into_iter()
        .filter_map(|effect| match effect {
            TurnEffect::DelayedSpotDamage {
                source_player,
                target_player,
                target_in_play_idx,
                amount,
            } if target_player == player_ending_turn => {
                Some((source_player, target_player, target_in_play_idx, amount))
            }
            _ => None,
        })
        .collect();

    for (source_player, target_player, target_in_play_idx, amount) in triggered_spot_damages {
        if state.in_play_pokemon[target_player][target_in_play_idx].is_none() {
            continue;
        }

        debug!(
            "Delayed spot damage: Applying {} damage to player {} slot {}",
            amount, target_player, target_in_play_idx
        );
        crate::actions::handle_damage(
            state,
            (source_player, 0),
            &[(amount, target_player, target_in_play_idx)],
            false,
            None,
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
                if matches!(
                    get_ability_mechanic(&pokemon.card),
                    Some(AbilityMechanic::EndFirstTurnAttachEnergyToSelf {
                        energy_type: EnergyType::Lightning
                    })
                ) {
                    return Some(in_play_idx);
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

    // Hiking Trail: At the end of each player's turn, draw cards until they have 3 in hand.
    if is_hiking_trail_active(state) {
        let hand_size = state.hands[player_ending_turn].len();
        if hand_size < 3 {
            let amount = 3 - hand_size;
            debug!(
                "Hiking Trail: Player {} drawing {} card(s) to reach 3 in hand",
                player_ending_turn, amount
            );
            for _ in 0..amount {
                state.maybe_draw_card(player_ending_turn);
            }
        }
    }

    apply_bad_dreams_damage(state);
}

/// Apply Bad Dreams ability damage: for each player's Darkrai in play, if that player's
/// opponent has an Asleep Active Pokémon, deal 20 damage to it.
fn apply_bad_dreams_damage(state: &mut State) {
    let sources: Vec<(usize, usize, u32)> = (0..2)
        .flat_map(|player| {
            state
                .enumerate_in_play_pokemon(player)
                .filter_map(move |(idx, pokemon)| {
                    if pokemon.is_knocked_out() {
                        return None;
                    }
                    get_ability_mechanic(&pokemon.card).and_then(|m| match m {
                        AbilityMechanic::BadDreamsEndOfTurn { amount } => {
                            Some((player, idx, *amount))
                        }
                        _ => None,
                    })
                })
        })
        .collect();

    for (darkrai_owner, darkrai_idx, amount) in sources {
        if state.in_play_pokemon[darkrai_owner][darkrai_idx].is_none() {
            continue;
        }
        let opponent = (darkrai_owner + 1) % 2;
        let Some(opponent_active) = state.in_play_pokemon[opponent][0].as_ref() else {
            continue;
        };
        if !opponent_active.is_asleep() {
            continue;
        }
        debug!(
            "Bad Dreams: Player {}'s Darkrai deals {} damage to opponent's Asleep active",
            darkrai_owner, amount
        );
        crate::actions::handle_damage(
            state,
            (darkrai_owner, darkrai_idx),
            &[(amount, opponent, 0)],
            false,
            None,
        );
    }
}

pub(crate) fn can_play_support(state: &State) -> bool {
    let has_modifiers = state
        .get_current_turn_effects()
        .iter()
        .any(|x| matches!(x, TurnEffect::NoSupportCards));

    // Check if opponent has Gengar ex with Shadowy Spellbind in active spot
    let opponent = (state.current_player + 1) % 2;
    let blocked_by_gengar =
        state.in_play_pokemon[opponent][0]
            .as_ref()
            .is_some_and(|opponent_active| {
                matches!(
                    get_ability_mechanic(&opponent_active.card),
                    Some(AbilityMechanic::NoOpponentSupportInActive)
                )
            });

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
    // Metal Core Barrier: "The [M] Pokémon this card is attached to takes -50 damage..."
    if has_tool(defending_pokemon, CardId::B2148MetalCoreBarrier)
        && defending_pokemon.get_energy_type() == Some(EnergyType::Metal)
    {
        debug!("Metal Core Barrier: Reducing damage by 50");
        return 50;
    }
    0
}

fn get_steel_apron_reduction(
    state: &State,
    attacking_player: usize,
    (target_player, target_idx): (usize, usize),
    is_from_active_attack: bool,
) -> u32 {
    if !is_from_active_attack || attacking_player == target_player {
        return 0;
    }

    let defending_pokemon = &state.in_play_pokemon[target_player][target_idx]
        .as_ref()
        .expect("Defending Pokemon should be there when checking Steel Apron");
    // Steel Apron: "The [M] Pokémon this card is attached to takes -10 damage..."
    if has_tool(defending_pokemon, CardId::A4153SteelApron)
        && defending_pokemon.get_energy_type() == Some(EnergyType::Metal)
    {
        debug!("Steel Apron: Reducing damage by 10");
        return 10;
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

    // Invariant: the defending player always has an active Pokemon while any other
    // action can be processed. If a knockout empties the active spot,
    // `trigger_promotion_or_declare_winner` immediately queues an `Activate` choice
    // at the top of `move_generation_stack` (or ends the game), which
    // `generate_possible_actions` short-circuits to. So no further action - including
    // this `ApplyDamage` - can run until the active spot is refilled (or the game ends).
    let defenders_active = state.in_play_pokemon[target_player][0]
        .as_ref()
        .expect("Defending Pokemon should be there when checking Intimidating Fang");
    // Reads the unified effect list: Intimidating Fang (a passive ability) presents as a
    // `ReduceOpponentActiveDamage` effect on the Active defender.
    defenders_active
        .get_effective_card_effects()
        .iter()
        .filter_map(|effect| match effect {
            CardEffect::ReduceOpponentActiveDamage { amount } => Some(*amount),
            _ => None,
        })
        .sum()
}

fn get_ability_damage_reduction(
    receiving_pokemon: &crate::models::PlayedCard,
    is_from_active_attack: bool,
) -> u32 {
    if !is_from_active_attack {
        return 0;
    }
    // Reads the unified effect list, so Cloyster's Shell Armor (a passive ability) is handled the
    // same way as any stored effect.
    receiving_pokemon
        .get_effective_card_effects()
        .iter()
        .filter_map(|effect| match effect {
            CardEffect::ReduceDamageFromAttacks { amount } => Some(*amount),
            _ => None,
        })
        .sum()
}

fn get_ability_damage_increase(
    state: &State,
    attacking_player: usize,
    attacking_pokemon: &crate::models::PlayedCard,
    is_active_to_active: bool,
) -> u32 {
    if !is_active_to_active {
        return 0;
    }

    let Some(ability) = attacking_pokemon.card.get_ability() else {
        return 0;
    };

    if let Some(AbilityMechanic::IncreaseDamageWhenRemainingHpAtMost {
        amount,
        hp_threshold,
    }) = ability_mechanic_from_effect(&ability.effect)
    {
        if attacking_pokemon.get_remaining_hp() <= *hp_threshold {
            debug!(
                "IncreaseDamageWhenRemainingHpAtMost: Increasing damage by {}",
                amount
            );
            return *amount;
        }
    }

    if let Some(AbilityMechanic::IncreaseDamageIfArceusInPlay { amount }) =
        ability_mechanic_from_effect(&ability.effect)
    {
        let has_arceus = state
            .enumerate_in_play_pokemon(attacking_player)
            .any(|(_, pokemon)| {
                let name = pokemon.get_name();
                name == "Arceus" || name == "Arceus ex"
            });
        if has_arceus {
            debug!(
                "IncreaseDamageIfArceusInPlay: Increasing damage by {}",
                amount
            );
            return *amount;
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
            TurnEffect::IncreasedDamageForSpecificPokemonAgainstEx {
                amount,
                pokemon_names,
            } if target_is_ex => {
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
            TurnEffect::IncreasedDamageForTypeAgainstEx {
                amount,
                energy_type,
            } if target_is_ex && attacking_pokemon.get_energy_type() == Some(*energy_type) => {
                *amount
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

fn get_increased_vulnerability_modifiers(
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
        .filter(|effect| matches!(effect, CardEffect::IncreasedVulnerability { .. }))
        .map(|effect| match effect {
            CardEffect::IncreasedVulnerability { amount } => *amount,
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

enum WeaknessApplication {
    None,
    Flat(u32),
    Double,
}

const DAMAGE_UNAFFECTED_BY_WEAKNESS_EFFECT: &str =
    "This attack's damage isn't affected by Weakness.";

/// Sawk's Brick Break (and any card sharing this text): the attack's damage ignores every effect
/// on the opponent's Active Pokémon — ability-derived reductions/preventions, stored CardEffects,
/// and damage-reducing Tools alike. See `attack_ignores_opponent_active_effects`.
pub(crate) const DAMAGE_UNAFFECTED_BY_OPPONENT_ACTIVE_EFFECTS_EFFECT: &str =
    "This attack's damage isn't affected by any effects on your opponent's Active Pokémon.";

#[derive(Clone, Copy, Default)]
pub(crate) struct DamageModifierContext<'a> {
    pub(crate) attack_name: Option<&'a str>,
    pub(crate) attack_effect: Option<&'a str>,
}

fn attack_effect_ignores_weakness(context: DamageModifierContext<'_>) -> bool {
    // TODO: If more attack text needs to alter damage-modifier stages, replace this
    // effect-string check with a typed attack metadata/damage-modifier capability.
    context.attack_effect == Some(DAMAGE_UNAFFECTED_BY_WEAKNESS_EFFECT)
}

fn attack_ignores_opponent_active_effects(context: DamageModifierContext<'_>) -> bool {
    context.attack_effect == Some(DAMAGE_UNAFFECTED_BY_OPPONENT_ACTIVE_EFFECTS_EFFECT)
}

fn get_weakness_application(
    state: &State,
    is_active_to_active: bool,
    target_player: usize,
    attacking_pokemon: &crate::models::PlayedCard,
    context: DamageModifierContext<'_>,
) -> WeaknessApplication {
    if !is_active_to_active || attack_effect_ignores_weakness(context) {
        return WeaknessApplication::None;
    }
    let receiving = state.get_active(target_player);

    if receiving
        .get_active_effects()
        .iter()
        .any(|effect| matches!(effect, CardEffect::NoWeakness))
    {
        debug!("NoWeakness: Ignoring weakness damage");
        return WeaknessApplication::None;
    }

    if let Card::Pokemon(pokemon_card) = &receiving.card {
        if pokemon_card.weakness == attacking_pokemon.card.get_type() {
            debug!(
                "Weakness! {:?} is weak to {:?}",
                pokemon_card,
                attacking_pokemon.card.get_type()
            );
            // Bounded Field: ×2 all damage (including other modifiers) for non-Mega-ex attackers
            if is_bounded_field_active(state)
                && !(attacking_pokemon.card.is_mega() && attacking_pokemon.card.is_ex())
            {
                debug!("Bounded Field: Applying weakness as ×2 of total damage");
                return WeaknessApplication::Double;
            }
            return WeaknessApplication::Flat(20);
        }
    }
    WeaknessApplication::None
}

fn get_future_booster_damage_bonus(attacking_pokemon: &PlayedCard) -> u32 {
    if has_tool(attacking_pokemon, CardId::B3a070FutureBoosterEnergyCapsule)
        && is_future_pokemon(&attacking_pokemon.get_name())
    {
        debug!("Future Booster Energy Capsule: Increasing damage by 20");
        return 20;
    }
    0
}

// TODO: Confirm is_from_attack and goes to enemy active
pub(crate) fn modify_damage(
    state: &State,
    attacking_ref: (usize, usize),
    target_ref: (u32, usize, usize),
    is_from_active_attack: bool,
    context: DamageModifierContext<'_>,
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

    // "Effects on the opponent's Active Pokémon" (Sawk's Brick Break) — when this attack ignores
    // them, treat the receiver's effect list as empty for every defensive stage below, and also
    // bypass its damage-reducing Tools. Only the opponent's Active is targeted by such attacks.
    let skip_target_effects = attack_ignores_opponent_active_effects(context)
        && is_from_active_attack
        && target_idx == 0
        && attacking_player != target_player;

    // The unified "effects on this Pokémon" list: stored CardEffects plus any derived from the
    // receiver's passive ability (Cloyster's Shell Armor, Oricorio's Safeguard, Meowth's Carefree
    // Steps, ...). Damage code checks this single list instead of scanning the board for abilities.
    let target_effects: Vec<CardEffect> = if skip_target_effects {
        Vec::new()
    } else {
        receiving_pokemon.get_effective_card_effects()
    };

    // Safeguard (Oricorio): prevent all damage from the opponent's Pokémon ex.
    if target_effects
        .iter()
        .any(|e| matches!(e, CardEffect::PreventAllDamageFromEx))
        && is_from_active_attack
        && attacking_pokemon.card.is_ex()
    {
        debug!("Safeguard: Preventing all damage from opponent's Pokémon ex");
        return 0;
    }
    // Shell Shield (Wartortle): prevent all damage while benched.
    if target_effects
        .iter()
        .any(|e| matches!(e, CardEffect::PreventDamageWhileBenched))
        && is_from_active_attack
        && target_idx != 0
    {
        debug!("Shell Shield: Preventing all damage to benched Wartortle");
        return 0;
    }

    // Protective Poncho: prevent all damage to benched Pokémon with this tool attached
    if target_idx != 0
        && !skip_target_effects
        && has_tool(receiving_pokemon, CardId::B2147ProtectivePoncho)
    {
        debug!("Protective Poncho: Preventing all damage to benched Pokémon");
        return 0;
    }

    // Check for PreventAllDamageAndEffects (Shinx's Hide)
    if target_effects
        .iter()
        .any(|effect| matches!(effect, CardEffect::PreventAllDamageAndEffects))
    {
        debug!("PreventAllDamageAndEffects: Preventing all damage and effects");
        return 0;
    }

    // Check for PreventDamageFromBasic (Carracosta's Blocking Shell)
    if attacking_pokemon.card.is_basic()
        && target_effects
            .iter()
            .any(|effect| matches!(effect, CardEffect::PreventDamageFromBasic))
    {
        debug!("PreventDamageFromBasic: Preventing all damage from a Basic Pokémon");
        return 0;
    }

    // Calculate all modifiers
    let is_active_to_active = target_idx == 0 && attacking_idx == 0 && is_from_active_attack;
    let target_is_ex = receiving_pokemon.card.is_ex();
    let attacker_is_eevee_evolution = attacking_pokemon.evolved_from("Eevee");

    // Every damage-reducing effect/tool below sits on the *target*, so an attack that ignores
    // effects on the opponent's Active Pokémon (Sawk) zeroes all of them.
    let intimidating_fang_reduction = if skip_target_effects {
        0
    } else {
        get_intimidating_fang_reduction(state, attacking_ref, target_ref, is_from_active_attack)
    };
    let heavy_helmet_reduction = if skip_target_effects {
        0
    } else {
        get_heavy_helmet_reduction(state, (target_player, target_idx))
    };
    let metal_core_barrier_reduction = if skip_target_effects {
        0
    } else {
        get_metal_core_barrier_reduction(state, (target_player, target_idx), is_from_active_attack)
    };
    let steel_apron_reduction = if skip_target_effects {
        0
    } else {
        get_steel_apron_reduction(
            state,
            attacking_player,
            (target_player, target_idx),
            is_from_active_attack,
        )
    };
    let ability_damage_reduction = if skip_target_effects {
        0
    } else {
        get_ability_damage_reduction(receiving_pokemon, is_from_active_attack)
    };
    let ability_damage_increase = get_ability_damage_increase(
        state,
        attacking_player,
        attacking_pokemon,
        is_active_to_active,
    );
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
        context.attack_name,
    );
    // Reductions and vulnerability are both "effects on the target"; ignore *any* of them (whether
    // they help or hurt the attacker) when the attack bypasses opponent-active effects.
    let reduced_card_effect_modifiers = if skip_target_effects {
        0
    } else {
        get_reduced_card_effect_modifiers(state, is_active_to_active, target_player)
    };
    let increased_vulnerability_modifiers = if skip_target_effects {
        0
    } else {
        get_increased_vulnerability_modifiers(state, is_active_to_active, target_player)
    };
    let reduced_turn_effect_modifiers = if skip_target_effects {
        0
    } else {
        get_turn_effect_damage_reduction(
            state,
            target_player,
            receiving_pokemon,
            attacking_player,
            is_from_active_attack,
        )
    };
    let weakness_application = get_weakness_application(
        state,
        is_active_to_active,
        target_player,
        attacking_pokemon,
        context,
    );

    // Type-specific damage boost abilities (e.g., Lucario's Fighting Coach, Aegislash's Royal Command)
    // These check if certain ability-holders are in play and boost damage for specific energy types
    // Only applies to active-to-active attacks (not damage moves like Dusknoir's Shadow Void)
    let type_boost_bonus = if is_active_to_active {
        calculate_type_boost_bonus(state, attacking_player, attacking_pokemon)
    } else {
        0
    };

    let future_booster_damage_bonus = if is_active_to_active {
        get_future_booster_damage_bonus(attacking_pokemon)
    } else {
        0
    };

    // Stadium damage bonus (e.g., Training Area for Stage 1 Pokemon)
    // Only applies to attacks against the opponent's Active Pokemon
    let stadium_damage_bonus = if is_active_to_active {
        let training_area = get_training_area_damage_bonus(state, get_stage(attacking_pokemon));
        let arena_of_antiquity = get_arena_of_antiquity_damage_bonus(
            state,
            attacking_pokemon
                .get_energy_type()
                .unwrap_or(EnergyType::Colorless),
            target_is_ex,
        );
        training_area + arena_of_antiquity
    } else {
        0
    };

    debug!(
        "Attack: {:?}, IncreasedDamage: {}, IncreasedAttackSpecific: {}, IncreasedVulnerability: {}, ReducedDamage: {}, TurnEffectReduction: {}, HeavyHelmet: {}, MetalCoreBarrier: {}, SteelApron: {}, IntimidatingFang: {}, AbilityReduction: {}, AbilityIncrease: {}, TypeBoost: {}, StadiumBonus: {}, FutureBooster: {}",
        base_damage,
        increased_turn_effect_modifiers,
        increased_attack_specific_modifiers,
        increased_vulnerability_modifiers,
        reduced_card_effect_modifiers,
        reduced_turn_effect_modifiers,
        heavy_helmet_reduction,
        metal_core_barrier_reduction,
        steel_apron_reduction,
        intimidating_fang_reduction,
        ability_damage_reduction,
        ability_damage_increase,
        type_boost_bonus,
        stadium_damage_bonus,
        future_booster_damage_bonus
    );
    let pre_weakness = (base_damage
        + ability_damage_increase
        + increased_turn_effect_modifiers
        + increased_attack_specific_modifiers
        + increased_vulnerability_modifiers
        + type_boost_bonus
        + stadium_damage_bonus
        + future_booster_damage_bonus)
        .saturating_sub(
            reduced_card_effect_modifiers
                + reduced_turn_effect_modifiers
                + heavy_helmet_reduction
                + metal_core_barrier_reduction
                + steel_apron_reduction
                + intimidating_fang_reduction
                + ability_damage_reduction,
        );
    let final_damage = match weakness_application {
        WeaknessApplication::None => pre_weakness,
        WeaknessApplication::Flat(amount) => pre_weakness + amount,
        WeaknessApplication::Double => pre_weakness * 2,
    };

    // Threshold-based prevention (e.g. Cascoon's Harden): prevent all damage if it is low enough.
    let prevented_by_threshold = target_effects
        .iter()
        .any(|effect| matches!(effect, CardEffect::PreventDamageIfLessOrEqual { threshold } if final_damage <= *threshold));
    if prevented_by_threshold {
        debug!("PreventDamageIfLessOrEqual: Preventing {final_damage} damage");
        return 0;
    }

    final_damage
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
        if let Some(mechanic) = get_ability_mechanic(&pokemon.card) {
            match mechanic {
                AbilityMechanic::IncreaseDamageForTypeInPlay {
                    energy_type,
                    amount,
                } if attacker_energy_type == *energy_type => {
                    debug!("Type damage bonus: Increasing damage by {}", amount);
                    bonus += amount;
                }
                AbilityMechanic::IncreaseDamageForTwoTypesInPlay {
                    energy_type_a,
                    energy_type_b,
                    amount,
                } if attacker_energy_type == *energy_type_a
                    || attacker_energy_type == *energy_type_b =>
                {
                    debug!("Type damage bonus: Increasing damage by {}", amount);
                    bonus += amount;
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
    let mut modified_cost = base_cost.to_vec();

    // Check if opponent has Goomy with Sticky Membrane in the active spot
    let opponent = (attacking_player + 1) % 2;
    if let Some(opponent_active) = &state.in_play_pokemon[opponent][0] {
        if matches!(
            get_ability_mechanic(&opponent_active.card),
            Some(AbilityMechanic::IncreaseAttackCostForOpponentActive { amount: 1 })
        ) {
            modified_cost.push(EnergyType::Colorless);
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

    // Check for Barry-style turn effects that reduce colorless cost for specific pokemon
    if let Some(active) = &state.in_play_pokemon[attacking_player][0] {
        let active_name = active.get_name();
        let reduction: usize = state
            .get_current_turn_effects()
            .iter()
            .filter_map(|e| {
                if let TurnEffect::ReducedAttackCostForSpecificPokemon {
                    amount,
                    pokemon_names,
                } = e
                {
                    if pokemon_names.contains(&active_name) {
                        return Some(*amount as usize);
                    }
                }
                None
            })
            .sum();
        for _ in 0..reduction {
            if let Some(pos) = modified_cost
                .iter()
                .position(|e| *e == EnergyType::Colorless)
            {
                modified_cost.remove(pos);
            }
        }
    }

    modified_cost = future_system_cost(modified_cost, state, attacking_player);

    modified_cost
}

fn future_system_cost(mut cost: Vec<EnergyType>, state: &State, player: usize) -> Vec<EnergyType> {
    let attacker_is_future = state.in_play_pokemon[player][0]
        .as_ref()
        .is_some_and(|active| is_future_pokemon(&active.get_name()));
    let has_future_system = attacker_is_future
        && state.in_play_pokemon[player].iter().flatten().any(|p| {
            matches!(
                get_ability_mechanic(&p.card),
                Some(AbilityMechanic::FutureSystem)
            )
        });
    if has_future_system {
        if let Some(pos) = cost.iter().position(|e| *e == EnergyType::Colorless) {
            debug!("Future System: Reducing attack cost by 1 Colorless");
            cost.remove(pos);
        }
    }
    cost
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
    attacking_ref: (usize, usize),
    is_from_active_attack: bool,
) {
    // A genuine opponent KO: an active attack that knocked out a Pokémon belonging to a
    // different player than the attacker (not a self/recoil KO).
    let is_opponent_attack = is_from_active_attack && attacking_ref.0 != knocked_out_player;
    apply_lucky_egg(
        state,
        knocked_out_player,
        knocked_out_idx,
        is_opponent_attack,
    );
    apply_electrical_cord(
        state,
        knocked_out_player,
        knocked_out_idx,
        is_from_active_attack,
    );
    apply_offload_pass(
        state,
        knocked_out_player,
        knocked_out_idx,
        is_opponent_attack,
    );
}

/// Lucky Egg: when the holder is Knocked Out by an opponent's attack, draw until hand has 5.
/// Position-agnostic — triggers whether the holder was KO'd in the Active Spot or on the Bench.
fn apply_lucky_egg(
    state: &mut State,
    knocked_out_player: usize,
    knocked_out_idx: usize,
    is_opponent_attack: bool,
) {
    let has_lucky_egg = {
        let knocked_out_pokemon = state.in_play_pokemon[knocked_out_player][knocked_out_idx]
            .as_ref()
            .expect("Pokemon should be there if knocked out");
        has_tool(knocked_out_pokemon, CardId::B3148LuckyEgg)
    };
    if !has_lucky_egg || !is_opponent_attack {
        return;
    }

    debug!("Lucky Egg: Drawing cards until hand has 5");
    let draws_needed = 5usize.saturating_sub(state.hands[knocked_out_player].len());
    for _ in 0..draws_needed {
        if state.decks[knocked_out_player].cards.is_empty() {
            break;
        }
        state.maybe_draw_card(knocked_out_player);
    }
}

/// Electrical Cord: "If the [L] Pokémon this card is attached to is in the Active Spot and is
/// Knocked Out by damage from an attack..." move up to 2 of its Lightning Energy to Benched
/// Pokémon (1 each to the two lowest-index Benched Pokémon). Note this uses the raw
/// `is_from_active_attack` flag, so it fires even on a self-KO from one's own active attack.
///
/// The early returns below only exit this helper (not `on_knockout`), letting control fall
/// through to `apply_offload_pass`. That is behavior-preserving because every return path here
/// is gated on the holder being a Lightning Pokémon, and no Lightning Pokémon carries the
/// `MoveAllTypedEnergyToBenchOnKnockout` ability that Offload Pass requires — so Offload Pass
/// would be a no-op in these cases regardless.
fn apply_electrical_cord(
    state: &mut State,
    knocked_out_player: usize,
    knocked_out_idx: usize,
    is_from_active_attack: bool,
) {
    let has_electrical_cord = {
        let knocked_out_pokemon = state.in_play_pokemon[knocked_out_player][knocked_out_idx]
            .as_ref()
            .expect("Pokemon should be there if knocked out");
        has_tool(knocked_out_pokemon, CardId::A3a065ElectricalCord)
            && knocked_out_pokemon.get_energy_type() == Some(EnergyType::Lightning)
    };
    if !has_electrical_cord {
        return;
    }
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
            if let Some(pokemon) = state.in_play_pokemon[knocked_out_player][bench_idx].as_mut() {
                pokemon.attached_energy.push(energy);
                debug!(
                    "Electrical Cord: Attached Lightning Energy to benched Pokemon at position {}",
                    bench_idx
                );
            }
        }
    }
}

/// Passimian ex's Offload Pass: if this Pokémon is Knocked Out in the Active Spot by an
/// opponent's attack, move all of its typed Energy to 1 of your Benched Pokémon (your choice).
fn apply_offload_pass(
    state: &mut State,
    knocked_out_player: usize,
    knocked_out_idx: usize,
    is_opponent_attack: bool,
) {
    if !is_opponent_attack || knocked_out_idx != 0 {
        return;
    }

    let offload_energy = state.in_play_pokemon[knocked_out_player][knocked_out_idx]
        .as_ref()
        .and_then(|pokemon| match get_ability_mechanic(&pokemon.card) {
            Some(AbilityMechanic::MoveAllTypedEnergyToBenchOnKnockout { energy_type }) => {
                Some(*energy_type)
            }
            _ => None,
        });
    let Some(energy_type) = offload_energy else {
        return;
    };

    let bench_indices: Vec<usize> = state
        .enumerate_bench_pokemon(knocked_out_player)
        .map(|(idx, _)| idx)
        .collect();
    // With no Benched Pokémon there is nowhere to move the Energy (and the game is about to
    // end), so leave it to be discarded with this Pokémon.
    if bench_indices.is_empty() {
        return;
    }

    // Remove the Energy from the KO'd Pokémon first so it isn't sent to the discard
    // pile; the chosen Attach below re-attaches it to a Benched Pokémon.
    let moved = {
        let ko_pokemon = state.in_play_pokemon[knocked_out_player][knocked_out_idx]
            .as_mut()
            .expect("Pokemon should be there if knocked out");
        let before = ko_pokemon.attached_energy.len();
        ko_pokemon.attached_energy.retain(|&e| e != energy_type);
        (before - ko_pokemon.attached_energy.len()) as u32
    };
    if moved == 0 {
        return;
    }

    // One choice per Benched Pokémon; all of the Energy goes to the chosen one.
    // Pushed here (before promotion, which is inserted at the bottom of the stack)
    // so it resolves while the Bench is still intact.
    let choices = bench_indices
        .into_iter()
        .map(|idx| SimpleAction::Attach {
            attachments: vec![(moved, energy_type, idx)],
            is_turn_energy: false,
        })
        .collect::<Vec<_>>();
    state
        .move_generation_stack
        .push((knocked_out_player, choices));
}

pub(crate) fn on_attack_knockout(
    state: &mut State,
    attacking_ref: (usize, usize),
    knocked_out_player: usize,
    is_from_active_attack: bool,
) {
    if !is_from_active_attack || knocked_out_player == attacking_ref.0 {
        return;
    }

    let Some(attacking_pokemon) = state.in_play_pokemon[attacking_ref.0][attacking_ref.1].as_mut()
    else {
        return;
    };
    if matches!(
        get_ability_mechanic(&attacking_pokemon.card),
        Some(AbilityMechanic::ProtectSelfNextTurnAfterAttackKnockout)
    ) {
        attacking_pokemon.add_effect(CardEffect::PreventAllDamageAndEffects, 1);
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
        let base_damage = modify_damage(
            &state,
            (0, 0),
            (attack.fixed_damage, 1, 0),
            true,
            DamageModifierContext::default(),
        );

        // Add Giovanni effect
        state.add_turn_effect(TurnEffect::IncreasedDamage { amount: 10 }, 0);

        // Get damage with Giovanni effect
        let damage_with_giovanni = modify_damage(
            &state,
            (0, 0),
            (attack.fixed_damage, 1, 0),
            true,
            DamageModifierContext::default(),
        );

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
        let base_damage_non_ex = modify_damage(
            &non_ex_state,
            (0, 0),
            (40, 1, 0),
            true,
            DamageModifierContext::default(),
        );
        non_ex_state.add_turn_effect(TurnEffect::IncreasedDamageAgainstEx { amount: 20 }, 0);
        let damage_with_red_vs_non_ex = modify_damage(
            &non_ex_state,
            (0, 0),
            (40, 1, 0),
            true,
            DamageModifierContext::default(),
        );
        assert_eq!(
            damage_with_red_vs_non_ex, base_damage_non_ex,
            "Red should not increase damage against non-EX Pokémon"
        );

        // EX opponent should receive the bonus damage
        let mut ex_state = State::default();
        ex_state.in_play_pokemon[0][0] = Some(to_playable_card(&attacker_card, false));
        let ex_defender = get_card_by_enum(CardId::A3122SolgaleoEx);
        ex_state.in_play_pokemon[1][0] = Some(to_playable_card(&ex_defender, false));
        let base_damage_ex = modify_damage(
            &ex_state,
            (0, 0),
            (40, 1, 0),
            true,
            DamageModifierContext::default(),
        );
        ex_state.add_turn_effect(TurnEffect::IncreasedDamageAgainstEx { amount: 20 }, 0);
        let damage_with_red_vs_ex = modify_damage(
            &ex_state,
            (0, 0),
            (40, 1, 0),
            true,
            DamageModifierContext::default(),
        );
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
        let damage_with_stiffen = modify_damage(
            &state,
            (0, 0),
            (120, 1, 0),
            true,
            DamageModifierContext::default(),
        );

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
