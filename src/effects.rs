use serde::{Deserialize, Serialize};

/// I believe these are the "clearable" ones by retreating...
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardEffect {
    NoRetreat,
    ReducedDamage { amount: u32 },
    CannotAttack,
    CannotUseAttack(String),
    IncreasedDamageForAttack { attack_name: String, amount: u32 },
    PreventAllDamageAndEffects,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum TurnEffect {
    NoSupportCards,
    NoItemCards,
    ReducedRetreatCost { amount: u8 },
    IncreasedDamage { amount: u32 },
    IncreasedDamageAgainstEx { amount: u32 },
    IncreasedDamageForEeveeEvolutions { amount: u32 },
    IncreasedDamageForSpecificPokemon {
        amount: u32,
        pokemon_names: Vec<String>,
    },
}
