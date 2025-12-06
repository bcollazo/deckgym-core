use std::collections::HashMap;

use log::debug;
use rand::rngs::StdRng;

use crate::{
    actions::SimpleAction,
    hooks::{
        get_counterattack_damage, modify_damage, on_end_turn, on_knockout, should_poison_attacker,
    },
    models::Card,
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
    let in_initial_setup_phase = state.turn_count == 0;
    if in_initial_setup_phase {
        (
            vec![1.0],
            vec![Box::new({
                |_, state, _| {
                    // advance current_player, but only advance "turn" (i.e. stay in 0) when both players done.
                    state.current_player = (state.current_player + 1) % 2;
                    let both_players_initiated = state.in_play_pokemon[0][0].is_some()
                        && state.in_play_pokemon[1][0].is_some();
                    if both_players_initiated {
                        // Actually start game (no energy generation)
                        state.turn_count = 1;
                        state.end_turn_maintenance();
                        state.queue_draw_action(state.current_player, 1);
                    }
                }
            })],
        )
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
    let probabilities = vec![1.0 / outcome_ids.len() as f64; outcome_ids.len()];
    let mut outcomes: Mutations = vec![];
    for outcome in outcome_ids {
        let sleeps_to_handle = sleeps_to_handle.clone();
        let paralyzed_to_handle = paralyzed_to_handle.clone();
        let poisons_to_handle = poisons_to_handle.clone();
        let burns_to_handle = burns_to_handle.clone();
        outcomes.push(Box::new({
            |_, state, action| {
                // Important for these to happen before Pokemon Checkup (Zeraora, Suicune, etc)
                on_end_turn(action.actor, state);

                apply_pokemon_checkup(
                    state,
                    sleeps_to_handle,
                    paralyzed_to_handle,
                    poisons_to_handle,
                    burns_to_handle,
                    outcome,
                );
            }
        }));
    }
    (probabilities, outcomes)
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

    // Poison always deals 10 damage
    for (player, in_play_idx) in poisons_to_handle {
        let attacking_ref = (player, in_play_idx); // present it as self-damage
        handle_damage(
            mutated_state,
            attacking_ref,
            &[(10, player, in_play_idx)],
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

/// NOTE: This function also handles Counter-Attack logic, attack modifiers, and
///  queues up promotion actions if any K.O.s happen.
pub(crate) fn handle_damage(
    state: &mut State,
    attacking_ref: (usize, usize), // (attacking_player, attacking_pokemon_idx)
    targets: &[(u32, usize, usize)], // damage, target_player, in_play_idx
    is_from_active_attack: bool,
    attack_name: Option<&str>,
) {
    let attacking_player = attacking_ref.0;
    let mut knockouts: Vec<(usize, usize)> = vec![];

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
        if damage == 0 {
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
                damage, target_pokemon_idx, target_pokemon.remaining_hp
            );
            if target_pokemon.remaining_hp == 0 {
                knockouts.push((target_player, target_pokemon_idx));
            }
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
                    counter_damage, attacking_pokemon.remaining_hp
                );
                if attacking_pokemon.remaining_hp == 0 {
                    knockouts.push((attacking_player, 0));
                }
            }

            if should_poison {
                attacking_pokemon.poisoned = true;
                debug!("Poison Barb: Poisoned the attacking Pokemon");
            }
        }
    }

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

// Apply common mutations for all outcomes
// TODO: Is there a way outcome implementations don't have to remember to call this?
pub(crate) fn apply_common_mutation(state: &mut State, action: &Action) {
    if action.is_stack {
        state.move_generation_stack.pop();
    }
    if let SimpleAction::Play { trainer_card } = &action.action {
        let card = Card::Trainer(trainer_card.clone());
        state.discard_card_from_hand(action.actor, &card);
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
    if let SimpleAction::Attack(_) = &action.action {
        state
            .move_generation_stack
            .push((action.actor, vec![SimpleAction::EndTurn]));
    }
}
