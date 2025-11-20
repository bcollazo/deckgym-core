use crate::models::EnergyType;

pub enum Mechanic {
    SelfHeal {
        amount: u32,
    },
    // Fairly unique mechanics
    ManaphyOceanicGift,
    PalkiaExDimensionalStorm,
    MegaBlazikenExMegaBurningAttack,
    MoltresExInfernoDance,
    CelebiExPowerfulBloom,
    MagikarpWaterfallEvolution,
    ChargeBenchGrass {
        amount: u32,
        energy_type: EnergyType,
    },
}
