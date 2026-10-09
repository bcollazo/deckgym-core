use crate::{
    actions::{abilities::AbilityMechanic, SimpleAction},
    card_ids::CardId,
    card_logic::{
        acerola_targets, active_has_psychic_attack, can_rare_candy_evolve, diantha_targets,
        distinct_attached_energy_types, fishing_net_candidates, ilima_targets, mallow_targets,
        psychic_energy_sources, quick_grow_extract_candidates, wallace_candidates,
        LT_SURGE_TARGETS,
    },
    effects::TurnEffect,
    hooks::{
        can_play_item, can_play_support, get_stage, is_ancient_pokemon, is_future_pokemon,
        is_ultra_beast,
    },
    models::{Card, EnergyType, TrainerCard, TrainerType},
    stadiums::is_stadium_effect_implemented,
    tools::{enumerate_tool_choices, is_tool_effect_implemented},
    trainer_identity::canonical_trainer_id,
    State,
};

/// Helper function to check if a trainer card can be played and return the appropriate action
fn can_play_trainer(_state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    Some(vec![SimpleAction::Play {
        trainer_card: trainer_card.clone(),
    }])
}

/// Helper function to return empty action vector
fn cannot_play_trainer() -> Option<Vec<SimpleAction>> {
    Some(vec![])
}

/// Generate possible actions for a trainer card.
pub fn generate_possible_trainer_actions(
    state: &State,
    trainer_card: &TrainerCard,
) -> Option<Vec<SimpleAction>> {
    if state.turn_count == 0 {
        return cannot_play_trainer(); // No trainers on initial setup phase
    }
    let no_trainers = state
        .get_current_turn_effects()
        .iter()
        .any(|x| matches!(x, TurnEffect::NoTrainerCards));
    if no_trainers {
        return cannot_play_trainer();
    }
    if trainer_card.trainer_card_type == TrainerType::Supporter && !can_play_support(state) {
        return cannot_play_trainer(); // dont even check which type it is
    }
    if trainer_card.trainer_card_type == TrainerType::Item && !can_play_item(state) {
        return cannot_play_trainer(); // cant play item cards
    }

    trainer_move_generation_implementation(state, trainer_card)
}

/// Returns None instead of panicing if the trainer card is not implemented; this is so that the
/// card_validation module can do "feature detection", and know if a card is implemented.
pub fn trainer_move_generation_implementation(
    state: &State,
    trainer_card: &TrainerCard,
) -> Option<Vec<SimpleAction>> {
    // Pokemon tools can be played if there is a space in the mat for them.
    if trainer_card.trainer_card_type == TrainerType::Tool {
        if is_tool_effect_implemented(trainer_card) {
            return can_play_tool(state, trainer_card);
        }
        return None;
    }

    // Stadium cards can be played if a different stadium is active (or none)
    if trainer_card.trainer_card_type == TrainerType::Stadium {
        if is_stadium_effect_implemented(trainer_card) {
            return can_play_stadium(state, trainer_card);
        }
        return None;
    }

    // Fossil cards are played as if they were Basic Pokemon
    if trainer_card.trainer_card_type == TrainerType::Fossil {
        return can_place_fossil(state, trainer_card);
    }

    // Dispatch on the first printing of this effect text, so every reprint shares one arm.
    let trainer_id = canonical_trainer_id(trainer_card)?;
    match trainer_id {
        // Complex cases: need to check specific conditions
        CardId::PA001Potion => can_play_potion(state, trainer_card),
        CardId::A1219Erika => can_play_erika(state, trainer_card),
        CardId::A1220Misty => can_play_misty(state, trainer_card),
        CardId::A1221Blaine => can_play_trainer(state, trainer_card),
        CardId::A2152Cynthia => can_play_trainer(state, trainer_card),
        CardId::A1224Brock => can_play_trainer(state, trainer_card),
        CardId::A2a072Irida => can_play_irida(state, trainer_card),
        CardId::A3155Lillie => can_play_lillie(state, trainer_card),
        CardId::A1222Koga => can_play_koga(state, trainer_card),
        CardId::A1225Sabrina => can_play_sabrina(state, trainer_card),
        CardId::A2150Cyrus => can_play_cyrus(state, trainer_card),
        CardId::A3152Lana => can_play_lana(state, trainer_card),
        CardId::A2155Mars => can_play_trainer(state, trainer_card),
        CardId::A3144RareCandy => can_play_rare_candy(state, trainer_card),
        CardId::A2b070PokemonCenterLady => can_play_pokemon_center_lady(state, trainer_card),
        CardId::A2154Dawn => can_play_dawn(state, trainer_card),
        CardId::A4151ElementalSwitch => can_play_elemental_switch(state, trainer_card),
        CardId::A3a064Repel => can_play_repel(state, trainer_card),
        CardId::A2146PokemonCommunication => can_play_pokemon_communication(state, trainer_card),
        CardId::A3a067Gladion => can_play_gladion(state, trainer_card),
        CardId::A3a069Lusamine => can_play_lusamine(state, trainer_card),
        CardId::A2153Volkner => can_play_volkner(state, trainer_card),
        CardId::A3149Ilima => can_play_ilima(state, trainer_card),
        CardId::A3154Mallow => can_play_mallow(state, trainer_card),
        CardId::A3150Kiawe => can_play_kiawe(state, trainer_card),
        CardId::A4157Lyra => can_play_lyra(state, trainer_card),
        // Simple cases: always can play
        CardId::A4158Silver
        | CardId::A4156Will
        | CardId::A4160Jasmine
        | CardId::PA002XSpeed
        | CardId::A2b111PokeBall
        | CardId::PA006RedCard
        | CardId::A4b373ProfessorsResearch
        | CardId::A1223Giovanni
        | CardId::A1a065MythicalSlab
        | CardId::A1a068Leaf
        | CardId::A2b071Red
        | CardId::A3151Guzma => can_play_trainer(state, trainer_card),
        CardId::A3b066EeveeBag => can_play_eevee_bag(state, trainer_card),
        CardId::B1217FlamePatch => can_play_flame_patch(state, trainer_card),
        CardId::B1225Copycat => can_play_trainer(state, trainer_card),
        CardId::A2b069Iono => can_play_trainer(state, trainer_card),
        CardId::B1221Marlon => can_play_marlon(state, trainer_card),
        CardId::B1223May => can_play_trainer(state, trainer_card),
        CardId::B1224Fantina => can_play_trainer(state, trainer_card),
        CardId::B1226Lisia => can_play_trainer(state, trainer_card),
        CardId::A2a073CelesticTownElder => can_play_celestic_town_elder(state, trainer_card),
        CardId::A2a074Barry => can_play_trainer(state, trainer_card),
        CardId::A2a075Adaman => can_play_trainer(state, trainer_card),
        CardId::B2149Diantha => can_play_diantha(state, trainer_card),
        CardId::B2152Piers => can_play_piers(state, trainer_card),
        CardId::B1a066ClemontsBackpack => can_play_trainer(state, trainer_card),
        CardId::B1a068Clemont => can_play_trainer(state, trainer_card),
        CardId::B1a067QuickGrowExtract => can_play_quick_grow_extract(state, trainer_card),
        CardId::B1a069Serena => can_play_trainer(state, trainer_card),
        CardId::B2a090Nemona => can_play_trainer(state, trainer_card),
        CardId::B2a091Arven => can_play_trainer(state, trainer_card),
        CardId::B2a086ElectricGenerator => can_play_electric_generator(state, trainer_card),
        CardId::B2a088TeamStarGrunt => can_play_team(state, trainer_card),
        CardId::A1216HelixFossil => can_play_fossil(state, trainer_card),
        CardId::B2145LuckyIcePop => can_play_lucky_ice_pop(state, trainer_card),
        CardId::B2b066Maintenance => can_play_maintenance(state, trainer_card),
        CardId::B2b067Iris => can_play_trainer(state, trainer_card),
        CardId::B2b068Calem => can_play_trainer(state, trainer_card),
        CardId::B2b065NastyNotice => can_play_trainer(state, trainer_card),
        CardId::A3b068Hau => can_play_trainer(state, trainer_card),
        CardId::A3142BigMalasada => can_play_big_malasada(state, trainer_card),
        CardId::B2150Sightseer => can_play_trainer(state, trainer_card),
        CardId::A2b072TeamRocketGrunt => can_play_team_rocket_grunt(state, trainer_card),
        CardId::B3147FieldBlower => can_play_field_blower(state, trainer_card),
        CardId::B3149Korrina => can_play_trainer(state, trainer_card),
        CardId::B3151Cheren => can_play_trainer(state, trainer_card),
        CardId::B3150Cabbie => can_play_cabbie(state, trainer_card),
        CardId::B3152ParasolLady => can_play_parasol_lady(state, trainer_card),
        CardId::B3a071Juliana => can_play_trainer(state, trainer_card),
        CardId::B3a072ProfessorSada => can_play_professor_sada(state, trainer_card),
        CardId::B3a073ProfessorTuro => can_play_professor_turo(state, trainer_card),
        CardId::B3b066Elesa => can_play_trainer(state, trainer_card),
        CardId::B3b067PuppyLovingGirl => can_play_trainer(state, trainer_card),
        CardId::B3b068Wallace => can_play_wallace(state, trainer_card),
        CardId::B4145OrderPad => can_play_trainer(state, trainer_card),
        CardId::B4152Skyla => can_play_skyla(state, trainer_card),
        CardId::B4153Wally => can_play_wally(state, trainer_card),
        CardId::B4150Psychic => can_play_psychic(state, trainer_card),
        CardId::B4151Drayden => can_play_trainer(state, trainer_card),
        CardId::B4a070TeamRocketsMasterPlan => can_play_trainer(state, trainer_card),
        CardId::B4a067TeamRocketsThievingMachine => {
            can_play_team_rockets_thieving_machine(state, trainer_card)
        }
        CardId::B4a068TeamRocketsGoozooka => can_play_trainer(state, trainer_card),
        CardId::B4a069TeamRocketsResearcher => can_play_trainer(state, trainer_card),
        CardId::B4a071TeamRocketsBoss => can_play_trainer(state, trainer_card),
        // Information-only cards. Looking at cards without moving them between zones has no
        // observable effect in this engine (the bots already forecast over the full deck
        // distribution), so these are playable and resolve as no-ops. See the matching arms in
        // `forecast_trainer_action` for the per-card reasoning.
        CardId::PA003HandScope
        | CardId::PA004PokedEx
        | CardId::A3a068Looker
        | CardId::A4161Hiker
        | CardId::A4a071Morty => can_play_trainer(state, trainer_card),
        CardId::A3145RotomDEx => can_play_trainer(state, trainer_card),
        CardId::A1a067Blue => can_play_trainer(state, trainer_card),
        CardId::A1226LtSurge => can_play_lt_surge(state, trainer_card),
        CardId::A1a064PokemonFlute => can_play_pokemon_flute(state, trainer_card),
        CardId::A1a066BuddingExpeditioner => can_play_budding_expeditioner(state, trainer_card),
        CardId::A2151TeamGalacticGrunt => can_play_trainer(state, trainer_card),
        CardId::A3143FishingNet => can_play_fishing_net(state, trainer_card),
        CardId::A3148Acerola => can_play_acerola(state, trainer_card),
        CardId::A3153Sophocles => can_play_trainer(state, trainer_card),
        CardId::A3a063BeastWall => can_play_beast_wall(state, trainer_card),
        CardId::A4152SquirtBottle => can_play_squirt_bottle(state, trainer_card),
        CardId::A4159Fisher => can_play_trainer(state, trainer_card),
        CardId::A4a069Whitney => can_play_whitney(state, trainer_card),
        CardId::A4a070TravelingMerchant => can_play_trainer(state, trainer_card),
        CardId::B1213PrankSpinner => can_play_prank_spinner(state, trainer_card),
        CardId::B1215HittingHammer => can_play_trainer(state, trainer_card),
        CardId::B1222Hala => can_play_trainer(state, trainer_card),
        CardId::B2151Juggler => can_play_juggler(state, trainer_card),
        CardId::A3b069Penny => can_play_trainer(state, trainer_card),
        _ => None,
    }
}

/// Lt. Surge: "Move all [L] Energy from your Benched Pokémon to your Raichu, Electrode, or
/// Electabuzz in the Active Spot." Needs a matching Active and at least one [L] Energy on the Bench.
fn can_play_lt_surge(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let active_matches = state
        .maybe_get_active(state.current_player)
        .is_some_and(|p| LT_SURGE_TARGETS.contains(&p.get_name().as_str()));
    let bench_has_lightning = state
        .enumerate_bench_pokemon(state.current_player)
        .any(|(_, p)| p.attached_energy.contains(&EnergyType::Lightning));
    if active_matches && bench_has_lightning {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Budding Expeditioner: "Put your Mew ex in the Active Spot into your hand."
fn can_play_budding_expeditioner(
    state: &State,
    trainer_card: &TrainerCard,
) -> Option<Vec<SimpleAction>> {
    let active_is_mew_ex = state
        .maybe_get_active(state.current_player)
        .is_some_and(|p| p.get_name() == "Mew ex");
    if active_is_mew_ex {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Pokémon Flute: "Put a Basic Pokémon from your opponent's discard pile onto their Bench."
fn can_play_pokemon_flute(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let opponent = (state.current_player + 1) % 2;
    let has_basic = state.discard_piles[opponent]
        .iter()
        .any(|card| matches!(card, Card::Pokemon(p) if p.stage == 0));
    let has_bench_space = state.in_play_pokemon[opponent]
        .iter()
        .enumerate()
        .any(|(i, slot)| i > 0 && slot.is_none());
    if has_basic && has_bench_space {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Fishing Net: "Put a random Basic [W] Pokémon from your discard pile into your hand."
fn can_play_fishing_net(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    if fishing_net_candidates(state, state.current_player).is_empty() {
        cannot_play_trainer()
    } else {
        can_play_trainer(state, trainer_card)
    }
}

/// Acerola: needs one of the actor's Palossand or Mimikyu to have damage on it.
fn can_play_acerola(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    if acerola_targets(state, state.current_player).is_empty() {
        cannot_play_trainer()
    } else {
        can_play_trainer(state, trainer_card)
    }
}

/// Beast Wall: "You can use this card only if your opponent hasn't gotten any points."
fn can_play_beast_wall(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let opponent = (state.current_player + 1) % 2;
    if state.points[opponent] == 0 {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Squirt Bottle: "Discard a [R] Energy from your opponent's Active Pokémon."
fn can_play_squirt_bottle(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let opponent = (state.current_player + 1) % 2;
    let has_fire = state
        .maybe_get_active(opponent)
        .is_some_and(|p| p.attached_energy.contains(&EnergyType::Fire));
    if has_fire {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Whitney: "Heal 60 damage from 1 of your Miltank..."
fn can_play_whitney(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let has_miltank = state
        .enumerate_in_play_pokemon(state.current_player)
        .any(|(_, p)| p.get_name() == "Miltank");
    if has_miltank {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Prank Spinner: needs at least one card across both hands to choose from.
fn can_play_prank_spinner(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    if state.hands[0].is_empty() && state.hands[1].is_empty() {
        cannot_play_trainer()
    } else {
        can_play_trainer(state, trainer_card)
    }
}

/// Juggler: "You can use this card only if your Pokémon in play have 3 or more different types of
/// Energy attached."
fn can_play_juggler(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    if distinct_attached_energy_types(state, state.current_player) >= 3 {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if a Fossil card can be played (requires at least 1 empty bench spot)
fn can_play_fossil(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let empty_bench_slots: Vec<_> = state.in_play_pokemon[state.current_player]
        .iter()
        .enumerate()
        .filter(|(i, p)| *i > 0 && p.is_none())
        .map(|(i, _)| i)
        .collect();

    if empty_bench_slots.is_empty() {
        cannot_play_trainer()
    } else {
        Some(
            empty_bench_slots
                .into_iter()
                .map(|i| SimpleAction::Place(Card::Trainer(trainer_card.clone()), i))
                .collect(),
        )
    }
}

/// Check if a Pokemon tool can be played (requires at least 1 pokemon in play without a tool)
fn can_play_tool(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let valid_targets = enumerate_tool_choices(trainer_card, state, state.current_player).len();
    if valid_targets > 0 {
        Some(vec![SimpleAction::Play {
            trainer_card: trainer_card.clone(),
        }])
    } else {
        Some(vec![])
    }
}

/// Check if a Stadium can be played (cannot play if same-named Stadium is already active)
fn can_play_stadium(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    // Cannot play same-name stadium
    if let Some(active_name) = state.get_active_stadium_name() {
        if active_name == trainer_card.name {
            return cannot_play_trainer();
        }
    }

    // Snorlax's Massive Body: as long as it is in the opponent's Active Spot, this player
    // can't play any Stadium cards from their hand.
    let opponent = (state.current_player + 1) % 2;
    let blocked_by_massive_body =
        state.in_play_pokemon[opponent][0]
            .as_ref()
            .is_some_and(|opponent_active| {
                matches!(
                    opponent_active.ability_mechanic(),
                    Some(AbilityMechanic::NoOpponentStadiumInActive)
                )
            });
    if blocked_by_massive_body {
        return cannot_play_trainer();
    }

    can_play_trainer(state, trainer_card)
}

/// Check if Potion can be played (requires at least 1 damaged pokemon in play)
fn can_play_potion(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let damaged_count = state
        .enumerate_in_play_pokemon(state.current_player)
        .filter(|(_, x)| x.is_damaged())
        .count();
    if damaged_count > 0 {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Ilima can be played (requires a damaged Colorless Pokemon in play)
fn can_play_ilima(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    if !ilima_targets(state, state.current_player).is_empty() {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

fn can_play_lucky_ice_pop(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    if let Some(active) = state.maybe_get_active(state.current_player) {
        if active.is_damaged() {
            return can_play_trainer(state, trainer_card);
        }
    }
    cannot_play_trainer()
}

/// Check if Erika can be played (requires at least 1 damaged Grass pokemon in play)
fn can_play_erika(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let damaged_grass_count = state
        .enumerate_in_play_pokemon(state.current_player)
        .filter(|(_, x)| x.is_damaged() && x.is_type(EnergyType::Grass))
        .count();
    if damaged_grass_count > 0 {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Names of Pokémon that Marlon can heal.
const MARLON_TARGETS: [&str; 2] = ["Carracosta", "Jellicent"];

/// Check if Marlon can be played (requires a damaged Carracosta or Jellicent in play)
fn can_play_marlon(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let has_valid_target = state
        .enumerate_in_play_pokemon(state.current_player)
        .any(|(_, x)| x.is_damaged() && MARLON_TARGETS.contains(&x.get_name().as_str()));
    if has_valid_target {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Irida can be played (requires at least 1 damaged pokemon with Water energy attached)
fn can_play_irida(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let damaged_water_count = state
        .enumerate_in_play_pokemon(state.current_player)
        .filter(|(_, x)| x.is_damaged() && x.attached_energy.contains(&EnergyType::Water))
        .count();
    if damaged_water_count > 0 {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Dawn can be played (requires active pokemon and at least 1 benched pokemon with energy)
fn can_play_dawn(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    if state.maybe_get_active(state.current_player).is_none() {
        return cannot_play_trainer();
    }
    let has_bench_with_energy = state
        .enumerate_bench_pokemon(state.current_player)
        .any(|(_, pokemon)| !pokemon.attached_energy.is_empty());
    if has_bench_with_energy {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

fn can_play_elemental_switch(
    state: &State,
    trainer_card: &TrainerCard,
) -> Option<Vec<SimpleAction>> {
    if state.maybe_get_active(state.current_player).is_none() {
        return cannot_play_trainer();
    }
    let allowed_types = [EnergyType::Fire, EnergyType::Water, EnergyType::Lightning];
    let has_valid_source =
        state
            .enumerate_bench_pokemon(state.current_player)
            .any(|(_, pokemon)| {
                pokemon
                    .attached_energy
                    .iter()
                    .any(|energy| allowed_types.contains(energy))
            });

    if has_valid_source {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Pokemon Center Lady can be played (requires at least 1 damaged or status-affected pokemon)
fn can_play_pokemon_center_lady(
    state: &State,
    trainer_card: &TrainerCard,
) -> Option<Vec<SimpleAction>> {
    let has_valid_target = state
        .enumerate_in_play_pokemon(state.current_player)
        .any(|(_, pokemon)| pokemon.is_damaged() || pokemon.has_status_condition());
    if has_valid_target {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Lillie can be played (requires at least 1 damaged Stage 2 pokemon in play)
fn can_play_lillie(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let damaged_stage2_count = state
        .enumerate_in_play_pokemon(state.current_player)
        .filter(|(_, x)| x.is_damaged() && get_stage(x) == 2)
        .count();
    if damaged_stage2_count > 0 {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Misty can be played (requires at least 1 water pokemon in play)
fn can_play_misty(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let water_in_player_count = state.num_in_play_of_type(state.current_player, EnergyType::Water);
    if water_in_player_count > 0 {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Koga can be played (requires active pokemon to be Weezing or Muk)
fn can_play_koga(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let active_pokemon = &state.maybe_get_active(state.current_player);
    if let Some(played_card) = active_pokemon {
        let card_id =
            CardId::from_card_id(played_card.get_id().as_str()).expect("CardId should be known");
        match card_id {
            CardId::A1177Weezing | CardId::A1243Weezing | CardId::A1175Muk => {
                return can_play_trainer(state, trainer_card);
            }
            _ => {}
        }
    }
    cannot_play_trainer()
}

/// Check if Sabrina can be played (requires opponent to have benched pokemon)
fn can_play_sabrina(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let opponent = (state.current_player + 1) % 2;
    let opponent_has_bench = state.enumerate_bench_pokemon(opponent).count() > 0;
    if opponent_has_bench {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Cyrus can be played (requires opponent to have at least 1 damaged bench pokemon)
fn can_play_cyrus(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let opponent = (state.current_player + 1) % 2;
    let damaged_bench_count = state
        .enumerate_bench_pokemon(opponent)
        .filter(|(_, x)| x.is_damaged())
        .count();
    if damaged_bench_count > 0 {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Lana can be played (requires Araquanid in play and opponent to have a benched pokemon)
fn can_play_lana(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let has_araquanid = state
        .enumerate_in_play_pokemon(state.current_player)
        .any(|(_, pokemon)| pokemon.get_name() == "Araquanid");
    let opponent = (state.current_player + 1) % 2;
    let opponent_has_bench = state.enumerate_bench_pokemon(opponent).count() > 0;
    if has_araquanid && opponent_has_bench {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Repel can be played (requires opponent's active to be a Basic pokemon)
fn can_play_repel(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let opponent = (state.current_player + 1) % 2;
    let opponent_active = &state.maybe_get_active(opponent);
    let opponent_bench_count = state.enumerate_bench_pokemon(opponent).count();
    if let Some(opponent_active) = opponent_active {
        if opponent_active.card.is_basic() && opponent_bench_count > 0 {
            return can_play_trainer(state, trainer_card);
        }
    }
    cannot_play_trainer()
}

fn can_play_rare_candy(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    if state.is_users_first_turn() {
        return cannot_play_trainer();
    }

    let player = state.current_player;
    let hand = &state.hands[player];

    // Check if there's at least 1 basic pokemon in field with a corresponding stage2-rare-candy-evolvable in hand
    let has_valid_evolution_pair = state
        .enumerate_in_play_pokemon(player)
        .any(|(_, in_play)| hand.iter().any(|card| can_rare_candy_evolve(card, in_play)));
    if has_valid_evolution_pair {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Pokemon Communication can be played (requires at least 1 Pokemon in hand and 1 in deck)
fn can_play_pokemon_communication(
    state: &State,
    trainer_card: &TrainerCard,
) -> Option<Vec<SimpleAction>> {
    let player = state.current_player;
    let has_pokemon_in_hand = state.hands[player]
        .iter()
        .any(|card| matches!(card, Card::Pokemon(_)));
    let has_pokemon_in_deck = state.decks[player]
        .cards
        .iter()
        .any(|card| matches!(card, Card::Pokemon(_)));
    if has_pokemon_in_hand && has_pokemon_in_deck {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Gladion can be played (requires possibility of Type: Null or Silvally in deck)
fn can_play_gladion(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let player = state.current_player;

    // Count Type: Null and Silvally in play and discard
    let mut type_null_count = 0;
    let mut silvally_count = 0;

    // Count in play Pokemon (including cards_behind)
    for pokemon in state.in_play_pokemon[player].iter().flatten() {
        // Check the current card
        if pokemon.get_name() == "Type: Null" {
            type_null_count += 1;
        } else if pokemon.get_name() == "Silvally" {
            silvally_count += 1;
        }

        // Check cards_behind (evolution chain)
        for card in &pokemon.cards_behind {
            if card.get_name() == "Type: Null" {
                type_null_count += 1;
            } else if card.get_name() == "Silvally" {
                silvally_count += 1;
            }
        }
    }

    // Count in discard pile
    for card in &state.discard_piles[player] {
        if card.get_name() == "Type: Null" {
            type_null_count += 1;
        } else if card.get_name() == "Silvally" {
            silvally_count += 1;
        }
    }

    // Can play if we haven't accounted for all 2 Type: Null and 2 Silvally
    // (meaning there might still be some in the deck)
    if type_null_count >= 2 && silvally_count >= 2 {
        cannot_play_trainer()
    } else {
        can_play_trainer(state, trainer_card)
    }
}

fn can_play_professor_sada(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let player = state.current_player;

    let has_ancient = state.in_play_pokemon[player]
        .iter()
        .flatten()
        .any(|pokemon| is_ancient_pokemon(&pokemon.get_name()));
    if !has_ancient {
        return cannot_play_trainer();
    }

    if state.discard_energies[player].is_empty() {
        return cannot_play_trainer();
    }

    can_play_trainer(state, trainer_card)
}

/// Check if Lusamine can be played (requires opponent has >= 1 point, player has Ultra Beast, >= 1 energy in discard)
fn can_play_lusamine(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let player = state.current_player;
    let opponent = (player + 1) % 2;

    // Check if opponent has at least 1 point
    if state.points[opponent] < 1 {
        return cannot_play_trainer();
    }

    // Check if player has at least 1 Ultra Beast in play
    let has_ultra_beast = state.in_play_pokemon[player]
        .iter()
        .flatten()
        .any(|pokemon| is_ultra_beast(&pokemon.get_name()));
    if !has_ultra_beast {
        return cannot_play_trainer();
    }

    // Check if player has at least 1 energy in discard
    if state.discard_energies[player].is_empty() {
        return cannot_play_trainer();
    }

    can_play_trainer(state, trainer_card)
}

/// Check if Volkner can be played (requires an Electivire or Luxray in play and
/// at least 1 Lightning Energy in discard)
fn can_play_volkner(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let player = state.current_player;

    let has_valid_target = state
        .enumerate_in_play_pokemon(player)
        .any(|(_, pokemon)| matches!(pokemon.get_name().as_str(), "Electivire" | "Luxray"));
    if !has_valid_target {
        return cannot_play_trainer();
    }

    let has_lightning_in_discard = state.discard_energies[player].contains(&EnergyType::Lightning);
    if !has_lightning_in_discard {
        return cannot_play_trainer();
    }

    can_play_trainer(state, trainer_card)
}

/// Check if Kiawe can be played (requires Alolan Marowak or Turtonator in play)
fn can_play_kiawe(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let has_valid_target = state
        .enumerate_in_play_pokemon(state.current_player)
        .any(|(_, pokemon)| matches!(pokemon.get_name().as_str(), "Alolan Marowak" | "Turtonator"));
    if has_valid_target {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Lyra can be played (requires active pokemon to have damage and at least 1 benched pokemon)
fn can_play_lyra(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let player = state.current_player;
    let active_pokemon = state.maybe_get_active(player);
    let bench_count = state.enumerate_bench_pokemon(player).count();

    if let Some(active) = active_pokemon {
        if active.is_damaged() && bench_count > 0 {
            return can_play_trainer(state, trainer_card);
        }
    }
    cannot_play_trainer()
}

/// Check if Skyla can be played (requires a Stage 1 Active Pokémon and at least 1 benched Pokémon)
fn can_play_skyla(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let player = state.current_player;
    let active_is_stage_1 = state
        .maybe_get_active(player)
        .is_some_and(|active| get_stage(active) == 1);
    let has_bench = state.enumerate_bench_pokemon(player).count() > 0;
    if active_is_stage_1 && has_bench {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Wally can be played (requires at least 1 Stage 2 Pokémon in play)
fn can_play_wally(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let has_stage_2 = state
        .enumerate_in_play_pokemon(state.current_player)
        .any(|(_, pokemon)| get_stage(pokemon) == 2);
    if has_stage_2 {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Psychic (Supporter) can be played (requires the Active Pokémon to have the Psychic
/// attack, and an opponent's Benched Pokémon with Energy to move)
fn can_play_psychic(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let player = state.current_player;
    let opponent = (player + 1) % 2;
    if active_has_psychic_attack(state, player)
        && !psychic_energy_sources(state, opponent).is_empty()
    {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Eevee Bag can be played (requires at least 1 Pokemon that evolved from Eevee in play)
fn can_play_eevee_bag(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let has_eevee_evolution = state
        .enumerate_in_play_pokemon(state.current_player)
        .any(|(_, pokemon)| pokemon.evolved_from("Eevee"));
    if has_eevee_evolution {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Flame Patch can be played (requires active Fire pokemon and Fire energy in discard)
fn can_play_flame_patch(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let player = state.current_player;
    let active_pokemon = state.maybe_get_active(player);

    // Check if active pokemon exists and is Fire type
    let active_is_fire = active_pokemon
        .map(|p| p.is_type(EnergyType::Fire))
        .unwrap_or(false);

    // Check if there's at least 1 Fire energy in discard pile
    let has_fire_energy_in_discard = state.discard_energies[player].contains(&EnergyType::Fire);

    if active_is_fire && has_fire_energy_in_discard {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Electric Generator can be played (requires at least 1 Benched Lightning Pokemon)
fn can_play_electric_generator(
    state: &State,
    trainer_card: &TrainerCard,
) -> Option<Vec<SimpleAction>> {
    let has_lightning_bench_target = state
        .enumerate_bench_pokemon(state.current_player)
        .any(|(_, pokemon)| pokemon.is_type(EnergyType::Lightning));

    if has_lightning_bench_target {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Piers can be played (requires Galarian Obstagoon in play)
fn can_play_piers(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let has_obstagoon = state
        .enumerate_in_play_pokemon(state.current_player)
        .any(|(_, pokemon)| pokemon.get_name() == "Galarian Obstagoon");
    if has_obstagoon {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Diantha can be played (requires damaged Psychic Pokemon with >= 2 Psychic Energy)
/// Check if Mallow can be played (requires a damaged Shiinotic or Tsareena in play)
fn can_play_mallow(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    if mallow_targets(state, state.current_player).is_empty() {
        cannot_play_trainer()
    } else {
        can_play_trainer(state, trainer_card)
    }
}

fn can_play_diantha(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let has_target = !diantha_targets(state, state.current_player).is_empty();
    if has_target {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Team Rocket's Thieving Machine can be played (requires an Item card, other than
/// itself, in the opponent's discard pile)
fn can_play_team_rockets_thieving_machine(
    state: &State,
    trainer_card: &TrainerCard,
) -> Option<Vec<SimpleAction>> {
    let opponent = (state.current_player + 1) % 2;
    let has_target = state.discard_piles[opponent]
        .iter()
        .any(is_thieving_machine_eligible_item);
    if has_target {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

fn is_thieving_machine_eligible_item(card: &Card) -> bool {
    matches!(card, Card::Trainer(t) if t.trainer_card_type == TrainerType::Item && t.name != "Team Rocket's Thieving Machine")
}

/// Check if Celestic Town Elder can be played (requires at least 1 Basic Pokemon in discard pile)
fn can_play_celestic_town_elder(
    state: &State,
    trainer_card: &TrainerCard,
) -> Option<Vec<SimpleAction>> {
    let player = state.current_player;
    let has_basic_pokemon_in_discard = state.discard_piles[player]
        .iter()
        .any(|card| card.is_basic());
    if has_basic_pokemon_in_discard {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Team can be played (requires opponent to have any Ability Pokemon with attached Energy)
fn can_play_team(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let opponent = (state.current_player + 1) % 2;
    let has_target = state
        .enumerate_in_play_pokemon(opponent)
        .any(|(_, pokemon)| pokemon.ability().is_some() && !pokemon.attached_energy.is_empty());

    if has_target {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Fossil cards can be placed in empty Active or Bench slots, like Basic Pokemon
fn can_place_fossil(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let current_player = state.current_player;
    let mut actions = Vec::new();

    // Fossils can be placed in any empty slot
    state.in_play_pokemon[current_player]
        .iter()
        .enumerate()
        .for_each(|(i, x)| {
            if x.is_none() {
                actions.push(SimpleAction::Place(Card::Trainer(trainer_card.clone()), i));
            }
        });

    Some(actions)
}

/// Check if Big Malasada can be played (requires active pokemon to be damaged or have a special condition)
fn can_play_big_malasada(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    if let Some(active) = state.maybe_get_active(state.current_player) {
        if active.is_damaged() || active.has_status_condition() {
            return can_play_trainer(state, trainer_card);
        }
    }
    cannot_play_trainer()
}

/// Check if Quick-Grow Extract can be played
/// Requires: not first turn, at least 1 Grass pokemon that wasn't played this turn,
/// with a valid Grass evolution available in deck
fn can_play_quick_grow_extract(
    state: &State,
    trainer_card: &TrainerCard,
) -> Option<Vec<SimpleAction>> {
    // Can't use during first turn
    if state.is_users_first_turn() {
        return cannot_play_trainer();
    }

    // Check if there are any valid evolution candidates
    if quick_grow_extract_candidates(state, state.current_player).is_empty() {
        cannot_play_trainer()
    } else {
        can_play_trainer(state, trainer_card)
    }
}

/// Check if Wallace can be played
/// Requires at least 1 Water pokemon with 50 HP or less, that wasn't played this turn,
/// with a valid Water evolution available in deck
fn can_play_wallace(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    if wallace_candidates(state, state.current_player).is_empty() {
        cannot_play_trainer()
    } else {
        can_play_trainer(state, trainer_card)
    }
}

/// Check if Team Rocket Grunt can be played (requires opponent's Active Pokémon to have energy)
fn can_play_team_rocket_grunt(
    state: &State,
    trainer_card: &TrainerCard,
) -> Option<Vec<SimpleAction>> {
    let opponent = (state.current_player + 1) % 2;
    let has_energy = state
        .maybe_get_active(opponent)
        .map(|p| !p.attached_energy.is_empty())
        .unwrap_or(false);
    if has_energy {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Maintenance can be played (requires 2 other cards in hand after playing it)
fn can_play_maintenance(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    if state.hands[state.current_player].len() >= 3 {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Field Blower can be played (requires any Pokémon with a tool attached, or an active stadium)
fn can_play_field_blower(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let any_tool = (0..2)
        .flat_map(|player| state.enumerate_in_play_pokemon(player))
        .any(|(_, pokemon)| pokemon.has_tool_attached());
    let any_stadium = state.active_stadium.is_some();
    if any_tool || any_stadium {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Cabbie can be played (requires at least one Stadium card in the deck)
fn can_play_cabbie(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let has_stadium = state.decks[state.current_player].cards.iter().any(
        |card| matches!(card, Card::Trainer(tc) if tc.trainer_card_type == TrainerType::Stadium),
    );
    if has_stadium {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Parasol Lady can be played (requires at least one [W] non-ex Pokémon in play)
fn can_play_parasol_lady(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let has_target = state
        .enumerate_in_play_pokemon(state.current_player)
        .any(|(_, pokemon)| pokemon.is_type(EnergyType::Water) && !pokemon.card.is_ex());
    if has_target {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Professor Turo can be played (requires at least one Future Pokémon in play)
fn can_play_professor_turo(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let has_future = state
        .enumerate_in_play_pokemon(state.current_player)
        .any(|(_, pokemon)| is_future_pokemon(&pokemon.get_name()));
    if has_future {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}
