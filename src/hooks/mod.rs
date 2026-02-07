/// These are the places/functions in the framework that custom logic is to be implemented per card.
/// That is those special "if Psyduck, do this", "if Darkrai, do that" kind of logic.
/// We call these "hooks" (like on_attach_energy, on_play, on_knockout, etc...).
mod core;
mod counterattack;
mod retreat;

pub(crate) use core::can_evolve_into;
pub(crate) use core::can_play_item;
pub(crate) use core::can_play_support;
pub(crate) use core::contains_energy;
pub(crate) use core::energy_missing;
pub(crate) use core::get_attack_cost;
pub(crate) use core::get_stage;
pub(crate) use core::is_ultra_beast;
pub(crate) use core::modify_damage;
pub(crate) use core::on_end_turn;
pub(crate) use core::on_evolve;
pub(crate) use core::on_knockout;
pub(crate) use core::on_play_to_bench;
pub use core::to_playable_card;
pub(crate) use counterattack::get_counterattack_damage;
pub(crate) use counterattack::should_poison_attacker;
pub(crate) use retreat::can_retreat;
pub(crate) use retreat::get_retreat_cost;
