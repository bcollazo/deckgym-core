---
name: Migrate AbilityId Cards
description: Migrates ability implementations from the old approach (using AbilityId) to use new approach (AbilityMechanic enum)
---

The codebase is in a dirty state, don't try to eliminate compilation warnings, or apply clippy suggestions.

- Read the `models` module and the `state` module.
- Find the `AbilityId` to migrate in the match statement in `apply_abilities_action.rs`
- Search for the card information with the following script (e.g. AbilityId::A1177Weezing):

  ```bash
  cargo run --bin search "Weezing"
  ```

- Find the ability effect text from the search JSON output.
- Search for that effect text in `effect_ability_mechanic_map.rs`.
- Decide if we should introduce a new `AbilityMechanic` variant in `src/actions/abilities/mechanic.rs` or re-use or generalize an existing one. Try to re-use existing ones first.
- Uncomment all the effect lines in `effect_ability_mechanic_map.rs` that just require different parameters on the decided AbilityMechanic variant, and map to the correct AbilityMechanic variant instance.
- Implement the mechanic logic in `forecast_ability_by_mechanic` in `apply_abilities_action.rs`.
  - Identify how it was implemented before. Refactor the old function to be usable with the new structure.
  - Keep the code as a simple one-liner in the match statement by using helper functions
- Implement the move generation logic in `can_use_ability_by_mechanic` in `move_generation_abilities.rs`.
  - Identify how it was checked before in `can_use_ability`. Replicate the same condition.
- Remove the old match arms for the migrated AbilityId from both `forecast_ability` and `can_use_ability`. Add an `unreachable!("Handled by AbilityMechanic")` arm for the enum variant instead.
- Remove the `ABILITY_ID_MAP` entries in `ability_ids.rs` for all card IDs that used the migrated AbilityId (keep the enum variant itself).
- DO NOT run `cargo fmt` or `clippy` for now, or try to cleanup unused functions for now.
