//! Official **Detailed battle FAQ**, "How do I calculate damage for Pokémon that have Weakness or
//! effects applied?" (article 57286049312665, `edited_at` 2026-05-18):
//!
//! > Follow these steps to calculate damage:
//! >
//! > 1. Start with the attack's damage.
//! > 2. Apply any effects that affect the Attacking Pokémon.
//! > 3. Apply Weakness.
//! > 4. Apply any effects that affect the Defending Pokémon.
//! >
//! > For example, when applying Weakness for a 50-damage attack against an opponent's Active
//! > Pokémon that has an effect stating "This Pokémon takes –20 damage from attacks" with the
//! > Stadium card Bounded Field in play, the opponent's Active Pokémon takes 80 damage.
//! > 50 × 2 – 20 = 80
//!
//! So Weakness sits **between** the two sides: attacker-side boosts (Giovanni, Red, the
//! attacker's own Abilities and Tools, Stadium attack bonuses) are already in the number that
//! Weakness multiplies, while every defender-side effect (Blue, "takes −N damage" effects,
//! damage-reducing Tools like Heavy Helmet and Steel Apron) is subtracted afterwards. Damage is
//! floored at 0 only at the very end, never before the Weakness step.

use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board, play_trainer, trainer_from_id},
    Game, State,
};

/// Board setup shared by the FAQ-example tests: Hitmontop's Spinning Attack ([F][F], 50 damage,
/// no effect text) against a Snorlax that is weak to [F] and wears Heavy Helmet ("If the Pokémon
/// this card is attached to has a Retreat Cost of 3 or more, it takes -20 damage from attacks
/// from your opponent's Pokémon" — Snorlax's Retreat Cost is 4). That is exactly the FAQ's
/// "50-damage attack" against "an effect stating 'This Pokémon takes –20 damage from attacks'".
fn hitmontop_vs_snorlax_with_heavy_helmet() -> Game<'static> {
    get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A2085Hitmontop)
            .with_energy(vec![EnergyType::Fighting, EnergyType::Fighting])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)
            .with_tool(get_card_by_enum(CardId::B1219HeavyHelmet))],
    )
}

fn spinning_attack() -> Action {
    Action {
        actor: 0,
        action: attack_action(CardId::A2085Hitmontop, 0),
        is_stack: false,
    }
}

fn put_bounded_field_in_play(game: &mut Game<'static>) {
    let mut state = game.get_state_clone();
    state.set_active_stadium(get_card_by_enum(CardId::B3155BoundedField));
    game.set_state(state);
}

/// Damage dealt to the opponent's Active by `attack`, read off the public API as the drop in its
/// remaining HP. Reading a delta rather than subtracting from the printed HP keeps the number
/// honest when a Tool or Stadium alters the defender's total HP.
fn damage_to_opponent_active(game: &mut Game<'static>, attack: &Action) -> u32 {
    let before = game.get_state_clone().get_remaining_hp(1, 0);
    game.apply_action(attack);
    let after = game.get_state_clone().get_remaining_hp(1, 0);
    before - after
}

/// The FAQ's own worked example: `50 × 2 – 20 = 80`.
///
/// Under the old "apply every modifier, then Weakness" order this was `(50 − 20) × 2 = 60`.
#[test]
fn test_faq_example_bounded_field_weakness_applies_before_the_defenders_reduction() {
    let mut game = hitmontop_vs_snorlax_with_heavy_helmet();
    put_bounded_field_in_play(&mut game);

    assert_eq!(
        damage_to_opponent_active(&mut game, &spinning_attack()),
        80,
        "Detailed battle FAQ: \"when applying Weakness for a 50-damage attack against an \
         opponent's Active Pokémon that has an effect stating 'This Pokémon takes -20 damage \
         from attacks' with the Stadium card Bounded Field in play, the opponent's Active \
         Pokémon takes 80 damage. 50 × 2 - 20 = 80\""
    );
}

/// Sanity anchor: with the ordinary flat +20 Weakness (no Bounded Field) addition and
/// subtraction commute, so the FAQ's order and the old one agree on `50 + 20 − 20 = 50`. This
/// pins the common case down so the reorder can only move the numbers it is meant to move.
#[test]
fn test_flat_weakness_with_a_defender_reduction_is_unchanged_by_the_order() {
    let mut game = hitmontop_vs_snorlax_with_heavy_helmet();

    assert_eq!(
        damage_to_opponent_active(&mut game, &spinning_attack()),
        50,
        "50 base + 20 Weakness - 20 Heavy Helmet = 50, which is the same number either order \
         produces"
    );
}

/// Flooring happens **once, at the end**. Sizzlipede's Gnaw does 10 to a Registeel that is weak
/// to [R] and carries −30 of defender-side reductions (Heavy Helmet −20 for its Retreat Cost of
/// 3, plus Steel Apron −10 for being a [M] Pokémon): `10 + 20 − 30 = 0`.
///
/// The old order clamped before adding Weakness — `max(0, 10 − 30) + 20 = 20` — which invented
/// 20 damage out of a reduction that should have swallowed the attack whole.
#[test]
fn test_defender_reduction_is_not_floored_at_zero_before_weakness_is_added() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1051Sizzlipede).with_energy(vec![EnergyType::Fire])],
        vec![PlayedCard::from_id(CardId::B3116Registeel)
            .with_tool(get_card_by_enum(CardId::B1219HeavyHelmet))
            .with_tool(get_card_by_enum(CardId::A4153SteelApron))],
    );
    let gnaw = Action {
        actor: 0,
        action: attack_action(CardId::A1051Sizzlipede, 0),
        is_stack: false,
    };

    assert_eq!(
        damage_to_opponent_active(&mut game, &gnaw),
        0,
        "Detailed battle FAQ steps 1-4: 10 base, no attacker-side effect, +20 Weakness, then \
         -30 of defender-side reductions = 0. Damage is floored at 0 only after the last step"
    );
}

/// All four FAQ steps at once: Giovanni ("During this turn, attacks used by your Pokémon do +10
/// damage to your opponent's Active Pokémon") is an effect on the **Attacking** Pokémon, so it
/// lands in step 2 and is inside what Bounded Field's ×2 multiplies; Heavy Helmet is an effect on
/// the **Defending** Pokémon and comes off afterwards. `(50 + 10) × 2 − 20 = 100`.
///
/// The old order gave `(50 + 10 − 20) × 2 = 80`.
#[test]
fn test_attacker_bonus_is_multiplied_by_weakness_and_the_defender_reduction_is_not() {
    let mut game = hitmontop_vs_snorlax_with_heavy_helmet();
    put_bounded_field_in_play(&mut game);

    let giovanni = trainer_from_id(CardId::A1223Giovanni);
    let mut state: State = game.get_state_clone();
    state.hands[0] = vec![Card::Trainer(giovanni.clone())];
    game.set_state(state);
    play_trainer(&mut game, 0, giovanni);

    assert_eq!(
        damage_to_opponent_active(&mut game, &spinning_attack()),
        100,
        "Detailed battle FAQ steps 1-4: 50 base, +10 Giovanni (attacking side), ×2 Weakness \
         under Bounded Field, then -20 Heavy Helmet (defending side)"
    );
}

/// The same three modifiers without Bounded Field: with a flat Weakness the two orders agree on
/// `50 + 10 + 20 − 20 = 60`. Kept next to the test above so a future change that only ever fixes
/// the ×2 path is visibly incomplete.
#[test]
fn test_attacker_bonus_flat_weakness_and_defender_reduction() {
    let mut game = hitmontop_vs_snorlax_with_heavy_helmet();

    let giovanni = trainer_from_id(CardId::A1223Giovanni);
    let mut state: State = game.get_state_clone();
    state.hands[0] = vec![Card::Trainer(giovanni.clone())];
    game.set_state(state);
    play_trainer(&mut game, 0, giovanni);

    assert_eq!(
        damage_to_opponent_active(&mut game, &spinning_attack()),
        60,
        "50 base + 10 Giovanni + 20 Weakness - 20 Heavy Helmet = 60"
    );
}
