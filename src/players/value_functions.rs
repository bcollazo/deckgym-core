// Collection of value functions for ExpectiMiniMaxPlayer
//
// Each value function evaluates a game state from a player's perspective
// and returns a score (higher is better for that player)

use crate::hooks::energy_missing;
use crate::models::{EnergyType, PlayedCard};
use crate::State;

// Re-export the baseline for convenience
pub use super::expectiminimax_player::baseline_value_function;

/// Helper function to calculate relevant energy for a Pokemon
fn get_relevant_energy(state: &State, player: usize, card: &PlayedCard) -> f64 {
    let most_expensive_attack_cost: Vec<EnergyType> = card
        .card
        .get_attacks()
        .iter()
        .map(|atk| atk.energy_required.clone())
        .max()
        .unwrap_or_default();

    let missing = energy_missing(card, &most_expensive_attack_cost, state, player);

    let total = most_expensive_attack_cost.len() as f64;
    total - missing.len() as f64
}

/// Variant 1: Emphasize active Pokemon more (3x instead of 2x)
pub fn aggressive_active_value_function(state: &State, myself: usize) -> f64 {
    let opponent = (myself + 1) % 2;
    let active_factor = 3.0; // Increased from 2.0

    // Points
    let points = state.points[myself] as f64;
    let opponent_points = state.points[opponent] as f64;

    // HP * Energy for my pokemon
    let my_value = state
        .enumerate_in_play_pokemon(myself)
        .map(|(pos, card)| {
            let relevant_energy = get_relevant_energy(state, myself, card);
            let hp_energy_product = card.remaining_hp as f64 * (relevant_energy + 1.0);
            if pos == 0 {
                hp_energy_product * active_factor
            } else {
                hp_energy_product
            }
        })
        .sum::<f64>();

    // HP * Energy for opponent's pokemon
    let opponent_value = state
        .enumerate_in_play_pokemon(opponent)
        .map(|(pos, card)| {
            let relevant_energy = get_relevant_energy(state, opponent, card);
            let hp_energy_product = card.remaining_hp as f64 * (relevant_energy + 1.0);
            if pos == 0 {
                hp_energy_product * active_factor
            } else {
                hp_energy_product
            }
        })
        .sum::<f64>();

    // Hand size advantage
    let hand_size = state.hands[myself].len() as f64;
    let opponent_hand_size = state.hands[opponent].len() as f64;

    (points - opponent_points) * 1000000.0
        + (my_value - opponent_value)
        + (hand_size - opponent_hand_size) * 1.0
}

/// Variant 2: Value hand cards more (2.0 weight instead of 1.0)
pub fn hand_value_function(state: &State, myself: usize) -> f64 {
    let opponent = (myself + 1) % 2;
    let active_factor = 2.0;
    let hand_weight = 2.0; // Increased from 1.0

    // Points
    let points = state.points[myself] as f64;
    let opponent_points = state.points[opponent] as f64;

    // HP * Energy for my pokemon
    let my_value = state
        .enumerate_in_play_pokemon(myself)
        .map(|(pos, card)| {
            let relevant_energy = get_relevant_energy(state, myself, card);
            let hp_energy_product = card.remaining_hp as f64 * (relevant_energy + 1.0);
            if pos == 0 {
                hp_energy_product * active_factor
            } else {
                hp_energy_product
            }
        })
        .sum::<f64>();

    // HP * Energy for opponent's pokemon
    let opponent_value = state
        .enumerate_in_play_pokemon(opponent)
        .map(|(pos, card)| {
            let relevant_energy = get_relevant_energy(state, opponent, card);
            let hp_energy_product = card.remaining_hp as f64 * (relevant_energy + 1.0);
            if pos == 0 {
                hp_energy_product * active_factor
            } else {
                hp_energy_product
            }
        })
        .sum::<f64>();

    // Hand size advantage (weighted more)
    let hand_size = state.hands[myself].len() as f64;
    let opponent_hand_size = state.hands[opponent].len() as f64;

    (points - opponent_points) * 1000000.0
        + (my_value - opponent_value)
        + (hand_size - opponent_hand_size) * hand_weight
}

/// Variant 3: Add bench depth as a new feature (more bench Pokemon = better board position)
pub fn bench_depth_value_function(state: &State, myself: usize) -> f64 {
    let opponent = (myself + 1) % 2;
    let active_factor = 2.0;

    // Points
    let points = state.points[myself] as f64;
    let opponent_points = state.points[opponent] as f64;

    // HP * Energy for my pokemon
    let my_value = state
        .enumerate_in_play_pokemon(myself)
        .map(|(pos, card)| {
            let relevant_energy = get_relevant_energy(state, myself, card);
            let hp_energy_product = card.remaining_hp as f64 * (relevant_energy + 1.0);
            if pos == 0 {
                hp_energy_product * active_factor
            } else {
                hp_energy_product
            }
        })
        .sum::<f64>();

    // HP * Energy for opponent's pokemon
    let opponent_value = state
        .enumerate_in_play_pokemon(opponent)
        .map(|(pos, card)| {
            let relevant_energy = get_relevant_energy(state, opponent, card);
            let hp_energy_product = card.remaining_hp as f64 * (relevant_energy + 1.0);
            if pos == 0 {
                hp_energy_product * active_factor
            } else {
                hp_energy_product
            }
        })
        .sum::<f64>();

    // Hand size advantage
    let hand_size = state.hands[myself].len() as f64;
    let opponent_hand_size = state.hands[opponent].len() as f64;

    // NEW FEATURE: Bench depth (number of benched Pokemon)
    // in_play_pokemon[player][0] is active, [1..4] are bench
    let my_bench_count = state.in_play_pokemon[myself][1..4]
        .iter()
        .filter(|p| p.is_some())
        .count() as f64;
    let opponent_bench_count = state.in_play_pokemon[opponent][1..4]
        .iter()
        .filter(|p| p.is_some())
        .count() as f64;
    let bench_advantage = (my_bench_count - opponent_bench_count) * 10.0;

    (points - opponent_points) * 1000000.0
        + (my_value - opponent_value)
        + (hand_size - opponent_hand_size) * 1.0
        + bench_advantage
}

/// Variant 4: Emphasize raw HP more (don't multiply by energy)
pub fn hp_focused_value_function(state: &State, myself: usize) -> f64 {
    let opponent = (myself + 1) % 2;
    let active_factor = 2.0;

    // Points
    let points = state.points[myself] as f64;
    let opponent_points = state.points[opponent] as f64;

    // Total HP for my pokemon (not multiplied by energy)
    let my_value = state
        .enumerate_in_play_pokemon(myself)
        .map(|(pos, card)| {
            let hp = card.remaining_hp as f64;
            if pos == 0 {
                hp * active_factor
            } else {
                hp
            }
        })
        .sum::<f64>();

    // Total HP for opponent's pokemon
    let opponent_value = state
        .enumerate_in_play_pokemon(opponent)
        .map(|(pos, card)| {
            let hp = card.remaining_hp as f64;
            if pos == 0 {
                hp * active_factor
            } else {
                hp
            }
        })
        .sum::<f64>();

    // Hand size advantage
    let hand_size = state.hands[myself].len() as f64;
    let opponent_hand_size = state.hands[opponent].len() as f64;

    (points - opponent_points) * 1000000.0
        + (my_value - opponent_value) * 2.0 // Weight HP more since not multiplied by energy
        + (hand_size - opponent_hand_size) * 1.0
}

/// Variant 5: Consider deck size (more cards left = better late game potential)
pub fn deck_awareness_value_function(state: &State, myself: usize) -> f64 {
    let opponent = (myself + 1) % 2;
    let active_factor = 2.0;

    // Points
    let points = state.points[myself] as f64;
    let opponent_points = state.points[opponent] as f64;

    // HP * Energy for my pokemon
    let my_value = state
        .enumerate_in_play_pokemon(myself)
        .map(|(pos, card)| {
            let relevant_energy = get_relevant_energy(state, myself, card);
            let hp_energy_product = card.remaining_hp as f64 * (relevant_energy + 1.0);
            if pos == 0 {
                hp_energy_product * active_factor
            } else {
                hp_energy_product
            }
        })
        .sum::<f64>();

    // HP * Energy for opponent's pokemon
    let opponent_value = state
        .enumerate_in_play_pokemon(opponent)
        .map(|(pos, card)| {
            let relevant_energy = get_relevant_energy(state, opponent, card);
            let hp_energy_product = card.remaining_hp as f64 * (relevant_energy + 1.0);
            if pos == 0 {
                hp_energy_product * active_factor
            } else {
                hp_energy_product
            }
        })
        .sum::<f64>();

    // Hand size advantage
    let hand_size = state.hands[myself].len() as f64;
    let opponent_hand_size = state.hands[opponent].len() as f64;

    // NEW FEATURE: Deck size advantage (more cards left in deck = better)
    let my_deck_size = state.decks[myself].cards.len() as f64;
    let opponent_deck_size = state.decks[opponent].cards.len() as f64;
    let deck_advantage = (my_deck_size - opponent_deck_size) * 0.5;

    (points - opponent_points) * 1000000.0
        + (my_value - opponent_value)
        + (hand_size - opponent_hand_size) * 1.0
        + deck_advantage
}
