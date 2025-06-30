import deckgym
from pathlib import Path

# Specify the path to the deck files
deck_dir = Path(__file__).parent.parent.parent.parent / "example_decks"
deck_a = deck_dir / "venusaur-exeggutor.txt"
deck_b = deck_dir / "weezing-arbok.txt"

# Run the simulation
results = deckgym.simulate(
    str(deck_a),
    str(deck_b),
    players=["r", "r"],  # Both players are random
    num_simulations=10,
)

# Print the results
print(f"Total games: {results.total_games}")
print(f"Player A wins: {results.player_a_wins}")
print(f"Player B wins: {results.player_b_wins}")
print(f"Ties: {results.ties}")
