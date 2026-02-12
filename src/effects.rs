use serde::{Deserialize, Serialize};

use crate::models::EnergyType;

/// I believe these are the "clearable" ones by retreating...
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardEffect {
    NoRetreat,
    ReducedDamage { amount: u32 },
    CannotAttack,
    CannotUseAttack(String),
    IncreasedDamageForAttack { attack_name: String, amount: u32 },
    PreventAllDamageAndEffects,
    NoWeakness,
    CoinFlipToBlockAttack,
    DelayedDamage { amount: u32 },
    CounterDamage { amount: u32 },
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum TurnEffect {
    NoSupportCards,
    NoItemCards,
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
}
