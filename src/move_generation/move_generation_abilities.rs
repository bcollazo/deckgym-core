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

    let Some(mechanic) = card
        .card
        .get_ability()
        .and_then(|a| ability_mechanic_from_effect(&a.effect))
    else {
        return false;
    };
    can_use_ability_by_mechanic(state, mechanic, in_play_index, card)
}

fn can_use_ability_by_mechanic(
    state: &State,
    mechanic: &AbilityMechanic,
    _in_play_index: usize,
    card: &PlayedCard,
) -> bool {
    match mechanic {
        AbilityMechanic::HealAllYourPokemon { .. } => !card.ability_used,
        AbilityMechanic::HealOneYourPokemonExAndDiscardRandomEnergy { .. } => {
            can_use_heal_one_your_pokemon_ex_and_discard_random_energy(state, card)
        }
        AbilityMechanic::DamageOneOpponentPokemon { .. } => !card.ability_used,
        AbilityMechanic::SwitchActiveTypedWithBench { energy_type } => {
            can_use_switch_active_typed_with_bench(state, card, *energy_type)
        }
        AbilityMechanic::AttachEnergyFromZoneToActiveTypedPokemon { energy_type } => {
            can_use_attach_energy_from_zone_to_active_typed(state, card, *energy_type)
        }
        AbilityMechanic::ReduceDamageFromAttacks { .. } => false,
        AbilityMechanic::IncreaseDamageWhenRemainingHpAtMost { .. } => false,
        AbilityMechanic::StartTurnRandomPokemonToHand { .. } => false,
        AbilityMechanic::PreventFirstAttack => false,
        AbilityMechanic::ElectromagneticWall => false,
        AbilityMechanic::InfiltratingInspection => false,
        AbilityMechanic::DiscardTopCardOpponentDeck => {
            !card.ability_used && !state.decks[(state.current_player + 1) % 2].cards.is_empty()
        }
        AbilityMechanic::CoinFlipToPreventDamage => false, // Passive ability
        AbilityMechanic::CheckupDamageToOpponentActive { .. } => false, // Passive ability
        AbilityMechanic::DiscardEnergyToIncreaseTypeDamage { discard_energy, .. } => {
            !card.ability_used && card.attached_energy.contains(discard_energy)
        }
        // ── Active (on-demand) abilities ──────────────────────────────────────
        AbilityMechanic::PoisonOpponentActive => _in_play_index == 0 && !card.ability_used,
        AbilityMechanic::SwitchOutOpponentActive => {
            if card.ability_used {
                return false;
            }
            let opponent = (state.current_player + 1) % 2;
            state.enumerate_bench_pokemon(opponent).next().is_some()
        }
        AbilityMechanic::SwitchInOpponentBenchedBasic => {
            if card.ability_used || _in_play_index != 0 {
                return false;
            }
            let opponent = (state.current_player + 1) % 2;
            state
                .enumerate_bench_pokemon(opponent)
                .any(|(_, pokemon)| pokemon.card.is_basic())
        }
        AbilityMechanic::AttachEnergyFromZoneToSelf { .. } => !card.ability_used,
        AbilityMechanic::AttachEnergyFromZoneToSelfEndTurn { .. } => !card.ability_used,
        AbilityMechanic::AttachMultipleEnergyFromZoneToSelfAndSelfDamage { .. } => {
            !card.ability_used
        }
        AbilityMechanic::AttachEnergyFromDiscardToSelfAndSelfDamage { energy_type, .. } => {
            !card.ability_used
                && state.discard_energies[state.current_player].contains(energy_type)
        }
        AbilityMechanic::HealActivePokemon { .. } => !card.ability_used,
        AbilityMechanic::HealOneOfYourPokemon { .. } => {
            if card.ability_used || _in_play_index != 0 {
                return false;
            }
            state
                .enumerate_in_play_pokemon(state.current_player)
                .any(|(_, pokemon)| pokemon.is_damaged())
        }
        AbilityMechanic::SwitchInOpponentDamagedBenched => {
            if card.ability_used || _in_play_index != 0 {
                return false;
            }
            let opponent = (state.current_player + 1) % 2;
            state
                .enumerate_bench_pokemon(opponent)
                .any(|(_, pokemon)| pokemon.is_damaged())
        }
        AbilityMechanic::SwitchBenchedSelfToActive => {
            _in_play_index != 0 && !card.ability_used
        }
        AbilityMechanic::DrawRandomPokemonFromDeck => !card.ability_used,
        AbilityMechanic::MoveAllDamageToSelf => {
            state
                .enumerate_in_play_pokemon(state.current_player)
                .any(|(i, p)| p.is_damaged() && i != _in_play_index)
        }
        AbilityMechanic::DamageOpponentActiveIfArceusInPlay { .. } => {
            if card.ability_used {
                return false;
            }
            state
                .enumerate_in_play_pokemon(state.current_player)
                .any(|(_, pokemon)| {
                    let name = pokemon.get_name();
                    name == "Arceus" || name == "Arceus ex"
                })
        }
        AbilityMechanic::AttachEnergyFromZoneToTypedPokemon { .. } => {
            _in_play_index == 0 && !card.ability_used
        }
        AbilityMechanic::MoveEnergyFromBenchedTypedToActiveTyped { energy_type } => {
            if card.ability_used {
                return false;
            }
            let active = state.get_active(state.current_player);
            if active.get_energy_type() != Some(*energy_type) {
                return false;
            }
            state
                .enumerate_bench_pokemon(state.current_player)
                .any(|(_, pokemon)| {
                    pokemon.card.get_type() == Some(*energy_type)
                        && pokemon.attached_energy.contains(energy_type)
                })
        }
        AbilityMechanic::SwitchUltraBeast => {
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
        // ── Passive (hook-driven) abilities ───────────────────────────────────
        AbilityMechanic::BlockSupportCards => false,
        AbilityMechanic::GrassEnergyDoubling => false,
        AbilityMechanic::BlockOpponentEvolution => false,
        AbilityMechanic::BenchReduceBasicRetreatCost => false,
        AbilityMechanic::NoRetreatCostWithEnergy => false,
        AbilityMechanic::TypeDamageBoost { .. } => false,
        AbilityMechanic::DamageOpponentActiveOnEnergyAttachFromZone { .. } => false,
        AbilityMechanic::SafeguardFromEx => false,
        AbilityMechanic::AsleepOnEnergyAttachFromZoneWhenActive => false,
        AbilityMechanic::ReduceOpponentAttackDamage { .. } => false,
        AbilityMechanic::AttachEnergyFromZoneToSelfOnFirstTurn { .. } => false,
        AbilityMechanic::MorePoisonDamage { .. } => false,
        AbilityMechanic::CounterattackOnDamage { .. } => false,
        AbilityMechanic::PoisonAttackerOnDamage => false,
        AbilityMechanic::ImmuneToSpecialConditions => false,
        AbilityMechanic::OnEvolveDraw { .. } => false,
        AbilityMechanic::VeeveeVolve => false,
        AbilityMechanic::EndTurnHealSelf { .. } => false,
        AbilityMechanic::EndTurnDrawCard => false,
        AbilityMechanic::OnEvolveHealTypedPokemon { .. } => false,
        AbilityMechanic::MultiTypeDamageBoost { .. } => false,
        AbilityMechanic::IncreaseOpponentAttackCost => false,
        AbilityMechanic::BoostedEvolution => false,
        AbilityMechanic::IncreaseOpponentRetreatCost => false,
        AbilityMechanic::OnEvolveAttachEnergyFromZoneToActiveTypedPokemon { .. } => false,
        AbilityMechanic::BenchSafeguard => false,
        AbilityMechanic::HpBonusPerEnergy { .. } => false,
        AbilityMechanic::HealOnEnergyAttachFromZone { .. } => false,
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

fn can_use_pidgeot_drive_off(state: &State, card: &PlayedCard) -> bool {
    if card.ability_used {
        return false;
    }
    // Opponent must have a benched Pokémon to switch to
    let opponent = (state.current_player + 1) % 2;
    state.enumerate_bench_pokemon(opponent).next().is_some()
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
