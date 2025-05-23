use crate::{actions::SimpleAction, hooks::contains_energy, State};

pub(crate) fn generate_attack_actions(state: &State) -> Vec<SimpleAction> {
    if state.turn_count <= 1 {
        return vec![];
    }

    let current_player = state.current_player;
    let mut actions = Vec::new();
    if let Some(active_pokemon) = &state.in_play_pokemon[current_player][0] {
        active_pokemon
            .get_attacks()
            .iter()
            .enumerate()
            .for_each(|(i, attack)| {
                if contains_energy(
                    active_pokemon.attached_energy.as_slice(),
                    &attack.energy_required,
                ) {
                    actions.push(SimpleAction::Attack(i));
                }
            });
    }
    actions
}
