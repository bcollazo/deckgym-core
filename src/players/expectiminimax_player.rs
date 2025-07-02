use log::{trace, LevelFilter};
use rand::rngs::StdRng;
use std::fmt::Debug;

use crate::actions::{forecast_action, Action};
use crate::{generate_possible_actions, Deck, State};

use super::Player;

pub struct ExpectiMiniMaxPlayer {
    pub deck: Deck,
    pub max_depth: usize, // max_depth = 1 it should be value function player
}

impl Player for ExpectiMiniMaxPlayer {
    fn decision_fn(
        &mut self,
        rng: &mut StdRng,
        state: &State,
        possible_actions: Vec<Action>,
    ) -> Action {
        let myself = possible_actions[0].actor;

        // Get value for each possible action
        let original_level = log::max_level();
        log::set_max_level(LevelFilter::Error); // Temporarily silence debug and trace logs
        let scores: Vec<f64> = possible_actions
            .iter()
            .map(|action| expected_value_function(rng, state, action, self.max_depth - 1, myself))
            .collect();
        log::set_max_level(original_level); // Restore the original logging level

        trace!("Scores: {scores:?}");
        // Select the one with best score
        let best_idx = scores
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap()
            .0;
        possible_actions[best_idx].clone()
    }

    fn get_deck(&self) -> Deck {
        self.deck.clone()
    }
}

fn expected_value_function(
    rng: &mut StdRng,
    state: &State,
    action: &Action,
    depth: usize,
    myself: usize,
) -> f64 {
    trace!("E({myself}) depth left: {depth} action: {action:?}");
    let (probabilities, mutations) = forecast_action(state, action);
    let mut outcomes: Vec<State> = vec![];
    for mutation in mutations {
        let mut state = state.clone();
        mutation(rng, &mut state, action);
        outcomes.push(state);
    }
    let score = outcomes
        .iter()
        .zip(probabilities.iter())
        .map(|(outcome, prob)| expectiminimax(rng, outcome, depth, myself) * prob)
        .sum();
    trace!("E({myself}) action: {action:?} score: {score}");
    score
}

fn expectiminimax(rng: &mut StdRng, state: &State, depth: usize, myself: usize) -> f64 {
    if state.is_game_over() || depth == 0 {
        return value_function(state, myself);
    }

    let (actor, actions) = generate_possible_actions(state);
    if actor == myself {
        // We are in maximing mode.
        let scores: Vec<f64> = actions
            .iter()
            .map(|action| expected_value_function(rng, state, action, depth - 1, myself))
            .collect();
        scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    } else {
        // TODO: If minimizing, we can't just generate_possible_actions since
        //  not everything is public information. So we would have to have
        //  our own version of it that only returns the actions that are
        let scores: Vec<f64> = actions
            .iter()
            .map(|action| expected_value_function(rng, state, action, depth - 1, myself))
            .collect();
        scores.iter().cloned().fold(f64::INFINITY, f64::min)
    }
}

fn value_function(state: &State, myself: usize) -> f64 {
    // TODO: Add more features
    // Give priorities to attached energies?
    let opponent = (myself + 1) % 2;

    // Points
    let points = state.points[myself] as f64;
    let opponent_points = state.points[opponent] as f64;

    // Attached energy
    let attached_energy_in_play = state
        .enumerate_in_play_pokemon(myself)
        .map(|(_, card)| card.attached_energy.len() as f64)
        .sum::<f64>();
    let enemy_attached_energy_in_play = state
        .enumerate_in_play_pokemon(opponent)
        .map(|(_, card)| card.attached_energy.len() as f64)
        .sum::<f64>();

    // Total health of Pokémon on the board
    let total_health_in_play = state
        .enumerate_in_play_pokemon(myself)
        .map(|(_, card)| card.total_hp as f64)
        .sum::<f64>();
    let enemy_total_health_in_play = state
        .enumerate_in_play_pokemon(opponent)
        .map(|(_, card)| card.total_hp as f64)
        .sum::<f64>();

    // Remaining health of Pokémon on the board
    let remaining_health_in_play = state
        .enumerate_in_play_pokemon(myself)
        .map(|(_, card)| card.remaining_hp as f64)
        .sum::<f64>();
    let enemy_remaining_total_health_in_play = state
        .enumerate_in_play_pokemon(opponent)
        .map(|(_, card)| card.remaining_hp as f64)
        .sum::<f64>();

    // Weighted value function
    trace!(
        "Value function: points: {points}, opponent_points: {opponent_points}, total_health_in_play: {total_health_in_play}, enemy_total_health_in_play: {enemy_total_health_in_play}, remaining_health_in_play: {remaining_health_in_play}, enemy_remaining_total_health_in_play: {enemy_remaining_total_health_in_play}, attached_energy_in_play: {attached_energy_in_play}, enemy_attached_energy_in_play: {enemy_attached_energy_in_play}"
    );
    let score = (points - opponent_points) * 1000.0
        + (total_health_in_play - enemy_total_health_in_play)
        + (remaining_health_in_play - enemy_remaining_total_health_in_play)
        + (attached_energy_in_play - enemy_attached_energy_in_play) * 50.0;
    trace!("Value function: {score}");
    score
}

impl Debug for ExpectiMiniMaxPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExpectiMiniMaxPlayer")
    }
}
