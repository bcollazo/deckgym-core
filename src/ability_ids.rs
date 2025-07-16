use std::collections::HashMap;

// TODO: Probably best to generate this file from database.json via card_enum_generator.rs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AbilityId {
    A1177Weezing,
    A1007Butterfree,
    A1132Gardevoir,
    A2a071Arceus,
    A3122SolgaleoExRisingRoad,
    A3a027ShiinoticIlluminate,
}

// Create a static HashMap for fast (pokemon, index) lookup
lazy_static::lazy_static! {
    static ref ABILITY_ID_MAP: HashMap<&'static str, AbilityId> = {
        let mut m = HashMap::new();
        m.insert("A1 007", AbilityId::A1007Butterfree);
        m.insert("A1 177", AbilityId::A1177Weezing);
        m.insert("A1 132", AbilityId::A1132Gardevoir);
        m.insert("A2a 071", AbilityId::A2a071Arceus);
        m.insert("A2a 086", AbilityId::A2a071Arceus);
        m.insert("A2a 095", AbilityId::A2a071Arceus);
        m.insert("A2a 096", AbilityId::A2a071Arceus);
        m.insert("A3 122", AbilityId::A3122SolgaleoExRisingRoad);
        m.insert("A3 189", AbilityId::A3122SolgaleoExRisingRoad);
        m.insert("A3 207", AbilityId::A3122SolgaleoExRisingRoad);
        m.insert("A3 239", AbilityId::A3122SolgaleoExRisingRoad);
        m.insert("A3a 027", AbilityId::A3a027ShiinoticIlluminate);
        m
    };
}

impl AbilityId {
    pub fn from_pokemon_id(pokemon_id: &str) -> Option<Self> {
        ABILITY_ID_MAP.get(&pokemon_id).copied()
    }
}
