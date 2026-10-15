use crate::{
    models::{Card, EnergyType},
    State,
};

/// The `energy_type` Pokémon in `player`'s deck that evolve from the Pokémon at `in_play_idx`.
///
/// Used by cards that "put a random [X] Pokémon from your deck that evolves from that Pokémon
/// onto that Pokémon" (Quick-Grow Extract, Wallace): the player picks the target, and the
/// evolution is drawn uniformly from this list.
pub fn random_evolution_candidates(
    state: &State,
    player: usize,
    in_play_idx: usize,
    energy_type: EnergyType,
) -> Vec<Card> {
    let Some(pokemon) = &state.in_play_pokemon[player][in_play_idx] else {
        return vec![];
    };
    state.decks[player]
        .cards
        .iter()
        .filter(|deck_card| {
            matches!(deck_card, Card::Pokemon(deck_pokemon) if deck_pokemon.energy_type == energy_type)
                && pokemon.card.can_evolve_into(deck_card)
        })
        .cloned()
        .collect()
}
