---
name: Migrate AttackId Cards
description: Migrates attack implementations from the old approach (using AttackId) to use new approach (Mechanic enum)
---

The codebase is in a dirty state, don't try to eliminate compilation warnings, or apply clippy suggestions.

- Read the `models` module and the `state` module. 
- Find the `AttackId` to migrate in the commented match statement in `apply_attack_action.rs`
- Search for the card information with the following script (e.g. AttackId::A1004VenusaurExGiantBloom):

  ```bash
  cargo run --bin search "Venusaur" --attack "Giant Bloom"
  ```

- Search for the effect text in the above JSON in the `attack_implementations.rs` file.
- Uncomment the corresponding line in `attack_implementations.rs`, and either add or reuse a `Mechanic` enum. 
- Implement the mechanic logic in `forecast_effect_attack` in `apply_attack_action.rs`.
  - Reuse methods already in `apply_attack_action.rs`, and uncomment from their old usaga in the big commented out match statement.
  - Keep the code as a one-liner in the match statement, and implement the logic using a helper function.
- DO NOT run `cargo fmt` or `clippy` for now, or try to cleanup unused functions for now.
