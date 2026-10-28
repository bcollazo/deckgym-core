use crate::{models::EnergyType, State};

/// Genome Converter Y: the Benched [P] Pokémon holding [P] Energy, as `(bench index, [P] count)`.
/// Empty unless the Active Spot holds a Mega Mewtwo Y ex.
pub fn genome_converter_y_sources(state: &State, player: usize) -> Vec<(usize, u32)> {
    let active_is_mega_mewtwo_y = state
        .maybe_get_active(player)
        .is_some_and(|active| active.get_name() == "Mega Mewtwo Y ex");
    if !active_is_mega_mewtwo_y {
        return vec![];
    }
    state
        .enumerate_bench_pokemon(player)
        .filter(|(_, x)| x.is_type(EnergyType::Psychic))
        .map(|(i, x)| {
            let count = x
                .attached_energy
                .iter()
                .filter(|e| **e == EnergyType::Psychic)
                .count() as u32;
            (i, count)
        })
        .filter(|(_, count)| *count > 0)
        .collect()
}
