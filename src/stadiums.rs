use std::sync::LazyLock;

use crate::{
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, TrainerCard, TrainerType},
    State,
};

pub(crate) fn ensure_stadium_card(card: &Card) -> &TrainerCard {
    match card {
        Card::Trainer(trainer_card) => ensure_stadium_trainer(trainer_card),
        _ => panic!("Expected TrainerCard of subtype Stadium, got non-trainer card"),
    }
}

pub(crate) fn ensure_stadium_trainer(trainer_card: &TrainerCard) -> &TrainerCard {
    if trainer_card.trainer_card_type != TrainerType::Stadium {
        panic!(
            "Expected TrainerCard of subtype Stadium, got {:?}",
            trainer_card.trainer_card_type
        );
    }
    trainer_card
}

fn stadium_effect_text_from_card_id(stadium_card_id: CardId) -> String {
    let card = get_card_by_enum(stadium_card_id);
    let trainer_card = ensure_stadium_card(&card);
    trainer_card.effect.clone()
}

static PECULIAR_PLAZA_EFFECT: LazyLock<String> =
    LazyLock::new(|| stadium_effect_text_from_card_id(CardId::B2155PeculiarPlaza));
static TRAINING_AREA_EFFECT: LazyLock<String> =
    LazyLock::new(|| stadium_effect_text_from_card_id(CardId::B2153TrainingArea));
static STARTING_PLAINS_EFFECT: LazyLock<String> =
    LazyLock::new(|| stadium_effect_text_from_card_id(CardId::B2154StartingPlains));
static MESAGOZA_EFFECT: LazyLock<String> =
    LazyLock::new(|| stadium_effect_text_from_card_id(CardId::B2a093Mesagoza));
static HIKING_TRAIL_EFFECT: LazyLock<String> =
    LazyLock::new(|| stadium_effect_text_from_card_id(CardId::B2b069HikingTrail));
static BOUNDED_FIELD_EFFECT: LazyLock<String> =
    LazyLock::new(|| stadium_effect_text_from_card_id(CardId::B3155BoundedField));
static ARENA_OF_ANTIQUITY_EFFECT: LazyLock<String> =
    LazyLock::new(|| stadium_effect_text_from_card_id(CardId::B3154ArenaofAntiquity));
static FRAGRANT_FOREST_EFFECT: LazyLock<String> =
    LazyLock::new(|| stadium_effect_text_from_card_id(CardId::B3153FragrantForest));
static AREA_ZERO_EFFECT: LazyLock<String> =
    LazyLock::new(|| stadium_effect_text_from_card_id(CardId::B3a074AreaZero));
static KIDS_ROOM_EFFECT: LazyLock<String> =
    LazyLock::new(|| stadium_effect_text_from_card_id(CardId::B3b069KidsRoom));

pub fn is_stadium_effect_implemented(trainer_card: &TrainerCard) -> bool {
    ensure_stadium_trainer(trainer_card);
    let effect = trainer_card.effect.as_str();
    matches!(
        effect,
        e if e == PECULIAR_PLAZA_EFFECT.as_str()
            || e == TRAINING_AREA_EFFECT.as_str()
            || e == STARTING_PLAINS_EFFECT.as_str()
            || e == MESAGOZA_EFFECT.as_str()
            || e == HIKING_TRAIL_EFFECT.as_str()
            || e == BOUNDED_FIELD_EFFECT.as_str()
            || e == ARENA_OF_ANTIQUITY_EFFECT.as_str()
            || e == FRAGRANT_FOREST_EFFECT.as_str()
            || e == AREA_ZERO_EFFECT.as_str()
            || e == KIDS_ROOM_EFFECT.as_str()
    )
}

pub fn is_hiking_trail_active(state: &State) -> bool {
    has_stadium(state, CardId::B2b069HikingTrail)
}

pub fn is_bounded_field_active(state: &State) -> bool {
    has_stadium(state, CardId::B3155BoundedField)
}

/// Returns true if Mesagoza stadium is active
pub fn is_mesagoza_active(state: &State) -> bool {
    has_stadium(state, CardId::B2a093Mesagoza)
}

/// Returns true if the player can use Mesagoza's effect (stadium is active, not used this turn, deck has Pokemon)
pub fn can_use_mesagoza(state: &State, player: usize) -> bool {
    if !is_mesagoza_active(state) {
        return false;
    }
    if state.has_used_stadium[player] {
        return false;
    }
    // Must have at least one Pokemon in deck
    state.decks[player]
        .cards
        .iter()
        .any(|card| matches!(card, Card::Pokemon(_)))
}

pub fn has_stadium(state: &State, reference_stadium_id: CardId) -> bool {
    let reference_effect = stadium_effect_text_from_card_id(reference_stadium_id);
    let Some(active_stadium) = &state.active_stadium else {
        return false;
    };
    let trainer_card = ensure_stadium_card(active_stadium);
    trainer_card.effect == reference_effect
}

pub fn is_starting_plains_active(state: &State) -> bool {
    has_stadium(state, CardId::B2154StartingPlains)
}

/// Returns the retreat cost reduction for Peculiar Plaza.
/// Peculiar Plaza: "The Retreat Cost of each [P] Pokemon in play (both yours and your opponent's) is 2 less."
pub fn get_peculiar_plaza_retreat_reduction(state: &State, energy_type: EnergyType) -> u8 {
    if energy_type == EnergyType::Psychic && has_stadium(state, CardId::B2155PeculiarPlaza) {
        2
    } else {
        0
    }
}

/// Returns the damage bonus for Training Area.
/// Training Area: "Attacks used by Stage 1 Pokémon in play (both yours and your opponent's) do +10 damage to the opponent's Active Pokémon."
pub fn get_training_area_damage_bonus(state: &State, attacker_stage: u8) -> u32 {
    if attacker_stage == 1 && has_stadium(state, CardId::B2153TrainingArea) {
        10
    } else {
        0
    }
}

pub fn is_arena_of_antiquity_active(state: &State) -> bool {
    has_stadium(state, CardId::B3154ArenaofAntiquity)
}

/// Returns the damage bonus for Arena of Antiquity.
/// Arena of Antiquity: "Attacks used by each [F] Pokémon in play (both yours and your opponent's) do +20 damage to the opponent's Active Pokémon ex."
pub fn get_arena_of_antiquity_damage_bonus(
    state: &State,
    attacker_energy_type: EnergyType,
    target_is_ex: bool,
) -> u32 {
    if attacker_energy_type == EnergyType::Fighting
        && target_is_ex
        && is_arena_of_antiquity_active(state)
    {
        20
    } else {
        0
    }
}

pub fn is_fragrant_forest_active(state: &State) -> bool {
    has_stadium(state, CardId::B3153FragrantForest)
}

/// Returns true if the player can use Fragrant Forest's effect (stadium is active, not used this turn, deck has Basic Grass Pokemon)
pub fn can_use_fragrant_forest(state: &State, player: usize) -> bool {
    if !is_fragrant_forest_active(state) {
        return false;
    }
    if state.has_used_stadium[player] {
        return false;
    }
    state.decks[player]
        .cards
        .iter()
        .any(|card| matches!(card, Card::Pokemon(p) if p.stage == 0 && p.energy_type == EnergyType::Grass))
}

pub fn is_area_zero_active(state: &State) -> bool {
    has_stadium(state, CardId::B3a074AreaZero)
}

/// Returns true if the player can use Area Zero's effect (stadium is active, not used this turn, hand has Basic Pokemon)
pub fn can_use_area_zero(state: &State, player: usize) -> bool {
    if !is_area_zero_active(state) {
        return false;
    }
    if state.has_used_stadium[player] {
        return false;
    }
    state.hands[player].iter().any(|card| card.is_basic())
}

pub fn is_kids_room_active(state: &State) -> bool {
    has_stadium(state, CardId::B3b069KidsRoom)
}

/// Returns true if the player can use Kid's Room's effect (stadium is active, not used this turn,
/// hand has a card, and deck has a Pokemon Tool card)
pub fn can_use_kids_room(state: &State, player: usize) -> bool {
    if !is_kids_room_active(state) {
        return false;
    }
    if state.has_used_stadium[player] {
        return false;
    }
    if state.hands[player].is_empty() {
        return false;
    }
    state.decks[player]
        .cards
        .iter()
        .any(|card| matches!(card, Card::Trainer(t) if t.trainer_card_type == TrainerType::Tool))
}
