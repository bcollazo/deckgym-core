# deckgym Python Bindings

Python bindings for the high-performance Pokémon TCG Pocket simulator, powered by Rust. This package allows you to run large-scale deck simulations from Python.

## Installation

You need Python 3.8+ and Rust toolchain installed.

Install dependencies and build the Python bindings using [uv](https://github.com/astral-sh/uv) and [maturin](https://github.com/PyO3/maturin):

```bash
# Install uv if you don't have it
curl -LsSf https://astral.sh/uv/install.sh | sh

# Sync dependencies (installs maturin and pytest)
uv sync
```

## Usage Example

Run a simulation between two decks:

```python
import deckgym
from pathlib import Path

deck_dir = Path(__file__).parent.parent.parent / "example_decks"
deck_a = deck_dir / "venusaur-exeggutor.txt"
deck_b = deck_dir / "weezing-arbok.txt"

results = deckgym.simulate(
    str(deck_a),
    str(deck_b),
    players=["r", "r"],  # Both players are random
    num_simulations=10,
)

print(f"Total games: {results.total_games}")
print(f"Player A wins: {results.player_a_wins}")
print(f"Player B wins: {results.player_b_wins}")
print(f"Ties: {results.ties}")
```

```bash
uv run python3 python/deckgym/examples/example_deckgym.py
```

See more examples in `python/deckgym/examples/`.

## Running Tests

Run tests using uv and pytest:

```bash
uv run pytest python/deckgym/tests/
```

## License

AGPL-3.0
