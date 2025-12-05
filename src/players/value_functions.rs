// Collection of value functions for ExpectiMiniMaxPlayer
//
// Each value function evaluates a game state from a player's perspective
// and returns a score (higher is better for that player)

use log::trace;

use crate::hooks::energy_missing;
use crate::models::{EnergyType, PlayedCard};
use crate::State;

pub fn baseline_value_function(state: &State, myself: usize) -> f64 {
    let opponent = (myself + 1) % 2;
    let (my, opp) = (
        extract_features(state, myself, 2.0),
        extract_features(state, opponent, 2.0),
    );
    let score = (my.points - opp.points) * 1000000.0
        + (my.pokemon_value - opp.pokemon_value) * 1.0
        + (my.hand_size - opp.hand_size) * 1.0
        + (opp.deck_size - my.deck_size) * 1.0;
    trace!("baseline_value_function: {score} (my: {my:?}, opp: {opp:?})");
    score
}

/// A variant of the baseline value function
pub fn variant_value_function(state: &State, myself: usize) -> f64 {
    let opponent = (myself + 1) % 2;
    let (my, opp) = (
        extract_features(state, myself, 2.0),
        extract_features(state, opponent, 2.0),
    );
    let score = (my.points - opp.points) * 1000000.0
        + (my.pokemon_value - opp.pokemon_value) * 1.0
        + (my.hand_size - opp.hand_size) * 1.0
        + (opp.deck_size - my.deck_size) * 1.0
        - my.active_retreat_cost * 1.0;
    trace!("variant_value_function: {score} (my: {my:?}, opp: {opp:?})");
    score
}

/// Features extracted from a player's game state
#[derive(Debug)]
struct Features {
    points: f64,
    pokemon_value: f64,
    hand_size: f64,
    deck_size: f64,
    active_retreat_cost: f64,
}

/// Extract features for a single player
fn extract_features(state: &State, player: usize, active_factor: f64) -> Features {
    let points = state.points[player] as f64;
    let pokemon_value = calculate_pokemon_value(state, player, active_factor);
    let hand_size = state.hands[player].len() as f64;
    let deck_size = state.decks[player].cards.len() as f64;
    let active_retreat_cost = get_active_retreat_cost(state, player) as f64;

    Features {
        points,
        pokemon_value,
        hand_size,
        deck_size,
        active_retreat_cost,
    }
}

fn get_active_retreat_cost(state: &State, player: usize) -> usize {
    state
        .maybe_get_active(player)
        .map(|card| card.card.get_retreat_cost().len())
        .unwrap_or(0)
}

/// Calculate total pokemon value (HP * Energy) for a player
fn calculate_pokemon_value(state: &State, player: usize, active_factor: f64) -> f64 {
    state
        .enumerate_in_play_pokemon(player)
        .map(|(pos, card)| {
            let relevant_energy = get_relevant_energy(state, player, card);
            let hp_energy_product = card.remaining_hp as f64 * (relevant_energy + 1.0);
            if pos == 0 {
                hp_energy_product * active_factor
            } else {
                hp_energy_product
            }
        })
        .sum()
}

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
