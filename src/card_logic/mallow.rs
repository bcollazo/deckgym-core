use crate::State;

/// Names of Pokémon that Mallow can heal.
const MALLOW_TARGETS: [&str; 2] = ["Shiinotic", "Tsareena"];

/// Mallow: "Heal all damage from 1 of your Shiinotic or Tsareena. If you do, discard all Energy
/// from that Pokémon."
///
/// Only damaged Pokémon qualify: healing an undamaged one would heal nothing, and the discard is
/// gated on the heal actually happening ("If you do").
pub fn mallow_targets(state: &State, player: usize) -> Vec<usize> {
    state
        .enumerate_in_play_pokemon(player)
        .filter(|(_, pokemon)| pokemon.is_damaged())
        .filter(|(_, pokemon)| MALLOW_TARGETS.contains(&pokemon.get_name().as_str()))
        .map(|(in_play_idx, _)| in_play_idx)
        .collect()
}
