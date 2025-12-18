---
name: Implementing Cards
description: Fill out the implementation of effects of different attacks, abilities, and trainer cards in this Pokemon TCG Pocket engine codebase.
---

# Implementing Cards

To implement cards, first read the `models` module and the `state` module. Cards
are not implemented if they are a Pokemon that is missing an Ability or Attack implementation,
or a Trainer card (be it a tool or a normal one) missig implementation.

If the user hasn't specified what card to implement, you can use the tool:
  ```bash
  cargo run --bin card_status
  ```
to see what cards are missing, and choose one. You can also that tool
to see what is missing from the specified card.

## Abilities

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


## Attack

- Get the details of the card with the attack you want to implement by using the following script:

  ```bash
  cargo run --bin search "Venusaur" --attack "Giant Bloom"
  ```

- Search for the effect text in the above JSON in the `effect_mechanic_map.rs` file.
- Decide if we should introduce a new Mechanic or re-use or generalize an existing one. Try to re-use existing ones first.
- Identify all the cards that have the same effect text template, and just differ by parameters.
- Uncomment all the `// map.insert("` lines that pertain to the mechanic, and add the correct value (an `Mechanic` enum variant with the corresponding parameters).
- Implement the mechanic logic in `forecast_effect_attack_by_mechanic` in `apply_attack_action.rs`.
  - Keep the code as a simple one-liner in the match statement by using helper functions
  - Review similar attacks in `apply_attack_action.rs` to ensure consistency in implementation.

## Tool

- Get the details of the tool card that you want to implement by using the following script:

  ```bash
  cargo run --bin search "Leaf Cape"
  ```

- Copy the ids of cards to implement (including full art versions) in the given JSON.
- In `tool_ids.rs` add the tool to the `ToolId` enum and the `TOOL_ID_MAP` map.
  - Keep the file ordered by set and number.
  - If the tool has attachment restrictions (e.g., only Grass pokémon), implement the `can_attach_to()` method to enforce these restrictions. This counts as the "move generation" for the tool.
- Implement the "on attach" logic in `on_attach_tool` in `hooks/core.rs`.
  - This is where you handle immediate effects when the tool is attached (e.g., +HP, stat modifications).
  - Review similar tools to ensure consistency in implementation.
  - Keep the `match tool_id` cases as one-liners when possible.
- Implement the "forecast action" logic in `forecast_trainer_action` in `apply_trainer_action.rs`.
  - Add the tool's CardId to the match branch that calls `doutcome(attach_tool)`.
  - Tools should be grouped together in a single match arm (e.g., `CardId::A2147GiantCape | CardId::A2148RockyHelmet | CardId::A3147LeafCape`).
- For tools with ongoing effects (not just on-attach):
  - Implement hooks in `hooks/core.rs` or other appropriate hook files.
  - Examples: Rocky Helmet deals damage when the holder is attacked.

## Trainer Cards

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

## Appendix

### Testing Your Implementation

After implementing a card test it like so:

1. Generate a temporary test deck for the card you implemented:
   ```bash
   cargo run --bin temp_deck_generator -- "Card ID" > test_deck.txt
   ```

2. Run 10,000 random games against all decks in the example_decks folder:
   ```bash
   cargo run simulate test_deck.txt example_decks/ --num 10000 --players r,r -v
   ```

   This will automatically distribute the 10,000 games evenly across all decks in the folder, testing your implementation against diverse strategies and card combinations.

3. Review the results to ensure the games complete without errors.

### Code Quality

Make sure to run `cargo clippy --fix --allow-dirty -- -D warnings` and `cargo fmt` to format the code. Also make sure `cargo test --features tui` still work.