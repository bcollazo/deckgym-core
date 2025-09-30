---
mode: agent
---

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
- Make sure to run `cargo clippy --fix --allow-dirty -- -D warnings` and `cargo fmt` to format the code.
