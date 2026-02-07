use core::panic;

use log::debug;
use rand::rngs::StdRng;

use crate::{
    ability_ids::AbilityId,
    actions::{
        abilities::AbilityMechanic,
        apply_action_helpers::{handle_damage, Mutation, Mutations, Probabilities},
        effect_ability_mechanic_map::ability_mechanic_from_effect,
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

    // Try AbilityMechanic first
    if let Some(mechanic) = pokemon
        .card
        .get_ability()
        .and_then(|a| ability_mechanic_from_effect(&a.effect))
    {
        return forecast_ability_by_mechanic(mechanic);
    }

    // Existing AbilityId fallback
    let ability_id = AbilityId::from_pokemon_id(&pokemon.get_id()[..])
        .expect("Pokemon should have ability implemented");
    match ability_id {
        AbilityId::A1020VictreebelFragranceTrap => doutcome(victreebel_ability),
        AbilityId::A1089GreninjaWaterShuriken => unreachable!("Handled by AbilityMechanic"),
        AbilityId::A1098MagnetonVoltCharge => doutcome_from_mutation(charge_magneton(in_play_idx)),
        AbilityId::A1123GengarExShadowySpellbind => {
            panic!("Shadowy Spellbind is a passive ability")
        }
        AbilityId::A1177Weezing => doutcome(weezing_ability),
        AbilityId::A1188PidgeotDriveOff => doutcome(pidgeot_drive_off),
        AbilityId::A1132Gardevoir => doutcome(gardevoir_ability),
        AbilityId::A1a006SerperiorJungleTotem => panic!("Serperior's ability is passive"),
        AbilityId::A1a046AerodactylExPrimevalLaw => panic!("Primeval Law is a passive ability"),
        AbilityId::A1a019VaporeonWashOut => doutcome(vaporeon_wash_out),
        AbilityId::A2a010LeafeonExForestBreath => doutcome(leafon_ex_ability),
        AbilityId::A2a069ShayminSkySupport => panic!("Sky Support is a passive ability"),
        AbilityId::A2a071Arceus => panic!("Arceus's ability cant be used on demand"),
        AbilityId::A2072DusknoirShadowVoid => {
            doutcome_from_mutation(dusknoir_shadow_void(in_play_idx))
        }
        AbilityId::A2078GiratinaLevitate => panic!("Levitate is a passive ability"),
        AbilityId::A2092LucarioFightingCoach => panic!("Fighting Coach is a passive ability"),
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
        AbilityId::B1073GreninjaExShiftingStream => unreachable!("Handled by AbilityMechanic"),
        AbilityId::B1121IndeedeeExWatchOver => doutcome(indeedee_ex_watch_over),
        AbilityId::B1157HydreigonRoarInUnison => {
            doutcome_from_mutation(charge_hydreigon_and_damage_self(in_play_idx))
        }
        AbilityId::B1172AegislashCursedMetal => panic!("Cursed Metal is a passive ability"),
        AbilityId::B1177GoomyStickyMembrane => panic!("Sticky Membrane is a passive ability"),
        AbilityId::B1184EeveeBoostedEvolution => panic!("Boosted Evolution is a passive ability"),
        AbilityId::PA037CresseliaExLunarPlumage => {
            panic!("Lunar Plumage is a passive ability")
        }
        AbilityId::A3a042NihilegoMorePoison => panic!("More Poison is a passive ability"),
        AbilityId::A1061PoliwrathCounterattack => {
            panic!("Counterattack is a passive ability")
        }
        AbilityId::A2a050CrobatCunningLink => doutcome(crobat_cunning_link),
        AbilityId::A4112UmbreonExDarkChase => doutcome(umbreon_dark_chase),
        AbilityId::B1160DragalgeExPoisonPoint => panic!("Poison Point is a passive ability"),
        AbilityId::B1a006AriadosTrapTerritory => panic!("Trap Territory is a passive ability"),
        AbilityId::B1a012CharmeleonIgnition => panic!("Ignition is triggered on evolve"),
        AbilityId::B1a018WartortleShellShield => panic!("Shell Shield is a passive ability"),
        AbilityId::B1a034ReuniclusInfiniteIncrease => {
            panic!("Infinite Increase is a passive ability")
        }
        AbilityId::B1a065FurfrouFurCoat => unreachable!("Handled by AbilityMechanic"),
        AbilityId::A4a032MisdreavusInfiltratingInspection => {
            unreachable!("Handled by AbilityMechanic")
        }
        AbilityId::A1007Butterfree | AbilityId::A2022ShayminFragrantFlowerGarden => {
            unreachable!("Handled by AbilityMechanic")
        }
    }
}

fn forecast_ability_by_mechanic(mechanic: &AbilityMechanic) -> (Probabilities, Mutations) {
    match mechanic {
        AbilityMechanic::HealAllYourPokemon { amount } => heal_all_your_pokemon(*amount),
        AbilityMechanic::DamageOneOpponentPokemon { amount } => damage_one_opponent(*amount),
        AbilityMechanic::SwitchActiveTypedWithBench { .. } => {
            switch_active_typed_with_bench_outcome()
        }
        AbilityMechanic::ReduceDamageFromAttacks { .. } => {
            panic!("ReduceDamageFromAttacks is a passive ability")
        }
        AbilityMechanic::StartTurnRandomPokemonToHand { .. } => {
            panic!("StartTurnRandomPokemonToHand is a passive ability")
        }
        AbilityMechanic::PreventFirstAttack => {
            panic!("PreventFirstAttack is a passive ability")
        }
        AbilityMechanic::ElectromagneticWall => {
            panic!("ElectromagneticWall is a passive ability")
        }
        AbilityMechanic::InfiltratingInspection => {
            panic!("InfiltratingInspection is triggered when played to bench")
        }
    }
}

fn heal_all_your_pokemon(amount: u32) -> (Probabilities, Mutations) {
    doutcome_from_mutation(Box::new(move |_rng, state, action| {
        for pokemon in state.in_play_pokemon[action.actor].iter_mut().flatten() {
            pokemon.heal(amount);
        }
    }))
}

fn damage_one_opponent(amount: u32) -> (Probabilities, Mutations) {
    doutcome_from_mutation(Box::new(move |_rng, state, action| {
        let SimpleAction::UseAbility {
            in_play_idx: attacking_idx,
        } = action.action
        else {
            panic!("Ability should be triggered by UseAbility action");
        };

        let opponent = (action.actor + 1) % 2;
        let possible_moves = state
            .enumerate_in_play_pokemon(opponent)
            .map(|(in_play_idx, _)| SimpleAction::ApplyDamage {
                attacking_ref: (action.actor, attacking_idx),
                targets: vec![(amount, opponent, in_play_idx)],
                is_from_active_attack: false,
            })
            .collect::<Vec<_>>();
        state
            .move_generation_stack
            .push((action.actor, possible_moves));
    }))
}

fn switch_active_typed_with_bench_outcome() -> (Probabilities, Mutations) {
    doutcome_from_mutation(Box::new(move |_rng, state, action| {
        let acting_player = action.actor;
        let choices = state
            .enumerate_bench_pokemon(acting_player)
            .map(|(in_play_idx, _)| SimpleAction::Activate {
                player: acting_player,
                in_play_idx,
            })
            .collect::<Vec<_>>();
        state.move_generation_stack.push((acting_player, choices));
    }))
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

fn pidgeot_drive_off(_: &mut StdRng, state: &mut State, action: &Action) {
    // Once during your turn, you may switch out your opponent's Active Pokémon to the Bench. (Your opponent chooses the new Active Pokémon.)
    debug!("Pidgeot's Drive Off: Forcing opponent to switch active");
    let opponent = (action.actor + 1) % 2;
    let mut choices = Vec::new();
    for (in_play_idx, _) in state.enumerate_bench_pokemon(opponent) {
        choices.push(SimpleAction::Activate {
            player: opponent,
            in_play_idx,
        });
    }
    if choices.is_empty() {
        return; // No benched pokemon to switch with
    }
    state.move_generation_stack.push((opponent, choices));
}

fn gardevoir_ability(_: &mut StdRng, state: &mut State, action: &Action) {
    // Once during your turn, you may attach a Psychic Energy to your Active Pokémon.
    debug!("Gardevoir's ability: Attaching Psychic Energy to active Pokemon");
    state.attach_energy_from_zone(action.actor, 0, EnergyType::Psychic, 1, false);
}

fn rising_road(index: usize) -> Mutation {
    Box::new(move |_, state, action| {
        // Once during your turn, if this Pokémon is on your Bench, you may switch it with your Active Pokémon.
        debug!("Solgaleo's ability: Switching with active Pokemon");
        let choices = vec![SimpleAction::Activate {
            player: action.actor,
            in_play_idx: index,
        }];
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
        .map(|(in_play_idx, _)| SimpleAction::Activate {
            player: opponent_player,
            in_play_idx,
        })
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
        .map(|(in_play_idx, _)| SimpleAction::Activate {
            player: acting_player,
            in_play_idx,
        })
        .collect::<Vec<_>>();
    if choices.is_empty() {
        return;
    }
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

fn charge_magneton(in_play_idx: usize) -> Mutation {
    Box::new(move |_, state, action| {
        // Once during your turn, you may take a Lightning Energy from your Energy Zone and attach it to this Pokémon.
        debug!("Magneton's Volt Charge: Attaching 1 Lightning Energy to Magneton");
        state.attach_energy_from_zone(action.actor, in_play_idx, EnergyType::Lightning, 1, false);
    })
}

fn charge_giratina_and_end_turn(in_play_idx: usize) -> Mutation {
    Box::new(move |_, state, action| {
        // Once during your turn, you may take a Psychic Energy from your Energy Zone and attach it to this Pokémon. If you use this Ability, your turn ends.
        debug!("Giratina ex's ability: Attaching 1 Psychic Energy and ending turn");
        let attached =
            state.attach_energy_from_zone(action.actor, in_play_idx, EnergyType::Psychic, 1, false);

        // End the turn after using this ability
        if attached {
            state
                .move_generation_stack
                .push((action.actor, vec![SimpleAction::EndTurn]));
        }
    })
}

fn dusknoir_shadow_void(dusknoir_idx: usize) -> Mutation {
    Box::new(move |_, state, action| {
        let choices: Vec<SimpleAction> = state
            .enumerate_in_play_pokemon(action.actor)
            .filter(|(i, p)| p.is_damaged() && *i != dusknoir_idx)
            .map(|(i, _)| SimpleAction::MoveAllDamage {
                from: i,
                to: dusknoir_idx,
            })
            .collect();

        if !choices.is_empty() {
            state.move_generation_stack.push((action.actor, choices));
        }
    })
}

fn charge_hydreigon_and_damage_self(in_play_idx: usize) -> Mutation {
    Box::new(move |_, state, action| {
        // Once during your turn, you may take 2 [D] Energy from your Energy Zone and attach it to this Pokémon. If you do, do 30 damage to this Pokémon.
        debug!(
            "Hydreigon's Roar in Unison: Attaching 2 Darkness Energy and dealing 30 damage to self"
        );
        let attached = state.attach_energy_from_zone(
            action.actor,
            in_play_idx,
            EnergyType::Darkness,
            2,
            false,
        );

        // Use handle_damage to properly trigger KO checks
        if attached {
            handle_damage(
                state,
                (action.actor, in_play_idx),
                &[(30, action.actor, in_play_idx)],
                false,
                None,
            );
        }
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

    // Attach the Fire Energy to Flareon EX
    state.attach_energy_from_discard(action.actor, in_play_idx, &[EnergyType::Fire]);

    // Deal 20 damage to Flareon EX using handle_damage
    handle_damage(
        state,
        (action.actor, in_play_idx),
        &[(20, action.actor, in_play_idx)],
        false,
        None,
    );
}

fn indeedee_ex_watch_over(_: &mut StdRng, state: &mut State, action: &Action) {
    // Once during your turn, you may heal 20 damage from your Active Pokémon.
    debug!("Indeedee ex's Watch Over: Healing 20 damage from Active Pokemon");
    let active = state.get_active_mut(action.actor);
    active.heal(20);
}

fn crobat_cunning_link(_: &mut StdRng, state: &mut State, action: &Action) {
    // Once during your turn, if you have Arceus or Arceus ex in play, you may do 30 damage to your opponent's Active Pokémon.
    debug!("Crobat's Cunning Link: Dealing 30 damage to opponent's active Pokemon");
    let SimpleAction::UseAbility {
        in_play_idx: crobat_idx,
    } = action.action
    else {
        panic!("Crobat's ability should be triggered by UseAbility action");
    };

    let opponent = (action.actor + 1) % 2;
    let attacking_ref = (action.actor, crobat_idx);
    handle_damage(state, attacking_ref, &[(30, opponent, 0)], false, None);
}

fn umbreon_dark_chase(_: &mut StdRng, state: &mut State, action: &Action) {
    // Once during your turn, if this Pokémon is in the Active Spot, you may switch in 1 of your opponent's Benched Pokémon that has damage on it to the Active Spot.
    debug!("Umbreon ex's Dark Chase: Switching in opponent's damaged benched Pokemon");
    let acting_player = action.actor;
    let opponent_player = (acting_player + 1) % 2;
    let possible_moves = state
        .enumerate_bench_pokemon(opponent_player)
        .filter(|(_, pokemon)| pokemon.is_damaged())
        .map(|(in_play_idx, _)| SimpleAction::Activate {
            player: opponent_player,
            in_play_idx,
        })
        .collect::<Vec<_>>();
    state
        .move_generation_stack
        .push((acting_player, possible_moves));
}

fn vaporeon_wash_out(_: &mut StdRng, state: &mut State, action: &Action) {
    // As often as you like during your turn, you may move a [W] Energy from 1 of your Benched [W] Pokémon to your Active [W] Pokémon.
    debug!("Vaporeon's Wash Out: Moving Water Energy from benched Water Pokemon to active");
    let acting_player = action.actor;
    let possible_moves = state
        .enumerate_bench_pokemon(acting_player)
        .filter(|(_, pokemon)| {
            pokemon.card.get_type() == Some(EnergyType::Water)
                && pokemon.attached_energy.contains(&EnergyType::Water)
        })
        .map(|(in_play_idx, _)| SimpleAction::MoveEnergy {
            from_in_play_idx: in_play_idx,
            to_in_play_idx: 0, // Active spot
            energy_type: EnergyType::Water,
            amount: 1,
        })
        .collect::<Vec<_>>();
    if possible_moves.is_empty() {
        return; // No benched Water Pokémon with Water Energy
    }
    state
        .move_generation_stack
        .push((acting_player, possible_moves));
}
