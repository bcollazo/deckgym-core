mod attach_attack_player;
mod end_turn_player;
mod evolution_rusher_player;
mod expectiminimax_player;
mod external_player;
mod human_player;
mod mcts_player;
mod new_mcts_player;
mod random_player;
mod value_function_player;
pub mod value_functions;
mod weighted_random_player;

pub use attach_attack_player::AttachAttackPlayer;
pub use end_turn_player::EndTurnPlayer;
pub use evolution_rusher_player::EvolutionRusherPlayer;
pub use expectiminimax_player::{ExpectiMiniMaxPlayer, ValueFunction};
pub use external_player::ExternalPlayer;
pub use human_player::HumanPlayer;
pub use mcts_player::MctsPlayer;
pub use new_mcts_player::NewMctsPlayer;
pub use random_player::RandomPlayer;
pub use value_function_player::ValueFunctionPlayer;
pub use value_functions::*;
pub use weighted_random_player::WeightedRandomPlayer;

use crate::{actions::Action, Deck, State};
use rand::rngs::StdRng;
use std::fmt::Debug;
use std::time::Duration;

pub trait Player: Debug {
    fn get_deck(&self) -> Deck;
    fn decision_fn(
        &mut self,
        rng: &mut StdRng,
        state: &State,
        possible_actions: &[Action],
    ) -> Action;

    /// An optional short note the player wants attached to its last decision in the replay (e.g.
    /// an external bot's own reasoning, sent back over the bot protocol — see
    /// `docs/bot-protocol.md`). Read once per decision, right after `decision_fn` returns. Default:
    /// no note, which is right for every built-in engine player.
    fn last_note(&self) -> Option<String> {
        None
    }
}

/// Enum for allowed player strategies
#[derive(Debug, Clone, PartialEq)]
pub enum PlayerCode {
    AA,
    ET,
    R,
    H,
    W,
    M,
    MN, // New (arena-based UCB1) MCTS
    V,
    E {
        max_depth: usize,
    },
    ER, // Evolution Rusher
    /// An external bot, spoken to over stdin/stdout (see `docs/bot-protocol.md`). The command to
    /// run comes from `--bot-a`/`--bot-b`, not from the code itself — `create_players` panics if
    /// asked to build an `X` player; use `create_players_with_bots`.
    X,
}
/// Custom parser function enforcing case-insensitivity
pub fn parse_player_code(s: &str) -> Result<PlayerCode, String> {
    let lower = s.to_ascii_lowercase();

    // Check if it starts with 'e' followed by digits (e.g., e2, e4)
    if lower.starts_with('e') && lower.len() > 1 {
        let rest = &lower[1..];
        if let Ok(max_depth) = rest.parse::<usize>() {
            return Ok(PlayerCode::E { max_depth });
        }
        // If it starts with 'e' but not followed by valid number, check if it's 'er'
        if lower == "er" {
            return Ok(PlayerCode::ER);
        }
        return Err(format!("Invalid player code: {s}. Use 'e<number>' for ExpectiMiniMax with depth, e.g., 'e2', 'e5'"));
    }

    match lower.as_str() {
        "aa" => Ok(PlayerCode::AA),
        "et" => Ok(PlayerCode::ET),
        "r" => Ok(PlayerCode::R),
        "h" => Ok(PlayerCode::H),
        "w" => Ok(PlayerCode::W),
        "m" => Ok(PlayerCode::M),
        "mn" => Ok(PlayerCode::MN),
        "v" => Ok(PlayerCode::V),
        "e" => Ok(PlayerCode::E { max_depth: 3 }), // Default depth
        "er" => Ok(PlayerCode::ER),
        "x" => Ok(PlayerCode::X),
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
        PlayerCode::MN => Box::new(NewMctsPlayer::new(deck, 100)),
        PlayerCode::V => Box::new(ValueFunctionPlayer { deck }),
        PlayerCode::E { max_depth } => Box::new(ExpectiMiniMaxPlayer {
            deck,
            max_depth: *max_depth,
            write_debug_trees: false,
            value_function: Box::new(value_functions::baseline_value_function),
        }),
        PlayerCode::ER => Box::new(EvolutionRusherPlayer { deck }),
        PlayerCode::X => panic!(
            "PlayerCode::X (external bot) has no command to run; use create_players_with_bots \
             (the CLI's --bot-a/--bot-b flags) instead of create_players"
        ),
    }
}

/// Like `create_players`, but resolves `PlayerCode::X` into an `ExternalPlayer` running
/// `bot_commands[i]` (required for that slot) instead of panicking. Every other code behaves
/// exactly as `create_players`/`get_player`. `bot_timeout` is the per-decision timeout passed to
/// each `ExternalPlayer` (see `docs/bot-protocol.md`).
pub fn create_players_with_bots(
    deck_a: Deck,
    deck_b: Deck,
    players: Vec<PlayerCode>,
    bot_commands: [Option<String>; 2],
    bot_timeout: Duration,
) -> Vec<Box<dyn Player>> {
    let decks = [deck_a, deck_b];
    let mut result: Vec<Box<dyn Player>> = Vec::with_capacity(2);
    for (i, code) in players.into_iter().enumerate() {
        let deck = decks[i].clone();
        let player: Box<dyn Player> = match code {
            PlayerCode::X => {
                let command = bot_commands[i].clone().unwrap_or_else(|| {
                    panic!(
                        "Player {} is code 'x' (external bot) but no command was given; pass \
                         --bot-{} \"<command>\"",
                        i,
                        if i == 0 { "a" } else { "b" }
                    )
                });
                Box::new(ExternalPlayer::new(deck, command, bot_timeout))
            }
            other => get_player(deck, &other),
        };
        result.push(player);
    }
    result
}
