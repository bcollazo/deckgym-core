use log::{info, warn};
use num_format::{Locale, ToFormattedString};
use std::time::{Duration, Instant};

use crate::{actions::Action, state::GameOutcome, State};

pub trait SimulationEventHandler {
    fn on_simulation_start(&mut self) {}

    fn on_game_start(&mut self, _id: u32) {}
    fn on_action(&mut self, _actor: usize, _playable_actions: &Vec<Action>, _action: &Action) {}
    fn on_game_end(&mut self, _id: u32, _state: State, _result: Option<GameOutcome>) {}

    fn on_simulation_end(&mut self) {}
}

// A general implementation of the SimulationEventHandler to compose multiple
pub struct CompositeSimulationEventHandler {
    handlers: Vec<Box<dyn SimulationEventHandler>>,
}

impl CompositeSimulationEventHandler {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn add_handler(&mut self, handler: Box<dyn SimulationEventHandler>) {
        self.handlers.push(handler);
    }
}

impl SimulationEventHandler for CompositeSimulationEventHandler {
    fn on_simulation_start(&mut self) {
        for handler in self.handlers.iter_mut() {
            handler.on_simulation_start();
        }
    }

    fn on_game_start(&mut self, id: u32) {
        for handler in self.handlers.iter_mut() {
            handler.on_game_start(id);
        }
    }

    fn on_action(&mut self, actor: usize, playable_actions: &Vec<Action>, action: &Action) {
        for handler in self.handlers.iter_mut() {
            handler.on_action(actor, playable_actions, action);
        }
    }

    fn on_game_end(&mut self, id: u32, state: State, result: Option<GameOutcome>) {
        for handler in self.handlers.iter_mut() {
            handler.on_game_end(id, state.clone(), result.clone());
        }
    }

    fn on_simulation_end(&mut self) {
        for handler in self.handlers.iter_mut() {
            handler.on_simulation_end();
        }
    }
}

// Example: Statistics collector
pub struct StatsCollector {
    start: Instant,
    num_games: u32,

    degrees_per_ply: Vec<u32>,

    player_a_wins: u32,
    player_b_wins: u32,
    ties: u32,
    turns_per_game: Vec<u8>,
    plys_per_game: Vec<u32>,
    total_degrees: Vec<u32>,
}

impl StatsCollector {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
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
    fn on_simulation_start(&mut self) {
        self.start = Instant::now(); // Start the timer
    }

    fn on_game_start(&mut self, _id: u32) {
        self.degrees_per_ply.clear();
    }

    fn on_action(&mut self, _actor: usize, playable_actions: &Vec<Action>, _action: &Action) {
        self.degrees_per_ply.push(playable_actions.len() as u32);
    }

    fn on_game_end(&mut self, id: u32, state: State, outcome: Option<GameOutcome>) {
        info!("Simulation {id}: Winner is {outcome:?}");

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

    fn on_simulation_end(&mut self) {
        let duration = self.start.elapsed(); // Measure elapsed time
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
        warn!(
            "Average number of turns per game: {:.2}",
            avg_turns_per_game
        );
        warn!("Average number of plys per game: {:.2}", avg_plys_per_game);
        warn!(
            "Average number of degrees per ply: {:.2}",
            avg_degrees_per_ply
        );

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
