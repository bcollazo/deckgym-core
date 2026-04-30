use crate::deckgym::{
    actions::EFFECT_MECHANIC_MAP, card_ids::CardId, database::get_card_by_enum, models::Card,
};
use strum::IntoEnumIterator;

const LEGACY_ATTACK_CARD_INDEXES: &[(&str, usize)] = &[
    ("A1 115", 0),
    ("A1 136", 0),
    ("A1 149", 0),
    ("A1 153", 0),
    ("A1 163", 0),
    ("A1 178", 0),
    ("A1 181", 0),
    ("A1 201", 0),
    ("A1 203", 0),
    ("A1 264", 0),
    ("A1a 001", 0),
    ("A1a 010", 0),
    ("A1a 011", 0),
    ("A1a 017", 0),
    ("A1a 026", 0),
    ("A1a 061", 0),
    ("A2 023", 0),
    ("A2 029", 0),
    ("A2 049", 1),
    ("A2 056", 0),
    ("A2 060", 0),
    ("A2 084", 0),
    ("A2 098", 0),
    ("A2 118", 0),
    ("A2 131", 0),
    ("A2 141", 0),
    ("A2 181", 0),
    ("A2 182", 1),
    ("A2 197", 0),
    ("A2 204", 1),
    ("A2 206", 1),
    ("A2a 001", 0),
    ("A2a 063", 0),
    ("A2b 032", 0),
    ("A2b 044", 0),
    ("A3 002", 0),
    ("A3 012", 0),
    ("A3 019", 0),
    ("A3 020", 0),
    ("A3 040", 0),
    ("A3 071", 0),
    ("A3 085", 0),
    ("A3 112", 0),
    ("A3 116", 0),
    ("A3 122", 0),
    ("A3 156", 0),
    ("A3 158", 0),
    ("A3 162", 0),
    ("A3 171", 0),
    ("A3 180", 0),
    ("A3 189", 0),
    ("A3 198", 0),
    ("A3 207", 0),
    ("A3 236", 0),
    ("A3 239", 0),
    ("A3a 003", 0),
    ("A3a 019", 0),
    ("A3a 043", 0),
    ("A3a 044", 0),
    ("A3a 047", 0),
    ("A3a 060", 0),
    ("A3a 061", 0),
    ("A3a 062", 0),
    ("A3a 070", 0),
    ("A3a 074", 0),
    ("A3a 075", 0),
    ("A3a 077", 0),
    ("A3a 079", 0),
    ("A3a 080", 0),
    ("A3a 084", 0),
    ("A3a 086", 0),
    ("A3a 087", 0),
    ("A3b 013", 0),
    ("A3b 020", 0),
    ("A3b 057", 0),
    ("A3b 058", 0),
    ("A3b 084", 0),
    ("A3b 091", 0),
    ("A4 021", 0),
    ("A4 032", 0),
    ("A4 066", 0),
    ("A4 077", 0),
    ("A4 105", 0),
    ("A4 134", 0),
    ("A4 146", 0),
    ("A4 166", 0),
    ("A4 171", 0),
    ("A4 186", 0),
    ("A4 202", 0),
    ("A4 214", 0),
    ("A4 231", 0),
    ("A4a 021", 0),
    ("A4a 023", 0),
    ("A4a 101", 0),
    ("A4a 105", 0),
    ("A4b 023", 0),
    ("A4b 042", 0),
    ("A4b 075", 0),
    ("A4b 096", 0),
    ("A4b 097", 0),
    ("A4b 107", 1),
    ("A4b 148", 0),
    ("A4b 180", 0),
    ("A4b 181", 0),
    ("A4b 196", 0),
    ("A4b 242", 0),
    ("A4b 243", 0),
    ("A4b 248", 0),
    ("A4b 251", 0),
    ("A4b 259", 0),
    ("A4b 288", 0),
    ("A4b 300", 0),
    ("A4b 301", 0),
    ("A4b 302", 0),
    ("A4b 303", 0),
    ("A4b 304", 0),
    ("A4b 305", 0),
    ("A4b 363", 1),
    ("A4b 369", 0),
    ("B1 052", 0),
    ("B1 085", 0),
    ("B1 150", 0),
    ("B1 151", 0),
    ("B1 255", 0),
    ("B1 258", 0),
    ("B1 262", 0),
    ("B1 277", 0),
    ("B1 280", 0),
    ("B1 285", 0),
    ("B1 317", 0),
    ("B1 319", 1),
    ("B1 322", 0),
    ("B1 325", 0),
    ("B1a 097", 0),
    ("P-A 049", 0),
    ("P-A 060", 0),
    ("P-A 067", 0),
    ("P-A 069", 0),
    ("P-A 082", 0),
    ("P-A 084", 0),
    ("P-A 093", 0),
];

fn find_card_id_enum_by_printed_id(printed_id: &str) -> CardId {
    CardId::iter()
        .find(|card_id| get_card_by_enum(*card_id).get_id() == printed_id)
        .unwrap_or_else(|| panic!("Card ID {printed_id} should exist in CardId enum"))
}

#[test]
fn all_legacy_attack_id_attacks_are_mechanic_mapped() {
    let mut missing = Vec::new();
    for (printed_id, attack_index) in LEGACY_ATTACK_CARD_INDEXES {
        let card_id = find_card_id_enum_by_printed_id(printed_id);
        let card = get_card_by_enum(card_id);
        let Card::Pokemon(pokemon_card) = &card else {
            panic!(
                "Legacy attack mapping entry {printed_id}:{attack_index} must be a Pokemon card"
            );
        };

        let attack = pokemon_card
            .attacks
            .get(*attack_index)
            .unwrap_or_else(|| panic!("Card {printed_id} should have attack index {attack_index}"));

        let effect_text = attack.effect.as_ref().unwrap_or_else(|| {
            panic!("Card {printed_id} attack index {attack_index} should have an effect")
        });

        if !EFFECT_MECHANIC_MAP.contains_key(effect_text.as_str()) {
            missing.push(format!(
                "{printed_id}:{attack_index} {} => {}",
                attack.title, effect_text
            ));
        }
    }

    assert!(
        missing.is_empty(),
        "Missing mechanic mappings for legacy attacks:\n{}",
        missing.join("\n")
    );
}
