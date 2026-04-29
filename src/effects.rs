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
    NoWeakness,
    CoinFlipToBlockAttack,
    DelayedDamage {
        amount: u32,
    },
    DelayedAttackDamage {
        amount: u32,
        source_player: usize,
        source_play_id: u64,
    },
    ReactiveAttackDamageNextTurn {
        amount: u32,
        source_player: usize,
        source_play_id: u64,
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
    ForceFirstHeads {
        player: usize,
    },
    BonusPointForHaxorusActiveKO,
}
