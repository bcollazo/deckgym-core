use rand::rngs::StdRng;
use std::fmt::Debug;

use crate::{actions::Action, Deck, State};

use super::Player;

/// A player that asks the user to select one available
/// action by prompting the user via STDIN.
pub struct HumanPlayer {
    pub deck: Deck,
}

impl Player for HumanPlayer {
    fn decision_fn(
        &mut self,
        _: &mut StdRng,
        state: &State,
        possible_actions: &Vec<Action>,
    ) -> Action {
        if possible_actions.len() == 1 {
            println!("Only one possible action, selecting it.");
            return possible_actions[0].clone();
        }

        println!("=== {}|{:?}", state.turn_count, state.points);
        println!();
        println!("{}", state.debug_string());
        println!();
        println!("Select an action:");
        for (i, action) in possible_actions.iter().enumerate() {
            println!("{}: {}", i + 1, action.action);
        }
        let mut input = String::new();
        let mut valid = false;
        let mut index = 0;
        while !valid {
            std::io::stdin().read_line(&mut input).unwrap();
            index = input.trim().parse::<usize>().unwrap();
            if index != 0 && index <= possible_actions.len() {
                valid = true;
            } else {
                println!("Invalid input, try again.");
                input.clear();
            }
        }
        possible_actions[index - 1].clone()
    }

    fn get_deck(&self) -> Deck {
        self.deck.clone()
    }
}

impl Debug for HumanPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HumanPlayer")
    }
}
