use rand::rngs::StdRng;
use std::fmt::Debug;

use crate::{
    actions::{Action, SimpleAction},
    Deck, State,
};

use super::Player;

/// A player that always tries to Attach energy to active Pokemon
///   if it can. If it can't, it will attack with the active Pokemon.
/// Else it will just do the first possible action.
pub struct AttachAttackPlayer {
    pub deck: Deck,
}

impl Player for AttachAttackPlayer {
    fn decision_fn(&mut self, _: &mut StdRng, _: &State, possible_actions: &[Action]) -> Action {
        let maybe_attach = possible_actions
            .iter()
            .find(|action| matches!(action.action, SimpleAction::Attach { .. }));
        if let Some(attach) = maybe_attach {
            return attach.clone();
        }
        let maybe_attack = possible_actions
            .iter()
            .find(|action| matches!(action.action, SimpleAction::Attack(_)));
        if let Some(attack) = maybe_attack {
            return attack.clone();
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

impl Debug for AttachAttackPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AttachAttackPlayer")
    }
}
