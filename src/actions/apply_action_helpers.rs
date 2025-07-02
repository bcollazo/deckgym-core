use log::debug;
use rand::rngs::StdRng;

use crate::{
    actions::SimpleAction, hooks::get_counterattack_damage, state::GameOutcome, types::Card, State,
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
                |_, state, action| {
                    apply_common_mutation(state, action);
                    // advance current_player, but only advance "turn" (i.e. stay in 0) when both players done.
                    state.current_player = (state.current_player + 1) % 2;
                    let both_players_initiated = state.in_play_pokemon[0][0].is_some()
                        && state.in_play_pokemon[1][0].is_some();
                    if both_players_initiated {
                        // Actually start game (no energy generation)
                        state.turn_count = 1;
                        state.reset_turn_states();
                        state.queue_draw_action(state.current_player);
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
        }
    }

    // Get all binary vectors representing the possible outcomes.
    // These are the "outcome_ids" (e.g. outcome [true, false] might represent
    // waking up one pokemon and not another).
    let outcome_ids = generate_boolean_vectors(sleeps_to_handle.len());
    let probabilities = vec![1.0 / outcome_ids.len() as f64; outcome_ids.len()];
    let mut outcomes: Mutations = vec![];
    for outcome in outcome_ids {
        let sleeps_to_handle = sleeps_to_handle.clone();
        let paralyzed_to_handle = paralyzed_to_handle.clone();
        let poisons_to_handle = poisons_to_handle.clone();
        outcomes.push(Box::new({
            |_, state, action| {
                apply_common_mutation(state, action);
                apply_pokemon_checkup(
                    state,
                    sleeps_to_handle,
                    paralyzed_to_handle,
                    poisons_to_handle,
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
    outcome: Vec<bool>,
) {
    for (i, is_awake) in sleeps_to_handle.iter().zip(outcome) {
        if is_awake {
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
    for (player, in_play_idx) in poisons_to_handle {
        let opponent = (player + 1) % 2;
        handle_attack_damage(mutated_state, opponent, &vec![(10, in_play_idx)]);
    }
    // Advance turn
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

pub(crate) fn handle_attack_damage(
    state: &mut State,
    attacking_player: usize,
    targets: &Vec<(u32, usize)>, // damage, in_play_idx
) {
    let defending_player = (attacking_player + 1) % 2;
    let mut knockouts: Vec<(usize, usize)> = vec![];
    for (damage, target_pokemon_idx) in targets {
        if *damage == 0 {
            continue;
        }

        // Create a closure for target_pokemon's mutations
        let counter_damage = {
            let target_pokemon = state.in_play_pokemon[defending_player][*target_pokemon_idx]
                .as_mut()
                .expect("Pokemon should be there if taking damage");
            target_pokemon.apply_damage(*damage); // Applies without surpassing 0 HP
            debug!(
                "Dealt {} damage to opponent's {} Pokemon. Remaining HP: {}",
                damage, target_pokemon_idx, target_pokemon.remaining_hp
            );
            if target_pokemon.remaining_hp == 0 {
                knockouts.push((defending_player, *target_pokemon_idx));
            }

            if *target_pokemon_idx == 0 {
                get_counterattack_damage(target_pokemon)
            } else {
                0
            }
        };

        // If pokemon not active, don't even look at counter-attack logic.
        if *target_pokemon_idx == 0 && counter_damage > 0 {
            let attacking_pokemon = state.in_play_pokemon[attacking_player][0]
                .as_mut()
                .expect("Active Pokemon should be there");
            attacking_pokemon.apply_damage(counter_damage);
            debug!(
                "Dealt {} counterattack damage to active Pokemon. Remaining HP: {}",
                counter_damage, attacking_pokemon.remaining_hp
            );
            if attacking_pokemon.remaining_hp == 0 {
                knockouts.push((attacking_player, 0));
            }
        }
    }

    // Handle knockouts: Discard cards and award points (to potentially short-circuit promotions)
    for (ko_receiver, ko_pokemon_idx) in knockouts.clone() {
        let ko_pokemon = state.in_play_pokemon[ko_receiver][ko_pokemon_idx]
            .as_mut()
            .expect("Pokemon should be there if knocked out");

        // Award points
        let ko_initiator = (ko_receiver + 1) % 2;
        let points_won = if ko_pokemon.card.is_ex() { 2 } else { 1 };
        state.points[ko_initiator] += points_won;
        debug!(
            "Pokemon {:?} fainted. Player {} won {} points for a total of {}",
            ko_pokemon, ko_initiator, points_won, state.points[ko_initiator]
        );

        // Move card (and evolution chain) into discard pile
        let mut cards_to_discard = ko_pokemon.cards_behind.clone();
        cards_to_discard.push(ko_pokemon.card.clone());
        debug!("Discarding: {cards_to_discard:?}");
        state.discard_piles[ko_receiver].extend(cards_to_discard);
        state.in_play_pokemon[ko_receiver][ko_pokemon_idx] = None;
    }

    // If game ends because of knockouts, set winner and return so as to short-circuit promotion logic
    if state.points[attacking_player] >= 3 && state.points[defending_player] >= 3 {
        debug!("Both players have 3 points, it's a tie");
        state.winner = Some(GameOutcome::Tie);
        return;
    } else if state.points[attacking_player] >= 3 {
        state.winner = Some(GameOutcome::Win(attacking_player));
        return; // attacking player could lose by attacking into a RockyHelmet e.g.
    } else if state.points[defending_player] >= 3 {
        state.winner = Some(GameOutcome::Win(defending_player));
        return;
    }

    // Queue up promotion actions if the game is still on after a knockout
    for (ko_receiver, ko_pokemon_idx) in knockouts {
        if ko_pokemon_idx != 0 {
            continue; // Only promote if K.O. was on Active
        }

        // If K.O. was Active and ko_receiver hasn't win, check if can select from Bench
        let enumerated_bench_pokemon = state
            .enumerate_bench_pokemon(ko_receiver)
            .collect::<Vec<_>>();
        if enumerated_bench_pokemon.is_empty() {
            // If no bench pokemon, opponent loses
            let ko_initiator = (ko_receiver + 1) % 2;
            state.winner = Some(GameOutcome::Win(ko_initiator));
            debug!("Player {ko_receiver} lost due to no bench pokemon");
        } else {
            let possible_moves = state
                .enumerate_bench_pokemon(ko_receiver)
                .map(|(i, _)| SimpleAction::Activate { in_play_idx: i })
                .collect::<Vec<_>>();
            debug!("Triggering Activate moves: {possible_moves:?} to player {ko_receiver}");
            state
                .move_generation_stack
                .push((ko_receiver, possible_moves));
        }
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
}
