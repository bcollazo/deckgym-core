use crate::{
    models::{Card, EnergyType},
    State,
};

/// Find all valid evolution candidates for Quick-Grow Extract
///
/// Returns a vector of (in_play_index, evolution_card) tuples for all Grass-type
/// Pokémon in play that:
/// - Are owned by the specified player
/// - Were not played this turn
/// - Have at least one valid Grass-type evolution in the player's deck
///
/// This function is used by both:
/// - Move generation: to check if Quick-Grow Extract can be played
/// - Apply action: to create probabilistic evolution outcomes
///
/// # Arguments
/// * `state` - The current game state
/// * `player` - The player whose Pokémon to search
///
/// # Returns
/// A vector of tuples where each tuple contains:
/// - `usize`: The index of the in-play Pokémon that can evolve
/// - `Card`: The evolution card from the deck
pub fn quick_grow_extract_candidates(state: &State, player: usize) -> Vec<(usize, Card)> {
    let mut evolution_choices = vec![];

    for (in_play_idx, pokemon) in state.enumerate_in_play_pokemon(player) {
        if pokemon.get_energy_type() != Some(EnergyType::Grass) || pokemon.played_this_turn {
            continue;
        }

        // Find Grass evolutions in deck that evolve from this Pokemon
        for deck_card in state.decks[player].cards.iter() {
            if let Card::Pokemon(deck_pokemon) = deck_card {
                if deck_pokemon.energy_type == EnergyType::Grass
                    && pokemon.card.can_evolve_into(deck_card)
                {
                    evolution_choices.push((in_play_idx, deck_card.clone()));
                }
            }
        }
    }

    evolution_choices
}
