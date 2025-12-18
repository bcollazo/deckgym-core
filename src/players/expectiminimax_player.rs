use log::{trace, LevelFilter};
use rand::rngs::StdRng;
use std::fmt::Debug;
use std::fmt::Write;
use std::vec;

use crate::actions::{forecast_action, Action};
use crate::{generate_possible_actions, Deck, State};

use super::Player;

// Type alias for value functions
// Takes a state and player index, returns a score
// Using Box<dyn Fn> to allow closures with captured variables
pub type ValueFunction = Box<dyn Fn(&State, usize) -> f64>;

struct DebugStateNode {
    acting_player: usize,
    children: Vec<DebugActionNode>,
    proba: f64,
    value: f64,
}

struct DebugActionNode {
    action: Action,
    children: Vec<DebugStateNode>,
    value: f64,
}

pub struct ExpectiMiniMaxPlayer {
    pub deck: Deck,
    pub max_depth: usize, // max_depth = 1 it should be value function player
    pub write_debug_trees: bool,
    pub value_function: ValueFunction,
}

impl Player for ExpectiMiniMaxPlayer {
    fn decision_fn(
        &mut self,
        rng: &mut StdRng,
        state: &State,
        possible_actions: &[Action],
    ) -> Action {
        let myself = possible_actions[0].actor;

        // Create a tree for debugging purposes
        let mut root = DebugStateNode {
            acting_player: myself,
            children: vec![],
            proba: 1.0,
            value: 0.0,
        };

        // Get value for each possible action
        let original_level = log::max_level();
        log::set_max_level(LevelFilter::Info); // Temporarily silence debug and trace logs
        let mut scores: Vec<f64> = Vec::with_capacity(possible_actions.len());
        for action in possible_actions.iter() {
            let (score, action_node) = expected_value_function(
                rng,
                state,
                action,
                self.max_depth - 1,
                myself,
                &self.value_function,
            );
            scores.push(score);
            root.children.push(action_node);
        }
        log::set_max_level(original_level); // Restore the original logging level

        trace!("Scores: {scores:?}");
        // Select the one with best score
        let (best_idx, best_score) = scores
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(idx, score)| (idx, *score))
            .unwrap();
        root.value = best_score;

        // Output Tree in Dot format for visualization if enabled
        if self.write_debug_trees {
            let folder = "expectiminimax_trees";
            std::fs::create_dir_all(folder).unwrap();

            // Find next available filename to avoid overwriting
            let mut counter = 0;
            let filename = loop {
                let candidate = format!(
                    "{}/expectiminimax_tree_turn{}_p{}_{}.dot",
                    folder, state.turn_count, myself, counter
                );
                if !std::path::Path::new(&candidate).exists() {
                    break candidate;
                }
                counter += 1;
            };
            save_tree_as_dot(&root, state, filename).unwrap();
        }

        // You can now use both best_idx and best_score as needed
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
    value_function: &ValueFunction,
) -> (f64, DebugActionNode) {
    let indent = "\t".repeat(10 - depth.min(10));
    trace!("{indent}E({myself}) depth left: {depth} action: {action:?}");

    let (probabilities, mutations) = forecast_action(state, action);
    let mut outcomes: Vec<State> = vec![];
    for mutation in mutations {
        let mut state_copy = state.clone();
        mutation(rng, &mut state_copy, action);
        outcomes.push(state_copy);
    }

    // Mantain node
    let mut scores = vec![];
    let mut action_node = DebugActionNode {
        action: action.clone(),
        children: vec![],
        value: 0.0,
    };
    for (prob, outcome) in probabilities.iter().zip(outcomes.iter()) {
        let (score, mut state_node) = expectiminimax(rng, outcome, depth, myself, value_function);
        scores.push(score);
        state_node.proba = *prob;
        action_node.children.push(state_node);
    }

    let score = probabilities
        .iter()
        .zip(scores.iter())
        .map(|(p, s)| p * s)
        .sum::<f64>();

    action_node.value = score;
    trace!("{indent}E({myself}) action: {action:?} score: {score}");
    (score, action_node)
}

fn expectiminimax(
    rng: &mut StdRng,
    state: &State,
    depth: usize,
    myself: usize,
    value_function: &ValueFunction,
) -> (f64, DebugStateNode) {
    if state.is_game_over() || depth == 0 || state.current_player != myself {
        let score = value_function(state, myself);
        let state_node = DebugStateNode {
            acting_player: state.current_player,
            children: vec![],
            proba: 1.0,
            value: score,
        };
        return (score, state_node);
    }

    let (actor, actions) = generate_possible_actions(state);
    if actor == myself {
        // We are in maximing mode.
        let mut scores: Vec<f64> = Vec::with_capacity(actions.len());
        let mut children = vec![];
        for action in actions.iter() {
            let (score, action_node) =
                expected_value_function(rng, state, action, depth - 1, myself, value_function);
            scores.push(score);
            children.push(action_node);
        }
        let best_score = scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let state_node = DebugStateNode {
            acting_player: actor,
            children,
            proba: 0.0, // this will get set by parent
            value: best_score,
        };
        (best_score, state_node)
    } else {
        // TODO: If minimizing, we can't just generate_possible_actions since
        //  not everything is public information. So we would have to have
        //  our own version of it that only returns the actions that are
        let mut scores: Vec<f64> = Vec::with_capacity(actions.len());
        let mut children: Vec<DebugActionNode> = Vec::new();
        for action in actions.iter() {
            let (score, action_node) =
                expected_value_function(rng, state, action, depth - 1, myself, value_function);
            scores.push(score);
            children.push(action_node);
        }
        let best_score = scores.iter().cloned().fold(f64::INFINITY, f64::min);
        let state_node = DebugStateNode {
            acting_player: actor,
            children,
            proba: 0.0, // this will get set by parent
            value: best_score,
        };
        (best_score, state_node)
    }
}

impl Debug for ExpectiMiniMaxPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExpectiMiniMaxPlayer")
    }
}

fn save_tree_as_dot(
    root: &DebugStateNode,
    root_state: &State,
    filename: String,
) -> std::io::Result<()> {
    let dot_representation = generate_dot(root, root_state);
    std::fs::write(filename, dot_representation)
}

fn generate_dot(root: &DebugStateNode, root_state: &State) -> String {
    let mut dot = String::new();
    writeln!(dot, "digraph GameTree {{").unwrap();
    writeln!(dot, "    rankdir=TB;").unwrap();
    writeln!(dot, "    node [shape=box];").unwrap();

    // Add info node with root state debug string
    let debug_str = root_state
        .debug_string()
        .replace('"', "'")
        .replace('\n', "\\l")
        + "\\l";
    writeln!(
        dot,
        "    info [label=\"{}\", shape=box, style=filled, fillcolor=lightyellow, align=left];",
        debug_str
    )
    .unwrap();

    let mut state_counter = 0;
    let mut action_counter = 0;

    generate_dot_recursive(
        root,
        &mut dot,
        &mut state_counter,
        &mut action_counter,
        0,
        root.acting_player,
    );

    writeln!(dot, "}}").unwrap();
    dot
}

fn generate_dot_recursive(
    state: &DebugStateNode,
    dot: &mut String,
    state_counter: &mut usize,
    action_counter: &mut usize,
    current_state_id: usize,
    myself: usize,
) {
    // Define the state node with color based on acting player
    let color = if state.acting_player == myself {
        "lightgreen"
    } else {
        "lightcoral"
    };
    writeln!(
        dot,
        "    s{} [label=\"State\\nPlayer: {}\\nProba: {:.3}\\nValue: {:.3}\", style=filled, fillcolor={}];",
        current_state_id,
        state.acting_player,
        state.proba,
        state.value,
        color
    ).unwrap();

    // Process each action child
    for action_node in &state.children {
        *action_counter += 1;
        let action_id = *action_counter;

        // Define the action node (neutral color)
        writeln!(
            dot,
            "    a{} [label=\"P{} {}\\n{:?}\\nValue: {:.3}\", shape=ellipse, style=filled, fillcolor=lightgrey];",
            action_id,
            action_node.action.actor,
            action_node.action.is_stack,
            action_node.action.action,
            action_node.value
        ).unwrap();

        // Edge from state to action
        writeln!(dot, "    s{} -> a{};", current_state_id, action_id).unwrap();

        // Process each state child of this action
        for child_state in &action_node.children {
            *state_counter += 1;
            let child_state_id = *state_counter;

            // Edge from action to child state
            writeln!(dot, "    a{} -> s{};", action_id, child_state_id).unwrap();

            // Recursively process the child state
            generate_dot_recursive(
                child_state,
                dot,
                state_counter,
                action_counter,
                child_state_id,
                myself,
            );
        }
    }
}
