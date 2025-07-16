use log::debug;

use crate::{ability_ids::AbilityId, types::EnergyType, State};

// This is a reducer of all actions relating to abilities.
pub(crate) fn apply_abilities_action(acting_player: usize, state: &mut State, index: usize) {
    let pokemon = state.in_play_pokemon[acting_player][index]
        .as_mut()
        .expect("Pokemon should be there if using ability");
    pokemon.ability_used = true;
    let ability_id = AbilityId::from_pokemon_id(&pokemon.get_id()[..])
        .expect("Pokemon should have ability implemented");
    match ability_id {
        AbilityId::A1007Butterfree => {
            // Once during your turn, you may heal 20 damage from each of your Pokemon.
            debug!("Butterfree's ability: Healing 20 damage from each Pokemon");
            for pokemon in state.in_play_pokemon[acting_player].iter_mut().flatten() {
                pokemon.heal(20);
            }
        }
        AbilityId::A1177Weezing => {
            // Once during your turn, if this Pokémon is in the Active Spot, you may make your opponent’s Active Pokémon Poisoned.
            debug!("Weezing's ability: Poisoning opponent's active Pokemon");
            let opponent = (acting_player + 1) % 2;
            let opponent_active = state.in_play_pokemon[opponent][0]
                .as_mut()
                .expect("Opponent should have active pokemon");
            opponent_active.poisoned = true;
        }
        AbilityId::A1132Gardevoir => {
            // Once during your turn, you may take 1 Psychic Energy from your Energy\n        Zone and attach it to the Psychic Pokemon in the Active Spot.
            debug!("Gardevoir's ability: Attaching 1 Psychic Energy to active Pokemon");
            let active = state.get_active_mut(acting_player);
            active.attach_energy(&EnergyType::Psychic, 1);
        }
        AbilityId::A2a071Arceus => {
            panic!("Arceus's ability cant be used");
        }
        AbilityId::A3122SolgaleoExRisingRoad => {
            // Once during your turn, if this Pokémon is on your Bench, you may switch it with your Active Pokémon.
            debug!("Solgaleo's ability: Switching with active Pokemon");
            // TODO: Use a model method that carries out the switch logic (e.g. cures status, etc.)
            state.in_play_pokemon[acting_player].swap(0, index);
        }
    }
}
