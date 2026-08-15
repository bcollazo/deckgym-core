use crate::models::{EnergyType, StatusCondition};

#[derive(Debug, Clone, PartialEq)]
pub enum AbilityMechanic {
    VictreebelFragranceTrap,
    /// Heal `amount` damage from each of your Pokémon. `energy_type` restricts which Pokémon are
    /// healed: `None` heals all of them, `Some(t)` heals only your Pokémon of that type (e.g.
    /// Primarina's Melodious Healing, which heals only your [W] Pokémon).
    HealAllYourPokemon {
        amount: u32,
        energy_type: Option<EnergyType>,
    },
    HealOneYourPokemon {
        amount: u32,
    },
    /// Sylveon's Soothing Ribbon: "Once during your turn, if this Pokémon has a Pokémon Tool
    /// attached, you may heal `amount` damage from 1 of your Pokémon."
    HealOneYourPokemonIfHasTool {
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
    /// Porygon2's Buggy Evolution: "Whenever you attach an Energy from your Energy Zone to this
    /// Pokémon, put a random card from your deck that evolves from this Pokémon onto this
    /// Pokémon to evolve it."
    EvolveFromDeckOnZoneEnergyAttachToSelf,
    AttachEnergyFromDiscardToSelfAndDamage {
        energy_type: EnergyType,
        self_damage: u32,
    },
    /// Dragonair's Dragon's Blessing: "Once during your turn, if this Pokémon is on your Bench,
    /// you may attach an Energy from your discard pile to your Active Pokémon." The player
    /// chooses which discarded Energy type to attach when the discard pile holds more than one.
    AttachEnergyFromDiscardToActiveFromBench,
    ReduceDamageFromAttacks {
        amount: u32,
    },
    /// Magnezone's Resilience Link: like `ReduceDamageFromAttacks`, but only while its owner has
    /// Arceus or Arceus ex in play. Because it depends on the board it cannot be modeled as a
    /// plain `CardEffect` derived from the card alone; it is resolved in `hooks::modify_damage`.
    ReduceDamageFromAttacksIfArceusInPlay {
        amount: u32,
    },
    /// Abomasnow's Vigor Link: "If you have Arceus or Arceus ex in play, attacks used by this
    /// Pokémon cost `amount` less [C] Energy." Depends on the board, so it's resolved in
    /// `hooks::get_attack_cost` rather than being a plain `CardEffect`.
    ReduceAttackCostIfArceusInPlay {
        amount: u8,
    },
    /// Mamoswine's Thick Fat: "This Pokémon takes `amount` less damage from attacks from
    /// Pokémon of any of `attacker_types`." Depends on the attacker's type, so it's resolved in
    /// `hooks::modify_damage` rather than being a plain `CardEffect`.
    ReduceDamageFromAttacksByAttackerType {
        amount: u32,
        attacker_types: Vec<EnergyType>,
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
    /// Bastiodon's Guarded Grill / Hisuian Goodra's Securely Sheltered: if any damage is done to
    /// this Pokémon by attacks, flip a coin. If heads, this Pokémon takes `amount` less damage
    /// from that attack. Passive; handled like `CoinFlipToPreventDamage` via the
    /// abilities-as-effects pathway.
    CoinFlipToReduceDamage {
        amount: u32,
    },
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
    /// Dustox's Variety Powder: 1 Special Condition is chosen at random from `options` and
    /// inflicted on the opponent's Active Pokémon. Conditions already affecting that Pokémon are
    /// excluded from the draw, so the ability is unusable once all `options` are applied.
    RandomStatusConditionToOpponentActive {
        options: Vec<StatusCondition>,
    },
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
    /// Raichu's Evoshock: "Once during your turn, when you play this Pokémon from your hand to
    /// evolve 1 of your Pokémon, you may flip a coin. If heads, your opponent's Active Pokémon is
    /// now Paralyzed." Offered as an optional `UseAbility` when the evolution resolves.
    CoinFlipParalyzeOpponentActiveOnEvolve,
    DiscardRandomEnergyFromOpponentActiveOnEvolve,
    CanEvolveIntoEeveeEvolution,
    CanEvolveOnFirstTurnIfActive,
    CounterattackDamage {
        amount: u32,
    },
    PoisonAttackerOnDamaged,
    /// Jellicent's Bouncy Body: if this Pokémon is in the Active Spot and is damaged by an attack
    /// from the opponent's Pokémon, its owner takes an Energy of `energy_type` from their Energy
    /// Zone and attaches it to 1 of their Benched Pokémon (their choice). Passive; triggered from
    /// the damage path.
    AttachEnergyFromZoneToBenchedOnDamaged {
        energy_type: EnergyType,
    },
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
    /// Victini's Victory Star: "Once during your turn, after you flip any coins for an attack of
    /// 1 of your [R] Pokémon, you may ignore all results of those coin flips and begin flipping
    /// those coins again. You can't use more than 1 Victory Star Ability each turn."
    ///
    /// Reactive rather than freely activated: it is never offered by normal ability move
    /// generation, only pushed onto the move-generation stack by `apply_action` immediately
    /// after an eligible [R] attack's coins are flipped. See `PendingCoinReflip`.
    VictoryStarReflip,
}
