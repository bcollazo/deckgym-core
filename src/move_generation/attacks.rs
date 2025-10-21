use crate::{actions::SimpleAction, effects::CardEffect, hooks::contains_energy, State};

pub(crate) fn generate_attack_actions(state: &State) -> Vec<SimpleAction> {
    let current_player = state.current_player;
    let mut actions = Vec::new();
    if let Some(active_pokemon) = &state.in_play_pokemon[current_player][0] {
        // Check if the Pokemon has the CannotAttack effect
        if active_pokemon.has_effect(CardEffect::CannotAttack) {
            return actions; // Return empty list if Pokemon cannot attack
        }

        active_pokemon
            .get_attacks()
            .iter()
            .enumerate()
            .for_each(|(i, attack)| {
                if contains_energy(
                    active_pokemon,
                    &attack.energy_required,
                    state,
                    current_player,
                ) {
                    actions.push(SimpleAction::Attack(i));
                }
            });
    }
    actions
}
