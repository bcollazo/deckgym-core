mod attach_attack_player;
mod end_turn_player;
mod evolution_rusher_player;
mod expectiminimax_player;
mod human_player;
mod mcts_player;
mod random_player;
mod value_function_player;
mod weighted_random_player;

pub use attach_attack_player::AttachAttackPlayer;
use clap::ValueEnum;
pub use end_turn_player::EndTurnPlayer;
pub use evolution_rusher_player::EvolutionRusherPlayer;
pub use expectiminimax_player::ExpectiMiniMaxPlayer;
pub use human_player::HumanPlayer;
pub use mcts_player::MctsPlayer;
pub use random_player::RandomPlayer;
pub use value_function_player::ValueFunctionPlayer;
pub use weighted_random_player::WeightedRandomPlayer;

use crate::{actions::Action, Deck, State};
use rand::rngs::StdRng;
use std::fmt::Debug;

pub trait Player: Debug {
    fn get_deck(&self) -> Deck;
    fn decision_fn(
        &mut self,
        rng: &mut StdRng,
        state: &State,
        possible_actions: &[Action],
    ) -> Action;
}

/// Enum for allowed player strategies
#[derive(Debug, ValueEnum, Clone, PartialEq)]
pub enum PlayerCode {
    AA,
    ET,
    R,
    H,
    W,
    M,
    V,
    E,
    ER, // Evolution Rusher
}
/// Custom parser function enforcing case-insensitivity
pub fn parse_player_code(s: &str) -> Result<PlayerCode, String> {
    match s.to_ascii_lowercase().as_str() {
        "aa" => Ok(PlayerCode::AA),
        "et" => Ok(PlayerCode::ET),
        "r" => Ok(PlayerCode::R),
        "h" => Ok(PlayerCode::H),
        "w" => Ok(PlayerCode::W),
        "m" => Ok(PlayerCode::M),
        "v" => Ok(PlayerCode::V),
        "e" => Ok(PlayerCode::E),
        "er" => Ok(PlayerCode::ER),
        _ => Err(format!("Invalid player code: {s}")),
    }
}

pub fn parse_player_code_generic(s: String) -> Result<PlayerCode, String> {
    parse_player_code(s.as_ref())
}

pub fn fill_code_array(maybe_players: Option<Vec<PlayerCode>>) -> Vec<PlayerCode> {
    match maybe_players {
        Some(mut player_codes) => {
            if player_codes.is_empty() || player_codes.len() > 2 {
                panic!("Invalid number of players");
            } else if player_codes.len() == 1 {
                player_codes.push(PlayerCode::R);
            }
            player_codes
        }
        None => vec![PlayerCode::R, PlayerCode::R],
    }
}

pub fn create_players(
    deck_a: Deck,
    deck_b: Deck,
    players: Vec<PlayerCode>,
) -> Vec<Box<dyn Player>> {
    let player_a: Box<dyn Player> = get_player(deck_a.clone(), &players[0]);
    let player_b: Box<dyn Player> = get_player(deck_b.clone(), &players[1]);
    vec![player_a, player_b]
}

fn get_player(deck: Deck, player: &PlayerCode) -> Box<dyn Player> {
    match player {
        PlayerCode::AA => Box::new(AttachAttackPlayer { deck }),
        PlayerCode::ET => Box::new(EndTurnPlayer { deck }),
        PlayerCode::R => Box::new(RandomPlayer { deck }),
        PlayerCode::H => Box::new(HumanPlayer { deck }),
        PlayerCode::W => Box::new(WeightedRandomPlayer { deck }),
        PlayerCode::M => Box::new(MctsPlayer::new(deck, 100)),
        PlayerCode::V => Box::new(ValueFunctionPlayer { deck }),
        PlayerCode::E => Box::new(ExpectiMiniMaxPlayer {
            deck,
            max_depth: 3,
            write_debug_trees: false,
        }),
        PlayerCode::ER => Box::new(EvolutionRusherPlayer { deck }),
    }
}
