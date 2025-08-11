use log::debug;

use crate::{
    ability_ids::AbilityId,
    actions::{
        apply_action_helpers::{Mutations, Probabilities},
        mutations::{ability_doutcome, ability_mutation},
        shared_mutations::pokemon_search_outcomes,
        Action, SimpleAction,
    },
    types::EnergyType,
    State,
};

// This is a reducer of all actions relating to abilities.
pub(crate) fn forecast_ability(
    state: &State,
    action: &Action,
    index: usize,
) -> (Probabilities, Mutations) {
    let pokemon = state.in_play_pokemon[action.actor][index]
        .as_ref()
        .expect("Pokemon should be there if using ability");
    let ability_id = AbilityId::from_pokemon_id(&pokemon.get_id()[..])
        .expect("Pokemon should have ability implemented");
    match ability_id {
        AbilityId::A1007Butterfree => heal_your_pokemon(20),
        AbilityId::A1177Weezing => poison_opponent_active_pokemon(),
        AbilityId::A1132Gardevoir => charge_active(EnergyType::Psychic),
        AbilityId::A2a071Arceus => panic!("Arceus's ability cant be used on demand"),
        AbilityId::A3122SolgaleoExRisingRoad => rising_road(index),
        AbilityId::A3a027ShiinoticIlluminate => pokemon_search_outcomes(action.actor, state, false),
        AbilityId::A3b034SylveonExHappyRibbon => panic!("Happy Ribbon cant be used on demand"),
    }
}

fn heal_your_pokemon(amount: u32) -> (Probabilities, Mutations) {
    ability_doutcome(ability_mutation(move |_, state, action| {
        // Once during your turn, you may heal 20 damage from each of your Pokemon.
        debug!("Ability: Healing 20 damage from each Pokemon");
        for pokemon in state.in_play_pokemon[action.actor].iter_mut().flatten() {
            pokemon.heal(amount);
        }
    }))
}

fn poison_opponent_active_pokemon() -> (Probabilities, Mutations) {
    ability_doutcome(ability_mutation(|_, state, action| {
        // Once during your turn, if this Pokémon is in the Active Spot, you may make your opponent’s Active Pokémon Poisoned.
        debug!("Ability: Poisoning opponent's active Pokemon");
        let opponent = (action.actor + 1) % 2;
        let opponent_active = state.in_play_pokemon[opponent][0]
            .as_mut()
            .expect("Opponent should have active pokemon");
        opponent_active.poisoned = true;
    }))
}

fn charge_active(energy_type: EnergyType) -> (Probabilities, Mutations) {
    ability_doutcome(ability_mutation(move |_, state, action| {
        // Once during your turn, you may attach a * Energy to your Active Pokémon.
        debug!("Ability: Attaching 1 Psychic Energy to active Pokemon");
        let active = state.get_active_mut(action.actor);
        active.attach_energy(&energy_type, 1);
    }))
}

fn rising_road(index: usize) -> (Probabilities, Mutations) {
    ability_doutcome(ability_mutation(move |_, state, action| {
        // Once during your turn, if this Pokémon is on your Bench, you may switch it with your Active Pokémon.
        debug!("Solgaleo's ability: Switching with active Pokemon");
        let choices = vec![SimpleAction::Activate { in_play_idx: index }];
        state.move_generation_stack.push((action.actor, choices));
    }))
}
