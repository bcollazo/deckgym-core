use std::collections::HashMap;

use log::debug;
use rand::rngs::StdRng;

use crate::{
    actions::{
        abilities::AbilityMechanic, ability_mechanic_from_effect,
        effect_ability_mechanic_map::get_ability_mechanic, shared_mutations, SimpleAction,
    },
    hooks::{
        get_counterattack_damage, modify_damage, on_end_turn, on_knockout, should_poison_attacker,
    },
    models::{Card, TrainerType},
    state::GameOutcome,
    State,
};

use super::Action;

pub(crate) type Probabilities = Vec<f64>;

// Mutations should be deterministic. They take StdRng because we simplify some states spaces
//  like "shuffling a deck" (which would otherwise yield a huge state space) to a single
//  mutation/state ("shuffled deck"). Bots should not use deck order information when forecasting.
pub(crate) type FnMutation = Box<dyn Fn(&mut StdRng, &mut State, &Action)>;
pub(crate) type Mutation = Box<dyn FnOnce(&mut StdRng, &mut State, &Action)>;
pub(crate) type Mutations = Vec<Mutation>;

/// Advance state to the next turn (i.e. maintain current_player and turn_count)
pub(crate) fn forecast_end_turn(state: &State) -> (Probabilities, Mutations) {
    let in_setup_phase = state.turn_count == 0;
    if in_setup_phase {
        let both_players_initiated =
            state.in_play_pokemon[0][0].is_some() && state.in_play_pokemon[1][0].is_some();
        if !both_players_initiated {
            // Just advance the setup phase to the next player
            return (
                vec![1.0],
                vec![Box::new(|_, state, _| {
                    state.end_turn_pending = false;
                    state.current_player = (state.current_player + 1) % 2;
                })],
            );
        }

        let next_player = (state.current_player + 1) % 2;
        let (start_probs, start_mutations) = start_turn_ability_outcomes(state, next_player);

        let mut outcomes: Mutations = Vec::with_capacity(start_mutations.len());
        for start_mutation in start_mutations {
            outcomes.push(Box::new(move |rng, state, action| {
                state.end_turn_pending = false;
                state.current_player = (state.current_player + 1) % 2;

                // Actually start game (no energy generation)
                state.turn_count = 1;
                state.end_turn_maintenance();
                start_mutation(rng, state, action);
                state.queue_draw_action(state.current_player, 1);
            }));
        }

        (start_probs, outcomes)
    } else {
        forecast_pokemon_checkup(state)
    }
}

/// Handle Status Effects
fn forecast_pokemon_checkup(state: &State) -> (Probabilities, Mutations) {
    let mut sleeps_to_handle = vec![];
    let mut paralyzed_to_handle = vec![];
    let mut poisons_to_handle = vec![];
    let mut burns_to_handle = vec![];
    for player in 0..2 {
        for (i, pokemon) in state.enumerate_in_play_pokemon(player) {
            if pokemon.asleep {
                sleeps_to_handle.push((player, i));
            }
            if pokemon.paralyzed {
                paralyzed_to_handle.push((player, i));
            }
            if pokemon.poisoned {
                poisons_to_handle.push((player, i));
                debug!("{player}'s Pokemon {i} is poisoned");
            }
            if pokemon.burned {
                burns_to_handle.push((player, i));
                debug!("{player}'s Pokemon {i} is burned");
            }
        }
    }

    // Get all binary vectors representing the possible outcomes.
    // These are the "outcome_ids" for sleep and burn coin flips
    // (e.g. outcome [true, false] might represent waking up one pokemon and not healing another's burn).
    let total_coin_flips = sleeps_to_handle.len() + burns_to_handle.len();
    let outcome_ids = generate_boolean_vectors(total_coin_flips);
    let base_probability = 1.0 / outcome_ids.len() as f64;

    let next_player = (state.current_player + 1) % 2;
    let mut probabilities = Vec::with_capacity(outcome_ids.len());
    let mut outcomes: Mutations = Vec::with_capacity(outcome_ids.len());
    for outcome in outcome_ids {
        let sleeps_to_handle = sleeps_to_handle.clone();
        let paralyzed_to_handle = paralyzed_to_handle.clone();
        let poisons_to_handle = poisons_to_handle.clone();
        let burns_to_handle = burns_to_handle.clone();
        let (start_probs, start_mutations) = start_turn_ability_outcomes(state, next_player);
        for (start_prob, start_mutation) in start_probs.into_iter().zip(start_mutations) {
            let sleeps_to_handle = sleeps_to_handle.clone();
            let paralyzed_to_handle = paralyzed_to_handle.clone();
            let poisons_to_handle = poisons_to_handle.clone();
            let burns_to_handle = burns_to_handle.clone();
            let outcome = outcome.clone();
            probabilities.push(base_probability * start_prob);
            outcomes.push(Box::new(move |rng, state, action| {
                // Important for these to happen before Pokemon Checkup (Zeraora, Suicune, etc)
                on_end_turn(action.actor, state);

                apply_pokemon_checkup(
                    state,
                    sleeps_to_handle.clone(),
                    paralyzed_to_handle.clone(),
                    poisons_to_handle.clone(),
                    burns_to_handle.clone(),
                    outcome.clone(),
                );

                start_mutation(rng, state, action);
            }));
        }
    }
    (probabilities, outcomes)
}

fn start_turn_ability_outcomes(state: &State, player: usize) -> (Probabilities, Mutations) {
    let Some(active) = state.maybe_get_active(player) else {
        return (vec![1.0], vec![noop_mutation()]);
    };
    let Some(ability) = active.card.get_ability() else {
        return (vec![1.0], vec![noop_mutation()]);
    };
    let Some(mechanic) = ability_mechanic_from_effect(&ability.effect) else {
        return (vec![1.0], vec![noop_mutation()]);
    };

    match mechanic {
        AbilityMechanic::StartTurnRandomPokemonToHand { energy_type } => {
            shared_mutations::pokemon_search_outcomes_by_type_for_player(
                player,
                state,
                false,
                *energy_type,
            )
        }
        _ => (vec![1.0], vec![noop_mutation()]),
    }
}

/// Calculate poison damage based on base damage (10) plus +10 for each opponent's Nihilego with More Poison ability
/// Only applies the bonus if the poisoned Pokemon is in the active spot (index 0)
fn get_poison_damage(state: &State, player: usize, in_play_idx: usize) -> u32 {
    use crate::ability_ids::AbilityId;

    let base_damage = 10;

    // Nihilego's More Poison ability only affects the active Pokemon
    if in_play_idx != 0 {
        return base_damage;
    }

    let opponent = (player + 1) % 2;
    let nihilego_count = state
        .enumerate_in_play_pokemon(opponent)
        .filter(|(_, pokemon)| {
            AbilityId::from_pokemon_id(&pokemon.card.get_id()[..])
                .map(|id| id == AbilityId::A3a042NihilegoMorePoison)
                .unwrap_or(false)
        })
        .count();

    let total_damage = base_damage + (nihilego_count as u32 * 10);

    if nihilego_count > 0 {
        debug!(
            "Nihilego's More Poison: {} Nihilego in play, poison damage is {}",
            nihilego_count, total_damage
        );
    }

    total_damage
}

fn apply_pokemon_checkup(
    mutated_state: &mut State,
    sleeps_to_handle: Vec<(usize, usize)>,
    paralyzed_to_handle: Vec<(usize, usize)>,
    poisons_to_handle: Vec<(usize, usize)>,
    burns_to_handle: Vec<(usize, usize)>,
    outcome: Vec<bool>,
) {
    // First half of outcomes are for sleep, second half for burns
    let num_sleeps = sleeps_to_handle.len();

    // Handle sleep coin flips
    for (i, is_awake) in sleeps_to_handle.iter().zip(&outcome[0..num_sleeps]) {
        if *is_awake {
            let (player, in_play_idx) = i;
            let pokemon = mutated_state.in_play_pokemon[*player][*in_play_idx]
                .as_mut()
                .expect("Pokemon should be there...");
            pokemon.asleep = false;
            debug!("{player}'s Pokemon {in_play_idx} woke up");
        }
    }

    // These always happen regardless of outcome_binary_vector
    for (player, in_play_idx) in paralyzed_to_handle {
        let pokemon = mutated_state.in_play_pokemon[player][in_play_idx]
            .as_mut()
            .expect("Pokemon should be there...");
        pokemon.paralyzed = false;
        debug!("{player}'s Pokemon {in_play_idx} is un-paralyzed");
    }

    // Poison always deals 10 damage (+10 for each Nihilego with More Poison ability opponent has in play)
    for (player, in_play_idx) in poisons_to_handle {
        let attacking_ref = (player, in_play_idx); // present it as self-damage
        let poison_damage = get_poison_damage(mutated_state, player, in_play_idx);

        handle_damage(
            mutated_state,
            attacking_ref,
            &[(poison_damage, player, in_play_idx)],
            false,
            None,
        );
    }

    // Burn always deals 20 damage, then coin flip for healing
    for (i, (player, in_play_idx)) in burns_to_handle.iter().enumerate() {
        // Check if pokemon heals from burn (coin flip result)
        let heals_from_burn = outcome[num_sleeps + i];
        if heals_from_burn {
            let pokemon = mutated_state.in_play_pokemon[*player][*in_play_idx]
                .as_mut()
                .expect("Pokemon should be there...");
            pokemon.burned = false;
            debug!("{player}'s Pokemon {in_play_idx} healed from burn");
        }

        // Deal burn damage
        let attacking_ref = (*player, *in_play_idx); // present it as self-damage
        handle_damage(
            mutated_state,
            attacking_ref,
            &[(20, *player, *in_play_idx)],
            false,
            None,
        );
    }

    // Advance turn
    mutated_state.knocked_out_by_opponent_attack_last_turn =
        mutated_state.knocked_out_by_opponent_attack_this_turn;
    mutated_state.knocked_out_by_opponent_attack_this_turn = false;
    mutated_state.advance_turn();
}

fn generate_boolean_vectors(n: usize) -> Vec<Vec<bool>> {
    // The total number of combinations is 2^n
    let total_combinations = 1 << n; // 2^n

    // Generate all combinations
    (0..total_combinations)
        .map(|i| {
            // Convert the number `i` to its binary representation as a vector of booleans
            (0..n).map(|bit| (i & (1 << bit)) != 0).collect()
        })
        .collect()
}

fn checkapply_prevent_first_attack(
    state: &mut State,
    target_player: usize,
    target_pokemon_idx: usize,
    is_from_active_attack: bool,
) -> bool {
    if !is_from_active_attack {
        return false;
    }

    if let Some(target_pokemon) = state.in_play_pokemon[target_player][target_pokemon_idx].as_mut()
    {
        if !target_pokemon.prevent_first_attack_damage_used {
            if let Some(AbilityMechanic::PreventFirstAttack) =
                get_ability_mechanic(&target_pokemon.card)
            {
                debug!("PreventFirstAttackDamageAfterEnteringPlay: Preventing first attack damage");
                target_pokemon.prevent_first_attack_damage_used = true;
                return true;
            }
        }
    }
    false
}

/// This function applies damage (with modifiers and counterattacks) and handles K.O.s
/// and promotions.
pub(crate) fn handle_damage(
    state: &mut State,
    attacking_ref: (usize, usize), // (attacking_player, attacking_pokemon_idx)
    targets: &[(u32, usize, usize)], // damage, target_player, in_play_idx
    is_from_active_attack: bool,
    attack_name: Option<&str>,
) {
    handle_damage_only(
        state,
        attacking_ref,
        targets,
        is_from_active_attack,
        attack_name,
    );
    handle_knockouts(state, attacking_ref, is_from_active_attack);
}

// This function handles Counter-Attacks and Attack Modifiers, but doesn't handle K.O.s or
// queues up promotion decisions. Use carefully, probably just in a few places
pub(crate) fn handle_damage_only(
    state: &mut State,
    attacking_ref: (usize, usize), // (attacking_player, attacking_pokemon_idx)
    targets: &[(u32, usize, usize)], // damage, target_player, in_play_idx
    is_from_active_attack: bool,
    attack_name: Option<&str>,
) {
    let attacking_player = attacking_ref.0;

    // Reduce and sum damage for duplicate targets
    let mut damage_map: HashMap<(usize, usize), u32> = HashMap::new();
    for (damage, player, idx) in targets {
        *damage_map.entry((*player, *idx)).or_insert(0) += damage;
    }
    let targets: Vec<(u32, usize, usize)> = damage_map
        .into_iter()
        .map(|((player, idx), damage)| (damage, player, idx))
        .collect();

    // Modify to apply any multipliers (e.g. Oricorio, Giovanni, etc...)
    let modified_targets = targets
        .iter()
        .map(|target_ref| {
            let modified_damage = modify_damage(
                state,
                attacking_ref,
                *target_ref,
                is_from_active_attack,
                attack_name,
            );
            (modified_damage, target_ref.1, target_ref.2)
        })
        .collect::<Vec<(u32, usize, usize)>>();

    // Handle each target individually
    for (damage, target_player, target_pokemon_idx) in modified_targets {
        let applied = checkapply_prevent_first_attack(
            state,
            target_player,
            target_pokemon_idx,
            is_from_active_attack,
        );
        if applied || damage == 0 {
            continue;
        }

        // Apply damage
        {
            let target_pokemon = state.in_play_pokemon[target_player][target_pokemon_idx]
                .as_mut()
                .expect("Pokemon should be there if taking damage");
            target_pokemon.apply_damage(damage); // Applies without surpassing 0 HP
            debug!(
                "Dealt {} damage to opponent's {} Pokemon. Remaining HP: {}",
                damage,
                target_pokemon_idx,
                target_pokemon.get_remaining_hp()
            );
        }

        // Consider Counter-Attack (only if from Active Attack to Active)
        if !(is_from_active_attack && target_pokemon_idx == 0) {
            continue;
        }

        let target_pokemon = state.in_play_pokemon[target_player][target_pokemon_idx]
            .as_ref()
            .expect("Pokemon should be there if taking damage");
        let counter_damage = {
            if target_pokemon_idx == 0 {
                get_counterattack_damage(target_pokemon)
            } else {
                0
            }
        };
        let should_poison = should_poison_attacker(target_pokemon);

        // Apply counterattack damage and poison
        if counter_damage > 0 || should_poison {
            let attacking_pokemon = state.in_play_pokemon[attacking_player][0]
                .as_mut()
                .expect("Active Pokemon should be there");

            if counter_damage > 0 {
                attacking_pokemon.apply_damage(counter_damage);
                debug!(
                    "Dealt {} counterattack damage to active Pokemon. Remaining HP: {}",
                    counter_damage,
                    attacking_pokemon.get_remaining_hp()
                );
            }

            if should_poison {
                attacking_pokemon.poisoned = true;
                debug!("Poison Barb: Poisoned the attacking Pokemon");
            }
        }
    }
}

pub(crate) fn handle_knockouts(
    state: &mut State,
    attacking_ref: (usize, usize), // (attacking_player, attacking_pokemon_idx)
    is_from_active_attack: bool,
) {
    let knockouts = get_knocked_out(state);

    // Handle knockouts: Discard cards and award points (to potentially short-circuit promotions)
    for (ko_receiver, ko_pokemon_idx) in knockouts.clone() {
        // Call knockout hook (e.g., for Electrical Cord)
        on_knockout(state, ko_receiver, ko_pokemon_idx, is_from_active_attack);

        // Award points
        {
            let ko_pokemon = state.in_play_pokemon[ko_receiver][ko_pokemon_idx]
                .as_ref()
                .expect("Pokemon should be there if knocked out");
            let ko_initiator = (ko_receiver + 1) % 2;
            let points_won = ko_pokemon.card.get_knockout_points();
            state.points[ko_initiator] += points_won;
            debug!(
                "Pokemon {:?} fainted. Player {} won {} points for a total of {}",
                ko_pokemon, ko_initiator, points_won, state.points[ko_initiator]
            );
        }

        state.discard_from_play(ko_receiver, ko_pokemon_idx);
    }

    // Set knocked_out_by_opponent_attack_this_turn flag
    // Check if any of the current player's Pokémon were knocked out by an opponent's active attack
    if is_from_active_attack {
        // Only care about KOs from active attacks
        for (ko_receiver, _) in knockouts.clone() {
            let ko_initiator_of_this_damage = attacking_ref.0; // The player who caused the damage
                                                               // If the receiver is NOT the initiator, it's an opponent KO
            if ko_receiver != ko_initiator_of_this_damage {
                state.knocked_out_by_opponent_attack_this_turn = true;
                break; // Only need to set once
            }
        }
    }

    // If game ends because of knockouts, set winner and return so as to short-circuit promotion logic
    // Note even attacking player can lose by counterattack K.O.
    if state.points[0] >= 3 && state.points[1] >= 3 {
        debug!("Both players have 3 points, it's a tie");
        state.winner = Some(GameOutcome::Tie);
        return;
    } else if state.points[0] >= 3 {
        state.winner = Some(GameOutcome::Win(0));
        return;
    } else if state.points[1] >= 3 {
        state.winner = Some(GameOutcome::Win(1));
        return;
    }

    // Queue up promotion actions if the game is still on after a knockout
    for (ko_receiver, ko_pokemon_idx) in knockouts {
        if ko_pokemon_idx != 0 {
            continue; // Only promote if K.O. was on Active
        }
        // If K.O. was Active, trigger promotion or declare winner
        state.trigger_promotion_or_declare_winner(ko_receiver);
    }
}

fn get_knocked_out(state: &State) -> Vec<(usize, usize)> {
    let mut knockouts: Vec<(usize, usize)> = vec![];
    for (idx, card) in state.enumerate_in_play_pokemon(0) {
        if card.is_knocked_out() {
            knockouts.push((0, idx));
        }
    }
    for (idx, card) in state.enumerate_in_play_pokemon(1) {
        if card.is_knocked_out() {
            knockouts.push((1, idx));
        }
    }
    knockouts
}

// Apply common logic in outcomes
pub(crate) fn wrap_with_common_logic(mutation: Mutation) -> Mutation {
    Box::new(move |rng, state, action| {
        if action.is_stack {
            state.move_generation_stack.pop();
        }
        if let SimpleAction::Play { trainer_card } = &action.action {
            let card = Card::Trainer(trainer_card.clone());
            if trainer_card.trainer_card_type == TrainerType::Stadium {
                // Replace old stadium (discard it if exists)
                if let Some(old_stadium) = state.set_active_stadium(card.clone()) {
                    state.discard_piles[action.actor].push(old_stadium);
                }
                state.remove_card_from_hand(action.actor, &card);
                state.refresh_starting_plains_bonus_all();
                handle_knockouts(state, (action.actor, 0), false);
            } else {
                state.discard_card_from_hand(action.actor, &card);
            }
            if card.is_support() {
                state.has_played_support = true;
            }
        }
        if let SimpleAction::UseAbility { in_play_idx } = &action.action {
            let pokemon = state.in_play_pokemon[action.actor][*in_play_idx]
                .as_mut()
                .expect("Pokemon should be there if using ability");
            pokemon.ability_used = true;
        }

        mutation(rng, state, action); // in the case of attacks, have this be damage + effect.

        if let SimpleAction::Attack(_) = &action.action {
            // We use a flag instead of .move_generation_stack to reduce
            // stack surgery to make sure things happen in order.
            // This ensures move_generation_stack (effects and promotions)
            // has priority over ending the turn.
            state.end_turn_pending = true;
        }
    })
}

fn noop_mutation() -> Mutation {
    Box::new(|_, _, _| {})
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{card_ids::CardId, database::get_card_by_enum, hooks::to_playable_card};

    #[test]
    fn test_poison_damage_no_nihilego() {
        let state = State::default();
        // Poison damage should be 10 with no Nihilego in play
        assert_eq!(get_poison_damage(&state, 0, 0), 10);
    }

    #[test]
    fn test_poison_damage_with_nihilego() {
        let mut state = State::default();

        // Add 2 Nihilego to opponent's field (player 1)
        let nihilego = get_card_by_enum(CardId::A3a042Nihilego);
        state.in_play_pokemon[1][0] = Some(to_playable_card(&nihilego, false));
        state.in_play_pokemon[1][1] = Some(to_playable_card(&nihilego, false));

        // Player 0's active pokemon should take 30 damage (10 base + 10 per Nihilego)
        assert_eq!(get_poison_damage(&state, 0, 0), 30);
    }

    #[test]
    fn test_mimikyu_ex_disguise_prevents_first_attack_only() {
        let mut state = State::default();

        let attacker = get_card_by_enum(CardId::A1001Bulbasaur);
        let mimikyu_ex = get_card_by_enum(CardId::B2073MimikyuEx);

        state.in_play_pokemon[0][0] = Some(to_playable_card(&attacker, false));
        state.in_play_pokemon[1][0] = Some(to_playable_card(&mimikyu_ex, false));

        let starting_hp = state.get_active(1).get_remaining_hp();

        // First attack damage should be prevented
        handle_damage(&mut state, (0, 0), &[(30, 1, 0)], true, None);
        assert_eq!(state.get_active(1).get_remaining_hp(), starting_hp);

        // Second attack should deal damage normally
        handle_damage(&mut state, (0, 0), &[(30, 1, 0)], true, None);
        assert_eq!(state.get_active(1).get_remaining_hp(), starting_hp - 30);
    }
}
