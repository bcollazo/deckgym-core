#!/usr/bin/env python3
"""
Example script showing how to load and process exported simulation data from deckgym.

Usage:
    python examples/load_exported_data.py /tmp/exported_data
"""

import json
import sys
from pathlib import Path
from typing import Dict, List, Any


def load_game_data(game_folder: Path) -> List[Dict[str, Any]]:
    """Load all ply data for a single game."""
    ply_files = sorted(game_folder.glob("ply_*.json"))

    game_data = []
    for ply_file in ply_files:
        with open(ply_file, 'r') as f:
            data = json.load(f)
            game_data.append(data)

    return game_data


def extract_features(state: Dict[str, Any]) -> Dict[str, Any]:
    """
    Extract features from a game state for ML model training.

    This is a simple example - you can expand this to extract whatever
    features are relevant for your model.
    """
    features = {
        # Basic game state
        'turn_count': state['turn_count'],
        'current_player': state['current_player'],
        'points': state['points'],

        # Hand sizes
        'player_0_hand_size': len(state['hands'][0]),
        'player_1_hand_size': len(state['hands'][1]),

        # Deck sizes
        'player_0_deck_size': len(state['decks'][0]['cards']),
        'player_1_deck_size': len(state['decks'][1]['cards']),

        # In-play pokemon count
        'player_0_pokemon_count': sum(1 for p in state['in_play_pokemon'][0] if p is not None),
        'player_1_pokemon_count': sum(1 for p in state['in_play_pokemon'][1] if p is not None),

        # Current energy available
        'current_energy': state['current_energy'],

        # Turn flags
        'has_played_support': state['has_played_support'],
        'has_retreated': state['has_retreated'],
    }

    return features


def encode_action(action: Dict[str, Any]) -> str:
    """
    Encode an action into a string representation.

    You could also convert this to a numerical encoding for your model.
    """
    simple_action = action['action']

    # Get the action type (key of the dict)
    action_type = list(simple_action.keys())[0] if simple_action else 'Unknown'

    return f"{action_type}"


def process_exported_data(data_folder: Path):
    """Process all exported data and print statistics."""
    game_folders = [f for f in data_folder.iterdir() if f.is_dir()]

    print(f"Found {len(game_folders)} games in {data_folder}")
    print()

    total_plys = 0

    for game_folder in game_folders:
        game_id = game_folder.name
        game_data = load_game_data(game_folder)

        print(f"Game {game_id}:")
        print(f"  Total plys: {len(game_data)}")

        total_plys += len(game_data)

        # Show first ply as an example
        if game_data:
            first_ply = game_data[0]
            features = extract_features(first_ply['state'])
            action = encode_action(first_ply['chosen_action'])

            print(f"  First ply features: {features}")
            print(f"  First ply action: {action}")
            print(f"  Number of playable actions: {len(first_ply['playable_actions'])}")

        print()

    print(f"Total plys across all games: {total_plys}")
    print()
    print("You can now use this data to:")
    print("  1. Build feature vectors from the state")
    print("  2. Train a policy network to predict actions")
    print("  3. Train a value network to estimate game outcomes")
    print("  4. Analyze gameplay patterns and statistics")


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python examples/load_exported_data.py <data_folder>")
        print("Example: python examples/load_exported_data.py /tmp/exported_data")
        sys.exit(1)

    data_folder = Path(sys.argv[1])

    if not data_folder.exists():
        print(f"Error: Folder {data_folder} does not exist")
        sys.exit(1)

    process_exported_data(data_folder)
