use crate::{
    ability_ids::AbilityId,
    actions::SimpleAction,
    hooks::is_ultra_beast,
    models::{EnergyType, PlayedCard},
    State,
};

// Use the new function in the filter method
pub(crate) fn generate_ability_actions(state: &State) -> Vec<SimpleAction> {
    let current_player = state.current_player;
    state
        .enumerate_in_play_pokemon(current_player)
        .filter(|(in_play_index, card)| can_use_ability(state, (*in_play_index, *card)))
        .map(|(in_play_idx, _)| SimpleAction::UseAbility { in_play_idx })
        .collect()
}

fn can_use_ability(state: &State, (in_play_index, card): (usize, &PlayedCard)) -> bool {
    if card.card.get_ability().is_none() {
        return false;
    }

    let is_active = in_play_index == 0;
    let ability = AbilityId::from_pokemon_id(&card.card.get_id()[..]).unwrap_or_else(|| {
        panic!(
            "Ability seems not implemented for card ID: {}",
            card.card.get_id()
        )
    });
    match ability {
        AbilityId::A1007Butterfree => !card.ability_used,
        AbilityId::A1020VictreebelFragranceTrap => is_active && !card.ability_used,
        AbilityId::A1089GreninjaWaterShuriken => !card.ability_used,
        AbilityId::A1098MagnetonVoltCharge => !card.ability_used,
        AbilityId::A1177Weezing => is_active && !card.ability_used,
        AbilityId::A1132Gardevoir => !card.ability_used,
        AbilityId::A1a006SerperiorJungleTotem => false,
        AbilityId::A2a010LeafeonExForestBreath => is_active && !card.ability_used,
        AbilityId::A2a071Arceus => false,
        AbilityId::A2110DarkraiExNightmareAura => false,
        AbilityId::A2b035GiratinaExBrokenSpaceBellow => !card.ability_used,
        AbilityId::A3066OricoricSafeguard => false,
        AbilityId::A3122SolgaleoExRisingRoad => !is_active && !card.ability_used,
        AbilityId::A3141KomalaComatose => false,
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
        AbilityId::A4083EspeonExPsychicHealing => is_active && !card.ability_used,
        AbilityId::A4a010EnteiExLegendaryPulse => false,
        AbilityId::A4a020SuicuneExLegendaryPulse => false,
        AbilityId::A4a022MiloticHealingRipples => false,
        AbilityId::A4a025RaikouExLegendaryPulse => false,
        AbilityId::B1157HydreigonRoarInUnison => !card.ability_used,
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
