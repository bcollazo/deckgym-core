use log::{info, trace, LevelFilter};
use rand::rngs::StdRng;
use std::fmt::Debug;
use std::vec;

use crate::actions::{forecast_action, Action};
use crate::{generate_possible_actions, Deck, State};

use super::Player;

struct StateNode {
    acting_player: usize,
    state: State,
    children: Vec<ActionNode>,
    proba: f64,
    value: f64,
}

struct ActionNode {
    action: Action,
    children: Vec<StateNode>,
    value: f64,
}

pub struct ExpectiMiniMaxPlayer {
    pub deck: Deck,
    pub max_depth: usize, // max_depth = 1 it should be value function player
}

impl Player for ExpectiMiniMaxPlayer {
    fn decision_fn(
        &mut self,
        rng: &mut StdRng,
        state: &State,
        possible_actions: &[Action],
    ) -> Action {
        let myself = possible_actions[0].actor;

        // create a tree for debugging purposes
        let mut root = StateNode {
            acting_player: myself,
            state: state.clone(),
            children: vec![],
            proba: 1.0,
            value: 0.0,
        };

        // Get value for each possible action
        let original_level = log::max_level();
        log::set_max_level(LevelFilter::Info); // Temporarily silence debug and trace logs
        let mut scores: Vec<f64> = Vec::with_capacity(possible_actions.len());
        for action in possible_actions.iter() {
            let (score, action_node) =
                expected_value_function(rng, state, action, self.max_depth - 1, myself);
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

        // Output Tree in Dot format for visualization. Use a unique filename each time to avoid overwriting
        // all in one folder.
        let folder = "expectiminimax_trees";
        std::fs::create_dir_all(folder).unwrap();
        let filename = format!(
            "{}/expectiminimax_tree_{}.dot",
            folder,
            uuid::Uuid::new_v4()
        );
        save_tree_as_dot(&root, filename).unwrap();

        // You can now use both best_idx and best_score as needed
        possible_actions[best_idx].clone()
    }

    fn get_deck(&self) -> Deck {
        self.deck.clone()
    }
}

fn save_tree_as_dot(root: &StateNode, filename: String) -> std::io::Result<()> {
    let dot_representation = generate_dot(root);
    std::fs::write(filename, dot_representation)
}

use std::fmt::Write;

fn generate_dot(root: &StateNode) -> String {
    let mut dot = String::new();
    writeln!(dot, "digraph GameTree {{").unwrap();
    writeln!(dot, "    rankdir=TB;").unwrap();
    writeln!(dot, "    node [shape=box];").unwrap();

    let mut state_counter = 0;
    let mut action_counter = 0;

    generate_dot_recursive(root, &mut dot, &mut state_counter, &mut action_counter, 0);

    writeln!(dot, "}}").unwrap();
    dot
}

fn generate_dot_recursive(
    state: &StateNode,
    dot: &mut String,
    state_counter: &mut usize,
    action_counter: &mut usize,
    current_state_id: usize,
) {
    // Define the state node
    writeln!(
        dot,
        "    s{} [label=\"{}\\nPlayer: {}\\nProba: {:.3}\\nValue: {:.3}\", style=filled, fillcolor=lightblue];",
        current_state_id,
        // replay " with '
        state.state.debug_string().replace('"', "'"),
        state.acting_player,
        state.proba,
        state.value
    ).unwrap();

    // Process each action child
    for action_node in &state.children {
        *action_counter += 1;
        let action_id = *action_counter;

        // Define the action node
        writeln!(
            dot,
            "    a{} [label=\"{}\\nValue: {:.3}\", shape=ellipse, style=filled, fillcolor=lightgreen];",
            action_id,
            format!("P{} {:?} {}", action_node.action.actor, action_node.action.action, action_node.action.is_stack),
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
            );
        }
    }
}

fn expected_value_function(
    rng: &mut StdRng,
    state: &State,
    action: &Action,
    depth: usize,
    myself: usize,
) -> (f64, ActionNode) {
    let indent = "  ".repeat(depth);
    info!("{indent}E({myself}) depth left: {depth} action: {action:?}");

    let (probabilities, mutations) = forecast_action(state, action);
    let mut outcomes: Vec<State> = vec![];
    for mutation in mutations {
        let mut state = state.clone();
        mutation(rng, &mut state, action);
        outcomes.push(state);
    }

    // Mantain node
    let mut scores = vec![];
    let mut action_node = ActionNode {
        action: action.clone(),
        children: vec![],
        value: 0.0,
    };
    for (i, outcome) in outcomes.iter().enumerate() {
        let (score, mut state_node) = expectiminimax(rng, outcome, depth, myself);
        scores.push(score);
        state_node.proba = if probabilities.is_empty() {
            1.0
        } else {
            probabilities[i]
        };
        action_node.children.push(state_node);
    }

    let score = if probabilities.is_empty() {
        // Deterministic action
        scores[0]
    } else {
        // Stochastic action
        probabilities
            .iter()
            .zip(scores.iter())
            .map(|(p, s)| p * s)
            .sum::<f64>()
    };
    action_node.value = score;
    info!("{indent}E({myself}) action: {action:?} score: {score}");
    (score, action_node)
}

fn expectiminimax(
    rng: &mut StdRng,
    state: &State,
    depth: usize,
    myself: usize,
) -> (f64, StateNode) {
    if state.is_game_over() || depth == 0 {
        let score = value_function(state, myself);
        let state_node = StateNode {
            acting_player: state.current_player,
            state: state.clone(),
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
                expected_value_function(rng, state, action, depth - 1, myself);
            scores.push(score);
            children.push(action_node);
        }
        let best_score = scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let state_node = StateNode {
            acting_player: actor,
            state: state.clone(),
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
        let mut children: Vec<ActionNode> = Vec::new();
        for action in actions.iter() {
            let (score, action_node) =
                expected_value_function(rng, state, action, depth - 1, myself);
            scores.push(score);
            children.push(action_node);
        }
        let best_score = scores.iter().cloned().fold(f64::INFINITY, f64::min);
        let state_node = StateNode {
            acting_player: actor,
            state: state.clone(),
            children,
            proba: 0.0, // this will get set by parent
            value: best_score,
        };
        (best_score, state_node)
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
        + (attached_energy_in_play - enemy_attached_energy_in_play) * 30.0;
    trace!("Value function: {score}");
    score
}

impl Debug for ExpectiMiniMaxPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExpectiMiniMaxPlayer")
    }
}
