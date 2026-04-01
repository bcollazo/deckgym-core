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
    models::{Card, EnergyType, StatusCondition},
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
        .expect("Pokemon should have ability implemented");

    forecast_ability_by_mechanic(mechanic, state, action, in_play_idx)
}

fn forecast_ability_by_mechanic(
    mechanic: &AbilityMechanic,
    state: &State,
    action: &Action,
    in_play_idx: usize,
) -> Outcomes {
    match mechanic {
        AbilityMechanic::VictreebelFragranceTrap => Outcomes::single_fn(victreebel_ability),
        AbilityMechanic::HealAllYourPokemon { amount } => heal_all_your_pokemon(*amount),
        AbilityMechanic::HealOneYourPokemon { amount } => heal_one_your_pokemon(*amount),
        AbilityMechanic::HealOneYourPokemonExAndDiscardRandomEnergy { amount } => {
            heal_one_your_pokemon_ex_and_discard_random_energy(*amount)
        }
        AbilityMechanic::DamageOneOpponentPokemon { amount } => damage_one_opponent(*amount),
        AbilityMechanic::IncreaseDamageIfArceusInPlay { .. } => {
            panic!("IncreaseDamageIfArceusInPlay is a passive ability")
        }
        AbilityMechanic::DamageOpponentActiveIfArceusInPlay { amount } => {
            damage_opponent_active_if_arceus_in_play(*amount)
        }
        AbilityMechanic::SwitchDamagedOpponentBenchToActive => {
            Outcomes::single_fn(umbreon_dark_chase)
        }
        AbilityMechanic::SwitchThisBenchWithActive => Outcomes::single(rising_road(in_play_idx)),
        AbilityMechanic::SwitchActiveTypedWithBench { .. } => {
            switch_active_typed_with_bench_outcome()
        }
        AbilityMechanic::SwitchActiveUltraBeastWithBench => {
            Outcomes::single_fn(celesteela_ultra_thrusters)
        }
        AbilityMechanic::MoveTypedEnergyFromBenchToActive { .. } => {
            Outcomes::single_fn(vaporeon_wash_out)
        }
        AbilityMechanic::AttachEnergyFromZoneToActiveTypedPokemon { energy_type } => {
            attach_energy_from_zone_to_active_typed_outcome(*energy_type)
        }
        AbilityMechanic::AttachEnergyFromZoneToYourTypedPokemon { energy_type } => {
            attach_energy_from_zone_to_your_typed_outcome(*energy_type)
        }
        AbilityMechanic::AttachEnergyFromZoneToSelf {
            energy_type,
            amount,
        } => Outcomes::single(attach_energy_from_zone_to_self(
            in_play_idx,
            *energy_type,
            *amount,
        )),
        AbilityMechanic::AttachEnergyFromZoneToSelfAndEndTurn { energy_type } => Outcomes::single(
            attach_energy_from_zone_to_self_and_end_turn(in_play_idx, *energy_type),
        ),
        AbilityMechanic::AttachEnergyFromZoneToSelfAndDamage {
            energy_type,
            amount,
            self_damage,
        } => Outcomes::single(attach_energy_from_zone_to_self_and_damage(
            in_play_idx,
            *energy_type,
            *amount,
            *self_damage,
        )),
        AbilityMechanic::DamageOpponentActiveOnZoneAttachToSelf { .. } => {
            panic!("DamageOpponentActiveOnZoneAttachToSelf is a passive ability")
        }
        AbilityMechanic::AttachEnergyFromDiscardToSelfAndDamage {
            energy_type,
            self_damage,
        } => Outcomes::single_fn({
            let energy_type = *energy_type;
            let self_damage = *self_damage;
            move |_rng, state, action| {
                attach_energy_from_discard_to_self_and_damage(
                    state,
                    action,
                    energy_type,
                    self_damage,
                )
            }
        }),
        AbilityMechanic::ReduceDamageFromAttacks { .. } => {
            panic!("ReduceDamageFromAttacks is a passive ability")
        }
        AbilityMechanic::ReduceOpponentActiveDamage { .. } => {
            panic!("ReduceOpponentActiveDamage is a passive ability")
        }
        AbilityMechanic::IncreaseDamageWhenRemainingHpAtMost { .. } => {
            panic!("IncreaseDamageWhenRemainingHpAtMost is a passive ability")
        }
        AbilityMechanic::IncreaseDamageForTypeInPlay { .. }
        | AbilityMechanic::IncreaseDamageForTwoTypesInPlay { .. } => {
            panic!("Type damage bonus mechanics are passive abilities")
        }
        AbilityMechanic::StartTurnRandomPokemonToHand { .. } => {
            panic!("StartTurnRandomPokemonToHand is a passive ability")
        }
        AbilityMechanic::SearchRandomPokemonFromDeck => {
            pokemon_search_outcomes(action.actor, state, false)
        }
        AbilityMechanic::MoveDamageFromOneYourPokemonToThisPokemon => {
            Outcomes::single(dusknoir_shadow_void(in_play_idx))
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
        AbilityMechanic::DiscardTopCardOpponentDeck => discard_top_card_opponent_deck(),
        AbilityMechanic::CoinFlipToPreventDamage => {
            panic!("CoinFlipToPreventDamage is a passive ability")
        }
        AbilityMechanic::CheckupDamageToOpponentActive { .. } => {
            panic!("CheckupDamageToOpponentActive is a passive ability")
        }
        AbilityMechanic::BadDreamsEndOfTurn { .. } => {
            panic!("BadDreamsEndOfTurn is a passive ability")
        }
        AbilityMechanic::EndTurnDrawCardIfActive { .. } => {
            panic!("EndTurnDrawCardIfActive is triggered at end of turn")
        }
        AbilityMechanic::EndTurnHealSelfIfActive { .. } => {
            panic!("EndTurnHealSelfIfActive is triggered at end of turn")
        }
        AbilityMechanic::DiscardEnergyToIncreaseTypeDamage {
            discard_energy,
            attack_type,
            amount,
        } => discard_energy_to_increase_type_damage(*discard_energy, *attack_type, *amount),
        AbilityMechanic::PoisonOpponentActive => poison_opponent_active(),
        AbilityMechanic::HealActiveYourPokemon { amount } => heal_active_your_pokemon(*amount),
        AbilityMechanic::SwitchOutOpponentActiveToBench => switch_out_opponent_active_to_bench(),
        AbilityMechanic::CoinFlipSleepOpponentActive => coin_flip_sleep_opponent_active(),
        AbilityMechanic::DiscardFromHandToDrawCard => discard_from_hand_to_draw_card(),
        AbilityMechanic::ImmuneToStatusConditions => {
            panic!("ImmuneToStatusConditions is a passive ability")
        }
        AbilityMechanic::SoothingWind { .. } => {
            panic!("SoothingWind is a passive ability")
        }
        AbilityMechanic::NoOpponentSupportInActive => {
            panic!("NoOpponentSupportInActive is a passive ability")
        }
        AbilityMechanic::DoubleGrassEnergy => panic!("DoubleGrassEnergy is a passive ability"),
        AbilityMechanic::PreventOpponentActiveEvolution => {
            panic!("PreventOpponentActiveEvolution is a passive ability")
        }
        AbilityMechanic::ReduceRetreatCostOfYourActiveBasicFromBench { .. } => {
            panic!("ReduceRetreatCostOfYourActiveBasicFromBench is a passive ability")
        }
        AbilityMechanic::NoRetreatIfHasEnergy => {
            panic!("NoRetreatIfHasEnergy is a passive ability")
        }
        AbilityMechanic::PreventAllDamageFromEx => {
            panic!("PreventAllDamageFromEx is a passive ability")
        }
        AbilityMechanic::SleepOnZoneAttachToSelfWhileActive => {
            panic!("SleepOnZoneAttachToSelfWhileActive is a passive ability")
        }
        AbilityMechanic::IncreasePoisonDamage { .. } => {
            panic!("IncreasePoisonDamage is a passive ability")
        }
        AbilityMechanic::DrawCardsOnEvolve { .. } => {
            panic!("DrawCardsOnEvolve is triggered on evolve")
        }
        AbilityMechanic::HealTypedPokemonOnEvolve { .. } => {
            panic!("HealTypedPokemonOnEvolve is triggered on evolve")
        }
        AbilityMechanic::AttachEnergyFromZoneToActiveTypedOnEvolve { .. } => {
            panic!("AttachEnergyFromZoneToActiveTypedOnEvolve is triggered on evolve")
        }
        AbilityMechanic::CanEvolveIntoEeveeEvolution => {
            panic!("CanEvolveIntoEeveeEvolution is a passive ability")
        }
        AbilityMechanic::CanEvolveOnFirstTurnIfActive => {
            panic!("CanEvolveOnFirstTurnIfActive is a passive ability")
        }
        AbilityMechanic::CounterattackDamage { .. } => {
            panic!("CounterattackDamage is a passive ability")
        }
        AbilityMechanic::PoisonAttackerOnDamaged => {
            panic!("PoisonAttackerOnDamaged is a passive ability")
        }
        AbilityMechanic::IncreaseAttackCostForOpponentActive { .. } => {
            panic!("IncreaseAttackCostForOpponentActive is a passive ability")
        }
        AbilityMechanic::IncreaseRetreatCostForOpponentActive { .. } => {
            panic!("IncreaseRetreatCostForOpponentActive is a passive ability")
        }
        AbilityMechanic::PreventDamageWhileBenched => {
            panic!("PreventDamageWhileBenched is a passive ability")
        }
        AbilityMechanic::IncreaseHpPerAttachedEnergy { .. } => {
            panic!("IncreaseHpPerAttachedEnergy is a passive ability")
        }
        AbilityMechanic::HealSelfOnZoneAttach { .. } => {
            panic!("HealSelfOnZoneAttach is a passive ability")
        }
        AbilityMechanic::EndFirstTurnAttachEnergyToSelf { .. } => {
            panic!("EndFirstTurnAttachEnergyToSelf is triggered at end of first turn")
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

fn heal_one_your_pokemon(amount: u32) -> Outcomes {
    Outcomes::single_fn(move |_rng, state, action| {
        let choices = state
            .enumerate_in_play_pokemon(action.actor)
            .filter(|(_, pokemon)| pokemon.is_damaged())
            .map(|(in_play_idx, _)| SimpleAction::Heal {
                in_play_idx,
                amount,
                cure_status: false,
            })
            .collect::<Vec<_>>();
        if !choices.is_empty() {
            state.move_generation_stack.push((action.actor, choices));
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

fn attach_energy_from_zone_to_your_typed_outcome(energy_type: EnergyType) -> Outcomes {
    Outcomes::single_fn(move |_rng, state, action| {
        let choices = state
            .enumerate_in_play_pokemon(action.actor)
            .filter(|(_, pokemon)| pokemon.card.get_type() == Some(energy_type))
            .map(|(in_play_idx, _)| SimpleAction::Attach {
                attachments: vec![(1, energy_type, in_play_idx)],
                is_turn_energy: false,
            })
            .collect::<Vec<_>>();
        if !choices.is_empty() {
            state.move_generation_stack.push((action.actor, choices));
        }
    })
}

fn attach_energy_from_zone_to_active_typed_outcome(energy_type: EnergyType) -> Outcomes {
    Outcomes::single_fn(move |_rng, state, action| {
        state.attach_energy_from_zone(action.actor, 0, energy_type, 1, false);
    })
}

fn attach_energy_from_zone_to_self(
    in_play_idx: usize,
    energy_type: EnergyType,
    amount: u32,
) -> Mutation {
    Box::new(move |_, state, action| {
        state.attach_energy_from_zone(action.actor, in_play_idx, energy_type, amount, false);
    })
}

fn attach_energy_from_zone_to_self_and_end_turn(
    in_play_idx: usize,
    energy_type: EnergyType,
) -> Mutation {
    Box::new(move |_, state, action| {
        let attached =
            state.attach_energy_from_zone(action.actor, in_play_idx, energy_type, 1, false);
        if let Some(pokemon) = &state.in_play_pokemon[action.actor][in_play_idx] {
            if attached && !pokemon.is_knocked_out() {
                state
                    .move_generation_stack
                    .push((action.actor, vec![SimpleAction::EndTurn]));
            }
        }
    })
}

fn attach_energy_from_zone_to_self_and_damage(
    in_play_idx: usize,
    energy_type: EnergyType,
    amount: u32,
    self_damage: u32,
) -> Mutation {
    Box::new(move |_, state, action| {
        let attached =
            state.attach_energy_from_zone(action.actor, in_play_idx, energy_type, amount, false);
        if let Some(pokemon) = &state.in_play_pokemon[action.actor][in_play_idx] {
            if attached && !pokemon.is_knocked_out() {
                handle_damage(
                    state,
                    (action.actor, in_play_idx),
                    &[(self_damage, action.actor, in_play_idx)],
                    false,
                    None,
                );
            }
        }
    })
}

fn attach_energy_from_discard_to_self_and_damage(
    state: &mut State,
    action: &Action,
    energy_type: EnergyType,
    self_damage: u32,
) {
    let SimpleAction::UseAbility { in_play_idx } = action.action else {
        panic!("Ability should be triggered by UseAbility action");
    };
    state.attach_energy_from_discard(action.actor, in_play_idx, &[energy_type]);
    handle_damage(
        state,
        (action.actor, in_play_idx),
        &[(self_damage, action.actor, in_play_idx)],
        false,
        None,
    );
}

fn damage_opponent_active_if_arceus_in_play(amount: u32) -> Outcomes {
    Outcomes::single_fn(move |_rng, state, action| {
        let SimpleAction::UseAbility { in_play_idx } = action.action else {
            panic!("Ability should be triggered by UseAbility action");
        };
        let opponent = (action.actor + 1) % 2;
        handle_damage(
            state,
            (action.actor, in_play_idx),
            &[(amount, opponent, 0)],
            false,
            None,
        );
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

fn poison_opponent_active() -> Outcomes {
    Outcomes::single_fn(|_rng, state, action| {
        let opponent = (action.actor + 1) % 2;
        state.apply_status_condition(opponent, 0, StatusCondition::Poisoned);
    })
}

fn coin_flip_sleep_opponent_active() -> Outcomes {
    Outcomes::binary_coin(
        Box::new(|_, state, action| {
            let opponent = (action.actor + 1) % 2;
            state.apply_status_condition(opponent, 0, StatusCondition::Asleep);
        }),
        Box::new(|_, _, _| {}),
    )
}

fn heal_active_your_pokemon(amount: u32) -> Outcomes {
    Outcomes::single_fn(move |_rng, state, action| {
        let active = state.get_active_mut(action.actor);
        active.heal(amount);
    })
}

fn switch_out_opponent_active_to_bench() -> Outcomes {
    Outcomes::single_fn(|_rng, state, action| {
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
    })
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

fn discard_from_hand_to_draw_card() -> Outcomes {
    Outcomes::single_fn(|_rng, state, action| {
        // Queue draw first (LIFO: will execute after the discard choice resolves)
        state.queue_draw_action(action.actor, 1);
        // Push discard choices (executed first since pushed last onto LIFO stack)
        let hand_cards: Vec<Card> = state.hands[action.actor].to_vec();
        let mut seen = std::collections::HashSet::new();
        let choices: Vec<SimpleAction> = hand_cards
            .into_iter()
            .filter(|card| seen.insert(card.clone()))
            .map(|card| SimpleAction::DiscardOwnCards { cards: vec![card] })
            .collect();
        if !choices.is_empty() {
            state.move_generation_stack.push((action.actor, choices));
        }
    })
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
