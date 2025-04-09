use core::fmt;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use crate::{card_ids::CardId, tool_ids::ToolId};

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
}

/// Represents the data of a single trainer card.
#[derive(Clone, Serialize, Deserialize)]
pub struct TrainerCard {
    pub id: String,
    pub numeric_id: u16,
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

    pub(crate) fn get_attacks(&self) -> &Vec<Attack> {
        match self {
            Card::Pokemon(pokemon_card) => &pokemon_card.attacks,
            _ => panic!("Unsupported playable card type"),
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
}

/// This represents a card in the mat. Has a pointer to the card
/// description, but captures the extra variable properties while in mat.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PlayedCard {
    pub card: Card,
    pub remaining_hp: u32,
    pub total_hp: u32,
    pub attached_energy: Vec<EnergyType>,
    pub attached_tool: Option<ToolId>,
    pub played_this_turn: bool,
    pub ability_used: bool,
    pub poisoned: bool,
    pub paralyzed: bool,
    pub asleep: bool,
    pub cards_behind: Vec<Card>,
}
impl PlayedCard {
    pub(crate) fn get_id(&self) -> String {
        match &self.card {
            Card::Pokemon(pokemon_card) => pokemon_card.id.clone(),
            Card::Trainer(trainer_card) => trainer_card.id.clone(),
        }
    }

    pub(crate) fn get_name(&self) -> String {
        match &self.card {
            Card::Pokemon(pokemon_card) => pokemon_card.name.clone(),
            Card::Trainer(trainer_card) => trainer_card.name.clone(),
        }
    }

    pub(crate) fn get_attacks(&self) -> &Vec<Attack> {
        match &self.card {
            Card::Pokemon(pokemon_card) => &pokemon_card.attacks,
            _ => panic!("Unsupported playable card type"),
        }
    }

    pub(crate) fn heal(&mut self, amount: u32) {
        self.remaining_hp = (self.remaining_hp + amount).min(self.total_hp);
    }

    pub(crate) fn attach_energy(&mut self, energy: &EnergyType, amount: u8) {
        self.attached_energy
            .extend(std::iter::repeat_n(*energy, amount as usize));
    }

    // Discard 1 of energy type
    pub(crate) fn discard_energy(&mut self, energy: &EnergyType) {
        if let Some(pos) = self.attached_energy.iter().position(|x| x == energy) {
            self.attached_energy.swap_remove(pos);
        }
    }

    pub(crate) fn apply_damage(&mut self, damage: u32) {
        self.remaining_hp = self.remaining_hp.saturating_sub(damage);
    }

    // Option because if playing an item card... (?)
    pub(crate) fn get_energy_type(&self) -> Option<EnergyType> {
        match &self.card {
            Card::Pokemon(pokemon_card) => Some(pokemon_card.energy_type),
            _ => None,
        }
    }

    pub(crate) fn is_damaged(&self) -> bool {
        self.remaining_hp < self.total_hp
    }

    pub(crate) fn has_tool_attached(&self) -> bool {
        self.attached_tool.is_some()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StatusCondition {
    Poisoned,
    Paralyzed,
    Asleep,
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

impl fmt::Debug for PlayedCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(
                f,
                "{}({}hp,{:?})",
                self.get_name(),
                self.remaining_hp,
                self.attached_energy
            )
        } else {
            write!(
                f,
                "{}({}hp,{})",
                self.get_name(),
                self.remaining_hp,
                self.attached_energy.len()
            )
        }
    }
}
