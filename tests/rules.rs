//! Rules-accuracy tests derived from the game's own text: the in-app "Tips" (Battle Guide) panel
//! and the app-linked official "Detailed battle FAQ", both transcribed 2026-09-08. Each test
//! cites the sentence it enforces.
#[path = "rules/checkup_order_test.rs"]
mod checkup_order_test;
#[path = "rules/damage_order_test.rs"]
mod damage_order_test;
#[path = "rules/evolution_timing_test.rs"]
mod evolution_timing_test;
#[path = "rules/special_conditions_test.rs"]
mod special_conditions_test;
#[path = "rules/weakness_active_only_test.rs"]
mod weakness_active_only_test;
#[path = "rules/win_conditions_test.rs"]
mod win_conditions_test;
