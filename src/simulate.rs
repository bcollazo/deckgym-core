use env_logger::{Builder, Env};
use indicatif::{ProgressBar, ProgressStyle};
use log::warn;
use num_format::{Locale, ToFormattedString};
use rayon::prelude::*;
use std::io::Write;
use std::path::PathBuf;
use uuid::Uuid;

use crate::{
    data_exporter::DataExporter,
    optimize::{ParallelConfig, SimulationConfig},
    players::{create_players_with_bots, fill_code_array, PlayerCode},
    replay::ReplayRecorder,
    simulation_event_handler::{
        CompositeSimulationEventHandler, GameStartMetadata, SimulationEventHandler, StatsCollector,
    },
    state::GameOutcome,
    Deck, Game,
};
use std::sync::{atomic::AtomicUsize, Arc};
use std::time::Duration;

/// Default per-decision timeout for an external bot (see `docs/bot-protocol.md`), used when the
/// CLI's `--bot-timeout-ms` isn't given.
pub const DEFAULT_BOT_TIMEOUT_MS: u64 = 10_000;

/// Type alias for player factory function
pub type PlayerFactory =
    Box<dyn Fn(Deck, Deck) -> Vec<Box<dyn crate::players::Player>> + Send + Sync>;

/// Callbacks for simulation progress tracking
pub struct SimulationCallbacks<F>
where
    F: Fn() + Sync,
{
    pub on_game_complete: Option<F>,
}

impl<F> Default for SimulationCallbacks<F>
where
    F: Fn() + Sync,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<F> SimulationCallbacks<F>
where
    F: Fn() + Sync,
{
    pub fn new() -> Self {
        Self {
            on_game_complete: None,
        }
    }

    pub fn with_game_callback(mut self, callback: F) -> Self {
        self.on_game_complete = Some(callback);
        self
    }
}

pub struct Simulation {
    deck_a: Deck,
    deck_b: Deck,
    player_codes: Vec<PlayerCode>,
    num_simulations: u32,
    seed: Option<u64>,
    handler_factories: Vec<Box<dyn Fn() -> Box<dyn SimulationEventHandler> + Send + Sync>>,
    parallel: bool,
    num_threads: Option<usize>,
    event_handler: Option<CompositeSimulationEventHandler>,
    callbacks: Option<SimulationCallbacks<Box<dyn Fn() + Sync>>>,
    player_factory: Option<PlayerFactory>,
    /// Commands for player A/B when their `PlayerCode` is `X` (an external bot). See `with_bots`.
    bots: [Option<String>; 2],
    bot_timeout: Duration,
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
        Simulation::new_with_decks(
            deck_a,
            deck_b,
            player_codes,
            num_simulations,
            seed,
            parallel,
            num_threads,
        )
    }

    pub fn new_with_decks(
        deck_a: Deck,
        deck_b: Deck,
        player_codes: Vec<PlayerCode>,
        num_simulations: u32,
        seed: Option<u64>,
        parallel: bool,
        num_threads: Option<usize>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
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
            callbacks: None,
            player_factory: None,
            bots: [None, None],
            bot_timeout: Duration::from_millis(DEFAULT_BOT_TIMEOUT_MS),
        })
    }

    pub fn new_with_player_factory<F>(
        deck_a: Deck,
        deck_b: Deck,
        player_factory: F,
        num_simulations: u32,
        seed: Option<u64>,
        parallel: bool,
        num_threads: Option<usize>,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        F: Fn(Deck, Deck) -> Vec<Box<dyn crate::players::Player>> + Send + Sync + 'static,
    {
        Ok(Simulation {
            deck_a,
            deck_b,
            player_codes: vec![], // Not used when player_factory is provided
            num_simulations,
            seed,
            handler_factories: vec![],
            parallel,
            num_threads,
            event_handler: None,
            callbacks: None,
            player_factory: Some(Box::new(player_factory)),
            bots: [None, None],
            bot_timeout: Duration::from_millis(DEFAULT_BOT_TIMEOUT_MS),
        })
    }

    pub fn register<T: SimulationEventHandler + Default + 'static>(mut self) -> Self {
        self.handler_factories
            .push(Box::new(|| Box::new(T::default())));
        self
    }

    pub fn register_with_closure<F>(mut self, factory: F) -> Self
    where
        F: Fn() -> Box<dyn SimulationEventHandler> + Send + Sync + 'static,
    {
        self.handler_factories.push(Box::new(factory));
        self
    }

    pub fn with_callback<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Sync + 'static,
    {
        self.callbacks = Some(SimulationCallbacks::new().with_game_callback(Box::new(callback)));
        self
    }

    /// Sets the external-bot commands and per-decision timeout for `PlayerCode::X` slots (see
    /// `docs/bot-protocol.md`). Ignored when a `player_factory` was supplied instead of player
    /// codes.
    pub fn with_bots(
        mut self,
        bot_a: Option<String>,
        bot_b: Option<String>,
        timeout: Duration,
    ) -> Self {
        self.bots = [bot_a, bot_b];
        self.bot_timeout = timeout;
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

        // Extract the game callback to avoid capturing the entire callbacks struct
        let game_callback = self
            .callbacks
            .as_ref()
            .and_then(|cbs| cbs.on_game_complete.as_ref());

        // Closure to run a single simulation
        let run_single_simulation = |_| {
            // Make a thread-local event handler for this simulation
            let mut event_handler = CompositeSimulationEventHandler::new(
                self.handler_factories
                    .iter()
                    .map(|factory| factory())
                    .collect(),
            );

            let players = if let Some(ref factory) = self.player_factory {
                factory(self.deck_a.clone(), self.deck_b.clone())
            } else {
                create_players_with_bots(
                    self.deck_a.clone(),
                    self.deck_b.clone(),
                    self.player_codes.clone(),
                    self.bots.clone(),
                    self.bot_timeout,
                )
            };
            let seed = self.seed.unwrap_or(rand::random::<u64>());
            let game_id = Uuid::new_v4();
            let player_names = [format!("{:?}", players[0]), format!("{:?}", players[1])];
            let decks = [players[0].get_deck(), players[1].get_deck()];
            let metadata = GameStartMetadata {
                seed,
                player_names,
                decks: [&decks[0], &decks[1]],
            };
            event_handler.on_game_start_with_metadata(game_id, &metadata);

            // Give the event_handler a mutable reference to the Game
            let mut game =
                Game::new_with_event_handlers(game_id, players, seed, &mut event_handler);
            let outcome = game.play();
            let clone = game.get_state_clone();
            // done with the game, should be dropped now

            event_handler.on_game_end(game_id, clone, outcome);

            if let Some(callback) = game_callback {
                callback();
            }

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

/// Registers DataExporter with the given output folder
fn register_data_exporter(simulation: Simulation, output_folder: String) -> Simulation {
    let output_path = PathBuf::from(output_folder);

    // Create the output folder if it doesn't exist
    if let Err(e) = std::fs::create_dir_all(&output_path) {
        panic!(
            "Failed to create data output folder {:?}: {}",
            output_path, e
        );
    }

    warn!("Exporting simulation data to: {:?}", output_path);

    simulation.register_with_closure(move || Box::new(DataExporter::new(output_path.clone())))
}

/// Registers a `ReplayRecorder` writing one JSON replay file per game into `replay_dir`.
/// `replay_sample` (if set) caps how many of the games in this run get a replay written; the
/// counter is shared (via `Arc`) across every per-game `ReplayRecorder` instance so the cap holds
/// across parallel simulations too.
fn register_replay_recorder(
    simulation: Simulation,
    replay_dir: String,
    replay_sample: Option<usize>,
) -> Simulation {
    let output_path = PathBuf::from(replay_dir);

    if let Err(e) = std::fs::create_dir_all(&output_path) {
        panic!(
            "Failed to create replay output folder {:?}: {}",
            output_path, e
        );
    }

    warn!("Recording replays to: {:?}", output_path);
    if let Some(n) = replay_sample {
        warn!("\tReplay sample: first {} game(s) of this run", n);
    }

    let recorded_so_far = Arc::new(AtomicUsize::new(0));
    simulation.register_with_closure(move || {
        Box::new(ReplayRecorder::new(
            output_path.clone(),
            recorded_so_far.clone(),
            replay_sample,
        ))
    })
}

/// Functional API for running simulations
pub fn simulate(
    deck_a_path: &str,
    deck_b_path: &str,
    sim_config: SimulationConfig,
    parallel_config: ParallelConfig,
) {
    let player_codes = fill_code_array(sim_config.players);

    warn!(
        "Running {} games with players{}:",
        sim_config.num_games.to_formatted_string(&Locale::en),
        if parallel_config.enabled {
            " (parallel)"
        } else {
            ""
        }
    );
    warn!("\tPlayer 0: {:?}({})", player_codes[0], deck_a_path);
    warn!("\tPlayer 1: {:?}({})", player_codes[1], deck_b_path);
    if let Some(threads) = parallel_config.num_threads {
        warn!("\tThreads: {}", threads);
    }

    // Create progress bar
    let pb = create_progress_bar(sim_config.num_games as u64);
    pb.tick(); // Ensure progress bar is drawn immediately

    let mut simulation = Simulation::new(
        deck_a_path,
        deck_b_path,
        player_codes,
        sim_config.num_games,
        sim_config.seed,
        parallel_config.enabled,
        parallel_config.num_threads,
    )
    .expect("Failed to create simulation");

    simulation = simulation.register::<StatsCollector>();
    if let Some(output_folder) = sim_config.data_output {
        simulation = register_data_exporter(simulation, output_folder);
    }
    if let Some(replay_dir) = sim_config.replay_dir {
        simulation = register_replay_recorder(simulation, replay_dir, sim_config.replay_sample);
    }
    simulation = simulation.with_bots(
        sim_config.bot_a,
        sim_config.bot_b,
        Duration::from_millis(sim_config.bot_timeout_ms),
    );

    let pb_clone = pb.clone();
    simulation = simulation.with_callback(move || pb_clone.inc(1));
    simulation.run();

    pb.finish_with_message("Simulation complete!");

    // Retrieve and print statistics
    if let Some(collector) = simulation.get_event_handler::<StatsCollector>() {
        let stats = collector.compute_stats();
        print_stats(&stats);
    }
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

/// Print simulation statistics to the console
pub fn print_stats(stats: &crate::simulation_event_handler::ComputedStats) {
    warn!(
        "Ran {} simulations in {} ({} per game)!",
        stats.num_games.to_formatted_string(&Locale::en),
        humantime::format_duration(stats.duration),
        humantime::format_duration(stats.avg_duration)
    );
    warn!(
        "Average number of turns per game: {:.2}",
        stats.avg_turns_per_game
    );
    warn!(
        "Average number of plys per game: {:.2}",
        stats.avg_plys_per_game
    );
    warn!(
        "Average number of degrees per ply: {:.2}",
        stats.avg_degrees_per_ply
    );

    warn!(
        "Player 0 won: {} ({:.2}%)",
        stats.player_a_wins.to_formatted_string(&Locale::en),
        stats.player_a_win_rate * 100.0
    );
    warn!(
        "Player 1 won: {} ({:.2}%)",
        stats.player_b_wins.to_formatted_string(&Locale::en),
        stats.player_b_win_rate * 100.0
    );
    warn!(
        "Draws: {} ({:.2}%)",
        stats.ties.to_formatted_string(&Locale::en),
        stats.tie_rate * 100.0
    );
}

// Set up the logger according to the given verbosity.
pub fn initialize_logger(verbose: u8) {
    let level = match verbose {
        1 => "warn",
        2 => "info",
        3 => "debug",
        4 => "trace",
        _ => "error",
    };
    Builder::from_env(Env::default().default_filter_or(level))
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .init();
}
