use crate::{
    effects::{CardEffect, TurnEffect},
    models::{EnergyType, StatusCondition},
};

pub enum BenchSide {
    YourBench,
    OpponentBench,
    BothBenches,
}

pub enum Mechanic {
    SelfHeal {
        amount: u32,
    },
    SearchToHandByEnergy {
        energy_type: EnergyType,
    },
    SearchToBenchByName {
        name: String,
    },
    InflictStatusCondition {
        condition: StatusCondition,
    },
    ChanceStatusAttack {
        condition: StatusCondition,
    },
    CantAttackNextTurn {
        probability: f32,
    },
    DiscardRandomGlobalEnergy,
    DiscardEnergyFromOpponentActive,
    ExtraDamageIfEx {
        extra_damage: u32,
    },
    SelfDamage {
        amount: u32,
    },
    CoinFlipExtraDamage {
        extra_damage: u32,
    },
    CoinFlipExtraDamageOrSelfDamage {
        extra_damage: u32,
        self_damage: u32,
    },
    ExtraDamageForEachHeads {
        include_fixed_damage: bool,
        damage_per_head: u32,
        num_coins: usize,
    },
    CoinFlipNoEffect,
    SelfDiscardEnergy {
        energies: Vec<EnergyType>,
    },
    ExtraDamageIfExtraEnergy {
        required_extra_energy: Vec<EnergyType>,
        extra_damage: u32,
    },
    ExtraDamageIfBothHeads {
        extra_damage: u32,
    },
    DirectDamage {
        damage: u32,
        bench_only: bool,
    },
    DamageAndTurnEffect {
        effect: TurnEffect,
        duration: u8,
    },
    // Fairly unique mechanics
    ManaphyOceanicGift,
    PalkiaExDimensionalStorm,
    MegaBlazikenExMegaBurningAttack,
    MoltresExInfernoDance,
    CelebiExPowerfulBloom,
    MagikarpWaterfallEvolution,
    ChargeBench {
        energies: Vec<EnergyType>,
        target_benched_type: Option<EnergyType>,
    },
    // End Unique mechanics
    DamageAndCardEffect {
        opponent: bool,
        effect: CardEffect,
        duration: u8,
    },
    SelfDiscardAllEnergy,
    AlsoBenchDamage {
        opponent: bool,
        damage: u32,
        must_have_energy: bool,
    },
    AlsoChoiceBenchDamage {
        opponent: bool,
        damage: u32,
    },
    ExtraDamageIfHurt {
        extra_damage: u32,
        opponent: bool,
    },
    BenchCountDamage {
        include_fixed_damage: bool,
        damage_per: u32,
        energy_type: Option<EnergyType>,
        bench_side: BenchSide,
    },
    ExtraDamagePerEnergy {
        opponent: bool,
        damage_per_energy: u32,
    },
    ExtraDamageIfToolAttached {
        extra_damage: u32,
    },
}
