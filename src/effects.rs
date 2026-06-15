use serde::{Deserialize, Serialize};

use crate::models::EnergyType;

/// I believe these are the "clearable" ones by retreating...
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardEffect {
    NoRetreat,
    ReducedDamage {
        amount: u32,
    },
    IncreasedVulnerability {
        amount: u32,
    },
    IncreasedAttackCost {
        amount: u8,
    },
    CannotAttack,
    CannotUseAttack(String),
    IncreasedDamageForAttack {
        attack_name: String,
        amount: u32,
    },
    PreventAllDamageAndEffects,
    /// Prevent all damage from attacks if the incoming damage is at most `threshold` (e.g. Cascoon's Harden).
    PreventDamageIfLessOrEqual {
        threshold: u32,
    },
    /// Prevent all damage done by attacks from Basic Pokémon (e.g. Carracosta's Blocking Shell).
    PreventDamageFromBasic,
    NoWeakness,
    CoinFlipToBlockAttack,
    DelayedDamage {
        amount: u32,
    },
    /// If this Pokémon is damaged by an attack while in the Active Spot, deal `amount`
    /// damage to the Attacking Pokémon (e.g. Alolan Sandslash's Spike Armor). Temporary
    /// counterpart to RockyHelmet's always-on recoil.
    Counterattack {
        amount: u32,
    },
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum TurnEffect {
    NoSupportCards,
    NoItemCards,
    NoTrainerCards,
    NoEnergyFromZoneToActive,
    ReducedRetreatCost {
        amount: u8,
    },
    ReducedDamageForType {
        amount: u32,
        energy_type: EnergyType,
        player: usize,
    },
    IncreasedDamage {
        amount: u32,
    },
    IncreasedDamageForType {
        amount: u32,
        energy_type: EnergyType,
    },
    IncreasedDamageAgainstEx {
        amount: u32,
    },
    IncreasedDamageForEeveeEvolutions {
        amount: u32,
    },
    IncreasedDamageForSpecificPokemon {
        amount: u32,
        pokemon_names: Vec<String>,
    },
    IncreasedDamageForSpecificPokemonAgainstEx {
        amount: u32,
        pokemon_names: Vec<String>,
    },
    IncreasedDamageForTypeAgainstEx {
        amount: u32,
        energy_type: EnergyType,
    },
    DelayedSpotDamage {
        source_player: usize,
        target_player: usize,
        target_in_play_idx: usize,
        amount: u32,
    },
    ForceFirstHeads,
    BonusPointForHaxorusActiveKO,
    ReducedAttackCostForSpecificPokemon {
        amount: u8,
        pokemon_names: Vec<String>,
    },
}
