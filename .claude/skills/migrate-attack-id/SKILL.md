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

- Search for the effect text in the above JSON in the `effect_mechanic_map.rs` file.
- Decide if we should introduce a new Mechanic or re-use or generalize an existing one. Try to re-use existing ones first.
- Uncomment the all the effect lines in `effect_mechanic_map.rs` that just require different parameters on the decided Mechanic variant, and map to the correct Mechanic variant instance.
- Implement the mechanic logic in `forecast_effect_attack_by_mechanic` in `apply_attack_action.rs`.
  - Identify how it was implemented before. Refactor the old function to be usable with the new structure. 
  - Keep the code as a simple one-liner in the match statement by using helper functions
  - Remove their old usage in the big commented out match statement, and any other old implementation for this attack.
- DO NOT run `cargo fmt` or `clippy` for now, or try to cleanup unused functions for now.