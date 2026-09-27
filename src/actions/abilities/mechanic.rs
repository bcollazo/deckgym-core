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
    /// Eiscue's Ice Face: "If this Pokémon has full HP, it takes `amount` less damage from attacks
    /// from your opponent's Pokémon." Depends on the current damage counters, so it's resolved in
    /// `hooks::modify_damage` rather than being a plain `CardEffect`.
    ReduceDamageFromAttacksIfFullHp {
        amount: u32,
    },
    /// Falinks's Coordinated Unit: "If you have another <same name> in play, this Pokémon's
    /// attacks do +`damage_bonus` damage to your opponent's Active Pokémon, and this Pokémon
    /// takes -`damage_reduction` damage from attacks from your opponent's Pokémon."
    /// Board-dependent on both sides, so it's resolved in `hooks::modify_damage`.
    BuffIfAnotherSameNameInPlay {
        damage_bonus: u32,
        damage_reduction: u32,
    },
    /// Unown GUARD: "This Ability works if you have any Unown in play with an Ability other than
    /// GUARD. All of your Pokémon take -`amount` damage from attacks from your opponent's
    /// Pokémon." An aura over the owner's whole board; resolved in `hooks::modify_damage`.
    ReduceDamageToAllYourPokemonWithOtherUnown {
        amount: u32,
    },
    /// Unown POWER: "This Ability works if you have any Unown in play with an Ability other than
    /// POWER. Attacks used by your Pokémon do +`amount` damage to your opponent's Active
    /// Pokémon." Resolved in `hooks::modify_damage`.
    IncreaseDamageOfYourPokemonWithOtherUnown {
        amount: u32,
    },
    /// Politoed's Lordly Cheering: "As long as this Pokémon is on your Bench, attacks used by your
    /// Pokémon that evolve from `evolves_from` do +`amount` damage to your opponent's Active
    /// Pokémon." Resolved in `hooks::modify_damage`.
    IncreaseDamageForEvolvesFromWhileBenched {
        evolves_from: String,
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
    /// Team Rocket's Slowking ex's Evil Inspiration: "Once during your turn, if this Pokémon is
    /// in the Active Spot, you may draw a card." Unlike `EndTurnDrawCardIfActive`, this is an
    /// actively-used ability (offered any time during the turn, not just at its end).
    DrawCardIfActive {
        amount: u32,
    },
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
    /// "Once during your turn, you may switch out your opponent's Active Pokémon to the Bench."
    /// `require_active` demands that the ability's holder itself be in the Active Spot;
    /// `require_target_basic` restricts the ability to an opponent's Active *Basic* Pokémon
    /// (Swellow's Repelling Wind).
    SwitchOutOpponentActiveToBench {
        require_active: bool,
        require_target_basic: bool,
    },
    /// Abilities whose entire effect is looking at hidden cards — Porygon's Data Scan, Unown's
    /// CHECK, Team Rocket's Kecleon's Spy Ops. deckgym does not model per-player hidden
    /// information, so using them changes nothing observable; they are implemented as no-ops and
    /// never offered by move generation (offering them would only pad the search tree).
    LookAtCardsNoop,
    /// Grafaiai's Poison Coating: "Once during your turn, you may flip a coin. If heads, your
    /// opponent's Active Pokémon is now Poisoned."
    CoinFlipPoisonOpponentActive,
    /// Rillaboom's Captivating Rhythm: "Once during your turn, you may flip a coin. If heads,
    /// switch in 1 of your opponent's Benched Pokémon to the Active Spot." (You choose which.)
    CoinFlipSwitchOpponentBenchToActive,
    /// Tyranitar's Energy Plunder: "Once during your turn, you may move all [energy_type] Energy
    /// from each of your Pokémon to this Pokémon."
    MoveAllTypedEnergyFromAllYourPokemonToSelf {
        energy_type: EnergyType,
    },
    /// Ambipom's Catching Tail: "Once during your turn, you may put a random Pokémon Tool card
    /// from your deck into your hand."
    SearchRandomToolFromDeck,
    BadDreamsEndOfTurn {
        amount: u32,
    },
    EndTurnDrawCardIfActive {
        amount: u32,
    },
    EndTurnHealSelfIfActive {
        amount: u32,
    },
    /// Garganacl's Blessed Salt: "During Pokémon Checkup, heal `amount` damage from each of your
    /// Pokémon." Passive and unconditional — applies every Pokémon Checkup regardless of whether
    /// this Pokémon is Active or Benched, as long as it's in play.
    HealAllYourPokemonDuringCheckup {
        amount: u32,
    },
    CoinFlipSleepOpponentActive,
    DiscardFromHandToDrawCard,
    ImmuneToStatusConditions,
    /// Hoothoot's Insomnia: "This Pokémon can't be Asleep." Enforced in
    /// `State::apply_status_condition`.
    ImmuneToStatusCondition {
        condition: StatusCondition,
    },
    /// Cherubi's En-fruits-iastic: "If this Pokémon has a Pokémon Tool attached, attacks used by
    /// this Pokémon cost `amount` less [`energy_type`] Energy." Resolved in
    /// `hooks::get_attack_cost`.
    ReduceTypedAttackCostIfHasTool {
        energy_type: EnergyType,
        amount: u8,
    },
    /// Lilligant's Toughness Aroma: "Each of your [`energy_type`] Pokémon gets +`amount` HP."
    /// Board-dependent, so it is cached on each `PlayedCard` (like the Stadium HP bonus) and
    /// refreshed whenever the owner's board changes.
    IncreaseHpOfYourTypedPokemon {
        energy_type: EnergyType,
        amount: u32,
    },
    /// Claydol's Heal Block: "Pokémon (both yours and your opponent's) can't be healed." Cached on
    /// each `PlayedCard` and consulted by `PlayedCard::heal`.
    NoHealingForAnyone,
    /// Regigigas's Seal of Antiquity: "If you don't have <names> on your Bench, this Pokémon can't
    /// attack." Enforced in attack move generation.
    CannotAttackUnlessNamedOnBench {
        names: Vec<String>,
    },
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
    /// Urshifu's Double Type: "As long as this Pokémon is in play, it is [X] and [Y] type."
    ///
    /// Passive. The granted types are unioned with the card's printed type by
    /// `PlayedCard::get_energy_types` / `PlayedCard::is_type`, which is what every in-play "is
    /// this a [X] Pokémon" rule consults — so Weakness, typed auras, typed Energy attachment,
    /// typed Tools and so on all see both types at once. Because the lookup goes through the
    /// board-aware `PlayedCard::ability_mechanic`, suppressing the Ability reverts the Pokémon
    /// to its printed type.
    GrantedTypes {
        energy_types: Vec<EnergyType>,
    },
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
    /// "If you have <names> in play, this Pokémon has no Retreat Cost." (Heatran/Rotom's Speed
    /// Link with Arceus, Latios's Fantastical Floating with Latias.) Passive; resolved in
    /// `hooks::get_retreat_cost`.
    NoRetreatCostIfNamedPokemonInPlay {
        names: Vec<String>,
    },
    /// "Your Active Pokémon has no Retreat Cost." (Jumpluff's Fluffy Flight; Tatsugiri's Retreat
    /// Directive restricts it to an Active Dondozo via `pokemon_name`.) An aura: the holder can be
    /// Active or Benched. Passive; resolved in `hooks::get_retreat_cost`.
    NoRetreatCostForYourActive {
        pokemon_name: Option<String>,
    },
    /// Wimpod's Wimp Out: "During your first turn, this Pokémon has no Retreat Cost."
    /// Passive; resolved in `hooks::get_retreat_cost`.
    NoRetreatCostDuringFirstTurn,
    /// Alolan Raichu's Surge Surfer: "If a Stadium is in play, this Pokémon has no Retreat Cost."
    /// Passive; resolved in `hooks::get_retreat_cost`.
    NoRetreatCostIfStadiumInPlay,
    /// Beldum's Conductive Body: "If you have another <same name> in play, this Pokémon's Retreat
    /// Cost is `amount` less." Passive; resolved in `hooks::get_retreat_cost`.
    ReduceRetreatCostIfAnotherSameNameInPlay {
        amount: u8,
    },
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
    /// Team Rocket's Weezing ex's Boiler Smog: "Once during your turn, when you play this
    /// Pokémon from your hand to evolve 1 of your Pokémon, you may make your opponent's Active
    /// Pokémon Poisoned and Burned."
    PoisonAndBurnOpponentActiveOnEvolve,
    /// Team Rocket's Raticate ex's Thieving Incisors: "Once during your turn, when you play this
    /// Pokémon from your hand to evolve 1 of your Pokémon, you may move a random Energy from your
    /// opponent's Active Pokémon to this Pokémon." Mirrors the "move the last-attached Energy
    /// instead of a random one" simplification used elsewhere (e.g.
    /// `DiscardRandomEnergyFromOpponentActiveOnEvolve`).
    MoveRandomEnergyFromOpponentActiveToSelfOnEvolve,
    /// Galarian Perrserker's Dig Up: "Once during your turn, when you play this Pokémon from your
    /// hand to evolve 1 of your Pokémon, you may put `amount` random Pokémon Tool cards from your
    /// discard pile into your hand."
    PutRandomToolsFromDiscardToHandOnEvolve {
        amount: usize,
    },
    /// Raticate's Treasure Collecting: "…you may look at the top `amount` cards of your deck and
    /// put all Item cards you find there into your hand. Shuffle the other cards back into your
    /// deck."
    TakeItemsFromTopOfDeckOnEvolve {
        amount: usize,
    },
    /// Delcatty's Search for Friends: "…you may put a Supporter card from your discard pile into
    /// your hand." The player chooses which Supporter.
    PutSupporterFromDiscardToHandOnEvolve,
    /// Polteageist's Refreshing Tea: "…you may have your opponent shuffle their hand into their
    /// deck. For each remaining point that your opponent needs to win, they draw a card."
    OpponentShuffleHandAndDrawPerRemainingPointOnEvolve,
    /// Samurott's Stance: "…you may prevent all damage from—and effects of—attacks from your
    /// opponent's Pokémon done to this Pokémon until the end of your opponent's next turn."
    PreventAllDamageAndEffectsOnEvolve,
    /// Poltchageist's Hospitality: "Once during your turn, when you put this Pokémon from your
    /// hand onto your Bench, you may heal `amount` damage from your Active [`energy_type`]
    /// Pokémon."
    HealActiveTypedOnBench {
        energy_type: EnergyType,
        amount: u32,
    },
    CanEvolveIntoEeveeEvolution,
    CanEvolveOnFirstTurnIfActive,
    CounterattackDamage {
        amount: u32,
    },
    /// Pyukumuku's Innards Out / Team Rocket's Electrode's Destiny Burst: "If this Pokémon is in
    /// the Active Spot and is Knocked Out by damage from an attack from your opponent's Pokémon,
    /// do `amount` damage to the Attacking Pokémon." Passive; resolved alongside the ordinary
    /// counterattack recoil in `handle_damage_only`.
    DamageAttackerOnKnockout {
        amount: u32,
    },
    /// Spiritomb's Final Scream: same trigger as `DamageAttackerOnKnockout`, but the damage is
    /// dealt to *each* of the attacking player's in-play Pokémon (Active and Benched).
    DamageEachOpponentPokemonOnKnockout {
        amount: u32,
    },
    /// Galarian Cursola's Perish Body: "If this Pokémon is in the Active Spot and is Knocked Out
    /// by damage from an attack from your opponent's Pokémon, flip a coin. If heads, the
    /// Attacking Pokémon is Knocked Out." Passive; resolved as a defender-side coin split on the
    /// attack's outcomes (`AttackOutcomes::split_with_knockout_coin_flip`).
    CoinFlipKnockOutAttackerOnKnockout,
    /// Dusknoir's Fade into Darkness / Glimmora's Shattering Crystal: "When this Pokémon is
    /// Knocked Out, flip a coin. If heads, your opponent can't get any points for it." Passive;
    /// resolved as a defender-side coin split on the attack's outcomes. Only Knock Outs caused by
    /// an opponent's attack flip — see the note on `split_with_knockout_coin_flip`.
    CoinFlipDenyPointsOnKnockout,
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
    /// Smeargle's Portrait: "Once during your turn, if this Pokémon is in the Active Spot, you may
    /// look at a random Supporter card from your opponent's hand. Use the effect of that card as
    /// the effect of this Ability."
    ///
    /// The opponent keeps the card (it is only copied), so nothing is removed from their hand.
    /// Only Supporters that deckgym implements *and* that are currently playable are eligible —
    /// copying an unplayable effect has no defined behavior in the engine.
    UseRandomOpponentSupporterEffect,
    /// Gholdengo's Luxury Coin: "Once during your turn, when you flip any coins for an effect of
    /// your Trainer cards, you may ignore all results of those coin flips and begin flipping those
    /// coins again. You can't use more than 1 Luxury Coin Ability each turn."
    ///
    /// The Trainer-card counterpart of `VictoryStarReflip`, resolved through the same
    /// `PendingCoinReflip` machinery.
    LuxuryCoinReflip,
}
