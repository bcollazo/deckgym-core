use serde::{Deserialize, Serialize};

/// I believe these are the "clearable" ones by retreating...
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardEffect {
    NoRetreat,
    ReducedDamage { amount: u32 },
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum TurnEffect {
    NoSupportCards,
    ReducedRetreatCost { amount: u8 },
    IncreasedDamage { amount: u32 },
    IncreasedDamageAgainstEx { amount: u32 },
}
