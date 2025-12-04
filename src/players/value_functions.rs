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
    let active_factor = 2.0; // Weight for active pokemon

    // Points
    let points = state.points[myself] as f64;
    let opponent_points = state.points[opponent] as f64;

    // HP * Energy for my pokemon
    let my_value = state
        .enumerate_in_play_pokemon(myself)
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

    let score = (points - opponent_points) * 1000000.0
        + (my_value - opponent_value)
        + (hand_size - opponent_hand_size) * 1.0;
    trace!("ValueFunction: {score} (points: {points}, opponent_points: {opponent_points}, my_value: {my_value}, opponent_value: {opponent_value}, hand_size: {hand_size}, opponent_hand_size: {opponent_hand_size})");
    score
}

/// A variant of the baseline value function
pub fn variant_value_function(state: &State, myself: usize) -> f64 {
    let opponent = (myself + 1) % 2;
    let active_factor = 2.0; // Weight for active pokemon

    // Points
    let points = state.points[myself] as f64;
    let opponent_points = state.points[opponent] as f64;

    // HP * Energy for my pokemon
    let my_value = state
        .enumerate_in_play_pokemon(myself)
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

    // NEW FEATURE: Deck size advantage (more cards left in deck = worse)
    let my_deck_size = state.decks[myself].cards.len() as f64;
    let opponent_deck_size = state.decks[opponent].cards.len() as f64;
    let deck_advantage = (opponent_deck_size - my_deck_size) * 0.5;

    let score = (points - opponent_points) * 1000000.0
        + (my_value - opponent_value)
        + (hand_size - opponent_hand_size) * 1.0
        + deck_advantage;
    trace!("ValueFunction: {score} (points: {points}, opponent_points: {opponent_points}, my_value: {my_value}, opponent_value: {opponent_value}, hand_size: {hand_size}, opponent_hand_size: {opponent_hand_size})");
    score
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
