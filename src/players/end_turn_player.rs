use rand::rngs::StdRng;
use std::fmt::Debug;

use crate::{
    actions::{Action, SimpleAction},
    Deck, State,
};

use super::Player;

/// A player that just Ends its turn if it can, or selects first action.
pub struct EndTurnPlayer {
    pub deck: Deck,
}

impl Player for EndTurnPlayer {
    fn decision_fn(&mut self, _: &mut StdRng, _: &State, possible_actions: &[Action]) -> Action {
        let maybe_end_turn = possible_actions
            .iter()
            .find(|action| matches!(action.action, SimpleAction::EndTurn));
        if let Some(end_turn) = maybe_end_turn {
            return end_turn.clone();
        }
        possible_actions
            .first()
            .expect("There should always be at least one playable action")
            .clone()
    }

    fn get_deck(&self) -> Deck {
        self.deck.clone()
    }
}

impl Debug for EndTurnPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EndTurnPlayer")
    }
}
