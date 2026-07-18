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
    CannotAttack,
    CannotUseAttack(String),
    IncreasedDamageForAttack {
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
    ForceFirstHeads,
    BonusPointForHaxorusActiveKO,
    ReducedAttackCostForSpecificPokemon {
        amount: u8,
        pokemon_names: Vec<String>,
    },
}
