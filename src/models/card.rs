use core::fmt;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use crate::{card_ids::CardId, AbilityId};

/// Represents the type of energy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum EnergyType {
    Grass,
    Fire,
    Water,
    Lightning,
    Psychic,
    Fighting,
    Darkness,
    Metal,
    Dragon,
    Colorless,
}
impl EnergyType {
    pub(crate) fn from_str(energy_type: &str) -> Option<Self> {
        match energy_type {
            "Grass" => Some(EnergyType::Grass),
            "Fire" => Some(EnergyType::Fire),
            "Water" => Some(EnergyType::Water),
            "Lightning" => Some(EnergyType::Lightning),
            "Psychic" => Some(EnergyType::Psychic),
            "Fighting" => Some(EnergyType::Fighting),
            "Darkness" => Some(EnergyType::Darkness),
            "Metal" => Some(EnergyType::Metal),
            "Dragon" => Some(EnergyType::Dragon),
            "Colorless" => Some(EnergyType::Colorless),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            EnergyType::Grass => "Grass",
            EnergyType::Fire => "Fire",
            EnergyType::Water => "Water",
            EnergyType::Lightning => "Lightning",
            EnergyType::Psychic => "Psychic",
            EnergyType::Fighting => "Fighting",
            EnergyType::Darkness => "Darkness",
            EnergyType::Metal => "Metal",
            EnergyType::Dragon => "Dragon",
            EnergyType::Colorless => "Colorless",
        }
    }
}

/// Represents an attack of a card.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attack {
    pub energy_required: Vec<EnergyType>,
    pub title: String,
    pub fixed_damage: u32,
    pub effect: Option<String>,
}

/// Represents an attack of a card.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ability {
    pub title: String,
    pub effect: String,
}

pub const BASIC_STAGE: u8 = 0;

/// Represents the data of a single pokemon card.
#[derive(Clone, Serialize, Deserialize)]
pub struct PokemonCard {
    pub id: String,
    pub name: String,
    pub stage: u8, // 0 for Basic, 1 for Stage 1, 2 for Stage 2
    pub evolves_from: Option<String>,
    pub hp: u32,
    pub energy_type: EnergyType,
    pub ability: Option<Ability>,
    pub attacks: Vec<Attack>,
    pub weakness: Option<EnergyType>,
    pub retreat_cost: Vec<EnergyType>,
    pub rarity: String,
    pub booster_pack: String,
}
impl PartialEq for PokemonCard {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for PokemonCard {}
impl Hash for PokemonCard {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrainerType {
    Supporter,
    Item,
    Tool,
    Fossil,
}

/// Represents the data of a single trainer card.
#[derive(Clone, Serialize, Deserialize)]
pub struct TrainerCard {
    pub id: String,
    pub trainer_card_type: TrainerType,
    pub name: String,
    pub effect: String,
    pub rarity: String,
    pub booster_pack: String,
}
impl PartialEq for TrainerCard {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for TrainerCard {}
impl Hash for TrainerCard {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Card {
    Pokemon(PokemonCard),
    Trainer(TrainerCard),
}
impl Hash for Card {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Card::Pokemon(pokemon_card) => pokemon_card.id.hash(state),
            Card::Trainer(trainer_card) => trainer_card.id.hash(state),
        }
    }
}
impl Card {
    pub fn get_id(&self) -> String {
        match self {
            Card::Pokemon(pokemon_card) => pokemon_card.id.clone(),
            Card::Trainer(trainer_card) => trainer_card.id.clone(),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            Card::Pokemon(pokemon_card) => pokemon_card.name.clone(),
            Card::Trainer(trainer_card) => trainer_card.name.clone(),
        }
    }

    pub(crate) fn get_attacks(&self) -> Vec<Attack> {
        match self {
            Card::Pokemon(pokemon_card) => pokemon_card.attacks.clone(),
            _ => vec![],
        }
    }

    pub fn get_retreat_cost(&self) -> Option<Vec<EnergyType>> {
        match self {
            Card::Pokemon(pokemon_card) => Some(pokemon_card.retreat_cost.clone()),
            _ => None, // Fossils
        }
    }

    pub(crate) fn is_ex(&self) -> bool {
        // A pokemon is EX if after splitting by spaces in the name, the last word is "EX"
        match self {
            Card::Pokemon(pokemon_card) => {
                pokemon_card.name.to_lowercase().split(' ').next_back() == Some("ex")
            }
            _ => false,
        }
    }

    pub(crate) fn is_mega(&self) -> bool {
        // A pokemon is Mega if the name starts with "Mega "
        match self {
            Card::Pokemon(pokemon_card) => pokemon_card.name.starts_with("Mega "),
            _ => false,
        }
    }

    pub(crate) fn get_knockout_points(&self) -> u8 {
        // Mega pokemon are worth 3 points, ex pokemon are worth 2, regular pokemon are worth 1
        if self.is_mega() {
            3
        } else if self.is_ex() {
            2
        } else {
            1
        }
    }

    pub(crate) fn get_ability(&self) -> Option<Ability> {
        match self {
            Card::Pokemon(pokemon_card) => pokemon_card.ability.clone(),
            _ => None,
        }
    }

    pub(crate) fn is_support(&self) -> bool {
        match self {
            Card::Trainer(trainer_card) => trainer_card.trainer_card_type == TrainerType::Supporter,
            _ => false,
        }
    }

    pub(crate) fn get_type(&self) -> Option<EnergyType> {
        match self {
            Card::Pokemon(pokemon_card) => Some(pokemon_card.energy_type),
            _ => None,
        }
    }

    pub fn get_card_id(&self) -> CardId {
        CardId::from_card_id(self.get_id().as_str()).expect("Card ID should be valid")
    }

    pub fn is_basic(&self) -> bool {
        match self {
            Card::Pokemon(pokemon_card) => pokemon_card.stage == BASIC_STAGE,
            _ => false,
        }
    }

    pub fn is_fossil(&self) -> bool {
        match self {
            Card::Trainer(trainer_card) => trainer_card.trainer_card_type == TrainerType::Fossil,
            _ => false,
        }
    }

    pub fn as_trainer(&self) -> TrainerCard {
        match self {
            Card::Trainer(trainer_card) => trainer_card.clone(),
            _ => panic!("Card is not a Trainer"),
        }
    }

    /// Check if this card can evolve into the given evolution card
    /// This handles special evolution rules like Eevee ex's Veevee 'volve ability
    pub fn can_evolve_into(&self, evolution_card: &Card) -> bool {
        if let Card::Pokemon(evolution_pokemon) = evolution_card {
            if let Some(evolves_from) = &evolution_pokemon.evolves_from {
                // Normal evolution: the evolution card evolves from this card's name
                if self.get_name() == *evolves_from {
                    return true;
                }

                // Special case: Eevee ex's Veevee 'volve ability
                // Allows Eevee ex to evolve into any Pokemon that evolves from "Eevee"
                if let Some(ability_id) = AbilityId::from_pokemon_id(&self.get_id()[..]) {
                    if ability_id == AbilityId::A3b056EeveeExVeeveeVolve && evolves_from == "Eevee"
                    {
                        return true;
                    }
                }
            }
        }
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum StatusCondition {
    Poisoned,
    Paralyzed,
    Asleep,
    Burned,
    Confused,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Card::Pokemon(pokemon_card) => write!(f, "{}", pokemon_card.name),
            Card::Trainer(trainer_card) => write!(f, "{}", trainer_card.name),
        }
    }
}

impl fmt::Display for EnergyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnergyType::Grass => write!(f, "Grass"),
            EnergyType::Fire => write!(f, "Fire"),
            EnergyType::Water => write!(f, "Water"),
            EnergyType::Lightning => write!(f, "Lightning"),
            EnergyType::Psychic => write!(f, "Psychic"),
            EnergyType::Fighting => write!(f, "Fighting"),
            EnergyType::Darkness => write!(f, "Darkness"),
            EnergyType::Metal => write!(f, "Metal"),
            EnergyType::Dragon => write!(f, "Dragon"),
            EnergyType::Colorless => write!(f, "Colorless"),
        }
    }
}

impl fmt::Debug for PokemonCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "{}", self.name)
        } else {
            write!(f, "{} {}", self.id, self.name)
        }
    }
}

impl fmt::Debug for TrainerCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "{}", self.name)
        } else {
            write!(f, "{} {}", self.id, self.name)
        }
    }
}
