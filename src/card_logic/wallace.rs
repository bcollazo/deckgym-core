use crate::{
    models::{Card, EnergyType},
    State,
};

use super::random_evolution_candidates;

/// Find the in-play Pokémon Wallace can target.
///
/// Returns the in-play indices of the specified player's Water-type Pokémon that:
/// - Have a maximum HP of 50 or less
/// - Have at least one valid Water-type evolution in the player's deck
///
/// Unlike a normal evolution, Wallace has no restriction against evolving a Pokémon that was
/// played this same turn.
pub fn wallace_targets(state: &State, player: usize) -> Vec<usize> {
    state
        .enumerate_in_play_pokemon(player)
        .filter(|(in_play_idx, pokemon)| {
            matches!(&pokemon.card, Card::Pokemon(pokemon_card) if pokemon_card.hp <= 50)
                && pokemon.is_type(EnergyType::Water)
                && !random_evolution_candidates(state, player, *in_play_idx, EnergyType::Water)
                    .is_empty()
        })
        .map(|(in_play_idx, _)| in_play_idx)
        .collect()
}
