use crate::models::EnergyType;

#[derive(Debug, Clone, PartialEq)]
pub enum AbilityMechanic {
    HealAllYourPokemon { amount: u32 },
    HealOneYourPokemonExAndDiscardRandomEnergy { amount: u32 },
    DamageOneOpponentPokemon { amount: u32 },
    SwitchActiveTypedWithBench { energy_type: EnergyType },
    AttachEnergyFromZoneToActiveTypedPokemon { energy_type: EnergyType },
    ReduceDamageFromAttacks { amount: u32 },
    StartTurnRandomPokemonToHand { energy_type: EnergyType },
    PreventFirstAttack,
    ElectromagneticWall,
    InfiltratingInspection,
    DiscardTopCardOpponentDeck,
    CoinFlipToPreventDamage,
}
