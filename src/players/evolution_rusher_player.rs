use rand::rngs::StdRng;
use std::fmt::Debug;

use crate::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    Deck, State,
};

use super::Player;

/// A player that first tries to accelerate deck as much as possible by:
///   1. Using Poke Ball if possible
///   2. Using Shiinotic Ability if possible
///   3. Using Prof Research if possible
///
/// Then it will try to Rush Evolution by:
///   1. Using Rare Candy if available
///   2. Evolve Pokemon if possible
///
/// After those priorities, it will try to attach energy and attack if possible.
pub struct EvolutionRusherPlayer {
    pub deck: Deck,
}

impl Player for EvolutionRusherPlayer {
    fn decision_fn(
        &mut self,
        _: &mut StdRng,
        state: &State,
        possible_actions: &Vec<Action>,
    ) -> Action {
        // Draw priorities
        let pokeball_trainer = get_card_by_enum(CardId::PA005PokeBall).as_trainer();
        let maybe_poke_ball = possible_actions
            .iter()
            .find(|action| matches!(&action.action, SimpleAction::Play { trainer_card } if trainer_card == &pokeball_trainer));
        if let Some(poke_ball) = maybe_poke_ball {
            return poke_ball.clone();
        }
        let shiinotic = get_card_by_enum(CardId::A3a027Shiinotic);
        let maybe_shiinotic = possible_actions.iter().find(|action| {
            matches!(action.action, SimpleAction::UseAbility(idx) if {
                let in_play_idx = action.actor;
                let pokemon = &state.in_play_pokemon[in_play_idx][idx];
                if let Some(pokemon) = pokemon {
                    pokemon.card == shiinotic
                } else {
                    false
                }
            })
        });
        if let Some(shiinotic) = maybe_shiinotic {
            return shiinotic.clone();
        }
        let profoak = get_card_by_enum(CardId::PA007ProfessorsResearch).as_trainer();
        let maybe_profoak = possible_actions.iter().find(|action| {
            matches!(&action.action, SimpleAction::Play { trainer_card } if trainer_card == &profoak)
        });
        if let Some(profoak) = maybe_profoak {
            return profoak.clone();
        }

        // Evolution priorities
        let rare_candy = get_card_by_enum(CardId::A3144RareCandy).as_trainer();
        let maybe_rare_candy = possible_actions.iter().find(|action| {
            matches!(&action.action, SimpleAction::Play { trainer_card } if trainer_card == &rare_candy)
        });
        if let Some(rare_candy) = maybe_rare_candy {
            return rare_candy.clone();
        }
        let maybe_evolve = possible_actions
            .iter()
            .find(|action| matches!(action.action, SimpleAction::Evolve(_, _)));
        if let Some(evolve) = maybe_evolve {
            return evolve.clone();
        }

        // Attach randomly and attack if possible
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

impl Debug for EvolutionRusherPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EvolutionRusherPlayer")
    }
}
