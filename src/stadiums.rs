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

pub fn is_stadium_effect_implemented(trainer_card: &TrainerCard) -> bool {
    ensure_stadium_trainer(trainer_card);
    let effect = trainer_card.effect.as_str();
    matches!(
        effect,
        e if e == PECULIAR_PLAZA_EFFECT.as_str()
            || e == TRAINING_AREA_EFFECT.as_str()
    )
}

pub fn has_stadium(state: &State, reference_stadium_id: CardId) -> bool {
    let reference_effect = stadium_effect_text_from_card_id(reference_stadium_id);
    let Some(active_stadium) = &state.active_stadium else {
        return false;
    };
    let trainer_card = ensure_stadium_card(active_stadium);
    trainer_card.effect == reference_effect
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
