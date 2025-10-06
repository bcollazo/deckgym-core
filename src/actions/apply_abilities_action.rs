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
        AbilityId::A1020VictreebelFragranceTrap => switch_opponent_basic_to_active(action.actor),
        AbilityId::A1089GreninjaWaterShuriken => damage_opponent_pokemon(action.actor, 20),
        AbilityId::A1177Weezing => poison_opponent_active_pokemon(),
        AbilityId::A1132Gardevoir => charge_active(EnergyType::Psychic),
        AbilityId::A1a006SerperiorJungleTotem => panic!("Serperior's ability is passive"),
        AbilityId::A2a010LeafeonExForestBreath => charge_grass_pokemon(action.actor),
        AbilityId::A2a071Arceus => panic!("Arceus's ability cant be used on demand"),
        AbilityId::A2b035GiratinaExBrokenSpaceBellow => charge_giratina_and_end_turn(index),
        AbilityId::A3122SolgaleoExRisingRoad => rising_road(index),
        AbilityId::A3a027ShiinoticIlluminate => pokemon_search_outcomes(action.actor, state, false),
        AbilityId::A3b034SylveonExHappyRibbon => panic!("Happy Ribbon cant be used on demand"),
        AbilityId::A4a020SuicuneExLegendaryPulse => {
            panic!("Legendary Pulse is triggered at end of turn")
        }
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

fn switch_opponent_basic_to_active(acting_player: usize) -> (Probabilities, Mutations) {
    ability_doutcome(ability_mutation(move |_, state, _| {
        // Switch in 1 of your opponent's Benched Basic Pokémon to the Active Spot.
        debug!("Victreebel's ability: Switching opponent's benched basic Pokemon to active");
        let opponent_player = (acting_player + 1) % 2;
        let possible_moves = state
            .enumerate_bench_pokemon(opponent_player)
            .filter(|(_, pokemon)| pokemon.card.is_basic())
            .map(|(in_play_idx, _)| SimpleAction::Activate { in_play_idx })
            .collect::<Vec<_>>();
        state
            .move_generation_stack
            .push((acting_player, possible_moves));
    }))
}

fn charge_grass_pokemon(acting_player: usize) -> (Probabilities, Mutations) {
    ability_doutcome(ability_mutation(move |_, state, _| {
        // Take a Grass Energy from Energy Zone and attach it to 1 of your Grass Pokémon.
        debug!("Leafeon ex's ability: Attaching 1 Grass Energy to a Grass Pokemon");
        let possible_moves = state
            .enumerate_in_play_pokemon(acting_player)
            .filter(|(_, pokemon)| pokemon.card.get_type() == Some(EnergyType::Grass))
            .map(|(in_play_idx, _)| SimpleAction::Attach {
                attachments: vec![(1, EnergyType::Grass, in_play_idx)],
                is_turn_energy: false,
            })
            .collect::<Vec<_>>();
        state
            .move_generation_stack
            .push((acting_player, possible_moves));
    }))
}

fn damage_opponent_pokemon(acting_player: usize, damage: u32) -> (Probabilities, Mutations) {
    ability_doutcome(ability_mutation(move |_, state, _| {
        // Once during your turn, you may do 20 damage to 1 of your opponent's Pokémon.
        debug!(
            "Greninja's ability: Dealing {} damage to 1 opponent's Pokemon",
            damage
        );
        let opponent = (acting_player + 1) % 2;
        let possible_moves = state
            .enumerate_in_play_pokemon(opponent)
            .map(|(in_play_idx, _)| SimpleAction::ApplyDamage {
                targets: vec![(damage, in_play_idx)],
            })
            .collect::<Vec<_>>();
        state
            .move_generation_stack
            .push((acting_player, possible_moves));
    }))
}

fn charge_giratina_and_end_turn(index: usize) -> (Probabilities, Mutations) {
    ability_doutcome(ability_mutation(move |_, state, action| {
        // Once during your turn, you may take a Psychic Energy from your Energy Zone and attach it to this Pokémon. If you use this Ability, your turn ends.
        debug!("Giratina ex's ability: Attaching 1 Psychic Energy and ending turn");
        let pokemon = state.in_play_pokemon[action.actor][index]
            .as_mut()
            .expect("Pokemon should be there");
        pokemon.attach_energy(&EnergyType::Psychic, 1);

        // End the turn after using this ability
        state
            .move_generation_stack
            .push((action.actor, vec![SimpleAction::EndTurn]));
    }))
}
