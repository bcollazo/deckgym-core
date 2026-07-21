use crate::models::EnergyType;

#[derive(Debug, Clone, PartialEq)]
pub enum AbilityMechanic {
    VictreebelFragranceTrap,
    HealAllYourPokemon {
        amount: u32,
    },
    HealOneYourPokemon {
        amount: u32,
    },
    HealOneYourPokemonExAndDiscardRandomEnergy {
        amount: u32,
    },
    DamageOneOpponentPokemon {
        amount: u32,
    },
    IncreaseDamageIfArceusInPlay {
        amount: u32,
    },
    DamageOpponentActiveIfArceusInPlay {
        amount: u32,
    },
    SwitchDamagedOpponentBenchToActive,
    SwitchThisBenchWithActive,
    SwitchActiveTypedWithBench {
        energy_type: EnergyType,
    },
    SwitchActiveUltraBeastWithBench,
    MoveTypedEnergyFromBenchToActive {
        energy_type: EnergyType,
    },
    /// Lunala ex's Psychic Connect: "Once during your turn, you may move all [energy_type] Energy
    /// from 1 of your Benched [energy_type] Pokémon to your Active Pokémon." Unlike
    /// `MoveTypedEnergyFromBenchToActive`, all of the chosen Pokémon's matching Energy moves at
    /// once, it is once per turn, and the Active Pokémon may be any type.
    MoveAllTypedEnergyFromBenchToActive {
        energy_type: EnergyType,
    },
    AttachEnergyFromZoneToActiveTypedPokemon {
        energy_type: EnergyType,
    },
    AttachEnergyFromZoneToYourTypedPokemon {
        energy_type: EnergyType,
    },
    AttachEnergyFromZoneToSelf {
        energy_type: EnergyType,
        amount: u32,
    },
    AttachEnergyFromZoneToSelfAndEndTurn {
        energy_type: EnergyType,
    },
    AttachEnergyFromZoneToSelfAndDamage {
        energy_type: EnergyType,
        amount: u32,
        self_damage: u32,
    },
    DamageOpponentActiveOnZoneAttachToSelf {
        energy_type: EnergyType,
        amount: u32,
        only_turn_energy: bool,
    },
    AttachEnergyFromDiscardToSelfAndDamage {
        energy_type: EnergyType,
        self_damage: u32,
    },
    ReduceDamageFromAttacks {
        amount: u32,
    },
    ReduceOpponentActiveDamage {
        amount: u32,
    },
    IncreaseDamageWhenRemainingHpAtMost {
        amount: u32,
        hp_threshold: u32,
    },
    IncreaseDamageForTypeInPlay {
        energy_type: EnergyType,
        amount: u32,
    },
    IncreaseDamageForTwoTypesInPlay {
        energy_type_a: EnergyType,
        energy_type_b: EnergyType,
        amount: u32,
    },
    /// Passive: each of your Pokémon of the given type gets +`amount` HP for as long as
    /// this Pokémon (the ability holder) is in play. Stacks if multiple sources are in play.
    IncreaseHpForTypeInPlay {
        energy_type: EnergyType,
        amount: u32,
    },
    StartTurnRandomPokemonToHand {
        energy_type: EnergyType,
    },
    SearchRandomPokemonFromDeck,
    MoveDamageFromOneYourPokemonToThisPokemon,
    DiscardOpponentActiveToolsAndDiscardSelf,
    PreventFirstAttack,
    ElectromagneticWall,
    InfiltratingInspection,
    DiscardTopCardOpponentDeck,
    CoinFlipToPreventDamage,
    /// Ursaluna's Guts: if this Pokémon would be Knocked Out by damage from an attack, flip a
    /// coin. If heads, it is not Knocked Out and its remaining HP becomes 10.
    CoinFlipToSurviveKnockOut,
    /// Passimian ex's Offload Pass: if this Pokémon is in the Active Spot and is Knocked Out by
    /// damage from an opponent's attack, move all of its `energy_type` Energy to 1 of your Benched
    /// Pokémon (your choice). Passive; handled in the `on_knockout` hook.
    MoveAllTypedEnergyToBenchOnKnockout {
        energy_type: EnergyType,
    },
    CheckupDamageToOpponentActive {
        amount: u32,
    },
    CheckupDamageToAllOpponentPokemon {
        amount: u32,
    },
    DiscardEnergyToIncreaseTypeDamage {
        discard_energy: EnergyType,
        attack_type: EnergyType,
        amount: u32,
    },
    PoisonOpponentActive,
    ConfuseOpponentActive,
    BurnOpponentActive,
    RemoveRandomSpecialConditionFromActive,
    HealActiveYourPokemon {
        amount: u32,
    },
    SwitchOutOpponentActiveToBench {
        require_active: bool,
    },
    BadDreamsEndOfTurn {
        amount: u32,
    },
    EndTurnDrawCardIfActive {
        amount: u32,
    },
    EndTurnHealSelfIfActive {
        amount: u32,
    },
    CoinFlipSleepOpponentActive,
    DiscardFromHandToDrawCard,
    ImmuneToStatusConditions,
    /// Passive ability shared by Teal Mask Ogerpon ex (Soothing Wind) and Comfey (Flower Shield):
    /// Each of your Pokémon that has the required Energy attached recovers from all Special
    /// Conditions and can't be affected by any Special Conditions.
    ///   - `energy_type: None`  → any energy (Ogerpon ex – Soothing Wind)
    ///   - `energy_type: Some(t)` → only the specified type (Comfey – Flower Shield, `[P]`)
    SoothingWind {
        energy_type: Option<EnergyType>,
    },
    NoOpponentSupportInActive,
    /// Snorlax's Massive Body: as long as this Pokémon is in the Active Spot, the opponent
    /// can't play any Stadium cards from their hand.
    NoOpponentStadiumInActive,
    DoubleGrassEnergy,
    PreventOpponentActiveEvolution,
    ReduceRetreatCostOfYourActiveBasicFromBench {
        amount: u32,
    },
    ReduceRetreatCostOfYourActiveTypedFromBench {
        energy_type: EnergyType,
        amount: u32,
    },
    NoRetreatIfHasEnergy,
    PreventAllDamageFromEx,
    SleepOnZoneAttachToSelfWhileActive,
    IncreasePoisonDamage {
        amount: u32,
    },
    DrawCardsOnEvolve {
        amount: u32,
    },
    HealTypedPokemonOnEvolve {
        energy_type: EnergyType,
        amount: u32,
    },
    AttachEnergyFromZoneToActiveTypedOnEvolve {
        energy_type: EnergyType,
    },
    DamageOpponentActiveOnEvolve {
        amount: u32,
    },
    DiscardRandomEnergyFromOpponentActiveOnEvolve,
    CanEvolveIntoEeveeEvolution,
    CanEvolveOnFirstTurnIfActive,
    CounterattackDamage {
        amount: u32,
    },
    PoisonAttackerOnDamaged,
    IncreaseAttackCostForOpponentActive {
        amount: u32,
    },
    IncreaseRetreatCostForOpponentActive {
        amount: u32,
    },
    PreventDamageWhileBenched,
    IncreaseHpPerAttachedEnergy {
        energy_type: EnergyType,
        amount: u32,
    },
    HealSelfOnZoneAttach {
        energy_type: EnergyType,
        amount: u32,
    },
    EndFirstTurnAttachEnergyToSelf {
        energy_type: EnergyType,
    },
    ProtectSelfNextTurnAfterAttackKnockout,
    MoveFixedDamageFromActiveToThisBenched {
        amount: u32,
    },
    /// "Once during your turn, when you put this Pokémon from your hand onto your Bench,
    /// you may switch it with your Active Pokémon. If you do, move all of your Energy
    /// in play to this Pokémon."
    LegendaryDrive,
    /// "Once during your turn, when you put this Pokémon from your hand onto your Bench,
    /// you may switch out your opponent's Active Pokémon to the Bench.
    /// (Your opponent chooses the new Active Pokémon.)"
    AncientRoar,
    /// "Attacks used by your Future Pokémon cost 1 less [C] Energy."
    FutureSystem,
    /// Celebi's Time Recall: "Each of your evolved Pokémon can use any attack from its previous
    /// Evolutions. (You still need the necessary Energy to use each attack.)"
    /// Passive: while a Pokémon with this ability is in play, attack generation also offers the
    /// active evolved Pokémon the attacks from its previous evolutions (its under-cards).
    TimeRecall,
    /// Caterpie's Quick Growth: "At the end of your opponent's turn, if this Pokémon is in the
    /// Active Spot, put a random card from your deck that evolves from this Pokémon onto this
    /// Pokémon to evolve it."
    QuickGrowth,
}
