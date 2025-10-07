use env_logger::{Builder, Env};
use log::warn;
use num_format::{Locale, ToFormattedString};
use std::io::Write;
use uuid::Uuid;

use crate::{
    export::TrainingDataCollector,
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
    event_handler: CompositeSimulationEventHandler,
}

impl Simulation {
    pub fn new(
        deck_a_path: &str,
        deck_b_path: &str,
        player_codes: Vec<PlayerCode>,
        num_simulations: u32,
        seed: Option<u64>,
        event_handler: CompositeSimulationEventHandler,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let deck_a = Deck::from_file(deck_a_path)?;
        let deck_b = Deck::from_file(deck_b_path)?;

        Ok(Simulation {
            deck_a,
            deck_b,
            player_codes,
            num_simulations,
            seed,
            event_handler,
        })
    }

    pub fn run(&mut self) -> Vec<Option<GameOutcome>> {
        self.event_handler.on_simulation_start();
        let mut outcomes = vec![];
        for _ in 1..=self.num_simulations {
            let players = create_players(
                self.deck_a.clone(),
                self.deck_b.clone(),
                self.player_codes.clone(),
            );
            let seed = self.seed.unwrap_or(rand::random::<u64>());
            let game_id = Uuid::new_v4();
            self.event_handler.on_game_start(game_id);

            // Give the self.event_handler a mutable reference to the Game
            let mut game =
                Game::new_with_event_handlers(game_id, players, seed, &mut self.event_handler);
            let outcome = game.play();
            let clone = game.get_state_clone();
            // done with the game, should be dropped now

            self.event_handler.on_game_end(game_id, clone, outcome);

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
    export_path: Option<&str>,
) {
    let player_codes = fill_code_array(players);

    warn!(
        "Running {} games with players:",
        num_simulations.to_formatted_string(&Locale::en),
    );
    warn!("\tPlayer 0: {:?}({})", player_codes[0], deck_a_path);
    warn!("\tPlayer 1: {:?}({})", player_codes[1], deck_b_path);

    if let Some(export_path) = export_path {
        run_simulation_with_export(deck_a_path, deck_b_path, player_codes, num_simulations, seed, export_path);
    } else {
        let stats_collector = Box::new(StatsCollector::new());
        let mut composite_handler = CompositeSimulationEventHandler::new();
        composite_handler.add_handler(stats_collector);
        let mut simulation = Simulation::new(
            deck_a_path,
            deck_b_path,
            player_codes,
            num_simulations,
            seed,
            composite_handler,
        )
        .expect("Failed to create simulation");
        simulation.run();
    }
}

/// Run simulation with training data export
fn run_simulation_with_export(
    deck_a_path: &str,
    deck_b_path: &str,
    player_codes: Vec<PlayerCode>,
    num_simulations: u32,
    seed: Option<u64>,
    export_path: &str,
) {
    let mut training_collector = TrainingDataCollector::new();
    let stats_collector = Box::new(StatsCollector::new());
    
    let mut composite_handler = CompositeSimulationEventHandler::new();
    composite_handler.add_handler(stats_collector);
    
    // We need to handle the training data collection manually due to ownership issues
    let deck_a = Deck::from_file(deck_a_path).expect("Failed to load deck A");
    let deck_b = Deck::from_file(deck_b_path).expect("Failed to load deck B");
    
    for _ in 1..=num_simulations {
        let players = create_players(deck_a.clone(), deck_b.clone(), player_codes.clone());
        let game_seed = seed.unwrap_or(rand::random::<u64>());
        let game_id = Uuid::new_v4();
        
        // Start collecting data for this game
        training_collector.on_game_start(game_id);
        
        let mut game = Game::new(players, game_seed);
        
        // Manually run the game loop and collect training data
        while !game.is_game_over() {
            let (actor, actions) = crate::generate_possible_actions(&game.get_state_clone());
            let state_before = game.get_state_clone();
            let action = game.play_tick();
            let state_after = game.get_state_clone();
            
            training_collector.on_action(game_id, &state_before, actor, &actions, &action, &state_after);
        }
        
        let outcome = game.get_state_clone().winner;
        training_collector.on_game_end(game_id, game.get_state_clone(), outcome);
    }
    
    // Export the collected training data
    if let Err(e) = training_collector.save_to_file(export_path) {
        warn!("Failed to save training data to {}: {}", export_path, e);
    } else {
        warn!("Training data exported to: {}", export_path);
    }
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
