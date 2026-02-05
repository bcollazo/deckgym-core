mod ability_ids;
pub mod actions;
mod attack_ids;
pub mod card_ids;
pub mod card_logic;
pub mod card_validation;
pub mod combinatorics;
pub mod database;
pub mod deck;
pub mod effects;
pub mod example_utils;
pub mod game;
pub mod gameplay_stats_collector;
mod hooks;
pub mod models;
pub mod move_generation;
pub mod optimize;
pub mod players;
pub mod simulate;
pub mod simulation_event_handler;
pub mod state;
pub mod temp_deck;
pub mod test_helpers; // TODO: Compile/Expose only in test mode?
pub mod tools;

pub use ability_ids::AbilityId;
pub use attack_ids::AttackId;
pub use deck::Deck;
pub use game::Game;
pub use hooks::to_playable_card;
pub use move_generation::generate_possible_actions;
pub use move_generation::generate_possible_trainer_actions;
pub use optimize::{
    cli_optimize, optimize, optimize_with_configs, EnemyDeckConfig, OptimizationConfig,
    ParallelConfig, SimulationConfig,
};
pub use simulate::{simulate, Simulation, SimulationCallbacks};
pub use simulation_event_handler::ComputedStats;
pub use state::State;

#[cfg(feature = "python")]
pub mod python_bindings;

#[cfg(feature = "tui")]
pub mod tui;

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::PyModule;

#[cfg(feature = "python")]
#[pymodule]
fn deckgym(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    python_bindings::deckgym(py, m)
}
