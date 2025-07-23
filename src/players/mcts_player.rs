use log::debug;
use rand::{rngs::StdRng, seq::SliceRandom, Rng};
use std::{collections::HashMap, fmt::Debug};

use super::{Player, RandomPlayer};
use crate::{
    actions::{apply_action, Action},
    generate_possible_actions,
    state::GameOutcome,
    Deck, Game, State,
};

pub struct MctsPlayer {
    pub deck: Deck,
    pub iterations: u64, // Number of iterations for MCTS
    node_lookup: HashMap<State, MctsNode>,
}
impl MctsPlayer {
    pub fn new(deck: Deck, iterations: u64) -> Self {
        Self {
            deck,
            iterations,
            node_lookup: HashMap::new(),
        }
    }
}

impl Player for MctsPlayer {
    /// Perform MCTS search and return the best action
    fn decision_fn(
        &mut self,
        rng: &mut StdRng,
        state: &State,
        possible_actions: &Vec<Action>,
    ) -> Action {
        // Step 1: Initialize the root node of the search tree
        let investigator = possible_actions[0].actor; // myself
        let mut root = self
            .node_lookup
            .entry(state.clone())
            .or_insert_with(|| MctsNode::new(state.clone(), possible_actions.clone()))
            .clone();

        // Step 2: Perform iterations of MCTS
        for _ in 0..self.iterations {
            // Selection: Traverse the tree to a leaf node
            let leaf = root.select(rng);

            // Expansion: Expand the leaf node if it is not terminal
            if !leaf.is_terminal() {
                leaf.expand(rng, &mut self.node_lookup);
            }

            // Simulation: Simulate a random playout from the expanded node
            let reward = leaf.simulate(rng, investigator);

            // Backpropagation: Update the tree with the simulation result
            leaf.backpropagate(reward);
        }

        // Step 3: Choose the best action from the root node
        root.best_action()
    }

    fn get_deck(&self) -> Deck {
        self.deck.clone()
    }
}

impl Debug for MctsPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MctsPlayer with {} iterations", self.iterations)
    }
}

// Struct to represent a node in the MCTS search tree
#[derive(Clone)]
struct MctsNode {
    state: State,
    actions: Vec<Action>,

    children: Vec<MctsNode>,
    visits: usize,
    reward: f64,
}

impl MctsNode {
    fn new(state: State, actions: Vec<Action>) -> Self {
        Self {
            state,
            actions,
            children: Vec::new(),
            visits: 0,
            reward: 0.0,
        }
    }

    fn is_terminal(&self) -> bool {
        self.state.winner.is_some()
    }

    fn expand(&mut self, rng: &mut StdRng, node_lookup: &mut HashMap<State, MctsNode>) {
        for action in &self.actions {
            let mut new_state = self.state.clone();
            apply_action(rng, &mut new_state, action);

            let node = node_lookup.get(&new_state).cloned();
            if node.is_none() {
                debug!("Missed a node in the lookup table");
            } else {
                debug!("Found a node in the lookup table");
            }

            // No need to have actor as part of possible_actions, since when
            // expanding we use the .apply_action, and the first line there
            // will take the actor from the action itself. This is different
            // than how .play_tick does it, because there we need the actor
            // to choose who plays.
            let (_, new_actions) = generate_possible_actions(&new_state);
            self.children.push(MctsNode::new(new_state, new_actions));
        }
    }

    fn select(&mut self, rng: &mut StdRng) -> &mut Self {
        if self.children.is_empty() {
            self
        } else {
            self.children.choose_mut(rng).unwrap()
        }
    }

    // Simulate a random playout from the current state, and return 1 or -1 or 0
    fn simulate(&self, rng: &mut StdRng, investigator: usize) -> f64 {
        let random_players: Vec<Box<dyn Player>> = vec![
            Box::new(RandomPlayer {
                deck: Deck::default(),
            }),
            Box::new(RandomPlayer {
                deck: Deck::default(),
            }),
        ];
        let seed: u64 = rng.gen();

        // Since we emplace the state, we can keep using our "seating position" as investigator
        let mut game_copy = Game::from_state(self.state.clone(), random_players, seed);
        let outcome = game_copy.play();

        // If winner is my ID, return 1.0, if winner is opponent ID, return -1.0, else return 0.0
        match outcome {
            Some(GameOutcome::Win(winner)) => {
                if winner == investigator {
                    1.0
                } else {
                    -1.0
                }
            }
            Some(GameOutcome::Tie) | None => 0.0,
        }
    }

    fn backpropagate(&mut self, reward: f64) {
        self.visits += 1;
        self.reward += reward;
    }

    fn best_action(&self) -> Action {
        let (best_index, _) = self
            .children
            .iter()
            .enumerate()
            .max_by(|(_, child_a), (_, child_b)| {
                child_a.reward.partial_cmp(&child_b.reward).unwrap()
            })
            .expect("There should be at least one child node");
        self.actions[best_index].clone()
    }
}
