use core::panic;

use log::debug;
use rand::rngs::StdRng;

use crate::{
    ability_ids::AbilityId,
    actions::{
        apply_action_helpers::{handle_damage, Mutation, Mutations, Probabilities},
        mutations::{doutcome, doutcome_from_mutation},
        shared_mutations::pokemon_search_outcomes,
        Action, SimpleAction,
    },
    hooks::is_ultra_beast,
    models::EnergyType,
    State,
};

// This is a reducer of all actions relating to abilities.
pub(crate) fn forecast_ability(
    state: &State,
    action: &Action,
    in_play_idx: usize,
) -> (Probabilities, Mutations) {
    let pokemon = state.in_play_pokemon[action.actor][in_play_idx]
        .as_ref()
        .expect("Pokemon should be there if using ability");
    let ability_id = AbilityId::from_pokemon_id(&pokemon.get_id()[..])
        .expect("Pokemon should have ability implemented");
    match ability_id {
        AbilityId::A1007Butterfree => doutcome(butterfree_heal),
        AbilityId::A1020VictreebelFragranceTrap => doutcome(victreebel_ability),
        AbilityId::A1089GreninjaWaterShuriken => doutcome(greninja_shuriken),
        AbilityId::A1098MagnetonVoltCharge => doutcome_from_mutation(charge_magneton(in_play_idx)),
        AbilityId::A1123GengarExShadowySpellbind => {
            panic!("Shadowy Spellbind is a passive ability")
        }
        AbilityId::A1177Weezing => doutcome(weezing_ability),
        AbilityId::A1132Gardevoir => doutcome(gardevoir_ability),
        AbilityId::A1a006SerperiorJungleTotem => panic!("Serperior's ability is passive"),
        AbilityId::A2a010LeafeonExForestBreath => doutcome(leafon_ex_ability),
        AbilityId::A2a071Arceus => panic!("Arceus's ability cant be used on demand"),
        AbilityId::A2110DarkraiExNightmareAura => panic!("Darkrai ex's ability is passive"),
        AbilityId::A2b035GiratinaExBrokenSpaceBellow => {
            doutcome_from_mutation(charge_giratina_and_end_turn(in_play_idx))
        }
        AbilityId::A3066OricoricSafeguard => panic!("Safeguard is a passive ability"),
        AbilityId::A3122SolgaleoExRisingRoad => doutcome_from_mutation(rising_road(in_play_idx)),
        AbilityId::A3141KomalaComatose => panic!("Comatose is a passive ability"),
        AbilityId::A3a015LuxrayIntimidatingFang => panic!("Intimidating Fang is a passive ability"),
        AbilityId::A3a021ZeraoraThunderclapFlash => {
            panic!("Thunderclap Flash is a passive ability")
        }
        AbilityId::A3a027ShiinoticIlluminate => pokemon_search_outcomes(action.actor, state, false),
        AbilityId::A3a062CelesteelaUltraThrusters => doutcome(celesteela_ultra_thrusters),
        AbilityId::A3b009FlareonExCombust => doutcome(combust),
        AbilityId::A3b034SylveonExHappyRibbon => panic!("Happy Ribbon cant be used on demand"),
        AbilityId::A3b056EeveeExVeeveeVolve => panic!("Veevee 'volve is a passive ability"),
        AbilityId::A3b057SnorlaxExFullMouthManner => {
            panic!("Full-Mouth Manner is triggered at end of turn")
        }
        AbilityId::A4083EspeonExPsychicHealing => doutcome(espeon_ex_ability),
        AbilityId::A4a010EnteiExLegendaryPulse => {
            panic!("Legendary Pulse is triggered at end of turn")
        }
        AbilityId::A4a020SuicuneExLegendaryPulse => {
            panic!("Legendary Pulse is triggered at end of turn")
        }
        AbilityId::A4a022MiloticHealingRipples => {
            panic!("Healing Ripples is triggered on evolve")
        }
        AbilityId::A4a025RaikouExLegendaryPulse => {
            panic!("Legendary Pulse is triggered at end of turn")
        }
        AbilityId::B1073GreninjaExShiftingStream => doutcome(greninja_ex_shifting_stream),
        AbilityId::B1121IndeedeeExWatchOver => doutcome(indeedee_ex_watch_over),
        AbilityId::B1157HydreigonRoarInUnison => {
            doutcome_from_mutation(charge_hydreigon_and_damage_self(in_play_idx))
        }
        AbilityId::B1177GoomyStickyMembrane => panic!("Sticky Membrane is a passive ability"),
    }
}

fn butterfree_heal(_: &mut StdRng, state: &mut State, action: &Action) {
    debug!("Ability: Healing 20 damage from each Pokemon");
    for pokemon in state.in_play_pokemon[action.actor].iter_mut().flatten() {
        pokemon.heal(20);
    }
}

fn weezing_ability(_: &mut StdRng, state: &mut State, action: &Action) {
    // Your opponent's Active Pokémon is now Poisoned.
    debug!("Weezing's ability: Poisoning opponent's active Pokemon");
    let opponent = (action.actor + 1) % 2;
    let opponent_active = state.in_play_pokemon[opponent][0]
        .as_mut()
        .expect("Opponent should have active pokemon");
    opponent_active.poisoned = true;
}

fn gardevoir_ability(_: &mut StdRng, state: &mut State, action: &Action) {
    // Once during your turn, you may attach a Psychic Energy to your Active Pokémon.
    debug!("Gardevoir's ability: Attaching Psychic Energy to active Pokemon");
    let active = state.get_active_mut(action.actor);
    active.attach_energy(&EnergyType::Psychic, 1);
}

fn rising_road(index: usize) -> Mutation {
    Box::new(move |_, state, action| {
        // Once during your turn, if this Pokémon is on your Bench, you may switch it with your Active Pokémon.
        debug!("Solgaleo's ability: Switching with active Pokemon");
        let choices = vec![SimpleAction::Activate { in_play_idx: index }];
        state.move_generation_stack.push((action.actor, choices));
    })
}

fn victreebel_ability(_: &mut StdRng, state: &mut State, action: &Action) {
    // Switch in 1 of your opponent's Benched Basic Pokémon to the Active Spot.
    debug!("Victreebel's ability: Switching opponent's benched basic Pokemon to active");
    let acting_player = action.actor;
    let opponent_player = (acting_player + 1) % 2;
    let possible_moves = state
        .enumerate_bench_pokemon(opponent_player)
        .filter(|(_, pokemon)| pokemon.card.is_basic())
        .map(|(in_play_idx, _)| SimpleAction::Activate { in_play_idx })
        .collect::<Vec<_>>();
    state
        .move_generation_stack
        .push((acting_player, possible_moves));
}

fn celesteela_ultra_thrusters(_: &mut StdRng, state: &mut State, action: &Action) {
    // Once during your turn, you may switch your Active Ultra Beast with 1 of your Benched Ultra Beasts.
    debug!("Celesteela's Ultra Thrusters: Switching to a benched Ultra Beast");
    let acting_player = action.actor;
    let choices = state
        .enumerate_bench_pokemon(acting_player)
        .filter(|(_, pokemon)| is_ultra_beast(&pokemon.get_name()))
        .map(|(in_play_idx, _)| SimpleAction::Activate { in_play_idx })
        .collect::<Vec<_>>();
    if choices.is_empty() {
        return;
    }
    state.move_generation_stack.push((acting_player, choices));
}

fn greninja_ex_shifting_stream(_: &mut StdRng, state: &mut State, action: &Action) {
    // Once during your turn, you may switch your Active [W] Pokémon with 1 of your Benched Pokémon.
    debug!("Greninja ex's Shifting Stream: Switching active Water Pokemon with a benched Pokemon");
    let acting_player = action.actor;
    let choices = state
        .enumerate_bench_pokemon(acting_player)
        .map(|(in_play_idx, _)| SimpleAction::Activate { in_play_idx })
        .collect::<Vec<_>>();
    state.move_generation_stack.push((acting_player, choices));
}

fn leafon_ex_ability(_: &mut StdRng, state: &mut State, action: &Action) {
    // Take a Grass Energy from Energy Zone and attach it to 1 of your Grass Pokémon.
    debug!("Leafeon ex's ability: Attaching 1 Grass Energy to a Grass Pokemon");
    let possible_moves = state
        .enumerate_in_play_pokemon(action.actor)
        .filter(|(_, pokemon)| pokemon.card.get_type() == Some(EnergyType::Grass))
        .map(|(in_play_idx, _)| SimpleAction::Attach {
            attachments: vec![(1, EnergyType::Grass, in_play_idx)],
            is_turn_energy: false,
        })
        .collect::<Vec<_>>();
    state
        .move_generation_stack
        .push((action.actor, possible_moves));
}

fn greninja_shuriken(_: &mut StdRng, state: &mut State, action: &Action) {
    // Once during your turn, you may do 20 damage to 1 of your opponent's Pokémon.
    debug!("Greninja's ability: Dealing 20 damage to 1 opponent's Pokemon");
    let SimpleAction::UseAbility {
        in_play_idx: attacking_idx,
    } = action.action
    else {
        panic!("Greninja's ability should be triggered by UseAbility action");
    };

    let opponent = (action.actor + 1) % 2;
    let possible_moves = state
        .enumerate_in_play_pokemon(opponent)
        .map(|(in_play_idx, _)| SimpleAction::ApplyDamage {
            attacking_ref: (action.actor, attacking_idx),
            targets: vec![(20, opponent, in_play_idx)],
            is_from_active_attack: false,
        })
        .collect::<Vec<_>>();
    state
        .move_generation_stack
        .push((action.actor, possible_moves));
}

fn charge_magneton(in_play_idx: usize) -> Mutation {
    Box::new(move |_, state, action| {
        // Once during your turn, you may take a Lightning Energy from your Energy Zone and attach it to this Pokémon.
        debug!("Magneton's Volt Charge: Attaching 1 Lightning Energy to Magneton");
        let pokemon = state.in_play_pokemon[action.actor][in_play_idx]
            .as_mut()
            .expect("Pokemon should be there");
        pokemon.attach_energy(&EnergyType::Lightning, 1);
    })
}

fn charge_giratina_and_end_turn(in_play_idx: usize) -> Mutation {
    Box::new(move |_, state, action| {
        // Once during your turn, you may take a Psychic Energy from your Energy Zone and attach it to this Pokémon. If you use this Ability, your turn ends.
        debug!("Giratina ex's ability: Attaching 1 Psychic Energy and ending turn");
        let pokemon = state.in_play_pokemon[action.actor][in_play_idx]
            .as_mut()
            .expect("Pokemon should be there");
        pokemon.attach_energy(&EnergyType::Psychic, 1);

        // End the turn after using this ability
        state
            .move_generation_stack
            .push((action.actor, vec![SimpleAction::EndTurn]));
    })
}

fn charge_hydreigon_and_damage_self(in_play_idx: usize) -> Mutation {
    Box::new(move |_, state, action| {
        // Once during your turn, you may take 2 [D] Energy from your Energy Zone and attach it to this Pokémon. If you do, do 30 damage to this Pokémon.
        debug!(
            "Hydreigon's Roar in Unison: Attaching 2 Darkness Energy and dealing 30 damage to self"
        );
        let pokemon = state.in_play_pokemon[action.actor][in_play_idx]
            .as_mut()
            .expect("Pokemon should be there");
        pokemon.attach_energy(&EnergyType::Darkness, 2);

        // Use handle_damage to properly trigger KO checks
        handle_damage(
            state,
            (action.actor, in_play_idx),
            &[(30, action.actor, in_play_idx)],
            false,
        );
    })
}

fn espeon_ex_ability(_: &mut StdRng, state: &mut State, action: &Action) {
    // Once during your turn, if this Pokémon is in the Active Spot, you may heal 30 damage from 1 of your Pokémon.
    debug!("Espeon ex's Psychic Healing: Healing 30 damage from 1 of your Pokemon");
    let possible_moves = state
        .enumerate_in_play_pokemon(action.actor)
        .filter(|(_, pokemon)| pokemon.is_damaged())
        .map(|(in_play_idx, _)| SimpleAction::Heal {
            in_play_idx,
            amount: 30,
            cure_status: false,
        })
        .collect::<Vec<_>>();
    state
        .move_generation_stack
        .push((action.actor, possible_moves));
}

fn combust(_: &mut StdRng, state: &mut State, action: &Action) {
    // Once during your turn, you may attach a Fire Energy from your discard pile to this Pokémon. If you do, do 20 damage to this Pokémon.
    debug!("Flareon ex's Combust: Attaching 1 Fire Energy and dealing 20 damage to itself");
    let SimpleAction::UseAbility { in_play_idx } = action.action else {
        panic!("Flareon ex's ability should be triggered by UseAbility action");
    };

    // Remove Fire Energy from discard pile
    let fire_position = state.discard_energies[action.actor]
        .iter()
        .position(|e| *e == EnergyType::Fire)
        .expect("Should have Fire Energy in discard pile");
    state.discard_energies[action.actor].swap_remove(fire_position);

    // Attach the Fire Energy to Flareon EX
    let flareon = state.in_play_pokemon[action.actor][in_play_idx]
        .as_mut()
        .expect("Flareon ex should be there");
    flareon.attach_energy(&EnergyType::Fire, 1);

    // Deal 20 damage to Flareon EX using handle_damage
    handle_damage(
        state,
        (action.actor, in_play_idx),
        &[(20, action.actor, in_play_idx)],
        false,
    );
}

fn indeedee_ex_watch_over(_: &mut StdRng, state: &mut State, action: &Action) {
    // Once during your turn, you may heal 20 damage from your Active Pokémon.
    debug!("Indeedee ex's Watch Over: Healing 20 damage from Active Pokemon");
    let active = state.get_active_mut(action.actor);
    active.heal(20);
}
