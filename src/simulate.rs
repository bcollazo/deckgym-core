use env_logger::{Builder, Env};
use log::warn;
use num_format::{Locale, ToFormattedString};
use std::io::Write;
use uuid::Uuid;

use crate::{
    players::{create_players, fill_code_array, PlayerCode},
    simulation_event_handler::{
        CompositeSimulationEventHandler, SimulationEventHandler, StatsCollector,
    },
    state::GameOutcome,
    Deck, Game,
};

pub struct Simulation {
    deck_a: Deck,
    deck_b: Deck,
    player_codes: Vec<PlayerCode>,
    num_simulations: u32,
    seed: Option<u64>,
    handler_factories: Vec<fn() -> Box<dyn SimulationEventHandler>>,
}

impl Simulation {
    pub fn new(
        deck_a_path: &str,
        deck_b_path: &str,
        player_codes: Vec<PlayerCode>,
        num_simulations: u32,
        seed: Option<u64>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let deck_a = Deck::from_file(deck_a_path)?;
        let deck_b = Deck::from_file(deck_b_path)?;

        Ok(Simulation {
            deck_a,
            deck_b,
            player_codes,
            num_simulations,
            seed,
            handler_factories: vec![],
        })
    }

    fn register<T: SimulationEventHandler + Default + 'static>(mut self) -> Self {
        self.handler_factories.push(|| Box::new(T::default()));
        self
    }

    pub fn run(&mut self) -> Vec<Option<GameOutcome>> {
        // Top-level event handler
        let mut main_event_handler = CompositeSimulationEventHandler::new(
            self.handler_factories
                .iter()
                .map(|factory| factory())
                .collect(),
        );

        let mut thread_event_handlers = vec![];
        let mut outcomes = vec![];
        for _ in 1..=self.num_simulations {
            // Make a thread-local event handler for this simulation
            let mut event_handler = CompositeSimulationEventHandler::new(
                self.handler_factories
                    .iter()
                    .map(|factory| factory())
                    .collect(),
            );

            let players = create_players(
                self.deck_a.clone(),
                self.deck_b.clone(),
                self.player_codes.clone(),
            );
            let seed = self.seed.unwrap_or(rand::random::<u64>());
            let game_id = Uuid::new_v4();
            event_handler.on_game_start(game_id);

            // Give the self.event_handler a mutable reference to the Game
            let mut game =
                Game::new_with_event_handlers(game_id, players, seed, &mut event_handler);
            let outcome = game.play();
            let clone = game.get_state_clone();
            // done with the game, should be dropped now

            event_handler.on_game_end(game_id, clone, outcome);

            outcomes.push(outcome);
            thread_event_handlers.push(event_handler);
        }

        // Merge all thread-local event handlers into the main one
        for handler in thread_event_handlers.iter() {
            main_event_handler.merge(handler);
        }
        main_event_handler.on_simulation_end();

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
    let player_codes = fill_code_array(players);

    warn!(
        "Running {} games with players:",
        num_simulations.to_formatted_string(&Locale::en),
    );
    warn!("\tPlayer 0: {:?}({})", player_codes[0], deck_a_path);
    warn!("\tPlayer 1: {:?}({})", player_codes[1], deck_b_path);

    let mut simulation = Simulation::new(
        deck_a_path,
        deck_b_path,
        player_codes,
        num_simulations,
        seed,
    )
    .expect("Failed to create simulation");
    simulation = simulation.register::<StatsCollector>();
    simulation.run();
}

// Set up the logger according to the given verbosity.
pub fn initialize_logger(verbose: u8) {
    let level = match verbose {
        1 => "warn",
        2 => "info",
        3 => "debug",
        _ => "trace",
    };
    Builder::from_env(Env::default().default_filter_or(level))
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .init();
}
