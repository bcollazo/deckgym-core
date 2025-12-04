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
    A3146PoisonBarb,
    A3147LeafCape,
    A3a065ElectricalCord,
    A4a067InflatableBoat,
    A4b318ElectricalCord,
    A4b319ElectricalCord,
    B1219HeavyHelmet,
}

lazy_static::lazy_static! {
    static ref TOOL_ID_MAP: HashMap<&'static str, ToolId> = {
        let mut m = HashMap::new();
        m.insert("A2 147", ToolId::A2147GiantCape);
        m.insert("A2 148", ToolId::A2148RockyHelmet);
        m.insert("A3 146", ToolId::A3146PoisonBarb);
        m.insert("A3 147", ToolId::A3147LeafCape);
        m.insert("A3a 065", ToolId::A3a065ElectricalCord);
        m.insert("A4a 067", ToolId::A4a067InflatableBoat);
        m.insert("A4b 318", ToolId::A4b318ElectricalCord);
        m.insert("A4b 319", ToolId::A4b319ElectricalCord);
        m.insert("A4b 320", ToolId::A2147GiantCape);
        m.insert("A4b 321", ToolId::A2147GiantCape);
        m.insert("A4b 322", ToolId::A2148RockyHelmet);
        m.insert("A4b 323", ToolId::A2148RockyHelmet);
        m.insert("B1 219", ToolId::B1219HeavyHelmet);
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
            // Most tools can be attached to any pokemon
            ToolId::A2147GiantCape
            | ToolId::A2148RockyHelmet
            | ToolId::A3146PoisonBarb
            | ToolId::B1219HeavyHelmet => true,
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
