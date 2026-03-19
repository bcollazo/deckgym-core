use core::panic;

use log::debug;
use rand::rngs::StdRng;

use crate::{
    actions::{
        abilities::AbilityMechanic,
        apply_action_helpers::{handle_damage, Mutation},
        effect_ability_mechanic_map::ability_mechanic_from_effect,
        outcomes::Outcomes,
        shared_mutations::pokemon_search_outcomes,
        Action, SimpleAction,
    },
    effects::TurnEffect,
    hooks::is_ultra_beast,
    models::EnergyType,
    State,
};

// This is a reducer of all actions relating to abilities.
pub(crate) fn forecast_ability(state: &State, action: &Action, in_play_idx: usize) -> Outcomes {
    let pokemon = state.in_play_pokemon[action.actor][in_play_idx]
        .as_ref()
        .expect("Pokemon should be there if using ability");

    let mechanic = pokemon
        .card
        .get_ability()
        .and_then(|a| ability_mechanic_from_effect(&a.effect))
        .unwrap_or_else(|| {
            panic!("Ability not implemented for card: {}", pokemon.get_id())
        });
    forecast_ability_by_mechanic(mechanic, state, action, in_play_idx)
}

fn forecast_ability_by_mechanic(
    mechanic: &AbilityMechanic,
    state: &State,
    action: &Action,
    in_play_idx: usize,
) -> Outcomes {
    match mechanic {
        AbilityMechanic::HealAllYourPokemon { amount } => heal_all_your_pokemon(*amount),
        AbilityMechanic::HealOneYourPokemonExAndDiscardRandomEnergy { amount } => {
            heal_one_your_pokemon_ex_and_discard_random_energy(*amount)
        }
        AbilityMechanic::DamageOneOpponentPokemon { amount } => damage_one_opponent(*amount),
        AbilityMechanic::SwitchActiveTypedWithBench { .. } => {
            switch_active_typed_with_bench_outcome()
        }
        AbilityMechanic::AttachEnergyFromZoneToActiveTypedPokemon { energy_type } => {
            attach_energy_from_zone_to_active_typed_outcome(*energy_type)
        }
        AbilityMechanic::DiscardTopCardOpponentDeck => discard_top_card_opponent_deck(),
        AbilityMechanic::DiscardEnergyToIncreaseTypeDamage {
            discard_energy,
            attack_type,
            amount,
        } => discard_energy_to_increase_type_damage(*discard_energy, *attack_type, *amount),
        // ── Active (on-demand) abilities ──────────────────────────────────────
        AbilityMechanic::PoisonOpponentActive => Outcomes::single_fn(poison_opponent_active),
        AbilityMechanic::SwitchOutOpponentActive => Outcomes::single_fn(switch_out_opponent_active),
        AbilityMechanic::SwitchInOpponentBenchedBasic => {
            Outcomes::single_fn(switch_in_opponent_benched_basic)
        }
        AbilityMechanic::AttachEnergyFromZoneToSelf { energy_type } => {
            attach_energy_from_zone_to_self(*energy_type)
        }
        AbilityMechanic::AttachEnergyFromZoneToSelfEndTurn { energy_type } => {
            attach_energy_from_zone_to_self_end_turn(*energy_type)
        }
        AbilityMechanic::AttachMultipleEnergyFromZoneToSelfAndSelfDamage {
            energy_type,
            count,
            damage,
        } => attach_multiple_energy_from_zone_to_self_and_self_damage(*energy_type, *count, *damage),
        AbilityMechanic::AttachEnergyFromDiscardToSelfAndSelfDamage { energy_type, damage } => {
            attach_energy_from_discard_to_self_and_self_damage(*energy_type, *damage)
        }
        AbilityMechanic::HealActivePokemon { amount } => heal_active_pokemon(*amount),
        AbilityMechanic::HealOneOfYourPokemon { amount } => heal_one_of_your_pokemon(*amount),
        AbilityMechanic::SwitchInOpponentDamagedBenched => {
            Outcomes::single_fn(switch_in_opponent_damaged_benched)
        }
        AbilityMechanic::SwitchBenchedSelfToActive => {
            Outcomes::single(switch_benched_self_to_active(in_play_idx))
        }
        AbilityMechanic::DrawRandomPokemonFromDeck => {
            pokemon_search_outcomes(action.actor, state, false)
        }
        AbilityMechanic::MoveAllDamageToSelf => {
            Outcomes::single(move_all_damage_to_self(in_play_idx))
        }
        AbilityMechanic::DamageOpponentActiveIfArceusInPlay { amount } => {
            damage_opponent_active_if_arceus_in_play(*amount)
        }
        AbilityMechanic::AttachEnergyFromZoneToTypedPokemon { energy_type } => {
            attach_energy_from_zone_to_typed_pokemon(*energy_type)
        }
        AbilityMechanic::MoveEnergyFromBenchedTypedToActiveTyped { energy_type } => {
            move_energy_from_benched_typed_to_active_typed(*energy_type)
        }
        AbilityMechanic::SwitchUltraBeast => Outcomes::single_fn(switch_ultra_beast),
        // ── Passive (hook-driven) abilities ───────────────────────────────────
        AbilityMechanic::ReduceDamageFromAttacks { .. } => {
            panic!("ReduceDamageFromAttacks is a passive ability")
        }
        AbilityMechanic::IncreaseDamageWhenRemainingHpAtMost { .. } => {
            panic!("IncreaseDamageWhenRemainingHpAtMost is a passive ability")
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
        AbilityMechanic::CoinFlipToPreventDamage => {
            panic!("CoinFlipToPreventDamage is a passive ability")
        }
        AbilityMechanic::CheckupDamageToOpponentActive { .. } => {
            panic!("CheckupDamageToOpponentActive is a passive ability")
        }
        AbilityMechanic::BlockSupportCards => panic!("BlockSupportCards is a passive ability"),
        AbilityMechanic::GrassEnergyDoubling => panic!("GrassEnergyDoubling is a passive ability"),
        AbilityMechanic::BlockOpponentEvolution => {
            panic!("BlockOpponentEvolution is a passive ability")
        }
        AbilityMechanic::BenchReduceBasicRetreatCost => {
            panic!("BenchReduceBasicRetreatCost is a passive ability")
        }
        AbilityMechanic::NoRetreatCostWithEnergy => {
            panic!("NoRetreatCostWithEnergy is a passive ability")
        }
        AbilityMechanic::TypeDamageBoost { .. } => {
            panic!("TypeDamageBoost is a passive ability")
        }
        AbilityMechanic::DamageOpponentActiveOnEnergyAttachFromZone { .. } => {
            panic!("DamageOpponentActiveOnEnergyAttachFromZone is a passive ability")
        }
        AbilityMechanic::SafeguardFromEx => panic!("SafeguardFromEx is a passive ability"),
        AbilityMechanic::AsleepOnEnergyAttachFromZoneWhenActive => {
            panic!("AsleepOnEnergyAttachFromZoneWhenActive is a passive ability")
        }
        AbilityMechanic::ReduceOpponentAttackDamage { .. } => {
            panic!("ReduceOpponentAttackDamage is a passive ability")
        }
        AbilityMechanic::AttachEnergyFromZoneToSelfOnFirstTurn { .. } => {
            panic!("AttachEnergyFromZoneToSelfOnFirstTurn is triggered at end of first turn")
        }
        AbilityMechanic::MorePoisonDamage { .. } => {
            panic!("MorePoisonDamage is a passive ability")
        }
        AbilityMechanic::CounterattackOnDamage { .. } => {
            panic!("CounterattackOnDamage is a passive ability")
        }
        AbilityMechanic::PoisonAttackerOnDamage => {
            panic!("PoisonAttackerOnDamage is a passive ability")
        }
        AbilityMechanic::ImmuneToSpecialConditions => {
            panic!("ImmuneToSpecialConditions is a passive ability")
        }
        AbilityMechanic::OnEvolveDraw { .. } => {
            panic!("OnEvolveDraw is triggered on evolve")
        }
        AbilityMechanic::VeeveeVolve => panic!("VeeveeVolve is a passive ability"),
        AbilityMechanic::EndTurnHealSelf { .. } => {
            panic!("EndTurnHealSelf is triggered at end of turn")
        }
        AbilityMechanic::EndTurnDrawCard => {
            panic!("EndTurnDrawCard is triggered at end of turn")
        }
        AbilityMechanic::OnEvolveHealTypedPokemon { .. } => {
            panic!("OnEvolveHealTypedPokemon is triggered on evolve")
        }
        AbilityMechanic::MultiTypeDamageBoost { .. } => {
            panic!("MultiTypeDamageBoost is a passive ability")
        }
        AbilityMechanic::IncreaseOpponentAttackCost => {
            panic!("IncreaseOpponentAttackCost is a passive ability")
        }
        AbilityMechanic::BoostedEvolution => panic!("BoostedEvolution is a passive ability"),
        AbilityMechanic::IncreaseOpponentRetreatCost => {
            panic!("IncreaseOpponentRetreatCost is a passive ability")
        }
        AbilityMechanic::OnEvolveAttachEnergyFromZoneToActiveTypedPokemon { .. } => {
            panic!("OnEvolveAttachEnergyFromZoneToActiveTypedPokemon is triggered on evolve")
        }
        AbilityMechanic::BenchSafeguard => panic!("BenchSafeguard is a passive ability"),
        AbilityMechanic::HpBonusPerEnergy { .. } => {
            panic!("HpBonusPerEnergy is a passive ability")
        }
        AbilityMechanic::HealOnEnergyAttachFromZone { .. } => {
            panic!("HealOnEnergyAttachFromZone is a passive ability")
        }
    }
}

fn discard_energy_to_increase_type_damage(
    discard_energy: EnergyType,
    attack_type: EnergyType,
    amount: u32,
) -> Outcomes {
    Outcomes::single_fn(move |_rng, state, action| {
        let SimpleAction::UseAbility { in_play_idx } = action.action else {
            panic!("Ability should be triggered by UseAbility action");
        };
        state.discard_energy_from_in_play(action.actor, in_play_idx, &[discard_energy]);
        state.add_turn_effect(
            TurnEffect::IncreasedDamageForType {
                amount,
                energy_type: attack_type,
            },
            0,
        );
    })
}

fn heal_all_your_pokemon(amount: u32) -> Outcomes {
    Outcomes::single_fn(move |_rng, state, action| {
        for pokemon in state.in_play_pokemon[action.actor].iter_mut().flatten() {
            pokemon.heal(amount);
        }
    })
}

fn heal_one_your_pokemon_ex_and_discard_random_energy(amount: u32) -> Outcomes {
    Outcomes::single_fn(move |_rng, state, action| {
        let choices = state
            .enumerate_in_play_pokemon(action.actor)
            .filter(|(_, pokemon)| pokemon.card.is_ex())
            .filter(|(_, pokemon)| pokemon.is_damaged())
            .filter(|(_, pokemon)| !pokemon.attached_energy.is_empty())
            .map(
                |(in_play_idx, pokemon)| SimpleAction::HealAndDiscardEnergy {
                    in_play_idx,
                    heal_amount: amount,
                    // Simplification: use last attached energy instead of true random to avoid
                    // adding extra hidden-random branches to the move tree.
                    discard_energies: vec![*pokemon
                        .attached_energy
                        .last()
                        .expect("attached energy is not empty by filter")],
                },
            )
            .collect::<Vec<_>>();
        state.move_generation_stack.push((action.actor, choices));
    })
}

fn damage_one_opponent(amount: u32) -> Outcomes {
    Outcomes::single_fn(move |_rng, state, action| {
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
    })
}

fn switch_active_typed_with_bench_outcome() -> Outcomes {
    Outcomes::single_fn(move |_rng, state, action| {
        let acting_player = action.actor;
        let choices = state
            .enumerate_bench_pokemon(acting_player)
            .map(|(in_play_idx, _)| SimpleAction::Activate {
                player: acting_player,
                in_play_idx,
            })
            .collect::<Vec<_>>();
        state.move_generation_stack.push((acting_player, choices));
    })
}

fn attach_energy_from_zone_to_active_typed_outcome(energy_type: EnergyType) -> Outcomes {
    Outcomes::single_fn(move |_rng, state, action| {
        state.attach_energy_from_zone(action.actor, 0, energy_type, 1, false);
    })
}

fn discard_top_card_opponent_deck() -> Outcomes {
    Outcomes::single_fn(move |_rng, state, action| {
        let opponent = (action.actor + 1) % 2;
        if let Some(card) = state.decks[opponent].draw() {
            state.discard_piles[opponent].push(card);
        }
    })
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
    if possible_moves.is_empty() {
        return;
    }
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
        if let Some(pokemon) = &state.in_play_pokemon[action.actor][in_play_idx] {
            if attached && !pokemon.is_knocked_out() {
                state
                    .move_generation_stack
                    .push((action.actor, vec![SimpleAction::EndTurn]));
            }
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

        // Use handle_damage to properly trigger KO checks, only if not already K.O.s (by say Jolteon ex)
        if let Some(pokemon) = &state.in_play_pokemon[action.actor][in_play_idx] {
            if attached && !pokemon.is_knocked_out() {
                handle_damage(
                    state,
                    (action.actor, in_play_idx),
                    &[(30, action.actor, in_play_idx)],
                    false,
                    None,
                );
            }
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
    if possible_moves.is_empty() {
        return;
    }
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

fn poison_opponent_active(_: &mut StdRng, state: &mut State, action: &Action) {
    let opponent = (action.actor + 1) % 2;
    let opponent_active = state.in_play_pokemon[opponent][0]
        .as_mut()
        .expect("Opponent should have active pokemon");
    opponent_active.poisoned = true;
}

fn switch_out_opponent_active(_: &mut StdRng, state: &mut State, action: &Action) {
    let opponent = (action.actor + 1) % 2;
    let mut choices = Vec::new();
    for (in_play_idx, _) in state.enumerate_bench_pokemon(opponent) {
        choices.push(SimpleAction::Activate {
            player: opponent,
            in_play_idx,
        });
    }
    if choices.is_empty() {
        return;
    }
    state.move_generation_stack.push((opponent, choices));
}

fn switch_in_opponent_benched_basic(_: &mut StdRng, state: &mut State, action: &Action) {
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
    if possible_moves.is_empty() {
        return;
    }
    state
        .move_generation_stack
        .push((acting_player, possible_moves));
}

fn switch_in_opponent_damaged_benched(_: &mut StdRng, state: &mut State, action: &Action) {
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

fn switch_ultra_beast(_: &mut StdRng, state: &mut State, action: &Action) {
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

fn attach_energy_from_zone_to_self(energy_type: EnergyType) -> Outcomes {
    Outcomes::single_fn(move |_rng, state, action| {
        let SimpleAction::UseAbility { in_play_idx } = action.action else {
            panic!("Ability should be triggered by UseAbility action");
        };
        state.attach_energy_from_zone(action.actor, in_play_idx, energy_type, 1, false);
    })
}

fn attach_energy_from_zone_to_self_end_turn(energy_type: EnergyType) -> Outcomes {
    Outcomes::single_fn(move |_rng, state, action| {
        let SimpleAction::UseAbility { in_play_idx } = action.action else {
            panic!("Ability should be triggered by UseAbility action");
        };
        let attached = state.attach_energy_from_zone(action.actor, in_play_idx, energy_type, 1, false);
        if let Some(pokemon) = &state.in_play_pokemon[action.actor][in_play_idx] {
            if attached && !pokemon.is_knocked_out() {
                state
                    .move_generation_stack
                    .push((action.actor, vec![SimpleAction::EndTurn]));
            }
        }
    })
}

fn attach_multiple_energy_from_zone_to_self_and_self_damage(
    energy_type: EnergyType,
    count: u32,
    damage: u32,
) -> Outcomes {
    Outcomes::single_fn(move |_rng, state, action| {
        let SimpleAction::UseAbility { in_play_idx } = action.action else {
            panic!("Ability should be triggered by UseAbility action");
        };
        let attached = state.attach_energy_from_zone(action.actor, in_play_idx, energy_type, count, false);
        if let Some(pokemon) = &state.in_play_pokemon[action.actor][in_play_idx] {
            if attached && !pokemon.is_knocked_out() {
                handle_damage(
                    state,
                    (action.actor, in_play_idx),
                    &[(damage, action.actor, in_play_idx)],
                    false,
                    None,
                );
            }
        }
    })
}

fn attach_energy_from_discard_to_self_and_self_damage(energy_type: EnergyType, damage: u32) -> Outcomes {
    Outcomes::single_fn(move |_rng, state, action| {
        let SimpleAction::UseAbility { in_play_idx } = action.action else {
            panic!("Ability should be triggered by UseAbility action");
        };
        state.attach_energy_from_discard(action.actor, in_play_idx, &[energy_type]);
        handle_damage(
            state,
            (action.actor, in_play_idx),
            &[(damage, action.actor, in_play_idx)],
            false,
            None,
        );
    })
}

fn heal_active_pokemon(amount: u32) -> Outcomes {
    Outcomes::single_fn(move |_rng, state, action| {
        let active = state.get_active_mut(action.actor);
        active.heal(amount);
    })
}

fn heal_one_of_your_pokemon(amount: u32) -> Outcomes {
    Outcomes::single_fn(move |_rng, state, action| {
        let possible_moves = state
            .enumerate_in_play_pokemon(action.actor)
            .filter(|(_, pokemon)| pokemon.is_damaged())
            .map(|(in_play_idx, _)| SimpleAction::Heal {
                in_play_idx,
                amount,
                cure_status: false,
            })
            .collect::<Vec<_>>();
        if possible_moves.is_empty() {
            return;
        }
        state
            .move_generation_stack
            .push((action.actor, possible_moves));
    })
}

fn switch_benched_self_to_active(index: usize) -> Mutation {
    Box::new(move |_, state, action| {
        let choices = vec![SimpleAction::Activate {
            player: action.actor,
            in_play_idx: index,
        }];
        state.move_generation_stack.push((action.actor, choices));
    })
}

fn move_all_damage_to_self(target_idx: usize) -> Mutation {
    Box::new(move |_, state, action| {
        let choices: Vec<SimpleAction> = state
            .enumerate_in_play_pokemon(action.actor)
            .filter(|(i, p)| p.is_damaged() && *i != target_idx)
            .map(|(i, _)| SimpleAction::MoveAllDamage {
                from: i,
                to: target_idx,
            })
            .collect();
        if !choices.is_empty() {
            state.move_generation_stack.push((action.actor, choices));
        }
    })
}

fn damage_opponent_active_if_arceus_in_play(amount: u32) -> Outcomes {
    Outcomes::single_fn(move |_rng, state, action| {
        let SimpleAction::UseAbility { in_play_idx: user_idx } = action.action else {
            panic!("Ability should be triggered by UseAbility action");
        };
        let opponent = (action.actor + 1) % 2;
        let attacking_ref = (action.actor, user_idx);
        handle_damage(state, attacking_ref, &[(amount, opponent, 0)], false, None);
    })
}

fn attach_energy_from_zone_to_typed_pokemon(energy_type: EnergyType) -> Outcomes {
    Outcomes::single_fn(move |_rng, state, action| {
        let possible_moves = state
            .enumerate_in_play_pokemon(action.actor)
            .filter(|(_, pokemon)| pokemon.card.get_type() == Some(energy_type))
            .map(|(in_play_idx, _)| SimpleAction::Attach {
                attachments: vec![(1, energy_type, in_play_idx)],
                is_turn_energy: false,
            })
            .collect::<Vec<_>>();
        state
            .move_generation_stack
            .push((action.actor, possible_moves));
    })
}

fn move_energy_from_benched_typed_to_active_typed(energy_type: EnergyType) -> Outcomes {
    Outcomes::single_fn(move |_rng, state, action| {
        let acting_player = action.actor;
        let possible_moves = state
            .enumerate_bench_pokemon(acting_player)
            .filter(|(_, pokemon)| {
                pokemon.card.get_type() == Some(energy_type)
                    && pokemon.attached_energy.contains(&energy_type)
            })
            .map(|(in_play_idx, _)| SimpleAction::MoveEnergy {
                from_in_play_idx: in_play_idx,
                to_in_play_idx: 0,
                energy_type,
                amount: 1,
            })
            .collect::<Vec<_>>();
        if possible_moves.is_empty() {
            return;
        }
        state
            .move_generation_stack
            .push((acting_player, possible_moves));
    })
}
