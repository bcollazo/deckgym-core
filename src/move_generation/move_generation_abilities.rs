use crate::{
    actions::abilities::AbilityMechanic,
    actions::{ability_mechanic_from_effect, SimpleAction},
    hooks::is_ultra_beast,
    models::{EnergyType, PlayedCard},
    State,
};

// Use the new function in the filter method
pub(crate) fn generate_ability_actions(state: &State) -> Vec<SimpleAction> {
    let current_player = state.current_player;
    let mut actions = vec![];

    for (in_play_idx, card) in state.enumerate_in_play_pokemon(current_player) {
        if card.card.is_fossil() {
            actions.push(SimpleAction::DiscardFossil { in_play_idx });
        } else if can_use_ability(state, (in_play_idx, card)) {
            actions.push(SimpleAction::UseAbility { in_play_idx });
        }
    }

    actions
}

fn can_use_ability(state: &State, (in_play_index, card): (usize, &PlayedCard)) -> bool {
    if card.card.get_ability().is_none() {
        return false;
    }

    let mechanic = card
        .card
        .get_ability()
        .and_then(|a| ability_mechanic_from_effect(&a.effect))
        .unwrap_or_else(|| {
            panic!(
                "Ability seems not implemented for card ID: {}",
                card.card.get_id()
            )
        });

    can_use_ability_by_mechanic(state, mechanic, in_play_index, card)
}

fn can_use_ability_by_mechanic(
    state: &State,
    mechanic: &AbilityMechanic,
    _in_play_index: usize,
    card: &PlayedCard,
) -> bool {
    let is_active = _in_play_index == 0;
    match mechanic {
        AbilityMechanic::VictreebelFragranceTrap => {
            is_active && can_use_victreebel_fragrance_trap(state, card)
        }
        AbilityMechanic::HealAllYourPokemon { .. } => !card.ability_used,
        AbilityMechanic::HealOneYourPokemon { .. } => {
            is_active && can_use_espeon_ex_psychic_healing(state, card)
        }
        AbilityMechanic::HealOneYourPokemonExAndDiscardRandomEnergy { .. } => {
            can_use_heal_one_your_pokemon_ex_and_discard_random_energy(state, card)
        }
        AbilityMechanic::DamageOneOpponentPokemon { .. } => !card.ability_used,
        AbilityMechanic::IncreaseDamageIfArceusInPlay { .. } => false,
        AbilityMechanic::DamageOpponentActiveIfArceusInPlay { .. } => {
            can_use_crobat_cunning_link(state, card)
        }
        AbilityMechanic::SwitchDamagedOpponentBenchToActive => {
            is_active && can_use_umbreon_dark_chase(state, card)
        }
        AbilityMechanic::SwitchThisBenchWithActive => !is_active && !card.ability_used,
        AbilityMechanic::SwitchActiveTypedWithBench { energy_type } => {
            can_use_switch_active_typed_with_bench(state, card, *energy_type)
        }
        AbilityMechanic::SwitchActiveUltraBeastWithBench => {
            can_use_celesteela_ultra_thrusters(state, card)
        }
        AbilityMechanic::MoveTypedEnergyFromBenchToActive { .. } => {
            can_use_vaporeon_wash_out(state)
        }
        AbilityMechanic::AttachEnergyFromZoneToActiveTypedPokemon { energy_type } => {
            can_use_attach_energy_from_zone_to_active_typed(state, card, *energy_type)
        }
        AbilityMechanic::AttachEnergyFromZoneToYourTypedPokemon { .. } => {
            is_active && !card.ability_used
        }
        AbilityMechanic::AttachEnergyFromZoneToSelf { .. } => !card.ability_used,
        AbilityMechanic::AttachEnergyFromZoneToSelfAndEndTurn { .. } => !card.ability_used,
        AbilityMechanic::AttachEnergyFromZoneToSelfAndDamage { .. } => !card.ability_used,
        AbilityMechanic::DamageOpponentActiveOnZoneAttachToSelf { .. } => false,
        AbilityMechanic::AttachEnergyFromDiscardToSelfAndDamage { energy_type, .. } => {
            !card.ability_used && state.discard_energies[state.current_player].contains(energy_type)
        }
        AbilityMechanic::ReduceDamageFromAttacks { .. } => false,
        AbilityMechanic::ReduceOpponentActiveDamage { .. } => false,
        AbilityMechanic::IncreaseDamageWhenRemainingHpAtMost { .. } => false,
        AbilityMechanic::IncreaseDamageForTypeInPlay { .. } => false,
        AbilityMechanic::IncreaseDamageForTwoTypesInPlay { .. } => false,
        AbilityMechanic::StartTurnRandomPokemonToHand { .. } => false,
        AbilityMechanic::SearchRandomPokemonFromDeck => {
            !card.ability_used
                && state
                    .iter_deck_pokemon(state.current_player)
                    .next()
                    .is_some()
        }
        AbilityMechanic::MoveDamageFromOneYourPokemonToThisPokemon => {
            can_use_dusknoir_shadow_void(state, _in_play_index)
        }
        AbilityMechanic::PreventFirstAttack => false,
        AbilityMechanic::ElectromagneticWall => false,
        AbilityMechanic::InfiltratingInspection => false,
        AbilityMechanic::DiscardTopCardOpponentDeck => {
            !card.ability_used && !state.decks[(state.current_player + 1) % 2].cards.is_empty()
        }
        AbilityMechanic::CoinFlipToPreventDamage => false, // Passive ability
        AbilityMechanic::CheckupDamageToOpponentActive { .. } => false, // Passive ability
        AbilityMechanic::BadDreamsEndOfTurn { .. } => false, // Passive ability
        AbilityMechanic::CoinFlipSleepOpponentActive => !card.ability_used,
        AbilityMechanic::DiscardEnergyToIncreaseTypeDamage { discard_energy, .. } => {
            !card.ability_used && card.attached_energy.contains(discard_energy)
        }
        AbilityMechanic::PoisonOpponentActive => _in_play_index == 0 && !card.ability_used,
        AbilityMechanic::RemoveRandomSpecialConditionFromActive => {
            can_use_remove_random_special_condition_from_active(state, card)
        }
        AbilityMechanic::HealActiveYourPokemon { .. } => !card.ability_used,
        AbilityMechanic::SwitchOutOpponentActiveToBench { require_active } => {
            let opponent = (state.current_player + 1) % 2;
            !card.ability_used
                && (!require_active || is_active)
                && state.enumerate_bench_pokemon(opponent).next().is_some()
        }
        AbilityMechanic::DiscardFromHandToDrawCard => {
            !card.ability_used && !state.hands[state.current_player].is_empty()
        }
        AbilityMechanic::ImmuneToStatusConditions => false, // Passive ability
        AbilityMechanic::SoothingWind { .. } => false,      // Passive ability
        AbilityMechanic::NoOpponentSupportInActive => false,
        AbilityMechanic::DoubleGrassEnergy => false,
        AbilityMechanic::PreventOpponentActiveEvolution => false,
        AbilityMechanic::ReduceRetreatCostOfYourActiveBasicFromBench { .. } => false,
        AbilityMechanic::ReduceRetreatCostOfYourActiveTypedFromBench { .. } => false,
        AbilityMechanic::NoRetreatIfHasEnergy => false,
        AbilityMechanic::PreventAllDamageFromEx => false,
        AbilityMechanic::SleepOnZoneAttachToSelfWhileActive => false,
        AbilityMechanic::IncreasePoisonDamage { .. } => false,
        AbilityMechanic::DrawCardsOnEvolve { .. } => false,
        AbilityMechanic::HealTypedPokemonOnEvolve { .. } => false,
        AbilityMechanic::AttachEnergyFromZoneToActiveTypedOnEvolve { .. } => false,
        AbilityMechanic::DamageOpponentActiveOnEvolve { .. } => false,
        AbilityMechanic::CanEvolveIntoEeveeEvolution => false,
        AbilityMechanic::CanEvolveOnFirstTurnIfActive => false,
        AbilityMechanic::CounterattackDamage { .. } => false,
        AbilityMechanic::PoisonAttackerOnDamaged => false,
        AbilityMechanic::IncreaseAttackCostForOpponentActive { .. } => false,
        AbilityMechanic::IncreaseRetreatCostForOpponentActive { .. } => false,
        AbilityMechanic::PreventDamageWhileBenched => false,
        AbilityMechanic::IncreaseHpPerAttachedEnergy { .. } => false,
        AbilityMechanic::HealSelfOnZoneAttach { .. } => false,
        AbilityMechanic::EndFirstTurnAttachEnergyToSelf { .. } => false,
        AbilityMechanic::EndTurnDrawCardIfActive { .. } => false,
        AbilityMechanic::EndTurnHealSelfIfActive { .. } => false,
        AbilityMechanic::ProtectSelfNextTurnAfterAttackKnockout => false,
    }
}

fn can_use_celesteela_ultra_thrusters(state: &State, card: &PlayedCard) -> bool {
    if card.ability_used {
        return false;
    }
    let active = state.get_active(state.current_player);
    if !is_ultra_beast(&active.get_name()) {
        return false;
    }
    state
        .enumerate_bench_pokemon(state.current_player)
        .any(|(_, pokemon)| is_ultra_beast(&pokemon.get_name()))
}

fn can_use_switch_active_typed_with_bench(
    state: &State,
    card: &PlayedCard,
    energy_type: EnergyType,
) -> bool {
    if card.ability_used {
        return false;
    }
    let active = state.get_active(state.current_player);
    if active.get_energy_type() != Some(energy_type) {
        return false;
    }
    state
        .enumerate_bench_pokemon(state.current_player)
        .next()
        .is_some()
}

fn can_use_remove_random_special_condition_from_active(state: &State, card: &PlayedCard) -> bool {
    !card.ability_used
        && state
            .maybe_get_active(state.current_player)
            .is_some_and(|active| {
                active.is_poisoned()
                    || active.is_paralyzed()
                    || active.is_asleep()
                    || active.is_burned()
                    || active.is_confused()
            })
}

fn can_use_heal_one_your_pokemon_ex_and_discard_random_energy(
    state: &State,
    card: &PlayedCard,
) -> bool {
    if card.ability_used {
        return false;
    }
    state
        .enumerate_in_play_pokemon(state.current_player)
        .any(|(_, pokemon)| {
            pokemon.card.is_ex() && pokemon.is_damaged() && !pokemon.attached_energy.is_empty()
        })
}

fn can_use_attach_energy_from_zone_to_active_typed(
    state: &State,
    card: &PlayedCard,
    energy_type: EnergyType,
) -> bool {
    if card.ability_used || !state.can_attach_energy_from_zone(0) {
        return false;
    }
    let active = state.get_active(state.current_player);
    active.get_energy_type() == Some(energy_type)
}

fn can_use_dusknoir_shadow_void(state: &State, dusknoir_idx: usize) -> bool {
    state
        .enumerate_in_play_pokemon(state.current_player)
        .any(|(i, p)| p.is_damaged() && i != dusknoir_idx)
}

fn can_use_crobat_cunning_link(state: &State, card: &PlayedCard) -> bool {
    if card.ability_used {
        return false;
    }
    // Check if player has Arceus or Arceus ex in play
    state
        .enumerate_in_play_pokemon(state.current_player)
        .any(|(_, pokemon)| {
            let name = pokemon.get_name();
            name == "Arceus" || name == "Arceus ex"
        })
}

fn can_use_umbreon_dark_chase(state: &State, card: &PlayedCard) -> bool {
    if card.ability_used {
        return false;
    }
    // Must be in the Active Spot (index 0)
    // Opponent must have a benched Pokémon with damage
    let opponent = (state.current_player + 1) % 2;
    state
        .enumerate_bench_pokemon(opponent)
        .any(|(_, pokemon)| pokemon.is_damaged())
}

fn can_use_vaporeon_wash_out(state: &State) -> bool {
    // Check if active Pokémon is Water type
    let active = state.get_active(state.current_player);
    if active.get_energy_type() != Some(EnergyType::Water) {
        return false;
    }
    // Check if there's a benched Water Pokémon with Water energy
    state
        .enumerate_bench_pokemon(state.current_player)
        .any(|(_, pokemon)| {
            pokemon.card.get_type() == Some(EnergyType::Water)
                && pokemon.attached_energy.contains(&EnergyType::Water)
        })
}

fn can_use_victreebel_fragrance_trap(state: &State, card: &PlayedCard) -> bool {
    if card.ability_used {
        return false;
    }
    let opponent = (state.current_player + 1) % 2;
    state
        .enumerate_bench_pokemon(opponent)
        .any(|(_, pokemon)| pokemon.card.is_basic())
}

fn can_use_espeon_ex_psychic_healing(state: &State, card: &PlayedCard) -> bool {
    if card.ability_used {
        return false;
    }
    state
        .enumerate_in_play_pokemon(state.current_player)
        .any(|(_, pokemon)| pokemon.is_damaged())
}
