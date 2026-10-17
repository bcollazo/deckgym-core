use crate::State;

/// Names of Pokémon whose damage Acerola can move.
const ACEROLA_TARGETS: [&str; 2] = ["Palossand", "Mimikyu"];

/// Acerola: "Choose 1 of your Palossand or Mimikyu that has damage on it, and move 40 of its
/// damage to your opponent's Active Pokémon."
///
/// Only damaged Pokémon qualify — the card text says so explicitly, and an undamaged one has no
/// damage to move.
pub fn acerola_targets(state: &State, player: usize) -> Vec<usize> {
    state
        .enumerate_in_play_pokemon(player)
        .filter(|(_, pokemon)| pokemon.is_damaged())
        .filter(|(_, pokemon)| ACEROLA_TARGETS.contains(&pokemon.get_name().as_str()))
        .map(|(in_play_idx, _)| in_play_idx)
        .collect()
}
