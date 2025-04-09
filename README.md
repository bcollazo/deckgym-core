<img src="./images/logo.svg" alt="Logo" width="100" height="100">

# deckgym-core: Pokémon TCG Pocket Simulator

![Card Implemented](https://img.shields.io/badge/Cards_Implemented-304_%2F_690_%2844.06%25%29-yellow)

**deckgym-core** is a high-performance Rust library designed for simulating Pokémon TCG Pocket games. It features a command-line interface (CLI) capable of running 10,000 simulations in approximately 3 seconds. This is the library that powers https://www.deckgym.com.

Its mission is to elevate the competitive TCG Pocket scene by helping players optimize their decks through large-scale simulations.

## Usage

The CLI runs simulations between two decks in DeckGym Format. To create these files, build your decks in https://www.deckgym.com/builder, select **Share** > **Copy as Text**, and save the content as a text file.

We already provide several example decks in the repo you can use to get started. For example, to face off a VenusaurEx-ExeggutorEx deck with a Weezing-Arbok deck 1,000 times, run:

```bash
cargo run simulate example_decks/venusaur-exeggutor.txt example_decks/weezing-arbok.txt --num 1000 -v
```

## Contributing

New to Open Source? See [CONTRIBUTING.md](./CONTRIBUTING.md).

The main contribution is to implement more cards, basically their attack and abilities logic. This makes the cards eligible for simulation and thus available for use in https://www.deckgym.com.

### Implement Attacks

- Copy the ids of cards to implement (including full art versions) in `database.json`.
- In `attack_ids.rs` add the attack to the `AttackId` enum and the `ATTACK_ID_MAP` map (with the correct index).
- Implement the attack logic in `forecast_effect_attack` in `apply_attack_action.rs`.

### Implement Abilities

- Copy the ids of cards to implement (including full art versions) in `database.json`.
- In `ability_ids.rs` add the ability to the `AbilityId` enum and the `ABILITY_ID_MAP` map.
- For abilities where the user selects _when_ to use it:
  - Implement the "move generation" logic. In `move_generation_abilities.rs` implement the `can_use_ability` case for this id. This is the code that checks if an ability can be used (e.g. Weezing's ability can only be used if weezing is in the active spot, and only once per turn).
  - Implement the "apply action" logic. In `apply_abilities_action.rs` implement the case for this ability. This is the code that actually carries out the logic (e.g. in Weezing's ability, this is the code that would actually poison the opponent's active).
- For others:
  - Some abilities are fairly unique and might need architectural changes to the engine. Propose away! If possible share the suggested solution in an Issue first to align on the archicture first!

### Implement Trainer / Support Cards

- Copy the ids of cards to implement (including full art versions) in `database.json`.
- Implement the "move generation" logic.
  - In `move_generation_trainer.rs` implement the switch branch. Its often the case the Trainer/Support can always be played, so just add to this case in the switch.
- Implement the "apply action" logic.
  - This is the code that actually runs when the card is played.
  - Visit `apply_trainer_action.rs`.
  - Often its just "applying an effect" in the field (like Leaf). For this, just
    add the card in the `.turn_effects` field in the state. Then implement the actual
    effect in `hooks.rs` or another place if necessary.

## Appendix: Useful Commands

Once you have Rust installed (see https://www.rust-lang.org/tools/install) you should be able to use the following commands from the root of the repo:

**Running Automated Test Suite**

```bash
cargo test
```

**Running Benchmarks**

```bash
cargo bench
```

**Running Main Script**

```bash
cargo run simulate example_decks/venusaur-exeggutor.txt example_decks/weezing-arbok.txt --num 1000 --players r,r
cargo run simulate example_decks/venusaur-exeggutor.txt example_decks/weezing-arbok.txt --num 1 --players r,r -vv
cargo run simulate example_decks/venusaur-exeggutor.txt example_decks/weezing-arbok.txt --num 1 --players r,r -vvvv
cargo run optimize example_decks/incomplete-chari.txt A2147,A2148 example_decks/ --num 10 --players e,e -v
```

**Generating database.rs**

Ensure database.json is up-to-date with latest data. Mock the `get_card_by_enum` in `database.rs` with a `_ => panic` so that
it compiles mid-way through the generation.

```bash
cargo run --bin card_enum_generator > tmp.rs && mv tmp.rs src/card_ids.rs && cargo fmt
cargo run --bin card_enum_generator -- --database > tmp.rs && mv tmp.rs src/database.rs && cargo fmt
```

**Profiling Main Script**

```
cargo install flamegraph
sudo cargo flamegraph --root --dev -- simulate example_decks/venusaur-exeggutor.txt example_decks/weezing-arbok.txt --num 1000 && open flamegraph.svg
```
