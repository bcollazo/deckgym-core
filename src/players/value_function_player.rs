use rand::rngs::StdRng;
use std::fmt::Debug;

use crate::actions::{forecast_action, Action};
use crate::{Deck, State};

use super::Player;

pub struct ValueFunctionPlayer {
    pub deck: Deck,
}

impl Player for ValueFunctionPlayer {
    fn decision_fn(
        &mut self,
        rng: &mut StdRng,
        state: &State,
        possible_actions: &[Action],
    ) -> Action {
        // Get value for the possible actions
        let myself = possible_actions[0].actor;
        let scores: Vec<f64> = possible_actions
            .iter()
            .map(|action| expected_value_function(rng, state, action, myself))
            .collect();

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

fn expected_value_function(rng: &mut StdRng, state: &State, action: &Action, myself: usize) -> f64 {
    let (probabilities, mutations) = forecast_action(state, action);
    let mut outcomes: Vec<State> = vec![];
    for mutation in mutations {
        let mut state = state.clone();
        mutation(rng, &mut state, action);
        outcomes.push(state);
    }
    outcomes
        .iter()
        .zip(probabilities.iter())
        .map(|(outcome, prob)| value_function(outcome, myself) * prob)
        .sum()
}

fn value_function(state: &State, myself: usize) -> f64 {
    // TODO: Add more features. Other ideas:
    // Attached energy on enemies in play?
    // Can we give priorities to attached energies?
    // Health on the Active spot?
    // Closeness to getting a point(?) Num Knockouts?
    let attached_energy_in_play = state
        .enumerate_in_play_pokemon(myself)
        .map(|(_, card)| card.attached_energy.len() as f64)
        .sum::<f64>();
    let points = state.points[myself] as f64;

    points * 100.0 + attached_energy_in_play
}

impl Debug for ValueFunctionPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ValueFunctionPlayer")
    }
}
