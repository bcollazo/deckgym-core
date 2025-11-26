use env_logger::{Builder, Env};
use indicatif::{ProgressBar, ProgressStyle};
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
    handler_factories: Vec<fn() -> Box<dyn SimulationEventHandler>>,
    parallel: bool,
    num_threads: Option<usize>,
    event_handler: Option<CompositeSimulationEventHandler>,
}

impl Simulation {
    pub fn new(
        deck_a_path: &str,
        deck_b_path: &str,
        player_codes: Vec<PlayerCode>,
        num_simulations: u32,
        seed: Option<u64>,
        parallel: bool,
        num_threads: Option<usize>,
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
            parallel,
            num_threads,
            event_handler: None,
        })
    }

    pub fn register<T: SimulationEventHandler + Default + 'static>(mut self) -> Self {
        self.handler_factories.push(|| Box::new(T::default()));
        self
    }

    pub fn run(&mut self) -> Vec<Option<GameOutcome>> {
        // Configure rayon thread pool if specified
        if let Some(num_threads) = self.num_threads {
            rayon::ThreadPoolBuilder::new()
                .num_threads(num_threads)
                .build_global()
                .ok(); // Ignore error if pool is already initialized
        }

        // Top-level event handler
        let mut main_event_handler = CompositeSimulationEventHandler::new(
            self.handler_factories
                .iter()
                .map(|factory| factory())
                .collect(),
        );

        // Create progress bar
        let pb = create_progress_bar(self.num_simulations as u64);

        // Closure to run a single simulation
        let run_single_simulation = |_| {
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

            // Give the event_handler a mutable reference to the Game
            let mut game =
                Game::new_with_event_handlers(game_id, players, seed, &mut event_handler);
            let outcome = game.play();
            let clone = game.get_state_clone();
            // done with the game, should be dropped now

            event_handler.on_game_end(game_id, clone, outcome);

            pb.inc(1);
            (outcome, event_handler)
        };

        // Run simulations either in parallel or sequentially
        let results: Vec<(Option<GameOutcome>, CompositeSimulationEventHandler)> = if self.parallel
        {
            (0..self.num_simulations)
                .into_par_iter()
                .map(run_single_simulation)
                .collect()
        } else {
            (0..self.num_simulations)
                .map(run_single_simulation)
                .collect()
        };

        pb.finish_with_message("Simulation complete!");

        // Split outcomes and event handlers
        let (outcomes, thread_event_handlers): (Vec<_>, Vec<_>) = results.into_iter().unzip();

        // Merge all thread-local event handlers into the main one
        for handler in thread_event_handlers.iter() {
            main_event_handler.merge(handler);
        }
        main_event_handler.on_simulation_end();

        // Store the merged event handler for later retrieval
        self.event_handler = Some(main_event_handler);

        outcomes
    }

    /// Get a reference to a specific event handler by type after simulation has run
    pub fn get_event_handler<T: SimulationEventHandler + 'static>(&self) -> Option<&T> {
        self.event_handler.as_ref()?.get_handler::<T>()
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
    num_threads: Option<usize>,
) {
    let player_codes = fill_code_array(players);

    warn!(
        "Running {} games with players{}:",
        num_simulations.to_formatted_string(&Locale::en),
        if parallel { " (parallel)" } else { "" }
    );
    warn!("\tPlayer 0: {:?}({})", player_codes[0], deck_a_path);
    warn!("\tPlayer 1: {:?}({})", player_codes[1], deck_b_path);
    if let Some(threads) = num_threads {
        warn!("\tThreads: {}", threads);
    }

    let mut simulation = Simulation::new(
        deck_a_path,
        deck_b_path,
        player_codes,
        num_simulations,
        seed,
        parallel,
        num_threads,
    )
    .expect("Failed to create simulation");
    simulation = simulation.register::<StatsCollector>();
    simulation.run();
}

/// Creates a styled progress bar with consistent styling across the codebase
pub fn create_progress_bar(total: u64) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
        )
        .expect("Failed to set progress bar template")
        .progress_chars("#>-"),
    );
    pb.enable_steady_tick(std::time::Duration::from_millis(100));
    pb
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
