use rand::{rngs::StdRng, seq::SliceRandom};
use std::fmt::Debug;

use crate::{actions::Action, Deck, State};

use super::Player;

pub struct RandomPlayer {
    pub deck: Deck,
}

impl Player for RandomPlayer {
    fn decision_fn(&mut self, rng: &mut StdRng, _: &State, possible_actions: &[Action]) -> Action {
        possible_actions
            .choose(rng)
            .expect("There should always be at least one playable action")
            .clone()
    }

    fn get_deck(&self) -> Deck {
        self.deck.clone()
    }
}

impl Debug for RandomPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RandomPlayer")
    }
}
