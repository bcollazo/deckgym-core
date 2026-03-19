use crate::models::EnergyType;

#[derive(Debug, Clone, PartialEq)]
pub enum AbilityMechanic {
    HealAllYourPokemon {
        amount: u32,
    },
    HealOneYourPokemonExAndDiscardRandomEnergy {
        amount: u32,
    },
    DamageOneOpponentPokemon {
        amount: u32,
    },
    SwitchActiveTypedWithBench {
        energy_type: EnergyType,
    },
    AttachEnergyFromZoneToActiveTypedPokemon {
        energy_type: EnergyType,
    },
    ReduceDamageFromAttacks {
        amount: u32,
    },
    IncreaseDamageWhenRemainingHpAtMost {
        amount: u32,
        hp_threshold: u32,
    },
    StartTurnRandomPokemonToHand {
        energy_type: EnergyType,
    },
    PreventFirstAttack,
    ElectromagneticWall,
    InfiltratingInspection,
    DiscardTopCardOpponentDeck,
    CoinFlipToPreventDamage,
    CheckupDamageToOpponentActive {
        amount: u32,
    },
    DiscardEnergyToIncreaseTypeDamage {
        discard_energy: EnergyType,
        attack_type: EnergyType,
        amount: u32,
    },
    // ── Active (on-demand) abilities ──────────────────────────────────────────
    PoisonOpponentActive,
    SwitchOutOpponentActive,
    SwitchInOpponentBenchedBasic,
    AttachEnergyFromZoneToSelf {
        energy_type: EnergyType,
    },
    AttachEnergyFromZoneToSelfEndTurn {
        energy_type: EnergyType,
    },
    AttachMultipleEnergyFromZoneToSelfAndSelfDamage {
        energy_type: EnergyType,
        count: u32,
        damage: u32,
    },
    AttachEnergyFromDiscardToSelfAndSelfDamage {
        energy_type: EnergyType,
        damage: u32,
    },
    HealActivePokemon {
        amount: u32,
    },
    HealOneOfYourPokemon {
        amount: u32,
    },
    SwitchInOpponentDamagedBenched,
    SwitchBenchedSelfToActive,
    DrawRandomPokemonFromDeck,
    MoveAllDamageToSelf,
    DamageOpponentActiveIfArceusInPlay {
        amount: u32,
    },
    AttachEnergyFromZoneToTypedPokemon {
        energy_type: EnergyType,
    },
    MoveEnergyFromBenchedTypedToActiveTyped {
        energy_type: EnergyType,
    },
    SwitchUltraBeast,
    // ── Passive (hook-driven) abilities ───────────────────────────────────────
    BlockSupportCards,
    GrassEnergyDoubling,
    BlockOpponentEvolution,
    BenchReduceBasicRetreatCost,
    NoRetreatCostWithEnergy,
    TypeDamageBoost {
        energy_type: EnergyType,
        amount: u32,
    },
    DamageOpponentActiveOnEnergyAttachFromZone {
        energy_type: EnergyType,
        amount: u32,
    },
    SafeguardFromEx,
    AsleepOnEnergyAttachFromZoneWhenActive,
    ReduceOpponentAttackDamage {
        amount: u32,
    },
    AttachEnergyFromZoneToSelfOnFirstTurn {
        energy_type: EnergyType,
    },
    MorePoisonDamage {
        amount: u32,
    },
    CounterattackOnDamage {
        amount: u32,
    },
    PoisonAttackerOnDamage,
    ImmuneToSpecialConditions,
    OnEvolveDraw {
        amount: u32,
    },
    VeeveeVolve,
    EndTurnHealSelf {
        amount: u32,
    },
    EndTurnDrawCard,
    OnEvolveHealTypedPokemon {
        energy_type: EnergyType,
        amount: u32,
    },
    MultiTypeDamageBoost {
        energy_types: Vec<EnergyType>,
        amount: u32,
    },
    IncreaseOpponentAttackCost,
    BoostedEvolution,
    IncreaseOpponentRetreatCost,
    OnEvolveAttachEnergyFromZoneToActiveTypedPokemon {
        energy_type: EnergyType,
    },
    BenchSafeguard,
    HpBonusPerEnergy {
        energy_type: EnergyType,
        per_energy_hp: u32,
    },
    HealOnEnergyAttachFromZone {
        energy_type: EnergyType,
        amount: u32,
    },
}
