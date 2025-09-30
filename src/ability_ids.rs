use std::collections::HashMap;

// TODO: Probably best to generate this file from database.json via card_enum_generator.rs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AbilityId {
    A1020VictreebelFragranceTrap,
    A1177Weezing,
    A1007Butterfree,
    A1132Gardevoir,
    A1a006SerperiorJungleTotem,
    A2a010LeafeonExForestBreath,
    A2a071Arceus,
    A3122SolgaleoExRisingRoad,
    A3a027ShiinoticIlluminate,
    A3b034SylveonExHappyRibbon,
}

// Create a static HashMap for fast (pokemon, index) lookup
lazy_static::lazy_static! {
    static ref ABILITY_ID_MAP: HashMap<&'static str, AbilityId> = {
        let mut m = HashMap::new();
        m.insert("A1 007", AbilityId::A1007Butterfree);
        m.insert("A1 020", AbilityId::A1020VictreebelFragranceTrap);
        m.insert("A1 177", AbilityId::A1177Weezing);
        m.insert("A1 132", AbilityId::A1132Gardevoir);
        m.insert("A1a 006", AbilityId::A1a006SerperiorJungleTotem);
        m.insert("A1a 070", AbilityId::A1a006SerperiorJungleTotem);
        m.insert("A2a 010", AbilityId::A2a010LeafeonExForestBreath);
        m.insert("A2a 082", AbilityId::A2a010LeafeonExForestBreath);
        m.insert("A2a 091", AbilityId::A2a010LeafeonExForestBreath);
        m.insert("A2a 071", AbilityId::A2a071Arceus);
        m.insert("A2a 086", AbilityId::A2a071Arceus);
        m.insert("A2a 095", AbilityId::A2a071Arceus);
        m.insert("A2a 096", AbilityId::A2a071Arceus);
        m.insert("A3 122", AbilityId::A3122SolgaleoExRisingRoad);
        m.insert("A3 189", AbilityId::A3122SolgaleoExRisingRoad);
        m.insert("A3 207", AbilityId::A3122SolgaleoExRisingRoad);
        m.insert("A3 239", AbilityId::A3122SolgaleoExRisingRoad);
        m.insert("A3a 027", AbilityId::A3a027ShiinoticIlluminate);
        m.insert("A3b 034", AbilityId::A3b034SylveonExHappyRibbon);
        m.insert("A3b 081", AbilityId::A3b034SylveonExHappyRibbon);
        m.insert("A3b 089", AbilityId::A3b034SylveonExHappyRibbon);
        m.insert("A4 233", AbilityId::A2a010LeafeonExForestBreath);
        m
    };
}

impl AbilityId {
    pub fn from_pokemon_id(pokemon_id: &str) -> Option<Self> {
        ABILITY_ID_MAP.get(&pokemon_id).copied()
    }
}
