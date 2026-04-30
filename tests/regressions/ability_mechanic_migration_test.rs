use crate::deckgym::{
    card_ids::CardId,
    card_validation::{get_implementation_status, ImplementationStatus},
};

const LEGACY_ABILITY_CARD_IDS: &[&str] = &[
    "A1 020", "A1 061", "A1 098", "A1 123", "A1 132", "A1 261", "A1 277", "A1a 006", "A1a 019",
    "A1a 046", "A1a 056", "A1a 070", "A1a 072", "A1a 078", "A1a 084", "A2 072", "A2 078", "A2 092",
    "A2 110", "A2 167", "A2 170", "A2 187", "A2 202", "A2a 010", "A2a 022", "A2a 050", "A2a 069",
    "A2a 071", "A2a 081", "A2a 082", "A2a 083", "A2a 086", "A2a 091", "A2a 092", "A2a 095",
    "A2a 096", "A2b 028", "A2b 035", "A2b 083", "A2b 096", "A3 066", "A3 122", "A3 141", "A3 165",
    "A3 179", "A3 189", "A3 207", "A3 234", "A3 239", "A3a 015", "A3a 021", "A3a 027", "A3a 042",
    "A3a 052", "A3a 062", "A3a 075", "A3a 101", "A3a 103", "A3b 009", "A3b 034", "A3b 056",
    "A3b 057", "A3b 079", "A3b 081", "A3b 083", "A3b 084", "A3b 087", "A3b 089", "A3b 091",
    "A3b 092", "A4 083", "A4 112", "A4 190", "A4 193", "A4 205", "A4 208", "A4 218", "A4 233",
    "A4 235", "A4a 010", "A4a 020", "A4a 022", "A4a 025", "A4a 065", "A4a 072", "A4a 079",
    "A4a 080", "A4a 081", "A4a 087", "A4a 088", "A4a 090", "A4b 029", "A4b 066", "A4b 099",
    "A4b 100", "A4b 106", "A4b 135", "A4b 136", "A4b 146", "A4b 147", "A4b 149", "A4b 150",
    "A4b 155", "A4b 160", "A4b 172", "A4b 177", "A4b 197", "A4b 212", "A4b 213", "A4b 230",
    "A4b 231", "A4b 241", "A4b 245", "A4b 246", "A4b 247", "A4b 259", "A4b 287", "A4b 288",
    "A4b 297", "A4b 298", "A4b 299", "A4b 304", "A4b 305", "A4b 369", "A4b 370", "A4b 372",
    "A4b 377", "A4b 378", "B1 157", "B1 160", "B1 172", "B1 177", "B1 184", "B1 245", "B1 247",
    "B1 263", "B1 281", "B1 289", "B1 297", "B1 328", "B1a 006", "B1a 012", "B1a 018", "B1a 034",
    "B1a 070", "B1a 072", "B1a 101", "B1a 102", "B2 225", "B2 228", "B2 229", "B2 231", "B2a 126",
    "B2a 127", "B2a 128", "P-A 037", "P-A 042", "P-A 054", "P-A 104", "P-A 109", "P-A 110",
    "P-B 020",
];

#[test]
fn legacy_ability_cards_stay_implemented_through_migration() {
    for card_id in LEGACY_ABILITY_CARD_IDS {
        let parsed = CardId::from_card_id(card_id).expect("legacy migration card id should parse");
        let status = get_implementation_status(parsed);
        assert_eq!(
            status,
            ImplementationStatus::Complete,
            "{card_id} regressed: {}",
            status.description()
        );
    }
}
