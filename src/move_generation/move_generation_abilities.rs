use crate::{
    ability_ids::AbilityId,
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

    // Try AbilityMechanic first
    if let Some(mechanic) = card
        .card
        .get_ability()
        .and_then(|a| ability_mechanic_from_effect(&a.effect))
    {
        return can_use_ability_by_mechanic(state, mechanic, in_play_index, card);
    }

    // Existing AbilityId fallback
    let is_active = in_play_index == 0;
    let ability = AbilityId::from_pokemon_id(&card.card.get_id()[..]).unwrap_or_else(|| {
        panic!(
            "Ability seems not implemented for card ID: {}",
            card.card.get_id()
        )
    });
    match ability {
        AbilityId::A1020VictreebelFragranceTrap => is_active && !card.ability_used,
        AbilityId::A1089GreninjaWaterShuriken => unreachable!("Handled by AbilityMechanic"),
        AbilityId::A1098MagnetonVoltCharge => !card.ability_used,
        AbilityId::A1123GengarExShadowySpellbind => false,
        AbilityId::A1177Weezing => is_active && !card.ability_used,
        AbilityId::A1188PidgeotDriveOff => can_use_pidgeot_drive_off(state, card),
        AbilityId::A1132Gardevoir => !card.ability_used,
        AbilityId::A1a006SerperiorJungleTotem => false,
        AbilityId::A1a046AerodactylExPrimevalLaw => false, // Passive
        AbilityId::A1a019VaporeonWashOut => can_use_vaporeon_wash_out(state),
        AbilityId::A2a010LeafeonExForestBreath => is_active && !card.ability_used,
        AbilityId::A2a069ShayminSkySupport => false, // Passive ability
        AbilityId::A2a071Arceus => false,
        AbilityId::A2072DusknoirShadowVoid => can_use_dusknoir_shadow_void(state, in_play_index),
        AbilityId::A2078GiratinaLevitate => false, // Passive ability
        AbilityId::A2092LucarioFightingCoach => false, // Passive ability, triggers via hooks
        AbilityId::A2110DarkraiExNightmareAura => false,
        AbilityId::A2b035GiratinaExBrokenSpaceBellow => !card.ability_used,
        AbilityId::A3066OricoricSafeguard => false,
        AbilityId::A3122SolgaleoExRisingRoad => !is_active && !card.ability_used,
        AbilityId::A3141KomalaComatose => false,
        AbilityId::A3a015LuxrayIntimidatingFang => false,
        AbilityId::A3a021ZeraoraThunderclapFlash => false,
        AbilityId::A3a027ShiinoticIlluminate => !card.ability_used,
        AbilityId::A3a062CelesteelaUltraThrusters => {
            can_use_celesteela_ultra_thrusters(state, card)
        }
        AbilityId::A3b009FlareonExCombust => {
            !card.ability_used
                && state.discard_energies[state.current_player].contains(&EnergyType::Fire)
        }
        AbilityId::A3b034SylveonExHappyRibbon => false,
        AbilityId::A3b056EeveeExVeeveeVolve => false,
        AbilityId::A3b057SnorlaxExFullMouthManner => false,
        AbilityId::A4083EspeonExPsychicHealing => is_active && !card.ability_used,
        AbilityId::A4a010EnteiExLegendaryPulse => false,
        AbilityId::A4a020SuicuneExLegendaryPulse => false,
        AbilityId::A4a022MiloticHealingRipples => false,
        AbilityId::A4a025RaikouExLegendaryPulse => false,
        AbilityId::B1073GreninjaExShiftingStream => unreachable!("Handled by AbilityMechanic"),
        AbilityId::B1121IndeedeeExWatchOver => is_active && !card.ability_used,
        AbilityId::B1157HydreigonRoarInUnison => !card.ability_used,
        AbilityId::B1172AegislashCursedMetal => false, // Passive ability, triggers via hooks
        AbilityId::B1177GoomyStickyMembrane => false,
        AbilityId::B1184EeveeBoostedEvolution => false, // Passive ability, triggers via hooks
        AbilityId::PA037CresseliaExLunarPlumage => false,
        AbilityId::A3a042NihilegoMorePoison => false, // Passive ability, triggers via hooks
        AbilityId::A1061PoliwrathCounterattack => false, // Passive ability, triggers via hooks
        AbilityId::A2a050CrobatCunningLink => can_use_crobat_cunning_link(state, card),
        AbilityId::A4112UmbreonExDarkChase => is_active && can_use_umbreon_dark_chase(state, card),
        AbilityId::B1160DragalgeExPoisonPoint => false, // Passive ability, triggers via hooks
        AbilityId::B1a006AriadosTrapTerritory => false, // Passive ability
        AbilityId::B1a012CharmeleonIgnition => false,   // Triggered on evolve
        AbilityId::B1a018WartortleShellShield => false, // Passive ability
        AbilityId::B1a034ReuniclusInfiniteIncrease => false, // Passive ability
        AbilityId::B1a065FurfrouFurCoat => unreachable!("Handled by AbilityMechanic"),
        AbilityId::A4a032MisdreavusInfiltratingInspection => false, // Triggered when played to bench
        AbilityId::A1007Butterfree | AbilityId::A2022ShayminFragrantFlowerGarden => {
            unreachable!("Handled by AbilityMechanic")
        }
    }
}

fn can_use_ability_by_mechanic(
    state: &State,
    mechanic: &AbilityMechanic,
    _in_play_index: usize,
    card: &PlayedCard,
) -> bool {
    match mechanic {
        AbilityMechanic::HealAllYourPokemon { .. } => !card.ability_used,
        AbilityMechanic::DamageOneOpponentPokemon { .. } => !card.ability_used,
        AbilityMechanic::SwitchActiveTypedWithBench { energy_type } => {
            can_use_switch_active_typed_with_bench(state, card, *energy_type)
        }
        AbilityMechanic::ReduceDamageFromAttacks { .. } => false,
        AbilityMechanic::StartTurnRandomPokemonToHand { .. } => false,
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
