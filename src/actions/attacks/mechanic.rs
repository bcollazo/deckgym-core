use crate::{
    effects::{CardEffect, TurnEffect},
    models::{EnergyType, StatusCondition, TrainerType},
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
    HealOneYourBenchedPokemon {
        amount: u32,
    },
    HealAllYourPokemon {
        amount: u32,
    },
    CoinFlipSelfHeal {
        amount: u32,
    },
    /// Cradily's Stick and Absorb: deal damage, heal `heal_amount` from the attacking Pokémon, then
    /// apply a `CardEffect` to an Active Pokémon (`opponent: true` → the Defending Pokémon).
    /// `SelfHeal` plus `DamageAndCardEffect` in one attack.
    SelfHealAndCardEffect {
        heal_amount: u32,
        opponent: bool,
        effect: CardEffect,
        duration: u8,
    },
    SearchToHandByEnergy {
        energy_type: EnergyType,
    },
    SearchToBenchByName {
        name: String,
    },
    /// Silcoon's & Cascoon's Cocoon Collector: "Put 3 random cards from among Silcoon and Cascoon
    /// from your deck onto your Bench." Puts up to `count` random cards named after any of `names`
    /// onto the Bench, limited by how many are in the deck and by the available Bench space.
    SearchToBenchByNames {
        names: Vec<String>,
        count: usize,
    },
    SearchToBenchBasic,
    SearchRandomPokemonToHand,
    SearchToHandByEvolvesFrom {
        name: String,
    },
    SearchToHandSupporterCard,
    InflictStatusConditions {
        conditions: Vec<StatusCondition>,
        target_opponent: bool,
    },
    InflictStatusConditionsOnBothActive {
        conditions: Vec<StatusCondition>,
    },
    ChanceStatusAttack {
        condition: StatusCondition,
    },
    /// Deal damage, then let the player choose one of these Special Conditions to
    /// inflict on the opponent's Active Pokémon (e.g. Dustox's Select Powder).
    ChooseStatusToInflict {
        options: Vec<StatusCondition>,
    },
    DamageAllOpponentPokemon {
        damage: u32,
    },
    DiscardRandomGlobalEnergy {
        count: usize,
    },
    /// Porygon-Z's Buggy Beam: replace the Energy queued up in the opponent's Energy Zone with a
    /// uniformly random one of the 8 basic Energy types, regardless of their deck's Energy.
    RandomizeOpponentNextEnergy,
    RandomDamageToOpponentPokemonPerSelfEnergy {
        energy_type: EnergyType,
        damage_per_hit: u32,
    },
    DiscardEnergyFromOpponentActive,
    CoinFlipDiscardEnergyFromOpponentActive,
    /// Pidgeot's Twister / Mega Pidgeot ex's Giant Twister: flip `num_coins` coins and discard a
    /// random Energy from the opponent's Active Pokémon for each heads. If every coin is tails
    /// the attack does nothing at all — not even its `fixed_damage`.
    CoinFlipsDiscardEnergyFromOpponentActiveOrNothing {
        num_coins: usize,
    },
    DiscardOpponentActiveToolsBeforeDamage,
    ExtraDamageIfEx {
        extra_damage: u32,
    },
    ExtraDamageIfDefenderType {
        energy_type: EnergyType,
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
    ExtraDamageIfDifferentEnergyTypesAttached {
        minimum_types: usize,
        extra_damage: u32,
    },
    ExtraDamageIfTypeEnergyInPlay {
        energy_type: EnergyType,
        minimum_count: usize,
        extra_damage: u32,
    },
    /// Medicham's "Psykick" / Mega Medicham ex's "Chakra Fist": extra damage if the attacking
    /// Pokémon has any Energy of `energy_type` attached. (Chakra Fist additionally shares Sawk's
    /// "isn't affected by any effects on your opponent's Active Pokémon" clause, which is detected
    /// separately from the attack's effect text in `hooks::modify_damage`.)
    ExtraDamageIfSelfHasTypeEnergy {
        energy_type: EnergyType,
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
    /// Gigalith ex's Megaton Cannon: `DirectDamage` that additionally leaves a `CardEffect` on the
    /// attacking Pokémon (e.g. "During your next turn, this Pokémon can't attack.").
    DirectDamageAndSelfCardEffect {
        damage: u32,
        bench_only: bool,
        effect: CardEffect,
        duration: u8,
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
    /// Manaphy's Oceanic Gift / Carbink's Glittering Gift: choose 2 of your Benched Pokémon and
    /// attach an Energy of the given type to each.
    AttachEnergyFromZoneToTwoBenched {
        energy_type: EnergyType,
    },
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
    /// Ho-Oh ex's Phoenix Turbo: deal `fixed_damage`, then attach each of these Energies to your
    /// Benched Basic Pokémon "in any way you like" (each Energy is placed independently, so all on
    /// one Pokémon is allowed). Fossils count as Basic. If there is no Benched Basic Pokémon the
    /// Energy simply fizzles; the damage is still dealt.
    AttachEnergiesAnyWayToBenchedBasic {
        energies: Vec<EnergyType>,
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
    /// Extra damage for each card of a given Trainer kind in your discard pile (e.g. Chandelure's
    /// Past Friends counts Supporters, Rotom ex's Junk Spark counts Items).
    ExtraDamagePerTrainerTypeInDiscard {
        trainer_type: TrainerType,
        damage_per_card: u32,
    },
    ExtraDamagePerPokemonTypeInDiscard {
        energy_type: EnergyType,
        damage_per_pokemon: u32,
    },
    ExtraDamagePerPokemonInDiscard {
        damage_per_pokemon: u32,
    },
    ExtraDamagePerOwnPoint {
        damage_per_point: u32,
    },
    ExtraDamageIfCardInDiscard {
        card_name: String,
        extra_damage: u32,
    },
    DamageUnaffectedByWeakness,
    /// Sawk's Brick Break: fixed damage whose value "isn't affected by any effects on your
    /// opponent's Active Pokémon." The bypass itself is handled in `hooks::modify_damage` and the
    /// defender-prevention path via the attack's effect text; this variant just routes the attack
    /// as ordinary active damage (like `DamageUnaffectedByWeakness`).
    DamageUnaffectedByOpponentActiveEffects,
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
    /// Mega Rayquaza ex's Mega Burst: discard every Energy of the listed types from the attacking
    /// Pokémon, dealing `damage_per_energy` for each Energy discarded in this way (the attack's
    /// `fixed_damage` is the per-Energy amount, so it is not added as a base).
    SelfDiscardAllTypesEnergyDamagePerDiscarded {
        energy_types: Vec<EnergyType>,
        damage_per_energy: u32,
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
    /// Vespiquen ex's Chase Order: "You may discard 1 of your Benched Basic [G] Pokémon. If you
    /// do, this attack does 70 more damage." The attacker chooses between the plain damage and
    /// discarding one eligible Benched Basic Pokémon for the boosted damage.
    OptionalDiscardBenchedBasicForExtraDamage {
        energy_type: EnergyType,
        extra_damage: u32,
    },
    ExtraDamageIfStage2OnBench {
        extra_damage: u32,
    },
    ExtraDamageIfPokemonOnBench {
        pokemon_name: String,
        extra_damage: u32,
    },
    DamageEqualToSelfDamage,
    ExtraDamageEqualToSelfDamage,
    ExtraDamageIfKnockedOutLastTurn {
        extra_damage: u32,
    },
    ExtraDamageIfAttackUsedDuringOwnLastTurn {
        attack_name: String,
        extra_damage: u32,
    },
    DamagePerAttackUsedThisGame {
        attack_name: String,
        damage_per_use: u32,
    },
    ExtraDamageIfMovedFromBench {
        extra_damage: u32,
    },
    ExtraDamageIfEvolvedThisTurn {
        extra_damage: u32,
    },
    /// Politoed's Raid: extra damage if this Pokémon evolved *from a specific Pokémon* during
    /// this turn. Unlike `ExtraDamageIfEvolvedThisTurn` this also checks the card directly
    /// underneath, so evolving via Rare Candy (which skips the named Stage 1) does not qualify.
    ExtraDamageIfEvolvedFromThisTurn {
        pokemon_name: String,
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
        include_fixed_damage: bool,
        opponent: bool,
        damage_per_energy: u32,
    },
    ExtraDamagePerEnergyType {
        damage_per_type: u32,
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
    ExtraDamagePerSpecificEnergyAllYours {
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
    /// Like `FlipUntilTailsDamage`, but the attack's `fixed_damage` is dealt as a base and each
    /// heads adds `damage_per_heads` on top (e.g. "does 30 more damage for each heads").
    FlipUntilTailsBonusDamage {
        damage_per_heads: u32,
    },
    DirectDamageIfDamaged {
        damage: u32,
    },
    AttachEnergyToBenchedBasic {
        energy_type: EnergyType,
    },
    DamageAndDiscardOpponentDeck {
        discard_count: usize,
    },
    /// Coalossal's Mountain Crush: deal the attack's `fixed_damage`, then flip a coin until
    /// tails, discarding the top card of the opponent's deck for each heads.
    FlipUntilTailsDiscardOpponentDeck,
    MegaAmpharosExLightningLancer,
    OminousClaw,
    DarknessClaw,
    BlockBasicAttack,
    SwitchSelfWithBench,
    MaySwitchSelfWithBench,
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
    /// Wailord ex's Wondrous Waves: after dealing damage, the attacking Pokémon recovers from
    /// all Special Conditions.
    SelfCureStatusConditions,
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
    ExtraDamageIfOpponentActiveHasAbility {
        extra_damage: u32,
    },
    /// Honchkrow – Evil Admonition: extra damage for each of the opponent's
    /// Pokémon in play (active and bench) that has an Ability.
    ExtraDamagePerOpponentPokemonWithAbility {
        damage_per: u32,
    },
    CoinFlipShuffleRandomOpponentHandCardIntoDeck,
    /// Persian - Shadow Claw: flip a coin; if heads, discard a random card
    /// from the opponent's hand after dealing damage.
    CoinFlipDiscardRandomOpponentHandCard,
    /// Krookodile - Poaching Fangs: flip 'num_coins' cions; for each heads, shuffle
    /// a random card from the opponent's hand into their deck.
    CoinFlipsShuffleOpponentHandCards {
        num_coins: usize,
    },
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
    /// Hatterene – Mental Crush: extra damage if opponent's active is Confused.
    ExtraDamageIfDefenderConfused {
        extra_damage: u32,
    },
    /// Breloom – Pre-Dawn Strike: extra damage if opponent's active is Asleep.
    ExtraDamageIfDefenderAsleep {
        extra_damage: u32,
    },
    /// Discard the top card of the attacker's own deck after dealing damage.
    DiscardTopSelfDeck {
        count: usize,
    },
    /// Tiered coin flip damage: flip `num_coins` coins and deal fixed_damage +
    /// extra_damage_by_heads[heads_count] total damage.
    TieredCoinFlipDamage {
        num_coins: usize,
        extra_damage_by_heads: Vec<u32>,
    },
    /// First attack after coming into play: conditionally apply a turn effect (e.g. Flutter Mane).
    FirstAttackBonusTurnEffect {
        effect: TurnEffect,
        duration: u8,
    },
    /// First attack after coming into play: conditionally deal extra damage and inflict status (e.g. Iron Bundle).
    FirstAttackBonusDamageAndStatus {
        extra_damage: u32,
        conditions: Vec<StatusCondition>,
    },
    /// Growlithe – Puppy Pile: deal damage_per × (number of own Pokémon in play and hand
    /// that have an attack named `attack_name`).
    DamagePerOwnPokemonWithAttackName {
        attack_name: String,
        damage_per: u32,
    },
    /// Emolga (Windup Thunder) / Dedenne ex (Dede-Circuit):
    /// deal `damage_per` damage for each Pokémon Tool attached to any of your
    /// Pokémon in play (active + bench).
    DamagePerOwnToolAttached {
        damage_per: u32,
    },
}
