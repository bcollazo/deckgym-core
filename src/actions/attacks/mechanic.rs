use crate::{
    effects::{CardEffect, TurnEffect},
    models::{EnergyType, StatusCondition, TrainerType},
};

use crate::card_ids::CardId;

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
    /// Heal `amount` from each Benched Pokémon; if `only_basic` is true, only Basic Pokémon
    /// (Alomomola heals all, Ho-Oh heals only Basic).
    HealAllBenchedPokemon {
        amount: u32,
        only_basic: bool,
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
    /// Drampa's Dragon Breath: flip a coin; on tails this attack does nothing (no damage either).
    /// On heads, deal the attack's fixed damage and inflict `status` on the opponent's Active.
    CoinFlipNoDamageOrStatusAttack {
        status: StatusCondition,
    },
    /// Flip a coin; heads inflicts `heads_status`, tails inflicts `tails_status`, both on the
    /// opponent's Active (e.g. Lanturn ex).
    CoinFlipStatusOutcome {
        heads_status: StatusCondition,
        tails_status: StatusCondition,
    },
    /// Flip a coin; heads inflicts ALL of `conditions` on the opponent's Active (e.g.
    /// Tentacruel's Poisoned and Paralyzed).
    ChanceMultipleStatusAttack {
        conditions: Vec<StatusCondition>,
    },
    /// Flip a coin; heads inflicts `status` on the opponent's Active, tails inflicts it on the
    /// attacking Pokémon itself (e.g. Psyduck's Confusion Wave).
    CoinFlipStatusSelfOrOpponent {
        status: StatusCondition,
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
    /// Discard one `energy_type` Energy from the opponent's Active (e.g. Dedenne, Surskit).
    DiscardOpponentActiveEnergyOfType {
        energy_type: EnergyType,
    },
    /// Discard a random Energy from BOTH Active Pokémon (e.g. Oricorio, Yveltal).
    DiscardRandomEnergyBothActive,
    CoinFlipDiscardEnergyFromOpponentActive,
    /// Pidgeot's Twister / Mega Pidgeot ex's Giant Twister: flip `num_coins` coins and discard a
    /// random Energy from the opponent's Active Pokémon for each heads. If every coin is tails
    /// the attack does nothing at all — not even its `fixed_damage`.
    CoinFlipsDiscardEnergyFromOpponentActiveOrNothing {
        num_coins: usize,
    },
    /// Maushold - Triple Gnawing: flip 'num_coins'; for each heads, discard a
    /// random Energy form the opponent's Active Pokémon. Damage always applies.
    CoinFlipsDiscardEnergyFromOpponentActive {
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
    /// Flip a coin; heads deals `damage` to the opponent's Active, tails heals `heal` from it
    /// (e.g. Delibird's Present).
    CoinFlipDamageOrHealOpponent {
        damage: u32,
        heal: u32,
    },
    CoinFlipSelfDamage {
        self_damage: u32,
    },
    /// Flip a coin; on tails discard `count` random Energy from the attacking Pokémon
    /// (e.g. Entei).
    CoinFlipSelfDiscardRandomEnergy {
        count: usize,
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
    /// Gouging Fire's Scorching Interruption: discard `count` random Energy from the attacking
    /// Pokémon, then leave a `CardEffect` on it (e.g. reduced damage taken next turn).
    SelfDiscardRandomEnergyAndCardEffect {
        count: usize,
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
    /// Team Rocket's Moltres ex's Heat Charged: flip `num_coins` coins; for each heads, produce
    /// an Energy of `energy_type` from the Energy Zone and attach it to the attacking Pokémon
    /// itself.
    CoinFlipsAttachEnergyToSelf {
        num_coins: usize,
        energy_type: EnergyType,
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
    /// Bellossom - Petal Dance: flip 'num_coins' coins; deal 'damage_per_head' per hedas,
    /// then apply 'status' to the ATTACKER (unlike the opponent-targeting variants).
    ExtraDamageForEachHeadsSelfStatus {
        num_coins: usize,
        damage_per_head: u32,
        status: StatusCondition,
    },
    /// Alolan Marowak - Burning Bonemerang: flip 'num_coins'. This attack does 70 damage
    /// for each heads. If at least 1 of them is heads, your opponent's Active Pokémon
    /// is now Burned.
    ExtraDamageForEachHeadsWithStatusAtLeast {
        num_coins: usize,
        damage_per_head: u32,
        status: StatusCondition,
        min_heads: usize,
    },
    /// Ambipom - Excited Tail: flip 'num_coins' coins; deal 'damage_per_head' per heads,
    /// but frlip 'boosted_num_coins' coins instead if the attacker has 'tool' attached.
    ExtraDamageForEachHeadsWithToolBoost {
        num_coins: usize,
        damage_per_head: u32,
        boosted_num_coins: usize,
        tool: CardId,
    },
    /// Croagunk / Toxicroak: flip one coin for each of your Pokémon in play, dealing
    /// `damage_per_head` for each heads.
    CoinFlipPerPokemonInPlay {
        damage_per_head: u32,
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
    ExtraDamagePerOpponentPoint {
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
    /// Draw a card for each of your Pokémon in play named `name` (e.g. Poochyena).
    DrawPerPokemonWithName {
        name: String,
    },
    /// Flip a coin; on heads set the opponent's Active remaining HP to `hp` (e.g. Xatu).
    CoinFlipSetOpponentHpTo {
        hp: u32,
    },
    SelfDiscardAllEnergy,
    /// Raging Bolt's Baneful Boom: discard all Energy from the attacking Pokémon, then Knock Out
    /// the opponent's Active Pokémon outright (no damage calculation involved).
    SelfDiscardAllEnergyAndKnockOutOpponentActive,
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
    SelfDiscardRandomEnergy {
        count: usize,
    },
    AlsoBenchDamage {
        opponent: bool,
        damage: u32,
        must_have_energy: bool,
    },
    /// Walking Wake's Sweeping Billow: discard `count` random Energy from the attacking
    /// Pokémon, and this attack also does `bench_damage` to each of the chosen player's
    /// Benched Pokémon (opponent = true → opponent's bench).
    SelfDiscardRandomEnergyAndBenchDamage {
        count: usize,
        opponent: bool,
        bench_damage: u32,
    },
    AlsoChoiceBenchDamage {
        opponent: bool,
        damage: u32,
    },
    /// Toxtricity ex's Damaging Spark: deal the attack's fixed damage to the Defending Pokémon,
    /// then also deal `damage` to EVERY one of `opponent`'s Benched Pokémon that already has
    /// damage on it.
    AlsoBenchDamageIfDamaged {
        opponent: bool,
        damage: u32,
    },
    /// Team Rocket's Zapdos ex's Thunderclaw: deal the attack's fixed damage to the Defending
    /// Pokémon, then also let the attacker choose 1 of `opponent`'s Benched Pokémon that already
    /// has damage on it to deal `damage` to.
    AlsoChoiceBenchDamageIfDamaged {
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
    /// Regidrago's Draconic Slam: "If this Pokémon has damage on it, this attack does -100
    /// damage." The attack's `fixed_damage` is the undamaged-self base; `reduction` is subtracted
    /// (floored at 0) when the attacking Pokémon already has damage on it.
    ReducedDamageIfSelfDamaged {
        reduction: u32,
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
    /// Drampa's Berserk: extra damage if any of the attacker's own Benched Pokémon already
    /// have damage on them.
    ExtraDamageIfAnyBenchedDamaged {
        extra_damage: u32,
    },
    /// Nidoqueen - Lovestrike: +'damage_per' for EACH benched Pokémon named 'pokemon_name'
    /// (unlike 'ExtraDamageIfPokemonBench', which is a flat bonus for a single presence).
    ExtraDamagePerPokemonWithNameOnBench {
        pokemon_name: String,
        damage_per: u32,
    },
    DamageEqualToSelfDamage,
    ExtraDamageEqualToSelfDamage,
    /// Marshadow's Revenge and friends: extra damage if any of your Pokemon were Knocked Out by
    /// an attack during the opponent's last turn. `energy_type` restricts which of your Pokemon
    /// count (e.g. Zarude's Dark Vengeance only counts `[D]` Pokemon); `None` counts any.
    ExtraDamageIfKnockedOutLastTurn {
        energy_type: Option<EnergyType>,
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
    /// Team Rocket's Slowking ex's Hand Kinesis: deal `damage_per_card` damage for each card in
    /// the attacker's own hand.
    DamagePerOwnHandCard {
        damage_per_card: u32,
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
        include_fixed_damage: bool,
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
    /// Kabutops - Leech Life: heal the same amount of damage dealt.
    HealEqualToDamageDealt,
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

    /// Tapu Koko's Volt Switch: like `SwitchSelfWithBench`, but the Bench choice is restricted to
    /// your Pokémon of `energy_type`.
    SwitchSelfWithBenchOfType {
        energy_type: EnergyType,
    },
    /// Uxie's Mind Boost: take an Energy of `energy_type` from your Energy Zone and attach it to
    /// one of your in-play Pokémon whose name is in `names` (e.g. Mesprit or Azelf).
    AttachEnergyFromZoneToNamed {
        energy_type: EnergyType,
        names: Vec<String>,
    },
    /// Sableye's Jeweled Gift: take one uniformly random basic Energy (of the 8 selectable types)
    /// from your Energy Zone and attach it to 1 of your Benched Pokémon.
    AttachRandomBasicEnergyToBenched,
    /// Forretress's Enormous Explosion: on top of the attack's damage to the Defending Pokémon,
    /// deal `self_damage` to the attacking Pokémon and `bench_damage` to EVERY Benched Pokémon on
    /// both sides.
    SelfDamageAndAllBenchDamage {
        self_damage: u32,
        bench_damage: u32,
    },
    /// Mimikyu's Shadow Hit: this attack also does `damage` to 1 of YOUR OWN Pokémon — the
    /// attacking Pokémon itself included (unlike `AlsoChoiceBenchDamage`, which is Bench-only).
    AlsoChoiceOwnPokemonDamage {
        damage: u32,
    },
    /// Archeops's Wild Spin: deal `damage` to each of the opponent's Pokémon, plus `increment` for
    /// each `IncreasedDamageForAttack` effect this attack left on itself during a previous turn.
    /// The spread damage cannot use the ordinary Active-to-Active `IncreasedDamageForAttack` hook,
    /// so the bonus is read off the attacker here instead.
    DamageAllOpponentPokemonEscalating {
        attack_name: String,
        damage: u32,
        increment: u32,
    },
    /// Wishiwashi ex's School Storm: like `ExtraDamagePerPokemonWithNameOnBench`, but counts each
    /// Benched Pokémon matching ANY of `pokemon_names` (e.g. "Wishiwashi" and "Wishiwashi ex").
    ExtraDamagePerPokemonWithNamesOnBench {
        pokemon_names: Vec<String>,
        damage_per: u32,
    },
    /// Team Rocket's Magmar's Derisive Roasting: extra damage for each Special Condition currently
    /// affecting the opponent's Active Pokémon.
    ExtraDamagePerOpponentSpecialCondition {
        damage_per: u32,
    },
    /// Teal Mask Ogerpon's Ogre's Whip: damage equal to the attacking Pokémon's remaining HP.
    DamageEqualToSelfRemainingHp,
    /// Swift: routes as ordinary Active damage. Both bypasses ("isn't affected by Weakness or by
    /// any effects on your opponent's Active Pokémon") are applied from the attack's effect text in
    /// `hooks::modify_damage`, exactly like `DamageUnaffectedByWeakness` and
    /// `DamageUnaffectedByOpponentActiveEffects`.
    DamageUnaffectedByWeaknessAndOpponentActiveEffects,
    /// Eldegoss's Float Up / Dunsparce's Bop 'n' Burrow: after damage, the attacker MAY shuffle
    /// itself and everything attached to it back into its owner's deck.
    MayShuffleSelfIntoDeck,
    /// Accelgor's Deck and Cover: inflict `conditions` on the opponent's Active Pokémon, then
    /// shuffle the attacking Pokémon and everything attached to it into your deck (not optional).
    InflictStatusConditionsAndShuffleSelfIntoDeck {
        conditions: Vec<StatusCondition>,
    },
    /// Roserade's Poison Ring: inflict `conditions` on the opponent's Active Pokémon and leave
    /// `effect` on it for `duration` turns (e.g. Poisoned plus "can't retreat next turn").
    InflictStatusConditionsAndCardEffect {
        conditions: Vec<StatusCondition>,
        effect: CardEffect,
        duration: u8,
    },
    /// Tsareena's Kick Down: a random card from the opponent's hand is shuffled into their deck.
    /// The deterministic counterpart of `CoinFlipShuffleRandomOpponentHandCardIntoDeck`.
    ShuffleRandomOpponentHandCardIntoDeck,
    /// Liepard's Snatch and Flee: `ShuffleRandomOpponentHandCardIntoDeck`, and then the attacking
    /// Pokémon is shuffled into its own owner's deck as well.
    ShuffleRandomOpponentHandCardIntoDeckAndSelfIntoDeck,
    /// Mew's Psy Report / Noctowl's Silent Wing: "Your opponent reveals their hand." `State` is
    /// fully observable to both players in this engine, so revealing a hand changes nothing — the
    /// attack reduces to its plain damage.
    RevealOpponentHand,
    /// Purugly's Interrupt: the opponent reveals their hand and the attacker chooses 1 card from it
    /// to shuffle into the opponent's deck.
    ChooseOpponentHandCardToShuffleIntoDeck,
    /// Gyarados's Wild Swing: the attacker may discard any number of their own Benched Pokémon of
    /// `energy_type`, dealing `damage_per` more damage for each Pokémon discarded this way.
    OptionalDiscardBenchedTypeForExtraDamage {
        energy_type: EnergyType,
        damage_per: u32,
    },
    /// Kingambit's Overlord's Blade: `damage_per` more damage for each time one of the attacker's
    /// own Pokémon has been Knocked Out during this game.
    ExtraDamagePerOwnKnockoutThisGame {
        damage_per: u32,
    },
    /// Hisuian Basculegion's Soul Counter: `damage_per` more damage for each point the opponent
    /// scored during their own previous turn.
    ExtraDamagePerOpponentPointLastTurn {
        damage_per: u32,
    },
    /// Toxicroak's Toxic / Toxapex's Severe Poison: Poison the opponent's Active Pokémon, but its
    /// Checkup damage is `poison_damage` instead of the usual 10.
    InflictPoisonWithDamage {
        poison_damage: u32,
    },
    /// Delcatty's Energy Blender: after damage, the attacker may freely redistribute the Energy
    /// attached to their own Pokémon in play. Resolved one Energy at a time through
    /// `SimpleAction::MoveEnergyAndReoffer`; see that variant for why.
    MoveOwnEnergyAnyWay,
    /// Mesprit's Supreme Blast: usable only while every Pokémon named in `required_bench_names` is
    /// on the attacker's Bench; on use, all Energy is discarded from the attacking Pokémon. The
    /// usability half is enforced in `move_generation::attacks`, which consults this variant.
    RequireBenchedNamesThenDiscardAllEnergy {
        required_bench_names: Vec<String>,
    },
}
