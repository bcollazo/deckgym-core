use std::time::{Duration, Instant};

use log::{info, warn};
use num_format::{Locale, ToFormattedString};

use crate::{
    players::{create_players, fill_code_array, PlayerCode},
    state::GameOutcome,
    Deck, Game, State,
};

/// Object-oriented simulation configuration and runner
pub struct Simulation {
    deck_a: Deck,
    deck_b: Deck,
    deck_a_path: String,
    deck_b_path: String,
    player_codes: Vec<PlayerCode>,
    num_simulations: u32,
    seed: Option<u64>,
    event_handlers: Vec<Box<dyn SimulationEventHandler + Send + Sync>>,
}

impl Simulation {
    /// Create a new simulation from file paths
    pub fn new(
        deck_a_path: &str,
        deck_b_path: &str,
        players: Option<Vec<PlayerCode>>,
        num_simulations: u32,
        seed: Option<u64>,
        event_handlers: Vec<Box<dyn SimulationEventHandler + Send + Sync>>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let deck_a = Deck::from_file(deck_a_path)?;
        let deck_b = Deck::from_file(deck_b_path)?;
        let player_codes = fill_code_array(players);

        Ok(Simulation {
            deck_a,
            deck_b,
            deck_a_path: deck_a_path.to_string(),
            deck_b_path: deck_b_path.to_string(),
            player_codes,
            num_simulations,
            seed,
            event_handlers,
        })
    }

    /// Create a new simulation from deck objects
    pub fn from_decks(
        deck_a: Deck,
        deck_b: Deck,
        players: Option<Vec<PlayerCode>>,
        num_simulations: u32,
        seed: Option<u64>,
        event_handlers: Vec<Box<dyn SimulationEventHandler + Send + Sync>>,
    ) -> Self {
        let player_codes = fill_code_array(players);

        Simulation {
            deck_a,
            deck_b,
            deck_a_path: "deck_a".to_string(),
            deck_b_path: "deck_b".to_string(),
            player_codes,
            num_simulations,
            seed,
            event_handlers,
        }
    }

    /// Run the simulation and return results
    pub fn run(&mut self) -> SimulationResults {
        // Simulate Games and accumulate statistics
        warn!(
            "Running {} games with players {:?}",
            self.num_simulations.to_formatted_string(&Locale::en),
            self.player_codes
        );

        let start = Instant::now(); // Start the timer
        let mut wins_per_deck = [0, 0, 0];
        let mut turns_per_game = Vec::new();
        let mut plys_per_game = Vec::new();
        let mut total_degrees = Vec::new();

        self.broadcast(|h| h.on_simulation_start());
        for i in 1..=self.num_simulations {
            let players = create_players(
                self.deck_a.clone(),
                self.deck_b.clone(),
                self.player_codes.clone(),
            );
            let seed = self.seed.unwrap_or(rand::random::<u64>());
            let mut game = Game::new(players, seed);

            self.broadcast(|h| h.on_game_start(game.get_state_clone()));
            let outcome = game.play();
            self.broadcast(|h| h.on_game_end(game.get_state_clone(), outcome));

            turns_per_game.push(game.get_state_clone().turn_count);
            plys_per_game.push(game.get_num_plys());
            total_degrees.extend(game.get_degrees_per_ply().iter());
            info!("Simulation {i}: Winner is {outcome:?}");
            match outcome {
                Some(GameOutcome::Win(winner_name)) => {
                    wins_per_deck[winner_name] += 1;
                }
                Some(GameOutcome::Tie) | None => {
                    wins_per_deck[2] += 1;
                }
            }
        }
        self.broadcast(|h| h.on_simulation_end());

        let duration = start.elapsed(); // Measure elapsed time
        let avg_time_per_game = duration.as_secs_f64() / self.num_simulations as f64;
        let avg_duration = Duration::from_secs_f64(avg_time_per_game);

        // Calculate averages
        let avg_turns_per_game = turns_per_game
            .iter()
            .map(|&turns| turns as u32)
            .sum::<u32>() as f32
            / self.num_simulations as f32;

        let avg_plys_per_game =
            plys_per_game.iter().sum::<u32>() as f32 / self.num_simulations as f32;

        let avg_degrees_per_ply = if total_degrees.is_empty() {
            0.0
        } else {
            total_degrees.iter().sum::<u32>() as f32 / total_degrees.len() as f32
        };

        SimulationResults {
            total_games: self.num_simulations,
            player_a_wins: wins_per_deck[0],
            player_b_wins: wins_per_deck[1],
            ties: wins_per_deck[2],
            player_a_win_rate: wins_per_deck[0] as f32 / self.num_simulations as f32,
            player_b_win_rate: wins_per_deck[1] as f32 / self.num_simulations as f32,
            tie_rate: wins_per_deck[2] as f32 / self.num_simulations as f32,
            avg_turns_per_game,
            avg_plys_per_game,
            avg_degrees_per_ply,
            duration,
            avg_time_per_game: avg_duration,
        }
    }

    /// Run the simulation and print the results
    pub fn run_and_print(&mut self) -> SimulationResults {
        let results = self.run();
        results.print_stats(&self.deck_a_path, &self.deck_b_path, &self.player_codes);
        results
    }

    fn broadcast<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut dyn SimulationEventHandler),
    {
        for handler in self.event_handlers.iter_mut() {
            f(handler.as_mut());
        }
    }
}

/// Legacy functional API for backwards compatibility
pub fn simulate(
    deck_a_path: &str,
    deck_b_path: &str,
    players: Option<Vec<PlayerCode>>,
    num_simulations: u32,
    seed: Option<u64>,
) {
    let mut simulation = Simulation::new(
        deck_a_path,
        deck_b_path,
        players,
        num_simulations,
        seed,
        vec![],
    )
    .expect("Failed to create simulation");
    simulation.run_and_print();
}

/// Results from running a simulation
#[derive(Debug, Clone)]
pub struct SimulationResults {
    pub total_games: u32,
    pub player_a_wins: u32,
    pub player_b_wins: u32,
    pub ties: u32,
    pub player_a_win_rate: f32,
    pub player_b_win_rate: f32,
    pub tie_rate: f32,
    pub avg_turns_per_game: f32,
    pub avg_plys_per_game: f32,
    pub avg_degrees_per_ply: f32,
    pub duration: Duration,
    pub avg_time_per_game: Duration,
}

impl SimulationResults {
    /// Print detailed statistics about the simulation results
    pub fn print_stats(&self, deck_a_path: &str, deck_b_path: &str, player_codes: &[PlayerCode]) {
        warn!(
            "Ran {} simulations in {} ({} per game)!",
            self.total_games.to_formatted_string(&Locale::en),
            humantime::format_duration(self.duration),
            humantime::format_duration(self.avg_time_per_game)
        );
        warn!(
            "Average number of turns per game: {:.2}",
            self.avg_turns_per_game
        );
        warn!(
            "Average number of plys per game: {:.2}",
            self.avg_plys_per_game
        );
        warn!(
            "Average number of degrees per ply: {:.2}",
            self.avg_degrees_per_ply
        );
        warn!(
            "Player {:?} with Deck {} wins: {} ({:.2}%)",
            player_codes[0],
            deck_a_path,
            self.player_a_wins.to_formatted_string(&Locale::en),
            self.player_a_win_rate * 100.0
        );
        warn!(
            "Player {:?} with Deck {} wins: {} ({:.2}%)",
            player_codes[1],
            deck_b_path,
            self.player_b_wins.to_formatted_string(&Locale::en),
            self.player_b_win_rate * 100.0
        );
        warn!(
            "Draws: {} ({:.2}%)",
            self.ties.to_formatted_string(&Locale::en),
            self.tie_rate * 100.0
        );
    }
}

pub trait SimulationEventHandler {
    fn on_simulation_start(&mut self) {}
    fn on_simulation_end(&mut self) {}

    fn on_game_start(&mut self, _state: State) {}
    fn on_game_end(&mut self, _state: State, _result: Option<GameOutcome>) {}

    // TODO: Implement
    // fn on_turn(&mut self, _turn: usize, _state: &State) {}
}
