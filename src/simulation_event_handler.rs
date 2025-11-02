use core::panic;
use log::{info, warn};
use num_format::{Locale, ToFormattedString};
use std::{
    any,
    time::{Duration, Instant},
};
use uuid::Uuid;

use crate::{actions::Action, state::GameOutcome, State};

/// Trait to listen to simulation events
/// Simulations are run in parallel. One instance of SimulationEventHandler will be created
/// on the main thread, plus one per game created. These n+1 instances will be merged
/// into one at the end of the simulation by chaining `reduce` calls.
pub trait SimulationEventHandler: any::Any {
    // Simulation Methods (these will be called on the "main" instance in the main thread)
    // Per-thread instances will NOT have these called
    // fn on_simulation_start(&mut self) {}
    fn on_simulation_end(&mut self) {}
    fn merge(&mut self, _other: &dyn SimulationEventHandler);

    // Game Methods (these will be called on per-thread instances of SimulationEventHandler)
    fn on_game_start(&mut self, _game_id: Uuid) {}
    fn on_action(
        &mut self,
        _game_id: Uuid,
        _state_before_action: &State,
        _actor: usize,
        _playable_actions: &[Action],
        _action: &Action,
    ) {
    }
    fn on_game_end(&mut self, _game_id: Uuid, _state: State, _result: Option<GameOutcome>) {}
}

// A general implementation of the SimulationEventHandler to compose multiple
pub struct CompositeSimulationEventHandler {
    handlers: Vec<Box<dyn SimulationEventHandler>>,
}

impl CompositeSimulationEventHandler {
    pub fn new(handlers: Vec<Box<dyn SimulationEventHandler>>) -> Self {
        Self { handlers }
    }
}

impl SimulationEventHandler for CompositeSimulationEventHandler {
    fn on_game_start(&mut self, game_id: Uuid) {
        for handler in self.handlers.iter_mut() {
            handler.on_game_start(game_id);
        }
    }

    fn on_action(
        &mut self,
        game_id: Uuid,
        state_before_action: &State,
        actor: usize,
        playable_actions: &[Action],
        action: &Action,
    ) {
        for handler in self.handlers.iter_mut() {
            handler.on_action(
                game_id,
                state_before_action,
                actor,
                playable_actions,
                action,
            );
        }
    }

    fn on_game_end(&mut self, game_id: Uuid, state: State, result: Option<GameOutcome>) {
        for handler in self.handlers.iter_mut() {
            handler.on_game_end(game_id, state.clone(), result);
        }
    }

    fn on_simulation_end(&mut self) {
        for handler in self.handlers.iter_mut() {
            handler.on_simulation_end();
        }
    }

    fn merge(&mut self, other: &dyn SimulationEventHandler) {
        if let Some(other_mytype) =
            (other as &dyn any::Any).downcast_ref::<CompositeSimulationEventHandler>()
        {
            if self.handlers.len() != other_mytype.handlers.len() {
                panic!("Attempted to merge CompositeSimulationEventHandler with different number of handlers");
            }
            for (a, b) in self.handlers.iter_mut().zip(other_mytype.handlers.iter()) {
                a.merge(&**b);
            }
        } else {
            panic!("Attempted to merge CompositeSimulationEventHandler with incompatible type");
        }
    }
}

// Example: Statistics collector
pub struct StatsCollector {
    start: Instant,
    end: Instant,
    num_games: u32,

    degrees_per_ply: Vec<u32>,

    player_a_wins: u32,
    player_b_wins: u32,
    ties: u32,
    turns_per_game: Vec<u8>,
    plys_per_game: Vec<u32>,
    total_degrees: Vec<u32>,
}

impl Default for StatsCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl StatsCollector {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            end: Instant::now(),
            num_games: 0,
            degrees_per_ply: vec![],
            player_a_wins: 0,
            player_b_wins: 0,
            ties: 0,
            turns_per_game: vec![],
            plys_per_game: vec![],
            total_degrees: vec![],
        }
    }
}

impl SimulationEventHandler for StatsCollector {
    fn on_game_start(&mut self, _game_id: Uuid) {
        self.start = self.start.min(Instant::now()); // minimum ever seen
        self.degrees_per_ply.clear();
    }

    fn on_action(
        &mut self,
        _game_id: Uuid,
        _state_before_action: &State,
        _actor: usize,
        playable_actions: &[Action],
        _action: &Action,
    ) {
        self.degrees_per_ply.push(playable_actions.len() as u32);
    }

    fn on_game_end(&mut self, game_id: Uuid, state: State, outcome: Option<GameOutcome>) {
        info!("Simulation {game_id}: Winner is {outcome:?}");

        self.end = self.end.max(Instant::now()); // maximum ever seen
        self.num_games += 1;
        self.turns_per_game.push(state.turn_count);
        self.plys_per_game.push(self.degrees_per_ply.len() as u32);
        self.total_degrees.extend(self.degrees_per_ply.iter());

        match outcome {
            Some(GameOutcome::Win(winner_name)) => {
                if winner_name == 0 {
                    self.player_a_wins += 1;
                } else {
                    self.player_b_wins += 1;
                }
            }
            Some(GameOutcome::Tie) | None => {
                self.ties += 1;
            }
        }
    }

    fn merge(&mut self, other: &dyn SimulationEventHandler)
    where
        Self: Sized,
    {
        if let Some(other_mytype) = (other as &dyn any::Any).downcast_ref::<StatsCollector>() {
            self.start = self.start.min(other_mytype.start);
            self.end = self.end.max(other_mytype.end);
            self.num_games += other_mytype.num_games;
            self.turns_per_game
                .extend(other_mytype.turns_per_game.iter());
            self.plys_per_game.extend(other_mytype.plys_per_game.iter());
            self.total_degrees.extend(other_mytype.total_degrees.iter());
            self.player_a_wins += other_mytype.player_a_wins;
            self.player_b_wins += other_mytype.player_b_wins;
            self.ties += other_mytype.ties;
        } else {
            panic!("Attempted to merge StatsCollector with incompatible type");
        }
    }

    fn on_simulation_end(&mut self) {
        let duration = self.end.duration_since(self.start);
        let avg_time_per_game = duration.as_secs_f64() / self.num_games as f64;
        let avg_duration = Duration::from_secs_f64(avg_time_per_game);

        let avg_turns_per_game = self
            .turns_per_game
            .iter()
            .map(|&turns| turns as u32)
            .sum::<u32>() as f32
            / self.num_games as f32;

        let avg_plys_per_game =
            self.plys_per_game.iter().sum::<u32>() as f32 / self.num_games as f32;

        let avg_degrees_per_ply = if self.total_degrees.is_empty() {
            0.0
        } else {
            self.total_degrees.iter().sum::<u32>() as f32 / self.total_degrees.len() as f32
        };

        warn!(
            "Ran {} simulations in {} ({} per game)!",
            self.num_games.to_formatted_string(&Locale::en),
            humantime::format_duration(duration),
            humantime::format_duration(avg_duration)
        );
        warn!("Average number of turns per game: {avg_turns_per_game:.2}");
        warn!("Average number of plys per game: {avg_plys_per_game:.2}");
        warn!("Average number of degrees per ply: {avg_degrees_per_ply:.2}");

        let player_a_win_rate = self.player_a_wins as f32 / self.num_games as f32;
        let player_b_win_rate = self.player_b_wins as f32 / self.num_games as f32;
        let tie_rate = self.ties as f32 / self.num_games as f32;
        warn!(
            "Player 0 won: {} ({:.2}%)",
            self.player_a_wins.to_formatted_string(&Locale::en),
            player_a_win_rate * 100.0
        );
        warn!(
            "Player 1 won: {} ({:.2}%)",
            self.player_b_wins.to_formatted_string(&Locale::en),
            player_b_win_rate * 100.0
        );
        warn!(
            "Draws: {} ({:.2}%)",
            self.ties.to_formatted_string(&Locale::en),
            tie_rate * 100.0
        );
    }
}
