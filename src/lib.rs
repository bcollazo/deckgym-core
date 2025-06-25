mod ability_ids;
pub mod actions;
mod attack_ids;
pub mod card_ids;
pub mod database;
pub mod deck;
pub mod game;
mod hooks;
pub mod move_generation;
mod optimize;
pub mod players;
pub mod simulate;
pub mod state;
pub mod test_helpers; // TODO: Compile/Expose only in test mode?
pub mod tool_ids;
pub mod types;

pub use ability_ids::AbilityId;
pub use attack_ids::AttackId;
pub use deck::Deck;
pub use game::Game;
pub use move_generation::generate_possible_actions;
pub use move_generation::generate_possible_trainer_actions;
pub use optimize::optimize;
pub use simulate::simulate;
pub use state::State;

#[cfg(feature = "python")]
pub mod python_bindings;

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymodule]
fn deckgym(_py: Python, m: &PyModule) -> PyResult<()> {
    python_bindings::deckgym(_py, m)
}
