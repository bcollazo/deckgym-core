import pytest
from pathlib import Path
import deckgym

deck_dir = Path(__file__).parent.parent.parent.parent / "example_decks"


@pytest.mark.parametrize(
    "deck_path",
    [
        "venusaur-exeggutor.txt",
        "weezing-arbok.txt",
        "mewtwoex.txt",
        "fire.txt",
    ],
)
def test_deck_loading(deck_path):
    path = deck_dir / deck_path
    if not path.exists():
        pytest.skip(f"Deck file not found: {path}")
    deck = deckgym.Deck(str(path))
    assert deck.card_count > 0


def test_player_types():
    player_types = deckgym.get_player_types()
    assert isinstance(player_types, dict)
    assert "r" in player_types


def test_single_game():
    deck_a = deck_dir / "venusaur-exeggutor.txt"
    deck_b = deck_dir / "weezing-arbok.txt"
    if not (deck_a.exists() and deck_b.exists()):
        pytest.skip("Required deck files not found")
    game = deckgym.Game(str(deck_a), str(deck_b), ["r", "r"], seed=42)
    state = game.get_state()
    for _ in range(5):
        if state.is_game_over():
            break
        game.play_tick()
        state = game.get_state()
    result = game.play()
    assert result is not None
    assert hasattr(result, "winner")


def test_simulation():
    deck_a = deck_dir / "venusaur-exeggutor.txt"
    deck_b = deck_dir / "weezing-arbok.txt"
    if not (deck_a.exists() and deck_b.exists()):
        pytest.skip("Required deck files not found")
    results = deckgym.simulate(
        str(deck_a), str(deck_b), players=["r", "r"], num_simulations=10
    )
    assert results.total_games == 10
    assert results.player_a_wins + results.player_b_wins + results.ties == 10


def test_performance():
    deck_a = deck_dir / "venusaur-exeggutor.txt"
    deck_b = deck_dir / "weezing-arbok.txt"
    if not (deck_a.exists() and deck_b.exists()):
        pytest.skip("Required deck files not found")
    num_simulations = 100
    results = deckgym.simulate(
        str(deck_a), str(deck_b), players=["r", "r"], num_simulations=num_simulations
    )
    assert results.total_games == num_simulations
    assert (
        results.player_a_wins + results.player_b_wins + results.ties == num_simulations
    )


def test_different_decks():
    available = [f for f in deck_dir.glob("*.txt")]
    if len(available) < 2:
        pytest.skip("Need at least 2 deck files")
    deck_a, deck_b = available[:2]
    results = deckgym.simulate(
        str(deck_a), str(deck_b), players=["r", "r"], num_simulations=5
    )
    assert results.total_games == 5
    assert results.player_a_wins + results.player_b_wins + results.ties == 5
