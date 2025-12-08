use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    models::{EnergyType, PlayedCard, TrainerCard},
    State,
};

// TODO: Probably best to generate this file from database.json via card_enum_generator.rs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ToolId {
    A2147GiantCape,
    A2148RockyHelmet,
    A2149LumBerry,
    A3146PoisonBarb,
    A3147LeafCape,
    A3a065ElectricalCord,
    A3a066Beastite,
    A3b067Leftovers,
    A4153SteelApron,
    A4154DarkPendant,
    A4155RescueScarf,
    A4a067InflatableBoat,
    A4a068MemoryLight,
    A4b318ElectricalCord,
    A4b319ElectricalCord,
    B1218SitrusBerry,
    B1219HeavyHelmet,
    B1220LuckyMittens,
}

lazy_static::lazy_static! {
    static ref TOOL_ID_MAP: HashMap<&'static str, ToolId> = {
        let mut m = HashMap::new();
        m.insert("A2 147", ToolId::A2147GiantCape);
        m.insert("A2 148", ToolId::A2148RockyHelmet);
        m.insert("A2 149", ToolId::A2149LumBerry);
        m.insert("A3 146", ToolId::A3146PoisonBarb);
        m.insert("A3 147", ToolId::A3147LeafCape);
        m.insert("A3a 065", ToolId::A3a065ElectricalCord);
        m.insert("A3a 066", ToolId::A3a066Beastite);
        m.insert("A3b 067", ToolId::A3b067Leftovers);
        m.insert("A4 153", ToolId::A4153SteelApron);
        m.insert("A4 154", ToolId::A4154DarkPendant);
        m.insert("A4 155", ToolId::A4155RescueScarf);
        m.insert("A4a 067", ToolId::A4a067InflatableBoat);
        m.insert("A4a 068", ToolId::A4a068MemoryLight);
        m.insert("A4b 318", ToolId::A4b318ElectricalCord);
        m.insert("A4b 319", ToolId::A4b319ElectricalCord);
        m.insert("A4b 320", ToolId::A2147GiantCape);
        m.insert("A4b 321", ToolId::A2147GiantCape);
        m.insert("A4b 322", ToolId::A2148RockyHelmet);
        m.insert("A4b 323", ToolId::A2148RockyHelmet);
        m.insert("A4b 324", ToolId::A3147LeafCape);
        m.insert("A4b 325", ToolId::A3147LeafCape);
        m.insert("B1 218", ToolId::B1218SitrusBerry);
        m.insert("B1 219", ToolId::B1219HeavyHelmet);
        m.insert("B1 220", ToolId::B1220LuckyMittens);
        m
    };
}

impl ToolId {
    pub fn from_trainer_card(trainer_card: &TrainerCard) -> Option<&Self> {
        TOOL_ID_MAP.get(&trainer_card.id.as_str())
    }

    /// Check if a tool can be attached to a specific pokemon
    pub fn can_attach_to(&self, pokemon: &PlayedCard) -> bool {
        match self {
            ToolId::A3147LeafCape => {
                // Leaf Cape can only be attached to Grass pokemon
                pokemon.card.get_type() == Some(EnergyType::Grass)
            }
            ToolId::A3a065ElectricalCord
            | ToolId::A4b318ElectricalCord
            | ToolId::A4b319ElectricalCord => {
                // Electrical Cord can only be attached to Lightning pokemon
                pokemon.card.get_type() == Some(EnergyType::Lightning)
            }
            ToolId::A4a067InflatableBoat => {
                // Inflatable Boat can only be attached to Water pokemon
                pokemon.card.get_type() == Some(EnergyType::Water)
            }
            ToolId::A3a066Beastite => {
                // Beastite can only be attached to Ultra Beasts
                use crate::hooks::is_ultra_beast;
                is_ultra_beast(&pokemon.get_name())
            }
            ToolId::A4153SteelApron => {
                // Steel Apron can only be attached to Metal pokemon
                pokemon.card.get_type() == Some(EnergyType::Metal)
            }
            ToolId::A4154DarkPendant => {
                // Dark Pendant can only be attached to Darkness pokemon
                pokemon.card.get_type() == Some(EnergyType::Darkness)
            }
            // Most tools can be attached to any pokemon
            ToolId::A2147GiantCape
            | ToolId::A2148RockyHelmet
            | ToolId::A2149LumBerry
            | ToolId::A3146PoisonBarb
            | ToolId::A3b067Leftovers
            | ToolId::A4155RescueScarf
            | ToolId::A4a068MemoryLight
            | ToolId::B1218SitrusBerry
            | ToolId::B1219HeavyHelmet
            | ToolId::B1220LuckyMittens => true,
        }
    }

    pub(crate) fn enumerate_choices<'a>(
        &self,
        state: &'a State,
        actor: usize,
    ) -> impl Iterator<Item = (usize, &'a PlayedCard)> {
        let tool_id = *self;
        state
            .enumerate_in_play_pokemon(actor)
            .filter(|(_, x)| !x.has_tool_attached())
            .filter(move |(_, x)| tool_id.can_attach_to(x))
    }
}
