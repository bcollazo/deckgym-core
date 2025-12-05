// Collection of value functions for ExpectiMiniMaxPlayer
//
// Each value function evaluates a game state from a player's perspective
// and returns a score (higher is better for that player)

use log::trace;

use crate::card_logic::get_highest_evolutions;
use crate::hooks::energy_missing;
use crate::models::{Card, EnergyType, PlayedCard};
use crate::state::GameOutcome;
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
        + (opp.deck_size - my.deck_size) * 1.0
        + (-my.active_retreat_cost) * 1.0;
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
        + (-my.active_retreat_cost) * 1.0
        + (my.active_pokemon_online_score - opp.active_pokemon_online_score) * 0.0
        + (my.active_safety - opp.active_safety) * 0.0
        + (my.active_has_tool - opp.active_has_tool) * 0.0
        + (my.is_winner - opp.is_winner) * 0.0
        + my.turns_until_opponent_wins * 0.0
        + (my.online_pokemon_count - opp.online_pokemon_count) * 0.0
        + (my.energy_distance_to_online - opp.energy_distance_to_online) * 0.0;
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
    online_pokemon_count: f64,
    energy_distance_to_online: f64,
    active_pokemon_online_score: f64,
    active_safety: f64,
    active_has_tool: f64,
    is_winner: f64,
    turns_until_opponent_wins: f64,
}

/// Extract features for a single player
fn extract_features(state: &State, player: usize, active_factor: f64) -> Features {
    let points = state.points[player] as f64;
    let pokemon_value = calculate_pokemon_value(state, player, active_factor);
    let hand_size = state.hands[player].len() as f64;
    let deck_size = state.decks[player].cards.len() as f64;
    let active_retreat_cost = get_active_retreat_cost(state, player) as f64;
    let (online_pokemon_count, energy_distance_to_online) =
        calculate_online_metrics(state, player, active_factor);
    let active_pokemon_online_score = calculate_active_pokemon_online_score(state, player);
    let active_safety = calculate_active_safety(state, player);
    let active_has_tool = get_active_has_tool(state, player);
    let is_winner = check_is_winner(state, player);
    let turns_until_opponent_wins = calculate_turns_until_opponent_wins(state, player);

    Features {
        points,
        pokemon_value,
        hand_size,
        deck_size,
        active_retreat_cost,
        online_pokemon_count,
        energy_distance_to_online,
        active_pokemon_online_score,
        active_safety,
        active_has_tool,
        is_winner,
        turns_until_opponent_wins,
    }
}

fn get_active_retreat_cost(state: &State, player: usize) -> usize {
    state
        .maybe_get_active(player)
        .map(|card| card.card.get_retreat_cost().len())
        .unwrap_or(0)
}

/// Check if active pokemon has a tool attached
fn get_active_has_tool(state: &State, player: usize) -> f64 {
    state
        .maybe_get_active(player)
        .map(|card| if card.has_tool_attached() { 1.0 } else { 0.0 })
        .unwrap_or(0.0)
}

/// Check if the player has won the game
fn check_is_winner(state: &State, player: usize) -> f64 {
    match state.winner {
        Some(GameOutcome::Win(winner)) if winner == player => 1.0,
        _ => 0.0,
    }
}

/// Calculate expected turns until opponent wins
/// Uses opponent's active damage and simulates KOs until opponent reaches 3 points
fn calculate_turns_until_opponent_wins(state: &State, player: usize) -> f64 {
    let opponent = (player + 1) % 2;

    // Find the closest pokemon to being able to deal damage (by energy requirements)
    let best_threat = state
        .enumerate_in_play_pokemon(opponent)
        .filter_map(|(_, pokemon)| {
            let best_attack = pokemon
                .card
                .get_attacks()
                .iter()
                .filter(|atk| atk.fixed_damage > 0) // Only consider attacks that deal damage
                .map(|atk| {
                    let missing = energy_missing(pokemon, &atk.energy_required, state, opponent);
                    (atk.fixed_damage, missing.len())
                })
                .min_by_key(|(damage, missing)| (*missing, 0 - (*damage))); // Prioritize by missing energy, then by damage

            best_attack
        })
        .min_by_key(|(damage, missing)| (*missing, 0 - (*damage))); // Find pokemon with least missing energy
    let (max_damage, missing_energy) = match best_threat {
        Some((damage, missing)) => (damage as f64, missing),
        None => return 30.0, // No pokemon can deal damage
    };

    let mut total_turns = 0.0;
    let mut opp_points = state.points[opponent];

    // If the best threat still can't attack (missing energy), factor that into the calculation
    total_turns += missing_energy as f64;

    // Calculate turns to KO my active pokemon
    if let Some(my_active) = state.maybe_get_active(player) {
        let turns_to_ko = (my_active.remaining_hp as f64 / max_damage).ceil();
        total_turns += turns_to_ko;
        opp_points += my_active.card.get_knockout_points();
    }

    // Simulate KOing bench pokemon until opponent has 3+ points
    while opp_points < 3 {
        // Find the safest bench pokemon (highest hp / ko_points)
        let safest_bench = state
            .enumerate_bench_pokemon(player)
            .max_by_key(|(_, card)| {
                // if missing 1 point, just do by HP. if missing more than 1 point,
                // do by point yield hp / ko_points
                if opp_points == 2 {
                    card.remaining_hp
                } else {
                    let ko_points = card.card.get_knockout_points().max(1) as u32;
                    card.remaining_hp * 1000 / ko_points
                }
            });

        let Some((_, safest_pokemon)) = safest_bench else {
            break; // No more bench pokemon
        };

        let turns_to_ko = (safest_pokemon.remaining_hp as f64 / max_damage).ceil();
        total_turns += turns_to_ko;
        opp_points += safest_pokemon.card.get_knockout_points();
    }

    total_turns
}

/// Calculate online pokemon metrics: (count of online pokemon, total energy distance to online)
/// A pokemon is "online" if it can use at least one attack
/// Applies active_factor bonus to the active pokemon (position 0)
fn calculate_online_metrics(state: &State, player: usize, active_factor: f64) -> (f64, f64) {
    let (online_count, total_distance) = state
        .enumerate_in_play_pokemon(player)
        .map(|(pos, card)| {
            let min_distance = card
                .card
                .get_attacks()
                .iter()
                .map(|atk| {
                    let missing = energy_missing(card, &atk.energy_required, state, player);
                    missing.len()
                })
                .min()
                .unwrap_or(0);

            let position_factor = if pos == 0 { active_factor } else { 1.0 };

            if min_distance == 0 {
                (position_factor, 0.0) // Online pokemon with position bonus
            } else {
                (0.0, min_distance as f64 * position_factor) // Offline pokemon with weighted distance
            }
        })
        .fold((0.0, 0.0), |(count, dist), (c, d)| (count + c, dist + d));

    (online_count, total_distance)
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

/// Calculate active safety
/// Defined as remaining HP divided by knockout points
fn calculate_active_safety(state: &State, player: usize) -> f64 {
    let Some(active_pokemon) = state.maybe_get_active(player) else {
        return 0.0; // No safety if no active pokemon
    };

    let ko_points = active_pokemon.card.get_knockout_points() as f64;
    let hp = active_pokemon.remaining_hp as f64;

    hp / ko_points.max(1.0)
}

/// Calculate online score for active pokemon (0.0 to 1.0)
/// Returns 1.0 if the active pokemon has enough energy to use the highest attack
/// of its highest evolution available in deck+hand
fn calculate_active_pokemon_online_score(state: &State, player: usize) -> f64 {
    let Some(active_pokemon) = state.maybe_get_active(player) else {
        return 0.0;
    };

    // Get all cards available in deck + hand
    let mut available_cards: Vec<Card> = state.decks[player].cards.to_vec();
    available_cards.extend(state.hands[player].iter().cloned());

    // Find the highest evolution available
    let highest_evolutions = get_highest_evolutions(&active_pokemon.card, &available_cards);

    // If no evolutions found, use the current card
    let target_card = if highest_evolutions.is_empty() {
        &active_pokemon.card
    } else {
        // Use the first highest evolution (they should all be same stage)
        &highest_evolutions[0]
    };

    // Get the highest attack energy cost from the target card
    let most_expensive_attack_cost: Vec<EnergyType> = target_card
        .get_attacks()
        .iter()
        .map(|atk| atk.energy_required.clone())
        .max()
        .unwrap_or_default();

    if most_expensive_attack_cost.is_empty() {
        return 1.0; // No attack requirements, fully online
    }

    // Calculate how much energy we have vs need
    let missing = energy_missing(active_pokemon, &most_expensive_attack_cost, state, player);
    let total_needed = most_expensive_attack_cost.len() as f64;
    let have = total_needed - missing.len() as f64;

    // Return ratio (0.0 to 1.0)
    (have / total_needed).clamp(0.0, 1.0)
}
