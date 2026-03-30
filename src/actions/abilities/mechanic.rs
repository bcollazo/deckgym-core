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
    PoisonOpponentActive,
    HealActiveYourPokemon {
        amount: u32,
    },
    SwitchOutOpponentActiveToBench,
    BadDreamsEndOfTurn {
        amount: u32,
    },
    CoinFlipSleepOpponentActive,
    DiscardFromHandToDrawCard,
    ImmuneToStatusConditions,
    /// Teal Mask Ogerpon ex – Soothing Wind (passive):
    /// Each of your Pokémon that has any Energy attached recovers from all Special Conditions
    /// and can't be affected by any Special Conditions.
    SoothingWind,
}
