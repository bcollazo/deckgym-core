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
    HealOneYourPokemon {
        amount: u32,
    },
    HealAllYourPokemon {
        amount: u32,
    },
    CoinFlipSelfHeal {
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
    DiscardOpponentActiveToolsBeforeDamage,
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
    CoinFlipSelfDamage {
        self_damage: u32,
    },
    ExtraDamageForEachHeads {
        include_fixed_damage: bool,
        damage_per_head: u32,
        num_coins: usize,
    },
    DiscardSelfEnergyPerHeadsExtraDamage {
        num_coins: usize,
        energy_type: EnergyType,
        damage_per_discarded_energy: u32,
    },
    CoinFlipNoEffect,
    SelfDiscardEnergy {
        energies: Vec<EnergyType>,
    },
    SelfDiscardEnergyAndInflictStatus {
        energies: Vec<EnergyType>,
        conditions: Vec<StatusCondition>,
    },
    SelfDiscardEnergyAndCardEffect {
        energies: Vec<EnergyType>,
        effect: CardEffect,
        duration: u8,
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
    ExtraDamageIfStadiumInPlay {
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
    CoinFlipSelfChargeActive {
        energies: Vec<EnergyType>,
    },
    ChargeYourTypeAnyWay {
        energy_type: EnergyType,
        count: usize,
    },
    // Fairly unique mechanics
    ManaphyOceanicGift,
    PalkiaExDimensionalStorm,
    MegaKangaskhanExDoublePunchingFamily,
    MoltresExInfernoDance,
    CelebiExPowerfulBloom,
    CoinFlipPerSpecificEnergyType {
        energy_type: EnergyType,
        include_fixed_damage: bool,
        damage_per_heads: u32,
    },
    MagikarpWaterfallEvolution,
    CoinFlipToBlockAttackNextTurn,
    MoveAllEnergyTypeToBench {
        energy_type: EnergyType,
    },
    MoveFixedEnergyTypeToBench {
        energy_type: EnergyType,
        amount: u32,
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
    ExtraDamagePerPokemonTypeInDiscard {
        energy_type: EnergyType,
        damage_per_pokemon: u32,
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
    CoinFlipNoDamageOrDamageAndCardEffect {
        opponent: bool,
        effect: CardEffect,
        duration: u8,
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
    ExtraDamageIfUndamaged {
        extra_damage: u32,
    },
    DamageEqualToSelfDamage,
    ExtraDamageEqualToSelfDamage,
    ExtraDamageIfKnockedOutLastTurn {
        extra_damage: u32,
    },
    ExtraDamageIfMovedFromBench {
        extra_damage: u32,
    },
    ExtraDamageIfEvolvedThisTurn {
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
    /// Choose 1 of the opponent's Pokémon; deal damage_per_energy × (energy on that Pokémon).
    DamageToAnyOpponentPerTargetEnergy {
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
    SelfHealIfStadiumInPlay {
        amount: u32,
    },
    InflictStatusIfStadiumInPlay {
        status: StatusCondition,
    },
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
    ExtraDamageIfOpponentHpMoreThanSelf {
        extra_damage: u32,
    },
    CoinFlipShuffleRandomOpponentHandCardIntoDeck,
    /// Teal Mask Ogerpon ex – Energized Leaves:
    /// If total energy on both Active Pokémon ≥ threshold, deal extra_damage more.
    ExtraDamageIfCombinedActiveEnergyAtLeast {
        threshold: usize,
        extra_damage: u32,
    },
    /// Hearthflame Mask Ogerpon – Hearthflame Dance:
    /// Flip a coin. If heads, take `count` energy of `energy_type` from your Energy Zone
    /// and attach them to 1 of your Benched Pokémon.
    CoinFlipChargeBench {
        energies: Vec<EnergyType>,
        target_benched_type: Option<EnergyType>,
    },
    /// Wellspring Mask Ogerpon – Wellspring Dance:
    /// Flip a coin. If heads, this attack also does `damage` to 1 of the chosen player's
    /// Benched Pokémon (opponent = true → opponent's bench).
    CoinFlipAlsoChoiceBenchDamage {
        opponent: bool,
        damage: u32,
    },
    /// Venoshock – extra damage if opponent's active is Poisoned.
    ExtraDamageIfDefenderPoisoned {
        extra_damage: u32,
    },
}
