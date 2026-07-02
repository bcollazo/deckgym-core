use crate::{
    models::{Card, EnergyType},
    State,
};

/// Find all valid evolution candidates for Wallace.
///
/// Returns a vector of (in_play_index, evolution_card) tuples for all Water-type Pokémon in
/// play that:
/// - Are owned by the specified player
/// - Have a maximum HP of 50 or less
/// - Have at least one valid Water-type evolution in the player's deck
///
/// Unlike a normal evolution, Wallace has no restriction against evolving a Pokémon that was
/// played this same turn.
pub fn wallace_candidates(state: &State, player: usize) -> Vec<(usize, Card)> {
    let mut evolution_choices = vec![];

    for (in_play_idx, pokemon) in state.enumerate_in_play_pokemon(player) {
        let Card::Pokemon(pokemon_card) = &pokemon.card else {
            continue;
        };
        if pokemon.get_energy_type() != Some(EnergyType::Water) || pokemon_card.hp > 50 {
            continue;
        }

        for deck_card in state.decks[player].cards.iter() {
            if let Card::Pokemon(deck_pokemon) = deck_card {
                if deck_pokemon.energy_type == EnergyType::Water
                    && pokemon.card.can_evolve_into(deck_card)
                {
                    evolution_choices.push((in_play_idx, deck_card.clone()));
                }
            }
        }
    }

    evolution_choices
}
