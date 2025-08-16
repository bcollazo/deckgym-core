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


def test_deck_properties():
    deck_path = deck_dir / "venusaur-exeggutor.txt"
    if not deck_path.exists():
        pytest.skip(f"Deck file not found: {deck_path}")
    deck = deckgym.Deck(str(deck_path))
    assert isinstance(deck.card_count, int)
    assert deck.card_count > 0
    assert repr(deck).startswith("PyDeck(")


def test_state_properties():
    deck_a = deck_dir / "venusaur-exeggutor.txt"
    deck_b = deck_dir / "weezing-arbok.txt"
    if not (deck_a.exists() and deck_b.exists()):
        pytest.skip("Required deck files not found")
    game = deckgym.Game(str(deck_a), str(deck_b), ["r", "r"], seed=42)
    state = game.get_state()
    # Check types and values
    assert isinstance(state.turn_count, int)
    assert state.turn_count >= 0
    assert isinstance(state.current_player, int)
    assert state.current_player in (0, 1)
    assert isinstance(state.points, (list, tuple))
    assert len(state.points) == 2
    assert all(isinstance(p, int) for p in state.points)
    # Check methods and their return values
    assert state.is_game_over() in (True, False)
    hand0 = state.get_hand(0)
    hand1 = state.get_hand(1)
    assert isinstance(hand0, list)
    assert isinstance(hand1, list)
    assert isinstance(state.get_deck_size(0), int)
    assert isinstance(state.get_deck_size(1), int)
    discard0 = state.get_discard_pile(0)
    discard1 = state.get_discard_pile(1)
    assert isinstance(discard0, list)
    assert isinstance(discard1, list)
    inplay0 = state.get_in_play_pokemon(0)
    inplay1 = state.get_in_play_pokemon(1)
    assert isinstance(inplay0, list)
    assert isinstance(inplay1, list)
    active0 = state.get_active_pokemon(0)
    active1 = state.get_active_pokemon(1)
    assert active0 is None or hasattr(active0, "remaining_hp")
    assert active1 is None or hasattr(active1, "remaining_hp")
    bench0 = state.get_bench_pokemon(0)
    bench1 = state.get_bench_pokemon(1)
    assert isinstance(bench0, list)
    assert isinstance(bench1, list)
    # get_pokemon_at_position returns None or a PlayedCard
    for pos in range(4):
        poke = state.get_pokemon_at_position(0, pos)
        assert poke is None or hasattr(poke, "remaining_hp")
    # get_remaining_hp returns int if pokemon exists
    for pos in range(4):
        try:
            hp = state.get_remaining_hp(0, pos)
            assert isinstance(hp, int)
            assert hp >= 0
        except Exception:
            pass  # allowed if no pokemon at this position
    # count_in_play_of_type returns int
    types = deckgym.get_player_types()
    for code in types:
        # Use PyEnergyType from a card if available
        for poke in inplay0:
            if poke and hasattr(poke, "energy_type") and poke.energy_type is not None:
                count = state.count_in_play_of_type(0, poke.energy_type)
                assert isinstance(count, int)
                assert count >= 0
                break
        break
    # enumerate_in_play_pokemon returns list of (int, PlayedCard)
    enum = state.enumerate_in_play_pokemon(0)
    assert isinstance(enum, list)
    for pos, poke in enum:
        assert isinstance(pos, int)
        assert hasattr(poke, "remaining_hp")
    # enumerate_bench_pokemon returns list of (int, PlayedCard)
    enum_bench = state.enumerate_bench_pokemon(0)
    assert isinstance(enum_bench, list)
    for pos, poke in enum_bench:
        assert isinstance(pos, int)
        assert hasattr(poke, "remaining_hp")
    # debug_string returns str
    assert isinstance(state.debug_string(), str)
    # get_hand_size, get_discard_pile_size, count_in_play_pokemon, has_active_pokemon, count_bench_pokemon
    assert isinstance(state.get_hand_size(0), int)
    assert isinstance(state.get_discard_pile_size(0), int)
    assert isinstance(state.count_in_play_pokemon(0), int)
    assert state.has_active_pokemon(0) in (True, False)
    assert isinstance(state.count_bench_pokemon(0), int)
    assert repr(state).startswith("PyState(")


def test_game_outcome_repr():
    from deckgym import Game

    deck_a = deck_dir / "venusaur-exeggutor.txt"
    deck_b = deck_dir / "weezing-arbok.txt"
    if not (deck_a.exists() and deck_b.exists()):
        pytest.skip("Required deck files not found")
    game = Game(str(deck_a), str(deck_b), ["r", "r"], seed=42)
    result = game.play()
    assert result is not None
    # Check winner and is_tie values
    assert hasattr(result, "winner")
    assert hasattr(result, "is_tie")
    # Only one of winner or is_tie should be set
    if result.is_tie:
        assert result.winner is None
    else:
        assert result.winner in (0, 1)
    r = repr(result)
    assert isinstance(r, str)
    # Check that repr contains the correct outcome
    if result.is_tie:
        assert "Tie" in r
    else:
        assert f"Win({result.winner})" in r


def test_simulation_results_repr():
    deck_a = deck_dir / "venusaur-exeggutor.txt"
    deck_b = deck_dir / "weezing-arbok.txt"
    if not (deck_a.exists() and deck_b.exists()):
        pytest.skip("Required deck files not found")
    results = deckgym.simulate(
        str(deck_a), str(deck_b), players=["r", "r"], num_simulations=3
    )
    r = repr(results)
    assert isinstance(r, str)
    # Check that repr contains expected values
    assert f"games={results.total_games}" in r
    assert f"A_wins={results.player_a_wins}" in r
    assert f"B_wins={results.player_b_wins}" in r
    assert f"ties={results.ties}" in r
    assert "SimulationResults(" in r
    assert hasattr(results, "player_a_win_rate")
    assert hasattr(results, "player_b_win_rate")
    assert hasattr(results, "tie_rate")


def test_player_types_dict():
    types = deckgym.get_player_types()
    assert isinstance(types, dict)
    expected = {
        "r": "Random Player",
        "aa": "Attach-Attack Player",
        "et": "End Turn Player",
        "h": "Human Player",
        "w": "Weighted Random Player",
        "m": "MCTS Player",
        "v": "Value Function Player",
        "e": "Expectiminimax Player",
    }
    for k, v in expected.items():
        assert k in types, f"Missing player type key: {k}"
        assert types[k] == v, f"Player type value for {k} is {types[k]}, expected {v}"
    # Also check all keys/values are str
    for k, v in types.items():
        assert isinstance(k, str)
        assert isinstance(v, str)
