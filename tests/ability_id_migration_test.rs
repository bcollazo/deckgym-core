use deckgym::{
    actions::EFFECT_ABILITY_MECHANIC_MAP, card_ids::CardId, database::get_card_by_enum,
    models::Card,
};
use strum::IntoEnumIterator;

/// Representative card IDs for each unique AbilityId that needs to be migrated.
/// After full migration all of these cards' ability effect texts must appear in
/// EFFECT_ABILITY_MECHANIC_MAP so the engine no longer relies on AbilityId for them.
const LEGACY_ABILITY_CARD_IDS: &[&str] = &[
    "A1 020",  // Victreebel – Fragrance Trap
    "A1 061",  // Poliwrath – Counterattack
    "A1 098",  // Magneton – Volt Charge
    "A1 123",  // Gengar ex – Shadowy Spellbind
    "A1 132",  // Gardevoir – (attach P to P active)
    "A1 177",  // Weezing – Gas Leak
    "A1 188",  // Pidgeot – Drive Off
    "A1a 006", // Serperior – Jungle Totem
    "A1a 019", // Vaporeon – Wash Out
    "A1a 046", // Aerodactyl ex – Primeval Law
    "A2 022",  // Shaymin – Fragrant Flower Garden
    "A2 072",  // Dusknoir – Shadow Void
    "A2 078",  // Giratina – Levitate
    "A2 092",  // Lucario – Fighting Coach
    "A2 110",  // Darkrai ex – Nightmare Aura
    "A2a 010", // Leafeon ex – Forest Breath
    "A2a 022", // Glaceon ex – Snowy Terrain
    "A2a 050", // Crobat – Cunning Link
    "A2a 069", // Shaymin (Sky) – Sky Support
    "A2a 071", // Arceus ex – (immune to Special Conditions)
    "A2b 035", // Giratina ex – Broken Space Bellow
    "A3 066",  // Oricorio – Safeguard
    "A3 122",  // Solgaleo ex – Rising Road
    "A3 141",  // Komala – Comatose
    "A3a 015", // Luxray – Intimidating Fang
    "A3a 021", // Zeraora – Thunderclap Flash
    "A3a 027", // Shiinotic – Illuminate
    "A3a 042", // Nihilego – More Poison
    "A3a 062", // Celesteela – Ultra Thrusters
    "A3b 009", // Flareon ex – Combust
    "A3b 034", // Sylveon ex – Happy Ribbon
    "A3b 056", // Eevee ex – Veevee 'volve
    "A3b 057", // Snorlax ex – Full-Mouth Manner
    "A4 083",  // Espeon ex – Psychic Healing
    "A4 112",  // Umbreon ex – Dark Chase
    "A4a 010", // Entei ex – Legendary Pulse
    "A4a 020", // Suicune ex – Legendary Pulse
    "A4a 022", // Milotic – Healing Ripples
    "A4a 025", // Raikou ex – Legendary Pulse
    "A4a 032", // Misdreavus – Infiltrating Inspection
    "B1 073",  // Greninja ex – Shifting Stream
    "B1 121",  // Indeedee ex – Watch Over
    "B1 157",  // Hydreigon – Roar in Unison
    "B1 160",  // Dragalge ex – Poison Point
    "B1 172",  // Aegislash – Cursed Metal
    "B1 177",  // Goomy – Sticky Membrane
    "B1 184",  // Eevee – Boosted Evolution
    "B1a 006", // Ariados – Trap Territory
    "B1a 012", // Charmeleon – Ignition
    "B1a 018", // Wartortle – Shell Shield
    "B1a 034", // Reuniclus – Infinite Increase
    "B1a 065", // Furfrou – Fur Coat
    "P-A 037", // Cresselia ex – Lunar Plumage
];

fn find_card_id_enum_by_printed_id(printed_id: &str) -> CardId {
    CardId::iter()
        .find(|card_id| get_card_by_enum(*card_id).get_id() == printed_id)
        .unwrap_or_else(|| panic!("Card ID {printed_id} should exist in CardId enum"))
}

#[test]
fn all_legacy_ability_id_cards_are_mechanic_mapped() {
    let mut missing = Vec::new();
    for &printed_id in LEGACY_ABILITY_CARD_IDS {
        let card_id = find_card_id_enum_by_printed_id(printed_id);
        let card = get_card_by_enum(card_id);
        let Card::Pokemon(pokemon_card) = &card else {
            panic!("Legacy ability mapping entry {printed_id} must be a Pokemon card");
        };

        let ability = pokemon_card.ability.as_ref().unwrap_or_else(|| {
            panic!("Card {printed_id} should have an ability")
        });

        if !EFFECT_ABILITY_MECHANIC_MAP.contains_key(ability.effect.as_str()) {
            missing.push(format!("{printed_id} {} => {}", ability.title, ability.effect));
        }
    }

    assert!(
        missing.is_empty(),
        "Missing mechanic mappings for legacy ability ID cards:\n{}",
        missing.join("\n")
    );
}
