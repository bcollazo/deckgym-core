use std::collections::HashSet;

use deckgym::{card_ids::CardId, database::get_card_by_enum, models::Card};
use strum::IntoEnumIterator;

#[test]
fn generated_catalog_matches_every_field_in_the_source_database() {
    let cards: Vec<Card> = serde_json::from_str(include_str!("../../database.json")).unwrap();
    let mut ids = HashSet::new();
    for source in &cards {
        let id = source.get_id();
        assert!(ids.insert(id.clone()), "Duplicate catalog ID: {id}");
        let enum_id = CardId::from_card_id(&id).unwrap_or_else(|| panic!("Missing ID: {id}"));
        let generated = get_card_by_enum(enum_id);
        // Card equality compares IDs only; compare serialized fields to catch stale generation.
        assert_eq!(
            serde_json::to_value(source).unwrap(),
            serde_json::to_value(generated).unwrap(),
            "{id}"
        );
    }
    assert_eq!(ids.len(), CardId::iter().count());
}

#[test]
fn new_release_has_all_printings_available_for_contributors() {
    let ids: Vec<_> = CardId::iter()
        .map(|id| get_card_by_enum(id).get_id())
        .collect();
    for (prefix, count) in [("C1 ", 217), ("SP1 ", 30), ("P-C ", 13)] {
        assert_eq!(
            ids.iter().filter(|id| id.starts_with(prefix)).count(),
            count,
            "{prefix}"
        );
    }
}
