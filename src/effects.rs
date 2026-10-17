use serde::{Deserialize, Serialize};

use crate::models::EnergyType;

/// I believe these are the "clearable" ones by retreating...
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardEffect {
    NoRetreat,
    ReducedDamage {
        amount: u32,
    },
    IncreasedVulnerability {
        amount: u32,
    },
    IncreasedAttackCost {
        amount: u8,
    },
    /// This Pokémon's Retreat Cost is `amount` more (e.g. Team Rocket's Goo-zooka).
    IncreasedRetreatCost {
        amount: u8,
    },
    CannotAttack,
    CannotUseAttack(String),
    IncreasedDamageForAttack {
        attack_name: String,
        amount: u32,
    },
    /// Like `IncreasedDamageForAttack`, but for a *spread* attack, whose bonus has to reach every
    /// target rather than only the Active-to-Active damage that `hooks::modify_damage` sees (e.g.
    /// Archeops's Wild Spin: "+20 damage to each of your opponent's Pokémon"). The attack's own
    /// mechanic reads this off the attacker and folds it into every target's damage, so
    /// `modify_damage` deliberately ignores it — using `IncreasedDamageForAttack` here would
    /// double-count the bonus on the Active.
    IncreasedSpreadDamageForAttack {
        attack_name: String,
        amount: u32,
    },
    PreventAllDamageAndEffects,
    /// Prevent all damage from attacks if the incoming damage is at most `threshold` (e.g. Cascoon's Harden).
    PreventDamageIfLessOrEqual {
        threshold: u32,
    },
    /// Prevent all damage done by attacks from Basic Pokémon (e.g. Carracosta's Blocking Shell).
    PreventDamageFromBasic,
    NoWeakness,
    /// Budew's Prickly Powder: "The Defending Pokémon loses all Abilities." While this effect is on
    /// a Pokémon, `PlayedCard::ability` / `PlayedCard::ability_mechanic` report it as having none,
    /// which is what every ability lookup in the engine goes through — so the Pokémon stops
    /// offering activated abilities, stops contributing passive ones, and stops deriving
    /// ability-based `CardEffect`s. Applied with a very long duration; the wording is "until the
    /// Defending Pokémon leaves the Active Spot", and leaving the Active Spot clears effects.
    AbilitiesDisabled,
    CoinFlipToBlockAttack,
    DelayedDamage {
        amount: u32,
    },
    /// If this Pokémon is damaged by an attack while in the Active Spot, deal `amount`
    /// damage to the Attacking Pokémon (e.g. Alolan Sandslash's Spike Armor). Temporary
    /// counterpart to RockyHelmet's always-on recoil.
    Counterattack {
        amount: u32,
    },
    /// Aegislash's Superb Shield: this Pokémon takes `amount` less damage, but only from attacks
    /// used by the opponent's Pokémon ex. The ex-only twin of `ReducedDamage`.
    ReducedDamageFromEx {
        amount: u32,
    },
    /// Gothitelle's Stellar Cradle: the next time this Pokémon's controller attaches Energy to it
    /// from their Energy Zone, it falls Asleep.
    AsleepWhenEnergyAttachedFromZone,
    /// Galarian Stunfisk's Snapping Trap: while the Pokémon carrying this effect is in the Active
    /// Spot, an opponent retreating their Active Pokémon takes `amount` damage on the Pokémon
    /// they promote in its place.
    DamageNewActiveOnOpponentRetreat {
        amount: u32,
    },
    // ---------------------------------------------------------------------------------------------
    // Ability-derived effects. These are not added via `add_effect`; they are *derived* on the fly
    // from a Pokémon's passive ability by `PlayedCard::get_effective_card_effects` (see
    // `card_effect_from_ability_mechanic`). Modeling passive abilities as `CardEffect`s lets damage
    // code check a single "effects on this Pokémon" list instead of scanning the board for
    // abilities, and lets attacks like Sawk's Brick Break ignore *all* effects on the opponent's
    // Active Pokémon uniformly. They have no turn duration — they are present exactly while the
    // ability-holder is in play.
    // ---------------------------------------------------------------------------------------------
    /// This Pokémon takes `amount` less damage from attacks (e.g. Cloyster's Shell Armor).
    /// Applies whether the holder is Active or Benched, mirroring `ReduceDamageFromAttacks`.
    ReduceDamageFromAttacks {
        amount: u32,
    },
    /// While this Pokémon is the Active Spot defender, attacks against it do `amount` less damage
    /// (e.g. Arbok's Intimidating Fang). Mirrors `ReduceOpponentActiveDamage`.
    ReduceOpponentActiveDamage {
        amount: u32,
    },
    /// Prevent all damage done to this Pokémon by attacks from the opponent's Pokémon ex
    /// (e.g. Oricorio's Safeguard). Mirrors `PreventAllDamageFromEx`.
    PreventAllDamageFromEx,
    /// Prevent all damage done to this Pokémon by attacks while it is on the Bench
    /// (e.g. Wartortle's Shell Shield). Mirrors `PreventDamageWhileBenched`.
    PreventDamageWhileBenched,
    /// If any damage is done to this Pokémon by attacks, flip a coin; on heads prevent that damage
    /// (e.g. Meowth's Carefree Steps). Mirrors `CoinFlipToPreventDamage`. Distinct from
    /// `CoinFlipToBlockAttack`, which is an attacker self-debuff on the holder's own attacks.
    CoinFlipToPreventIncomingDamage,
    /// If any damage is done to this Pokémon by attacks, flip a coin; on heads this Pokémon takes
    /// `amount` less damage from that attack (e.g. Bastiodon's Guarded Grill, Hisuian Goodra's
    /// Securely Sheltered). Mirrors `CoinFlipToReduceDamage`.
    CoinFlipToReduceIncomingDamage {
        amount: u32,
    },
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum TurnEffect {
    NoSupportCards,
    NoItemCards,
    NoTrainerCards,
    NoEnergyFromZoneToActive,
    ReducedRetreatCost {
        amount: u8,
    },
    ReducedDamageForType {
        amount: u32,
        energy_type: EnergyType,
        player: usize,
    },
    /// `player`'s Pokémon named in `pokemon_names` take `amount` less damage from the opponent's
    /// attacks (e.g. Jasmine, protecting Steelix and Skarmory ex). Like `ReducedDamageForType`,
    /// this covers Benched Pokémon too, since the wording is "all of your ...".
    ///
    /// `attacker_must_be_ex` narrows the reduction to attacks from the opponent's Pokémon ex
    /// (e.g. Cheren, protecting Watchog and Stoutland).
    ReducedDamageForSpecificPokemon {
        amount: u32,
        pokemon_names: Vec<String>,
        player: usize,
        attacker_must_be_ex: bool,
    },
    /// All of `player`'s Pokémon take `amount` less damage from attacks from the opponent's
    /// Pokémon (e.g. Blue). The untargeted sibling of `ReducedDamageForType` /
    /// `ReducedDamageForSpecificPokemon`; like those it covers Benched Pokémon too, since the
    /// wording is "all of your Pokémon".
    ReducedDamageForAllPokemon {
        amount: u32,
        player: usize,
    },
    /// If one of `player`'s Pokémon named in `pokemon_names` would be Knocked Out by damage from
    /// an attack, it is not Knocked Out and its remaining HP becomes `remaining_hp` (e.g. Hala,
    /// protecting Hariyama and Crabominable). The deterministic, turn-scoped counterpart of
    /// Ursaluna's coin-flip Guts ability; resolved in `handle_knockouts`.
    SurviveKnockOutForSpecificPokemon {
        pokemon_names: Vec<String>,
        player: usize,
        remaining_hp: u32,
    },
    IncreasedDamage {
        amount: u32,
    },
    IncreasedDamageForType {
        amount: u32,
        energy_type: EnergyType,
    },
    IncreasedDamageAgainstEx {
        amount: u32,
    },
    IncreasedDamageForEeveeEvolutions {
        amount: u32,
    },
    IncreasedDamageForSpecificPokemon {
        amount: u32,
        pokemon_names: Vec<String>,
    },
    IncreasedDamageForSpecificPokemonAgainstEx {
        amount: u32,
        pokemon_names: Vec<String>,
    },
    IncreasedDamageForTypeAgainstEx {
        amount: u32,
        energy_type: EnergyType,
    },
    DelayedSpotDamage {
        source_player: usize,
        target_player: usize,
        target_in_play_idx: usize,
        amount: u32,
    },
    /// Armaldo's Abyssal Drop: whatever occupies `target_in_play_idx` on `target_player`'s board
    /// at the end of their turn is Knocked Out outright. The knock-out twin of
    /// `DelayedSpotDamage` — it is not damage, so it ignores damage reduction and prevention.
    DelayedSpotKnockOut {
        source_player: usize,
        target_player: usize,
        target_in_play_idx: usize,
    },
    /// Malamar's Evolution Jammer: `player` can't play Pokémon from their hand to evolve their
    /// Pokémon.
    NoEvolutionFromHand {
        player: usize,
    },
    ForceFirstHeads,
    /// A random-spread attack with this name chooses a Pokémon `amount` more times
    /// (e.g. Drayden boosting Draco Meteor).
    ExtraRandomSpreadHits {
        amount: usize,
        attack_name: String,
    },
    BonusPointForHaxorusActiveKO,
    ReducedAttackCostForSpecificPokemon {
        amount: u8,
        pokemon_names: Vec<String>,
    },
}
