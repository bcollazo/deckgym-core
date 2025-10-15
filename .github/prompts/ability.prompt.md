---
mode: agent
---

- Decide what ability you want to implement. You can use:
  ```bash
  cargo run --bin card_status
  ```
  to see what cards are missing abilities.
- Get the details of all the cards that have the ability you want to implement by using the following script:

  ```bash
  cargo run --bin search "Venusaur"
  ```

- Copy the ids of cards to implement (including full art versions) in the given JSON. Only choose the ones with the ability you want to implement.
- In `ability_ids.rs` add the ability to the `AbilityId` enum and the `ABILITY_ID_MAP` map.
  - Keep the file ordered by set and number.
- For abilities where the user selects _when_ to use it:
  - Implement the "move generation" logic. In `move_generation_abilities.rs` implement the `can_use_ability` case for this id.
    This is the code that checks if an ability can be used (e.g. Weezing's ability can only be used if weezing is in the active spot, and only once per turn).
    Review file for similar abilities and have them share code when possible.
    Keep the `match ability` cases as one-liners (using helper functions if necessary).
  - Implement the "apply action" logic. In `apply_abilities_action.rs` implement the case for this ability.
    This is the code that actually carries out the logic (e.g. in Weezing's ability, this is the code that would actually poison the opponent's active).
    Review file for similar abilities and have them share code when possible.
    Keep the `match ability_id` cases as one-liners (using helper functions if necessary).
- For others:
  - Some abilities are fairly unique and might need architectural changes to the engine. For cards with considerable custom logic,
    try to find a generalizing pattern that can be presented as a "hook" in the `hooks.rs`. The idea of `hooks.rs` is to try to encapsulate
    most custom logic that goes outside of the normal business logic.
- Make sure to run `cargo clippy --fix --allow-dirty -- -D warnings` and `cargo fmt` to format the code.
