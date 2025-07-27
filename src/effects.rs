/// I believe these are the "clearable" ones by retreating...
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CardEffect {
    NoRetreat,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TurnEffect {
    NoSupportCards,
    ReducedRetreatCost { amount: u8 },
    IncreasedDamage { amount: u32 },
}
