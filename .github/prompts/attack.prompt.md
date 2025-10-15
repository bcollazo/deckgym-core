---
mode: agent
---

- Decide what attack you want to implement. You can use:
  ```bash
  cargo run --bin card_status
  ```
  to see what cards are missing attacks.
- Get the details of all the cards that have the attack you want to implement by using the following script:

  ```bash
  cargo run --bin search "Venusaur" --attack "Giant Bloom"
  ```

- Copy the ids of cards to implement (including full art versions) in the above JSON.
- In `attack_ids.rs` add the attack to the `AttackId` enum and the `ATTACK_ID_MAP` map (with the correct index).
  - Only implement attacks with effects.
  - Keep the file ordered by set and number.
- Review similar attacks in `apply_attack_action.rs` to ensure consistency in implementation.
- Implement the attack logic in `forecast_effect_attack` in `apply_attack_action.rs`.
  - Keep the code as a one-liner in the match statement, and implement the logic using a helper function.
- Make sure to run `cargo clippy --fix --allow-dirty -- -D warnings` and `cargo fmt` to format the code.
