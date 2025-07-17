use crate::{
    actions::SimpleAction,
    card_ids::CardId,
    database::get_card_by_enum,
    hooks::can_play_support,
    types::{EnergyType, TrainerCard, TrainerType},
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
///
/// Returns None instead of panicing if the trainer card is not implemented; this is so that the
/// WASM module can do "feature detection", and know if a card is implemented.
pub fn generate_possible_trainer_actions(
    state: &State,
    trainer_card: &TrainerCard,
) -> Option<Vec<SimpleAction>> {
    if trainer_card.trainer_card_type == TrainerType::Supporter && !can_play_support(state) {
        return Some(vec![]); // dont even check which type it is
    }

    // Pokemon tools can be played if there is a space in the mat for them.
    if trainer_card.trainer_card_type == TrainerType::Tool {
        return can_play_tool(state, trainer_card);
    }

    let trainer_id = CardId::from_numeric_id(trainer_card.numeric_id).expect("CardId should exist");
    match trainer_id {
        CardId::PA001Potion => can_play_potion(state, trainer_card),
        CardId::A1219Erika | CardId::A1266Erika => can_play_erika(state, trainer_card),
        CardId::A1220Misty | CardId::A1267Misty => can_play_misty(state, trainer_card),
        CardId::A1222Koga | CardId::A1269Koga => can_play_koga(state, trainer_card),
        CardId::A1225Sabrina | CardId::A1272Sabrina => can_play_sabrina(state, trainer_card),
        CardId::A2150Cyrus | CardId::A2190Cyrus => can_play_cyrus(state, trainer_card),
        CardId::PA002XSpeed
        | CardId::PA005PokeBall
        | CardId::PA006RedCard
        | CardId::PA007ProfessorsResearch
        | CardId::A1223Giovanni
        | CardId::A1270Giovanni
        | CardId::A1a065MythicalSlab
        | CardId::A1a068Leaf
        | CardId::A1a082Leaf => can_play_trainer(state, trainer_card),
        _ => None,
    }
}

/// Check if a Pokemon tool can be played (requires at least 1 pokemon in play without a tool)
fn can_play_tool(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let in_play_without_tools = state
        .enumerate_in_play_pokemon(state.current_player)
        .filter(|(_, x)| x.has_tool_attached())
        .count();
    if in_play_without_tools > 0 {
        return Some(vec![SimpleAction::Play {
            trainer_card: trainer_card.clone(),
        }]);
    } else {
        return Some(vec![]);
    }
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
    let active_pokemon = &state.in_play_pokemon[state.current_player][0];
    if let Some(played_card) = active_pokemon {
        let kogable_cards = vec![
            get_card_by_enum(CardId::A1177Weezing),
            get_card_by_enum(CardId::A1243Weezing),
            get_card_by_enum(CardId::A1175Muk),
        ];
        if kogable_cards.contains(&played_card.card) {
            return can_play_trainer(state, trainer_card);
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
