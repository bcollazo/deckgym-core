use crate::{
    effects::{CardEffect, TurnEffect},
    models::{EnergyType, StatusCondition},
};

#[derive(Debug, Clone, PartialEq)]
pub enum BenchSide {
    YourBench,
    OpponentBench,
    BothBenches,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CopyAttackSource {
    OpponentActive,
    OpponentInPlay,
    OwnBenchNonEx,
}

#[derive(Debug, Clone, PartialEq)]
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
    SearchToBenchBasic,
    SearchRandomPokemonToHand,
    SearchToHandSupporterCard,
    InflictStatusConditions {
        conditions: Vec<StatusCondition>,
        target_opponent: bool,
    },
    ChanceStatusAttack {
        condition: StatusCondition,
    },
    DamageAllOpponentPokemon {
        damage: u32,
    },
    DiscardRandomGlobalEnergy {
        count: usize,
    },
    RandomDamageToOpponentPokemonPerSelfEnergy {
        energy_type: EnergyType,
        damage_per_hit: u32,
    },
    DiscardEnergyFromOpponentActive,
    CoinFlipDiscardEnergyFromOpponentActive,
    ExtraDamageIfEx {
        extra_damage: u32,
    },
    ExtraDamageIfOpponentHasSpecialCondition {
        extra_damage: u32,
    },
    ExtraDamageIfSupportPlayedThisTurn {
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
    ExtraDamageIfTypeEnergyInPlay {
        energy_type: EnergyType,
        minimum_count: usize,
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
    SelfChargeActive {
        energies: Vec<EnergyType>,
    },
    ChargeYourTypeAnyWay {
        energy_type: EnergyType,
        count: usize,
    },
    // Fairly unique mechanics
    ManaphyOceanicGift,
    PalkiaExDimensionalStorm,
    MegaBlazikenExMegaBurningAttack,
    MegaKangaskhanExDoublePunchingFamily,
    MoltresExInfernoDance,
    CelebiExPowerfulBloom,
    MagikarpWaterfallEvolution,
    CoinFlipToBlockAttackNextTurn,
    MoveAllEnergyTypeToBench {
        energy_type: EnergyType,
    },
    ChargeBench {
        energies: Vec<EnergyType>,
        target_benched_type: Option<EnergyType>,
    },
    VaporeonHyperWhirlpool,
    ConditionalBenchDamage {
        required_extra_energy: Vec<EnergyType>,
        bench_damage: u32,
        num_bench_targets: usize,
        opponent: bool,
    },
    ExtraDamageForEachHeadsWithStatus {
        include_fixed_damage: bool,
        damage_per_head: u32,
        num_coins: usize,
        status: StatusCondition,
    },
    DamageAndMultipleCardEffects {
        opponent: bool,
        effects: Vec<CardEffect>,
        duration: u8,
    },
    DamageReducedBySelfDamage,
    ExtraDamagePerTrainerInOpponentDeck {
        damage_per_trainer: u32,
    },
    ExtraDamagePerSupporterInDiscard {
        damage_per_supporter: u32,
    },
    ExtraDamagePerOwnPoint {
        damage_per_point: u32,
    },
    ExtraDamageIfCardInDiscard {
        card_name: String,
        extra_damage: u32,
    },
    DelayedSpotDamage {
        amount: u32,
    },
    // End Unique mechanics
    DamageAndCardEffect {
        opponent: bool,
        effect: CardEffect,
        duration: u8,
        coin_flip: bool, // false = always apply, true = apply on heads
    },
    DrawCard {
        amount: u8,
    },
    SelfDiscardAllEnergy,
    SelfDiscardAllTypeEnergy {
        energy_type: EnergyType,
    },
    SelfDiscardAllTypeEnergyAndDamageAnyOpponentPokemon {
        energy_type: EnergyType,
        damage: u32,
    },
    SelfDiscardRandomEnergy,
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
    DamageEqualToSelfDamage,
    ExtraDamageEqualToSelfDamage,
    ExtraDamageIfKnockedOutLastTurn {
        extra_damage: u32,
    },
    BenchCountDamage {
        include_fixed_damage: bool,
        damage_per: u32,
        energy_type: Option<EnergyType>,
        bench_side: BenchSide,
    },
    EvolutionBenchCountDamage {
        include_fixed_damage: bool,
        damage_per: u32,
    },
    ExtraDamagePerEnergy {
        opponent: bool,
        damage_per_energy: u32,
    },
    ExtraDamagePerRetreatCost {
        damage_per_energy: u32,
    },
    DamagePerEnergyAll {
        opponent: bool,
        damage_per_energy: u32,
    },
    DiscardHandCards {
        count: usize,
    },
    ExtraDamagePerSpecificEnergy {
        energy_type: EnergyType,
        damage_per_energy: u32,
    },
    ExtraDamageIfToolAttached {
        extra_damage: u32,
    },
    RecoilIfKo {
        self_damage: u32,
    },
    ShuffleOpponentActiveIntoDeck,
    KnockBackOpponentActive,
    /// Random spread damage attack (e.g., Draco Meteor, Spurt Fire)
    /// Always targets opponent's active + bench. Optionally includes own bench.
    RandomSpreadDamage {
        times: usize,
        damage_per_hit: u32,
        include_own_bench: bool,
    },
    FlipUntilTailsDamage {
        damage_per_heads: u32,
    },
    DirectDamageIfDamaged {
        damage: u32,
    },
    AttachEnergyToBenchedBasic {
        energy_type: EnergyType,
    },
    DamageAndDiscardOpponentDeck {
        damage: u32,
        discard_count: usize,
    },
    MegaAmpharosExLightningLancer,
    OminousClaw,
    DarknessClaw,
    BlockBasicAttack,
    SwitchSelfWithBench,
    CopyAttack {
        source: CopyAttackSource,
        require_attacker_energy_match: bool,
    },
    SelfAsleepAndHeal {
        amount: u32,
    },
    FlipCoinsBenchDamagePerHead {
        num_coins: usize,
        bench_damage_per_head: u32,
    },
    ExtraDamageIfSelfHpAtMost {
        threshold: u32,
        extra_damage: u32,
    },
    CoinFlipShuffleRandomOpponentHandCardIntoDeck,
}
