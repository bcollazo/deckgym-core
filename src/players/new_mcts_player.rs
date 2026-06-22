use rand::{rngs::StdRng, Rng};
use std::fmt::Debug;

use super::{Player, RandomPlayer};
use crate::{
    actions::{apply_action, Action},
    state::GameOutcome,
    Deck, Game, State,
};

const EXPLORATION_CONSTANT: f64 = std::f64::consts::SQRT_2;

pub struct NewMctsPlayer {
    pub deck: Deck,
    pub iterations: u64,
}

impl NewMctsPlayer {
    pub fn new(deck: Deck, iterations: u64) -> Self {
        Self { deck, iterations }
    }
}

impl Player for NewMctsPlayer {
    fn decision_fn(
        &mut self,
        rng: &mut StdRng,
        state: &State,
        possible_actions: &[Action],
    ) -> Action {
        if possible_actions.len() == 1 {
            return possible_actions[0].clone();
        }

        let investigator = possible_actions[0].actor;
        let root = MctsNode::new(state.clone(), possible_actions.to_vec(), None);
        let mut arena: Vec<MctsNode> = vec![root];

        for _ in 0..self.iterations {
            let path = select(&arena, investigator);
            let leaf_idx = *path.last().unwrap();

            let simulate_from = if arena[leaf_idx].is_terminal() {
                leaf_idx
            } else {
                expand(&mut arena, leaf_idx, rng)
            };

            let reward = simulate(&arena, simulate_from, rng, investigator);
            backpropagate(&mut arena, simulate_from, reward);
        }

        best_action(&arena)
    }

    fn get_deck(&self) -> Deck {
        self.deck.clone()
    }
}

impl Debug for NewMctsPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NewMctsPlayer with {} iterations", self.iterations)
    }
}

struct MctsNode {
    state: State,
    // All legal actions from this state; children[i] was produced by actions[i]
    actions: Vec<Action>,
    num_expanded: usize,
    // Indices into the arena Vec
    children: Vec<usize>,
    parent: Option<usize>,
    visits: usize,
    reward: f64, // Cumulative reward from investigator's perspective
}

impl MctsNode {
    fn new(state: State, actions: Vec<Action>, parent: Option<usize>) -> Self {
        Self {
            state,
            actions,
            num_expanded: 0,
            children: Vec::new(),
            parent,
            visits: 0,
            reward: 0.0,
        }
    }

    fn is_terminal(&self) -> bool {
        self.state.winner.is_some()
    }

    fn is_fully_expanded(&self) -> bool {
        self.num_expanded >= self.actions.len()
    }
}

/// Walk from root to a node that still has unexplored actions (or is terminal).
/// Returns the path as a Vec of arena indices from root to the chosen leaf.
fn select(arena: &[MctsNode], investigator: usize) -> Vec<usize> {
    let mut path = vec![0usize];
    loop {
        let idx = *path.last().unwrap();
        let node = &arena[idx];
        if node.is_terminal() || !node.is_fully_expanded() {
            return path;
        }
        let next = best_ucb_child(arena, idx, investigator);
        path.push(next);
    }
}

/// Among fully-expanded children of `parent_idx`, pick the one with the best UCB1 score.
fn best_ucb_child(arena: &[MctsNode], parent_idx: usize, investigator: usize) -> usize {
    let parent = &arena[parent_idx];
    let parent_visits_ln = (parent.visits as f64).ln();
    // Flip exploitation sign based on whose turn it is
    let sign: f64 = if parent.state.current_player == investigator {
        1.0
    } else {
        -1.0
    };

    *parent
        .children
        .iter()
        .max_by(|&&a, &&b| {
            let score_a = ucb_score(&arena[a], parent_visits_ln, sign);
            let score_b = ucb_score(&arena[b], parent_visits_ln, sign);
            score_a.partial_cmp(&score_b).unwrap()
        })
        .unwrap()
}

fn ucb_score(node: &MctsNode, parent_visits_ln: f64, sign: f64) -> f64 {
    if node.visits == 0 {
        return f64::INFINITY;
    }
    let exploitation = sign * node.reward / node.visits as f64;
    let exploration = EXPLORATION_CONSTANT * (parent_visits_ln / node.visits as f64).sqrt();
    exploitation + exploration
}

/// Expand one unexplored action from the node at `node_idx`.
/// Adds the new child to the arena and returns its index.
fn expand(arena: &mut Vec<MctsNode>, node_idx: usize, rng: &mut StdRng) -> usize {
    let action = arena[node_idx].actions[arena[node_idx].num_expanded].clone();
    arena[node_idx].num_expanded += 1;

    let mut new_state = arena[node_idx].state.clone();
    apply_action(rng, &mut new_state, &action);
    let (_, new_actions) = new_state.generate_possible_actions();

    let child_idx = arena.len();
    arena.push(MctsNode::new(new_state, new_actions, Some(node_idx)));
    arena[node_idx].children.push(child_idx);

    child_idx
}

/// Run a random game from the node's state; return +1 (investigator wins), -1, or 0.
fn simulate(arena: &[MctsNode], node_idx: usize, rng: &mut StdRng, investigator: usize) -> f64 {
    let random_players: Vec<Box<dyn Player>> = vec![
        Box::new(RandomPlayer {
            deck: Deck::default(),
        }),
        Box::new(RandomPlayer {
            deck: Deck::default(),
        }),
    ];
    let seed: u64 = rng.gen();
    let mut game = Game::from_state(arena[node_idx].state.clone(), random_players, seed);
    match game.play() {
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

/// Walk up the parent chain, updating visits and reward at every node.
fn backpropagate(arena: &mut Vec<MctsNode>, start_idx: usize, reward: f64) {
    let mut current = Some(start_idx);
    while let Some(idx) = current {
        arena[idx].visits += 1;
        arena[idx].reward += reward;
        current = arena[idx].parent;
    }
}

/// Return the action from root that leads to the most-visited child.
fn best_action(arena: &[MctsNode]) -> Action {
    let root = &arena[0];
    let best_child_pos = root
        .children
        .iter()
        .enumerate()
        .max_by_key(|(_, &child_idx)| arena[child_idx].visits)
        .map(|(pos, _)| pos)
        .expect("Root must have at least one child after iterations");
    root.actions[best_child_pos].clone()
}
