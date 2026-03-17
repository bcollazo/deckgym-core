use crate::{models::EnergyType, State};

pub fn diantha_targets(state: &State, player: usize) -> Vec<usize> {
    state
        .enumerate_in_play_pokemon(player)
        .filter(|(_, pokemon)| pokemon.get_energy_type() == Some(EnergyType::Psychic))
        .filter(|(_, pokemon)| pokemon.is_damaged())
        .filter(|(_, pokemon)| {
            pokemon
                .attached_energy
                .iter()
                .filter(|e| **e == EnergyType::Psychic)
                .count()
                >= 2
        })
        .map(|(in_play_idx, _)| in_play_idx)
        .collect()
}
