use std::sync::LazyLock;

use crate::{
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard, TrainerCard, TrainerType},
    State,
};

pub(crate) fn ensure_tool_card(card: &Card) -> &TrainerCard {
    match card {
        Card::Trainer(trainer_card) => ensure_tool_trainer(trainer_card),
        _ => panic!("Expected TrainerCard of subtype Tool, got non-trainer card"),
    }
}

pub(crate) fn ensure_tool_trainer(trainer_card: &TrainerCard) -> &TrainerCard {
    if trainer_card.trainer_card_type != TrainerType::Tool {
        panic!(
            "Expected TrainerCard of subtype Tool, got {:?}",
            trainer_card.trainer_card_type
        );
    }
    trainer_card
}

fn tool_effect_text_from_card_id(tool_card_id: CardId) -> String {
    let card = get_card_by_enum(tool_card_id);
    let trainer_card = ensure_tool_card(&card);
    trainer_card.effect.clone()
}

static GIANT_CAPE_EFFECT: LazyLock<String> =
    LazyLock::new(|| tool_effect_text_from_card_id(CardId::A2147GiantCape));
static ROCKY_HELMET_EFFECT: LazyLock<String> =
    LazyLock::new(|| tool_effect_text_from_card_id(CardId::A2148RockyHelmet));
static POISON_BARB_EFFECT: LazyLock<String> =
    LazyLock::new(|| tool_effect_text_from_card_id(CardId::A3146PoisonBarb));
static LEAF_CAPE_EFFECT: LazyLock<String> =
    LazyLock::new(|| tool_effect_text_from_card_id(CardId::A3147LeafCape));
static ELECTRICAL_CORD_EFFECT: LazyLock<String> =
    LazyLock::new(|| tool_effect_text_from_card_id(CardId::A3a065ElectricalCord));
static INFLATABLE_BOAT_EFFECT: LazyLock<String> =
    LazyLock::new(|| tool_effect_text_from_card_id(CardId::A4a067InflatableBoat));
static HEAVY_HELMET_EFFECT: LazyLock<String> =
    LazyLock::new(|| tool_effect_text_from_card_id(CardId::B1219HeavyHelmet));
static PROTECTIVE_PONCHO_EFFECT: LazyLock<String> =
    LazyLock::new(|| tool_effect_text_from_card_id(CardId::B2147ProtectivePoncho));
static METAL_CORE_BARRIER_EFFECT: LazyLock<String> =
    LazyLock::new(|| tool_effect_text_from_card_id(CardId::B2148MetalCoreBarrier));

pub fn tool_effects_equal(trainer_card: &TrainerCard, reference_tool_id: CardId) -> bool {
    ensure_tool_trainer(trainer_card);
    trainer_card.effect == tool_effect_text_from_card_id(reference_tool_id)
}

pub fn has_tool(played_card: &PlayedCard, reference_tool_id: CardId) -> bool {
    let reference_effect = tool_effect_text_from_card_id(reference_tool_id);
    let Some(attached_tool) = &played_card.attached_tool else {
        return false;
    };
    let trainer_card = ensure_tool_card(attached_tool);
    trainer_card.effect == reference_effect
}

pub fn can_attach_tool_to(trainer_card: &TrainerCard, pokemon: &PlayedCard) -> bool {
    let trainer_card = ensure_tool_trainer(trainer_card);
    let effect = trainer_card.effect.as_str();
    if effect == LEAF_CAPE_EFFECT.as_str() {
        return pokemon.card.get_type() == Some(EnergyType::Grass);
    }
    if effect == ELECTRICAL_CORD_EFFECT.as_str() {
        return pokemon.card.get_type() == Some(EnergyType::Lightning);
    }
    if effect == INFLATABLE_BOAT_EFFECT.as_str() {
        return pokemon.card.get_type() == Some(EnergyType::Water);
    }
    if effect == METAL_CORE_BARRIER_EFFECT.as_str() {
        return pokemon.card.get_type() == Some(EnergyType::Metal);
    }
    true
}

pub(crate) fn enumerate_tool_choices<'a>(
    trainer_card: &TrainerCard,
    state: &'a State,
    actor: usize,
) -> Vec<(usize, &'a PlayedCard)> {
    let trainer_card = ensure_tool_trainer(trainer_card);
    state
        .enumerate_in_play_pokemon(actor)
        .filter(|(_, x)| !x.has_tool_attached())
        .filter(|(_, x)| can_attach_tool_to(trainer_card, x))
        .collect()
}

pub fn is_tool_effect_implemented(trainer_card: &TrainerCard) -> bool {
    let trainer_card = ensure_tool_trainer(trainer_card);
    let effect = trainer_card.effect.as_str();
    matches!(
        effect,
        e if e == GIANT_CAPE_EFFECT.as_str()
            || e == ROCKY_HELMET_EFFECT.as_str()
            || e == POISON_BARB_EFFECT.as_str()
            || e == LEAF_CAPE_EFFECT.as_str()
            || e == ELECTRICAL_CORD_EFFECT.as_str()
            || e == INFLATABLE_BOAT_EFFECT.as_str()
            || e == HEAVY_HELMET_EFFECT.as_str()
            || e == PROTECTIVE_PONCHO_EFFECT.as_str()
            || e == METAL_CORE_BARRIER_EFFECT.as_str()
    )
}
