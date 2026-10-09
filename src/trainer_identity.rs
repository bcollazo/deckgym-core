//! One implementation per Trainer effect, however many printings carry it.
//!
//! Trainer logic used to be keyed by `CardId`, so every alternate-art or reprint of a Supporter or
//! Item had to be listed by hand in both `move_generation_trainer.rs` and
//! `apply_trainer_action.rs`, and a new printing of an old card showed up as "Trainer logic not
//! implemented" until someone did. Attacks, Abilities and Tools are keyed by their effect text
//! instead, which is what makes them robust to new printings.
//!
//! This module gives Trainers the same property. [`canonical_trainer_id`] maps any Trainer card
//! to the *first* printing (in `CardId` order) that carries the same effect text, so the two
//! `match` statements only need to name that one printing, and a reprint resolves to it with no
//! code change. In the current database every Trainer name has exactly one effect text, and the
//! only effect text shared across names is the Fossil one, which is dispatched before the match.

use std::collections::HashMap;
use std::sync::LazyLock;

use strum::IntoEnumIterator;

use crate::{card_ids::CardId, database::get_card_by_enum, models::Card, models::TrainerCard};

/// Effect text -> the first `CardId` (in enum order) of a Trainer printed with that text.
static CANONICAL_BY_EFFECT: LazyLock<HashMap<String, CardId>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    for id in CardId::iter() {
        if let Card::Trainer(trainer) = get_card_by_enum(id) {
            map.entry(trainer.effect).or_insert(id);
        }
    }
    map
});

/// The printing that carries this Trainer's implementation: the first `CardId` with the same
/// effect text. "First" is `CardId` enum order, which is set order with the promo sets last, so
/// a promo card that was later reprinted in a Deluxe Pack resolves to the Deluxe Pack printing. For a card that is itself the first printing this is its own id. Returns `None`
/// only for an effect text that no card in the database has, which cannot happen for a card
/// that came out of the database.
pub fn canonical_trainer_id(trainer_card: &TrainerCard) -> Option<CardId> {
    CANONICAL_BY_EFFECT
        .get(trainer_card.effect.as_str())
        .copied()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::TrainerType;

    #[test]
    fn a_first_printing_is_its_own_canonical() {
        let Card::Trainer(copycat) = get_card_by_enum(CardId::B1225Copycat) else {
            panic!("B1 225 should be a Trainer");
        };
        assert_eq!(canonical_trainer_id(&copycat), Some(CardId::B1225Copycat));
    }

    #[test]
    fn reprints_resolve_to_the_first_printing() {
        for (reprint, first) in [
            (CardId::B1270Copycat, CardId::B1225Copycat),
            (CardId::B4b225Copycat, CardId::B1225Copycat),
            (CardId::B4b424Copycat, CardId::B1225Copycat),
            // Promo ids sort after every set in `CardId` order, so the Deluxe Pack printing
            // is the canonical one for Professor's Research, not P-A 007.
            (
                CardId::PA007ProfessorsResearch,
                CardId::A4b373ProfessorsResearch,
            ),
            (CardId::A4b329Erika, CardId::A1219Erika),
        ] {
            let Card::Trainer(card) = get_card_by_enum(reprint) else {
                panic!("{reprint:?} should be a Trainer");
            };
            assert_eq!(canonical_trainer_id(&card), Some(first), "{reprint:?}");
        }
    }

    /// Every Trainer in the database resolves, and to a printing with the same name and type:
    /// the guard that keeps effect-text keying honest if a future set ever reuses a text.
    #[test]
    fn every_trainer_resolves_to_a_same_named_printing() {
        for id in CardId::iter() {
            let Card::Trainer(card) = get_card_by_enum(id) else {
                continue;
            };
            let canonical = canonical_trainer_id(&card).unwrap_or_else(|| panic!("{id:?}"));
            let Card::Trainer(first) = get_card_by_enum(canonical) else {
                panic!("{canonical:?} should be a Trainer");
            };
            if card.trainer_card_type == TrainerType::Fossil {
                // All Fossils share one text and are played as Pokémon, never dispatched by id.
                continue;
            }
            assert_eq!(first.name, card.name, "{id:?} -> {canonical:?}");
            assert_eq!(first.trainer_card_type, card.trainer_card_type, "{id:?}");
        }
    }
}
