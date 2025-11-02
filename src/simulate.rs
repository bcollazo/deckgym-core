use env_logger::{Builder, Env};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use log::warn;
use num_format::{Locale, ToFormattedString};
use rayon::prelude::*;
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

    pub fn create_progress_bar(num_simulations: u32) -> ProgressBar {
        let pb = ProgressBar::new(num_simulations as u64);
        pb.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
        );
        pb
    }

    pub fn run(&mut self) -> Vec<Option<GameOutcome>> {
        self.event_handler.on_simulation_start();
        let mut outcomes = vec![];
        let pb = Self::create_progress_bar(self.num_simulations);

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

            pb.inc(1);
        }

        pb.finish();
        self.event_handler.on_simulation_end(false);
        outcomes
    }

    pub fn run_parallel(&mut self) -> Vec<Option<GameOutcome>> {
        self.event_handler.on_simulation_start();
        let pb = Self::create_progress_bar(self.num_simulations);

        // Capture immutable things that can be shared safely
        let deck_a = self.deck_a.clone();
        let deck_b = self.deck_b.clone();
        let player_codes = self.player_codes.clone();
        let seed = self.seed;
        // Note: self.event_handler cannot be shared safely across threads
        // because it mutates internal state, so we’ll collect results first,
        // then report them in sequence.

        let outcomes: Vec<_> = (1..=self.num_simulations)
            .into_par_iter() // rayon parallel iterator
            .progress_with(pb.clone()) // progress bar integration
            .map(|_| {
                let players = create_players(deck_a.clone(), deck_b.clone(), player_codes.clone());
                let seed = seed.unwrap_or(rand::random::<u64>());
                let game_id = Uuid::new_v4();

                // Run simulation fully isolated
                let mut local_handler = CompositeSimulationEventHandler::new();
                let mut game =
                    Game::new_with_event_handlers(game_id, players, seed, &mut local_handler);
                let outcome = game.play();
                let state = game.get_state_clone();

                (game_id, state, outcome)
            })
            .collect();

        // Now sequentially feed results to the event handler
        for (game_id, state, outcome) in outcomes.iter() {
            self.event_handler
                .on_game_end(*game_id, state.clone(), *outcome);
        }

        pb.finish();
        self.event_handler.on_simulation_end(true);

        // Extract only outcomes
        outcomes.into_iter().map(|(_, _, o)| o).collect()
    }
}

/// Legacy functional API for backwards compatibility
pub fn simulate(
    deck_a_path: &str,
    deck_b_path: &str,
    players: Option<Vec<PlayerCode>>,
    num_simulations: u32,
    seed: Option<u64>,
    parallel: bool,
) {
    let player_codes = fill_code_array(players);

    warn!(
        "Running {} games with players:",
        num_simulations.to_formatted_string(&Locale::en),
    );
    warn!("\tPlayer 0: {:?}({})", player_codes[0], deck_a_path);
    warn!("\tPlayer 1: {:?}({})", player_codes[1], deck_b_path);

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

    if parallel {
        simulation.run_parallel();
    } else {
        simulation.run();
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
