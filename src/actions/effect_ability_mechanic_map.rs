// This code is initially generated from the database.json by card_enum_generator.rs.
// but needs to be manually filled in with actual implementations.

use std::collections::HashMap;
use std::sync::LazyLock;

use crate::actions::abilities::AbilityMechanic;
use crate::models::{Card, EnergyType};

/// Map from ability effect text to its AbilityMechanic.
pub static EFFECT_ABILITY_MECHANIC_MAP: LazyLock<HashMap<&'static str, AbilityMechanic>> =
    LazyLock::new(|| {
        let mut map: HashMap<&'static str, AbilityMechanic> = HashMap::new();
        // map.insert("As long as this Pokémon is in the Active Spot, attacks used by your opponent's Active Pokémon cost 1 [C] more.", todo_implementation);
        // map.insert("As long as this Pokémon is in the Active Spot, attacks used by your opponent's Active Pokémon do -20 damage.", todo_implementation);
        // map.insert("As long as this Pokémon is in the Active Spot, it can evolve during your first turn or the turn you play it.", todo_implementation);
        // map.insert("As long as this Pokémon is in the Active Spot, whenever you attach an Energy from your Energy Zone to it, it is now Asleep.", todo_implementation);
        map.insert(
            "As long as this Pokémon is in the Active Spot, whenever your opponent attaches an Energy from their Energy Zone to 1 of their Pokémon, do 20 damage to that Pokémon.",
            AbilityMechanic::ElectromagneticWall,
        );
        // map.insert("As long as this Pokémon is in the Active Spot, your opponent can't use any Supporter cards from their hand.", todo_implementation);
        // map.insert("As long as this Pokémon is on your Bench, attacks used by your Pokémon that evolve from Poliwhirl do +40 damage to your opponent's Active Pokémon.", todo_implementation);
        // map.insert("As long as this Pokémon is on your Bench, prevent all damage done to this Pokémon by attacks.", todo_implementation);
        // map.insert("As long as this Pokémon is on your Bench, your Active Basic Pokémon's Retreat Cost is 1 less.", todo_implementation);
        // map.insert("As often as you like during your turn, you may choose 1 of your Pokémon that has damage on it, and move all of its damage to this Pokémon.", todo_implementation);
        // map.insert("As often as you like during your turn, you may move a [W] Energy from 1 of your Benched [W] Pokémon to your Active [W] Pokémon.", todo_implementation);
        map.insert(
            "At the beginning of your turn, if this Pokémon is in the Active Spot, put a random [P] Pokémon from your deck into your hand.",
            AbilityMechanic::StartTurnRandomPokemonToHand {
                energy_type: EnergyType::Psychic,
            },
        );
        // map.insert("At the end of your first turn, take a [L] Energy from your Energy Zone and attach it to this Pokémon.", todo_implementation);
        // map.insert("At the end of your turn, if this Pokémon is in the Active Spot, draw a card.", todo_implementation);
        // map.insert("At the end of your turn, if this Pokémon is in the Active Spot, heal 20 damage from it.", todo_implementation);
        // map.insert("Attacks used by your [F] Pokémon do +20 damage to your opponent's Active Pokémon.", todo_implementation);
        // map.insert("Attacks used by your [P] Pokémon and [M] Pokémon do +30 damage to your opponent's Active Pokémon.", todo_implementation);
        // map.insert("Basic Pokémon in play (both yours and your opponent's) have no Abilities.", todo_implementation);
        // map.insert("During Pokémon Checkup, if this Pokémon is in the Active Spot, do 10 damage to your opponent's Active Pokémon.", todo_implementation);
        // map.insert("During your first turn, this Pokémon has no Retreat Cost.", todo_implementation);
        // map.insert("Each [G] Energy attached to your [G] Pokémon provides 2 [G] Energy. This effect doesn't stack.", todo_implementation);
        // map.insert("Each of your Pokémon that has any Energy attached recovers from all Special Conditions and can't be affected by any Special Conditions.", todo_implementation);
        // map.insert("Each of your Pokémon that has any [P] Energy attached recovers from all Special Conditions and can't be affected by any Special Conditions.", todo_implementation);
        // map.insert("Each of your [G] Pokémon gets +20 HP.", todo_implementation);
        // map.insert("If a Stadium is in play, this Pokémon has no Retreat Cost.", todo_implementation);
        map.insert(
            "If any damage is done to this Pokémon by attacks, flip a coin. If heads, prevent that damage.",
            AbilityMechanic::CoinFlipToPreventDamage,
        );
        // map.insert("If any damage is done to this Pokémon by attacks, flip a coin. If heads, this Pokémon takes -100 damage from that attack.", todo_implementation);
        // map.insert("If this Pokémon has a Pokémon Tool attached, attacks used by this Pokémon cost 1 less [G] Energy.", todo_implementation);
        // map.insert("If this Pokémon has any Energy attached, it has no Retreat Cost.", todo_implementation);
        // map.insert("If this Pokémon has full HP, it takes -40 damage from attacks from your opponent's Pokémon.", todo_implementation);
        // map.insert("If this Pokémon is in the Active Spot and is Knocked Out by damage from an attack from your opponent's Pokémon, do 10 damage to each of your opponent's Pokémon.", todo_implementation);
        // map.insert("If this Pokémon is in the Active Spot and is Knocked Out by damage from an attack from your opponent's Pokémon, do 50 damage to the Attacking Pokémon.", todo_implementation);
        // map.insert("If this Pokémon is in the Active Spot and is Knocked Out by damage from an attack from your opponent's Pokémon, flip a coin. If heads, the Attacking Pokémon is Knocked Out.", todo_implementation);
        // map.insert("If this Pokémon is in the Active Spot and is Knocked Out by damage from an attack from your opponent's Pokémon, move all [F] Energy from this Pokémon to 1 of your Benched Pokémon.", todo_implementation);
        // map.insert("If this Pokémon is in the Active Spot and is damaged by an attack from your opponent's Pokémon, do 20 damage to the Attacking Pokémon.", todo_implementation);
        // map.insert("If this Pokémon is in the Active Spot and is damaged by an attack from your opponent's Pokémon, take a [W] Energy from your Energy Zone and attach it to 1 of your Benched Pokémon.", todo_implementation);
        // map.insert("If this Pokémon is in the Active Spot and is damaged by an attack from your opponent's Pokémon, the Attacking Pokémon is now Poisoned.", todo_implementation);
        // map.insert("If this Pokémon is in the Active Spot, once during your turn, you may switch in 1 of your opponent's Benched Basic Pokémon to the Active Spot.", todo_implementation);
        // map.insert("If this Pokémon would be Knocked Out by damage from an attack, flip a coin. If heads, this Pokémon is not Knocked Out, and its remaining HP becomes 10.", todo_implementation);
        // map.insert("If you have Arceus or Arceus ex in play, attacks used by this Pokémon cost 1 less [C] Energy.", todo_implementation);
        // map.insert("If you have Arceus or Arceus ex in play, attacks used by this Pokémon do +30 damage to your opponent's Active Pokémon.", todo_implementation);
        // map.insert("If you have Arceus or Arceus ex in play, this Pokémon has no Retreat Cost.", todo_implementation);
        // map.insert("If you have Arceus or Arceus ex in play, this Pokémon takes -30 damage from attacks.", todo_implementation);
        // map.insert("If you have Latias in play, this Pokémon has no Retreat Cost.", todo_implementation);
        // map.insert("If you have another Falinks in play, this Pokémon's attacks do +20 damage to your opponent's Active Pokémon, and this Pokémon takes -20 damage from attacks from your opponent's Pokémon.", todo_implementation);
        // map.insert("If your opponent's Pokémon is Knocked Out by damage from this Pokémon's attacks, during your opponent's next turn, prevent all damage from—and effects of—attacks done to this Pokémon.", todo_implementation);
        // map.insert("Once during your turn, if this Pokémon is in the Active Spot, you may heal 30 damage from 1 of your Pokémon.", todo_implementation);
        // map.insert("Once during your turn, if this Pokémon is in the Active Spot, you may look at a random Supporter card from your opponent's hand. Use the effect of that card as the effect of this Ability.", todo_implementation);
        // map.insert("Once during your turn, if this Pokémon is in the Active Spot, you may make your opponent's Active Pokémon Poisoned.", todo_implementation);
        // map.insert("Once during your turn, if this Pokémon is in the Active Spot, you may switch in 1 of your opponent's Benched Pokémon that has damage on it to the Active Spot.", todo_implementation);
        // map.insert("Once during your turn, if this Pokémon is in the Active Spot, you may take a [G] Energy from your Energy Zone and attach it to 1 of your [G] Pokémon.", todo_implementation);
        // map.insert("Once during your turn, if this Pokémon is on your Bench, you may discard all Pokémon Tools from your opponent's Active Pokémon. If you do, discard this Pokémon.", todo_implementation);
        // map.insert("Once during your turn, if this Pokémon is on your Bench, you may switch it with your Active Pokémon.", todo_implementation);
        // map.insert("Once during your turn, if you have Arceus or Arceus ex in play, you may do 30 damage to your opponent's Active Pokémon.", todo_implementation);
        // map.insert("Once during your turn, when you play this Pokémon from your hand to evolve 1 of your Pokémon, you may discard a random Energy from your opponent's Active Pokémon.", todo_implementation);
        // map.insert("Once during your turn, when you play this Pokémon from your hand to evolve 1 of your Pokémon, you may draw 2 cards.", todo_implementation);
        // map.insert("Once during your turn, when you play this Pokémon from your hand to evolve 1 of your Pokémon, you may have your opponent shuffle their hand into their deck. For each remaining point that your opponent needs to win, they draw a card.", todo_implementation);
        // map.insert("Once during your turn, when you play this Pokémon from your hand to evolve 1 of your Pokémon, you may heal 60 damage from 1 of your [W] Pokémon.", todo_implementation);
        // map.insert("Once during your turn, when you play this Pokémon from your hand to evolve 1 of your Pokémon, you may put 2 random Pokémon Tool cards from your discard pile into your hand.", todo_implementation);
        // map.insert("Once during your turn, when you play this Pokémon from your hand to evolve 1 of your Pokémon, you may put a Supporter card from your discard pile into your hand.", todo_implementation);
        // map.insert("Once during your turn, when you play this Pokémon from your hand to evolve 1 of your Pokémon, you may take a [R] Energy from your Energy Zone and attach it to your Active [R] Pokémon.", todo_implementation);
        map.insert(
            "Once during your turn, when you put this Pokémon from your hand onto your Bench, you may have your opponent reveal their hand.",
            AbilityMechanic::InfiltratingInspection,
        );
        // map.insert("Once during your turn, you may attach a [R] Energy from your discard pile to this Pokémon. If you do, do 20 damage to this Pokémon.", todo_implementation);
        // map.insert("Once during your turn, you may choose either player. Look at the top card of that player's deck.", todo_implementation);
        map.insert(
            "Once during your turn, you may discard the top card of your opponent's deck.",
            AbilityMechanic::DiscardTopCardOpponentDeck,
        );
        map.insert(
            "Once during your turn, you may do 20 damage to 1 of your opponent's Pokémon.",
            AbilityMechanic::DamageOneOpponentPokemon { amount: 20 },
        );
        // map.insert("Once during your turn, you may flip a coin. If heads, switch in 1 of your opponent's Benched Pokémon to the Active Spot.", todo_implementation);
        // map.insert("Once during your turn, you may flip a coin. If heads, your opponent's Active Pokémon is now Asleep.", todo_implementation);
        // map.insert("Once during your turn, you may flip a coin. If heads, your opponent's Active Pokémon is now Poisoned.", todo_implementation);
        map.insert(
            "Once during your turn, you may heal 10 damage from each of your Pokémon.",
            AbilityMechanic::HealAllYourPokemon { amount: 10 },
        );
        map.insert(
            "Once during your turn, you may heal 20 damage from each of your Pokémon.",
            AbilityMechanic::HealAllYourPokemon { amount: 20 },
        );
        // map.insert("Once during your turn, you may heal 20 damage from your Active Pokémon.", todo_implementation);
        // map.insert("Once during your turn, you may heal 30 damage from each of your [W] Pokémon.", todo_implementation);
        // map.insert("Once during your turn, you may look at the top card of your deck.", todo_implementation);
        // map.insert("Once during your turn, you may make your opponent's Active Pokémon Burned.", todo_implementation);
        // map.insert("Once during your turn, you may move all [D] Energy from each of your Pokémon to this Pokémon.", todo_implementation);
        // map.insert("Once during your turn, you may move all [P] Energy from 1 of your Benched [P] Pokémon to your Active Pokémon.", todo_implementation);
        // map.insert("Once during your turn, you may put a random Pokémon Tool card from your deck into your hand.", todo_implementation);
        // map.insert("Once during your turn, you may put a random Pokémon from your deck into your hand.", todo_implementation);
        // map.insert("Once during your turn, you may switch out your opponent's Active Basic Pokémon to the Bench. (Your opponent chooses the new Active Pokémon.)", todo_implementation);
        // map.insert("Once during your turn, you may switch out your opponent's Active Pokémon to the Bench. (Your opponent chooses the new Active Pokémon.)", todo_implementation);
        // map.insert("Once during your turn, you may switch your Active Ultra Beast with 1 of your Benched Ultra Beasts.", todo_implementation);
        map.insert("Once during your turn, you may switch your Active [W] Pokémon with 1 of your Benched Pokémon.", AbilityMechanic::SwitchActiveTypedWithBench { energy_type: EnergyType::Water });
        // map.insert("Once during your turn, you may take 2 [D] Energy from your Energy Zone and attach it to this Pokémon. If you do, do 30 damage to this Pokémon.", todo_implementation);
        // map.insert("Once during your turn, you may take a [L] Energy from your Energy Zone and attach it to this Pokémon.", todo_implementation);
        // map.insert("Once during your turn, you may take a [P] Energy from your Energy Zone and attach it to the [P] Pokémon in the Active Spot.", todo_implementation);
        // map.insert("Once during your turn, you may take a [P] Energy from your Energy Zone and attach it to this Pokémon. If you use this Ability, your turn ends.", todo_implementation);
        // map.insert("Pokémon (both yours and your opponent's) can't be healed.", todo_implementation);
        // map.insert("Prevent all damage done to this Pokémon by attacks from your opponent's Pokémon ex.", todo_implementation);
        // map.insert("Prevent all effects of attacks used by your opponent's Pokémon done to this Pokémon.", todo_implementation);
        // map.insert("This Ability works if you have any Unown in play with an Ability other than GUARD. All of your Pokémon take -10 damage from attacks from your opponent's Pokémon.", todo_implementation);
        // map.insert("This Ability works if you have any Unown in play with an Ability other than POWER. Attacks used by your Pokémon do +10 damage to your opponent's Active Pokémon.", todo_implementation);
        // map.insert("This Pokémon can evolve into any Pokémon that evolves from Eevee if you play it from your hand onto this Pokémon. (This Pokémon can't evolve during your first turn or the turn you play it.)", todo_implementation);
        // map.insert("This Pokémon can't be Asleep.", todo_implementation);
        // map.insert("This Pokémon can't be affected by any Special Conditions.", todo_implementation);
        // map.insert("This Pokémon gets +30 HP for each [P] Energy attached to it.", todo_implementation);
        // map.insert("This Pokémon takes -10 damage from attacks.", todo_implementation);
        // map.insert("This Pokémon takes -20 damage from attacks from [R] or [W] Pokémon.", todo_implementation);
        map.insert(
            "This Pokémon takes -20 damage from attacks.",
            AbilityMechanic::ReduceDamageFromAttacks { amount: 20 },
        );
        // map.insert("This Pokémon takes -30 damage from attacks from [F] Pokémon.", todo_implementation);
        // map.insert("This Pokémon takes -30 damage from attacks from [R] or [W] Pokémon.", todo_implementation);
        // map.insert("When this Pokémon is Knocked Out, flip a coin. If heads, your opponent can't get any points for it.", todo_implementation);
        map.insert(
            "When this Pokémon is first damaged by an attack after coming into play, prevent that damage.",
            AbilityMechanic::PreventFirstAttack,
        );
        // map.insert("Whenever you attach a [D] Energy from your Energy Zone to this Pokémon, do 20 damage to your opponent's Active Pokémon.", todo_implementation);
        // map.insert("Whenever you attach a [P] Energy from your Energy Zone to this Pokémon, heal 20 damage from this Pokémon.", todo_implementation);
        // map.insert("Whenever you attach an Energy from your Energy Zone to this Pokémon, put a random card from your deck that evolves from this Pokémon onto this Pokémon to evolve it.", todo_implementation);
        // map.insert("You must discard a card from your hand in order to use this Ability. Once during your turn, you may draw a card.", todo_implementation);
        // map.insert("Your Active Dondozo has no Retreat Cost.", todo_implementation);
        // map.insert("Your Active Pokémon has no Retreat Cost.", todo_implementation);
        // map.insert("Your opponent can't play any Pokémon from their hand to evolve their Active Pokémon.", todo_implementation);
        // map.insert("Your opponent's Active Pokémon takes +10 damage from being Poisoned.", todo_implementation);
        // map.insert("Your opponent's Active Pokémon's Retreat Cost is 1 more.", todo_implementation);

        // B2 and B2a mechanics
        // map.insert("Basic Pokémon in play (both yours and your opponent's) have no Abilities.", todo_implementation);
        // map.insert("If this Pokémon's remaining HP is 50 or less, attacks used by this Pokémon do +60 damage to your opponent's Active Pokémon.", todo_implementation);
        map.insert(
            "Once during your turn, you may discard 1 [R] Energy from this Pokémon in order to use this Ability. During this turn, attacks used by your [R] Pokémon do +50 damage to your opponent's Active Pokémon.",
            AbilityMechanic::DiscardEnergyToIncreaseTypeDamage {
                discard_energy: EnergyType::Fire,
                attack_type: EnergyType::Fire,
                amount: 50,
            },
        );
        // map.insert("Once during your turn, you may heal 60 damage from 1 of your Pokémon ex that has any Energy attached. If you do, discard a random Energy from that Pokémon.", todo_implementation);
        // map.insert("Once during your turn, you may switch out your opponent's Active Basic Pokémon to the Bench. (Your opponent chooses the new Active Pokémon.)", todo_implementation);
        // map.insert("Once during your turn, you may switch out your opponent's Active Pokémon to the Bench. (Your opponent chooses the new Active Pokémon.)", todo_implementation);
        // map.insert("Once during your turn, you may take a [W] Energy from your Energy Zone and attach it to the [W] Pokémon in the Active Spot.", todo_implementation);
        map.insert(
            "This Pokémon takes -30 damage from attacks.",
            AbilityMechanic::ReduceDamageFromAttacks { amount: 30 },
        );
        map
    });

pub fn ability_mechanic_from_effect(effect: &str) -> Option<&'static AbilityMechanic> {
    EFFECT_ABILITY_MECHANIC_MAP.get(effect)
}

pub fn get_ability_mechanic(card: &Card) -> Option<&'static AbilityMechanic> {
    let Card::Pokemon(pokemon) = card else {
        return None;
    };

    if let Some(ability) = &pokemon.ability {
        let mechanic = ability_mechanic_from_effect(&ability.effect);
        if let Some(mechanic) = mechanic {
            Some(mechanic)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn has_ability_mechanic(card: &Card, mechanic: &AbilityMechanic) -> bool {
    get_ability_mechanic(card) == Some(mechanic)
}
