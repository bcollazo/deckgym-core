use log::{info, warn};
use num_format::{Locale, ToFormattedString};

use crate::{
    players::{create_players, fill_code_array, PlayerCode},
    simulation_event_handler::{
        CompositeSimulationEventHandler, SimulationEventHandler, StatsCollector,
    },
    state::GameOutcome,
    Deck, Game,
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
    event_handler: CompositeSimulationEventHandler,
}

impl Simulation {
    /// Create a new simulation from file paths
    pub fn new(
        deck_a_path: &str,
        deck_b_path: &str,
        players: Option<Vec<PlayerCode>>,
        num_simulations: u32,
        seed: Option<u64>,
        event_handler: CompositeSimulationEventHandler,
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
            event_handler,
        })
    }

    /// Run the simulation and return results
    pub fn run(&mut self) -> Vec<Option<GameOutcome>> {
        // Simulate Games and accumulate statistics
        warn!(
            "Running {} games with players:",
            self.num_simulations.to_formatted_string(&Locale::en),
        );
        warn!(
            "\tPlayer 0: {:?}({})",
            self.player_codes[0], self.deck_a_path
        );
        warn!(
            "\tPlayer 1: {:?}({})",
            self.player_codes[1], self.deck_b_path
        );

        let mut outcomes = Vec::with_capacity(self.num_simulations as usize);
        self.event_handler.on_simulation_start();
        for i in 1..=self.num_simulations {
            let players = create_players(
                self.deck_a.clone(),
                self.deck_b.clone(),
                self.player_codes.clone(),
            );
            let seed = self.seed.unwrap_or(rand::random::<u64>());
            self.event_handler.on_game_start();
            // Give the self.event_handler a mutable reference to the Game
            let mut game = Game::new_with_event_handlers(players, seed, &mut self.event_handler);
            let outcome = game.play();
            let clone = game.get_state_clone();
            // done with the game, should be dropped now
            self.event_handler.on_game_end(clone, outcome);

            info!("Simulation {i}: Winner is {outcome:?}");
            outcomes.push(outcome);
        }
        self.event_handler.on_simulation_end();

        outcomes
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
    let stats_collector = Box::new(StatsCollector::new());
    let mut composite_handler = CompositeSimulationEventHandler::new();
    composite_handler.add_handler(stats_collector);
    let mut simulation = Simulation::new(
        deck_a_path,
        deck_b_path,
        players,
        num_simulations,
        seed,
        composite_handler,
    )
    .expect("Failed to create simulation");
    simulation.run();
}
