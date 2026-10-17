use crate::State;

/// Name of the attack the Psychic Supporter requires in the Active Spot.
const PSYCHIC_ATTACK: &str = "Psychic";

/// Psychic (Supporter) can only be used if the player's Active Pokémon has the Psychic attack.
pub fn active_has_psychic_attack(state: &State, player: usize) -> bool {
    state.maybe_get_active(player).is_some_and(|active| {
        active
            .card
            .get_attacks()
            .iter()
            .any(|attack| attack.title == PSYCHIC_ATTACK)
    })
}

/// The opponent's Benched Pokémon that have Energy to move to their Active Pokémon.
pub fn psychic_energy_sources(state: &State, opponent: usize) -> Vec<usize> {
    if state.maybe_get_active(opponent).is_none() {
        return vec![];
    }
    state
        .enumerate_bench_pokemon(opponent)
        .filter(|(_, pokemon)| !pokemon.attached_energy.is_empty())
        .map(|(in_play_idx, _)| in_play_idx)
        .collect()
}
