use std::collections::BTreeSet;

use crate::{
    models::{Card, EnergyType},
    State,
};

/// Names of Pokémon Lt. Surge can rally Lightning Energy onto, when one of them is Active.
pub const LT_SURGE_TARGETS: [&str; 3] = ["Raichu", "Electrode", "Electabuzz"];

/// How many distinct Energy types are attached across all of `player`'s Pokémon in play.
/// Juggler: "You can use this card only if your Pokémon in play have 3 or more different types of
/// Energy attached."
pub fn distinct_attached_energy_types(state: &State, player: usize) -> usize {
    state
        .enumerate_in_play_pokemon(player)
        .flat_map(|(_, pokemon)| pokemon.attached_energy.iter().copied())
        .collect::<BTreeSet<EnergyType>>()
        .len()
}

/// Fishing Net: "Put a random Basic [W] Pokémon from your discard pile into your hand."
/// Returns the eligible cards (one entry per copy, so equally-likely random selection is correct).
pub fn fishing_net_candidates(state: &State, player: usize) -> Vec<Card> {
    state.discard_piles[player]
        .iter()
        .filter(|card| {
            matches!(card, Card::Pokemon(p) if p.stage == 0 && p.energy_type == EnergyType::Water)
        })
        .cloned()
        .collect()
}

/// Fisher: "Flip 3 coins. For each heads, a [W] Pokémon is chosen at random from your discard pile
/// and put into your hand." Unlike Fishing Net this is not restricted to Basics.
pub fn water_pokemon_in_discard(state: &State, player: usize) -> Vec<Card> {
    state.discard_piles[player]
        .iter()
        .filter(|card| matches!(card, Card::Pokemon(p) if p.energy_type == EnergyType::Water))
        .cloned()
        .collect()
}
