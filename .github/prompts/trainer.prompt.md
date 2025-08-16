---
mode: agent
---

- Get the details of the trainer card that you want to implement by using the following script:

  ```bash
  cargo run --bin search "Rare Candy"
  ```

- Copy the ids of cards to implement (including full art versions) in the given JSON.
- Implement the "move generation" logic.
  - In `move_generation_trainer.rs` implement the switch branch. Its often the case the Trainer/Support can always be played, so just add to this case in the switch.
- Implement the "apply action" logic.

  - This is the code that actually runs when the card is played.
  - Visit `apply_trainer_action.rs`.
  - Often its just "applying an effect" in the field (like Leaf).

    - If the turn is something that affects all pokemon in play for a given turn use
      the `.turn_effects` field in the state. You can use to for effects that apply to
      this turn, or a future one.
    - Some cards might be fairly unique and might need architectural changes to the engine. For cards with considerable custom logic,
      try to find a generalizing pattern that can be presented as a "hook" in the `hooks.rs`. The idea of `hooks.rs` is to try to encapsulate
      most custom logic that goes outside of the normal business logic. Also consider adding new
      pieces of state to the `State` struct if necessary.

  - Try to keep the `match trainer_id` cases as one-liners (using helper functions if necessary).

- Make sure to run `cargo clippy --fix --allow-dirty -- -D warnings` and `cargo fmt` to format the code.
