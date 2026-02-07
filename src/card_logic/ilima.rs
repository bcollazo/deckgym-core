use crate::{models::EnergyType, State};

pub fn ilima_targets(state: &State, player: usize) -> Vec<usize> {
    state
        .enumerate_in_play_pokemon(player)
        .filter(|(_, pokemon)| {
            pokemon.is_damaged() && pokemon.get_energy_type() == Some(EnergyType::Colorless)
        })
        .map(|(in_play_idx, _)| in_play_idx)
        .collect()
}
