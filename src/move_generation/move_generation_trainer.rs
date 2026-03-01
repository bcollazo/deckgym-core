use crate::{
    actions::SimpleAction,
    card_ids::CardId,
    card_logic::{
        can_rare_candy_evolve, diantha_targets, ilima_targets, quick_grow_extract_candidates,
    },
    hooks::{can_play_item, can_play_support, get_stage, is_ultra_beast},
    models::{Card, EnergyType, TrainerCard, TrainerType},
    stadiums::is_stadium_effect_implemented,
    tools::{enumerate_tool_choices, is_tool_effect_implemented},
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

    let trainer_id = CardId::from_card_id(trainer_card.id.as_str()).expect("CardId should exist");
    match trainer_id {
        // Complex cases: need to check specific conditions
        CardId::PA001Potion => can_play_potion(state, trainer_card),
        CardId::A1219Erika | CardId::A1266Erika | CardId::A4b328Erika | CardId::A4b329Erika => {
            can_play_erika(state, trainer_card)
        }
        CardId::A1220Misty | CardId::A1267Misty => can_play_misty(state, trainer_card),
        CardId::A1221Blaine | CardId::A1268Blaine => can_play_trainer(state, trainer_card),
        CardId::A1224Brock | CardId::A1271Brock => can_play_trainer(state, trainer_card),
        CardId::A2a072Irida | CardId::A2a087Irida | CardId::A4b330Irida | CardId::A4b331Irida => {
            can_play_irida(state, trainer_card)
        }
        CardId::A3155Lillie
        | CardId::A3197Lillie
        | CardId::A3209Lillie
        | CardId::A4b348Lillie
        | CardId::A4b349Lillie
        | CardId::A4b374Lillie => can_play_lillie(state, trainer_card),
        CardId::A1222Koga | CardId::A1269Koga => can_play_koga(state, trainer_card),
        CardId::A1225Sabrina
        | CardId::A1272Sabrina
        | CardId::A4b338Sabrina
        | CardId::A4b339Sabrina => can_play_sabrina(state, trainer_card),
        CardId::A2150Cyrus | CardId::A2190Cyrus | CardId::A4b326Cyrus | CardId::A4b327Cyrus => {
            can_play_cyrus(state, trainer_card)
        }
        CardId::A2155Mars | CardId::A2195Mars | CardId::A4b344Mars | CardId::A4b345Mars => {
            can_play_trainer(state, trainer_card)
        }
        CardId::A3144RareCandy
        | CardId::A4b314RareCandy
        | CardId::A4b315RareCandy
        | CardId::A4b379RareCandy => can_play_rare_candy(state, trainer_card),
        CardId::A2b070PokemonCenterLady | CardId::A2b089PokemonCenterLady => {
            can_play_pokemon_center_lady(state, trainer_card)
        }
        CardId::A4151ElementalSwitch
        | CardId::A4b310ElementalSwitch
        | CardId::A4b311ElementalSwitch => can_play_elemental_switch(state, trainer_card),
        CardId::A3a064Repel => can_play_repel(state, trainer_card),
        CardId::A2146PokemonCommunication
        | CardId::A4b316PokemonCommunication
        | CardId::A4b317PokemonCommunication => can_play_pokemon_communication(state, trainer_card),
        CardId::A3a067Gladion | CardId::A3a081Gladion => can_play_gladion(state, trainer_card),
        CardId::A3a069Lusamine
        | CardId::A3a083Lusamine
        | CardId::A4b350Lusamine
        | CardId::A4b351Lusamine
        | CardId::A4b375Lusamine => can_play_lusamine(state, trainer_card),
        CardId::A3149Ilima | CardId::A3191Ilima => can_play_ilima(state, trainer_card),
        CardId::A4157Lyra | CardId::A4197Lyra | CardId::A4b332Lyra | CardId::A4b333Lyra => {
            can_play_lyra(state, trainer_card)
        }
        // Simple cases: always can play
        CardId::A4158Silver
        | CardId::A4198Silver
        | CardId::A4b336Silver
        | CardId::A4b337Silver
        | CardId::PA002XSpeed
        | CardId::PA005PokeBall
        | CardId::A2b111PokeBall
        | CardId::PA006RedCard
        | CardId::PA007ProfessorsResearch
        | CardId::A4b373ProfessorsResearch
        | CardId::A1223Giovanni
        | CardId::A1270Giovanni
        | CardId::A4b334Giovanni
        | CardId::A4b335Giovanni
        | CardId::A1a065MythicalSlab
        | CardId::A1a068Leaf
        | CardId::A1a082Leaf
        | CardId::A4b346Leaf
        | CardId::A4b347Leaf
        | CardId::A2b071Red
        | CardId::A2b090Red
        | CardId::A4b352Red
        | CardId::A4b353Red
        | CardId::A3151Guzma
        | CardId::A3193Guzma
        | CardId::A3208Guzma => can_play_trainer(state, trainer_card),
        CardId::A3b066EeveeBag
        | CardId::A3b107EeveeBag
        | CardId::A4b308EeveeBag
        | CardId::A4b309EeveeBag => can_play_eevee_bag(state, trainer_card),
        CardId::B1217FlamePatch | CardId::B1331FlamePatch => {
            can_play_flame_patch(state, trainer_card)
        }
        CardId::B1225Copycat | CardId::B1270Copycat => can_play_trainer(state, trainer_card),
        CardId::A2b069Iono | CardId::A2b088Iono | CardId::A4b340Iono | CardId::A4b341Iono => {
            can_play_trainer(state, trainer_card)
        }
        CardId::B1223May | CardId::B1268May => can_play_trainer(state, trainer_card),
        CardId::B1224Fantina | CardId::B1269Fantina => can_play_trainer(state, trainer_card),
        CardId::B1226Lisia | CardId::B1271Lisia => can_play_trainer(state, trainer_card),
        CardId::A2a073CelesticTownElder | CardId::A2a088CelesticTownElder => {
            can_play_celestic_town_elder(state, trainer_card)
        }
        CardId::A2a075Adaman | CardId::A2a090Adaman => can_play_trainer(state, trainer_card),
        CardId::B2149Diantha | CardId::B2190Diantha => can_play_diantha(state, trainer_card),
        CardId::B2152Piers | CardId::B2193Piers => can_play_piers(state, trainer_card),
        CardId::B1a066ClemontsBackpack => can_play_trainer(state, trainer_card),
        CardId::B1a068Clemont | CardId::B1a081Clemont => can_play_trainer(state, trainer_card),
        CardId::B1a067QuickGrowExtract | CardId::B1a103QuickGrowExtract => {
            can_play_quick_grow_extract(state, trainer_card)
        }
        CardId::B1a069Serena | CardId::B1a082Serena => can_play_trainer(state, trainer_card),
        CardId::B2a090Nemona | CardId::B2a107Nemona => can_play_trainer(state, trainer_card),
        CardId::B2a091Arven | CardId::B2a108Arven | CardId::B2a115Arven => {
            can_play_trainer(state, trainer_card)
        }
        CardId::A1216HelixFossil
        | CardId::A1217DomeFossil
        | CardId::A1218OldAmber
        | CardId::A1a063OldAmber
        | CardId::A2144SkullFossil
        | CardId::A2145ArmorFossil
        | CardId::A4b312OldAmber
        | CardId::A4b313OldAmber
        | CardId::B1214PlumeFossil
        | CardId::B1216CoverFossil => can_play_fossil(state, trainer_card),
        CardId::B2145LuckyIcePop => can_play_lucky_ice_pop(state, trainer_card),
        _ => None,
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
        .filter(|(_, x)| x.is_damaged() && x.get_energy_type() == Some(EnergyType::Grass))
        .count();
    if damaged_grass_count > 0 {
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
        .map(|p| p.get_energy_type() == Some(EnergyType::Fire))
        .unwrap_or(false);

    // Check if there's at least 1 Fire energy in discard pile
    let has_fire_energy_in_discard = state.discard_energies[player].contains(&EnergyType::Fire);

    if active_is_fire && has_fire_energy_in_discard {
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
fn can_play_diantha(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let has_target = !diantha_targets(state, state.current_player).is_empty();
    if has_target {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
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
