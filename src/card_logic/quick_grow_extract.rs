use crate::{models::EnergyType, State};

use super::random_evolution_candidates;

/// Find the in-play Pokémon Quick-Grow Extract can target.
///
/// Returns the in-play indices of the specified player's Grass-type Pokémon that:
/// - Were not played this turn
/// - Have at least one valid Grass-type evolution in the player's deck
///
/// Used by move generation (to check if Quick-Grow Extract can be played) and by the apply
/// action (to offer the player the choice of target).
pub fn quick_grow_extract_targets(state: &State, player: usize) -> Vec<usize> {
    state
        .enumerate_in_play_pokemon(player)
        .filter(|(in_play_idx, pokemon)| {
            pokemon.is_type(EnergyType::Grass)
                && !pokemon.played_this_turn
                && !random_evolution_candidates(state, player, *in_play_idx, EnergyType::Grass)
                    .is_empty()
        })
        .map(|(in_play_idx, _)| in_play_idx)
        .collect()
}
