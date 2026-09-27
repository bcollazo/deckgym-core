//! Urshifu's Double Type: "As long as this Pokémon is in play, it is [X] and [Y] type."
//!
//! B3 051 Rapid Strike Urshifu is printed [W] and is [W] + [F] in play.
//! B3 113 Single Strike Urshifu is printed [D] and is [F] + [D] in play.
//!
//! Every test drives the public `Game` API. `SimpleAction::ApplyDamage` from (0, 0) to (1, 0) with
//! `is_from_active_attack` runs the full active-to-active damage pipeline (Weakness, typed auras,
//! …) without dragging in each Urshifu's own attack effects, which are tested separately in
//! `koraidon_urshifu_test.rs`.

use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    effects::CardEffect,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// A raw 40-damage active-to-active hit from player 0's Active Pokémon.
fn hit_opponent_active_for_40(game: &mut deckgym::Game<'static>) {
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (0, 0),
            targets: vec![(40, 1, 0)],
            is_from_active_attack: true,
        },
        is_stack: false,
    });
}

fn remaining_hp_after_40(attacker: PlayedCard, defender: PlayedCard) -> u32 {
    let mut game = get_test_game_with_board(vec![attacker], vec![defender]);
    hit_opponent_active_for_40(&mut game);
    game.get_state_clone().get_active(1).get_remaining_hp()
}

// ---------------------------------------------------------------------------
// Weakness: the defender's Weakness is checked against the attacker's *type set*
// ---------------------------------------------------------------------------

/// Rapid Strike Urshifu is printed [W], so a [W]-weak defender takes the usual +20. This is the
/// control for the [F] case below: the printed type must keep working.
#[test]
fn test_rapid_strike_urshifu_gets_weakness_bonus_from_its_printed_water_type() {
    // Charizard: 150 HP, weak to [W].
    let hp = remaining_hp_after_40(
        PlayedCard::from_id(CardId::B3051RapidStrikeUrshifu),
        PlayedCard::from_id(CardId::A1035Charizard),
    );
    assert_eq!(
        hp,
        150 - 60,
        "Charizard is weak to [W]; Rapid Strike Urshifu's printed type should still apply +20"
    );
}

/// Double Type also makes Rapid Strike Urshifu [F], so a [F]-weak defender takes +20 as well.
#[test]
fn test_rapid_strike_urshifu_gets_weakness_bonus_from_its_granted_fighting_type() {
    // Snorlax: 150 HP, weak to [F].
    let hp = remaining_hp_after_40(
        PlayedCard::from_id(CardId::B3051RapidStrikeUrshifu),
        PlayedCard::from_id(CardId::A1211Snorlax),
    );
    assert_eq!(
        hp,
        150 - 60,
        "Snorlax is weak to [F], which Double Type grants Rapid Strike Urshifu"
    );
}

/// A defender weak to neither of Urshifu's two types gets no bonus at all.
#[test]
fn test_rapid_strike_urshifu_gets_no_weakness_bonus_against_a_fire_weak_defender() {
    // Venusaur: 160 HP, weak to [R] — neither [W] nor [F].
    let hp = remaining_hp_after_40(
        PlayedCard::from_id(CardId::B3051RapidStrikeUrshifu),
        PlayedCard::from_id(CardId::A1003Venusaur),
    );
    assert_eq!(
        hp,
        160 - 40,
        "Venusaur is weak to [R]; neither of Urshifu's types should trigger Weakness"
    );
}

/// Single Strike Urshifu is printed [D] and granted [F] + [D]: the printed type still applies.
#[test]
fn test_single_strike_urshifu_gets_weakness_bonus_from_its_printed_darkness_type() {
    // Golurk: 140 HP, weak to [D].
    let hp = remaining_hp_after_40(
        PlayedCard::from_id(CardId::B3113SingleStrikeUrshifu),
        PlayedCard::from_id(CardId::A1136Golurk),
    );
    assert_eq!(hp, 140 - 60, "Golurk is weak to [D]");
}

/// …and the granted [F] applies too.
#[test]
fn test_single_strike_urshifu_gets_weakness_bonus_from_its_granted_fighting_type() {
    let hp = remaining_hp_after_40(
        PlayedCard::from_id(CardId::B3113SingleStrikeUrshifu),
        PlayedCard::from_id(CardId::A1211Snorlax),
    );
    assert_eq!(
        hp,
        150 - 60,
        "Snorlax is weak to [F], which Double Type grants Single Strike Urshifu"
    );
}

/// Weakness names a single type, so a dual-type attacker can match it at most once — the bonus is
/// a flat +20, never +40.
#[test]
fn test_weakness_bonus_is_applied_once_for_a_dual_type_attacker() {
    let dual = remaining_hp_after_40(
        PlayedCard::from_id(CardId::B3051RapidStrikeUrshifu),
        PlayedCard::from_id(CardId::A1211Snorlax),
    );
    // Machop is a plain [F] Pokémon with no Ability, so nothing but Weakness moves the number.
    let single = remaining_hp_after_40(
        PlayedCard::from_id(CardId::A1143Machop),
        PlayedCard::from_id(CardId::A1211Snorlax),
    );
    assert_eq!(
        dual, single,
        "a dual-type attacker's Weakness bonus must match a single-type attacker's"
    );
}

/// Regression: a plain single-type attacker's Weakness behaviour is unchanged.
#[test]
fn test_single_type_attacker_weakness_is_unchanged() {
    // Wartortle is [W] only.
    let vs_water_weak = remaining_hp_after_40(
        PlayedCard::from_id(CardId::A1054Wartortle),
        PlayedCard::from_id(CardId::A1035Charizard),
    );
    assert_eq!(vs_water_weak, 150 - 60, "Charizard is weak to [W]");

    let vs_fighting_weak = remaining_hp_after_40(
        PlayedCard::from_id(CardId::A1054Wartortle),
        PlayedCard::from_id(CardId::A1211Snorlax),
    );
    assert_eq!(
        vs_fighting_weak,
        150 - 40,
        "a plain [W] Pokémon must not pick up a [F] Weakness bonus"
    );
}

// ---------------------------------------------------------------------------
// "If your opponent's Active Pokémon is a [F] Pokémon"
// ---------------------------------------------------------------------------

/// Snover's Ice Shard: 10 damage, +30 if the opponent's Active is a [F] Pokémon. Rapid Strike
/// Urshifu counts as [F] through Double Type. (Snover is [W]; Urshifu is weak to [L], so no
/// Weakness bonus muddies the number.)
#[test]
fn test_defender_type_condition_sees_urshifus_granted_fighting_type() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A2044Snover).with_energy(vec![EnergyType::Water])],
        vec![PlayedCard::from_id(CardId::B3051RapidStrikeUrshifu)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A2044Snover, 0),
        is_stack: false,
    });

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        120 - 40,
        "Ice Shard should deal 10 + 30 against a Pokémon that is [F] through Double Type"
    );
}

/// Regression: the same attack against a plain [W] Pokémon still gets only its base damage.
#[test]
fn test_defender_type_condition_does_not_fire_against_a_plain_water_pokemon() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A2044Snover).with_energy(vec![EnergyType::Water])],
        vec![PlayedCard::from_id(CardId::A1054Wartortle)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A2044Snover, 0),
        is_stack: false,
    });

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        80 - 10,
        "Wartortle is not a [F] Pokémon, so Ice Shard should deal only its base 10"
    );
}

// ---------------------------------------------------------------------------
// Typed auras and typed Energy attachment
// ---------------------------------------------------------------------------

/// Lucario's Fighting Coach: "Attacks used by your [F] Pokémon do +20 damage to your opponent's
/// Active Pokémon." Rapid Strike Urshifu is printed [W], but Double Type makes it a [F] Pokémon,
/// so the aura covers it.
#[test]
fn test_fighting_coach_aura_boosts_rapid_strike_urshifu() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B3051RapidStrikeUrshifu),
            PlayedCard::from_id(CardId::A2092Lucario),
        ],
        // Venusaur is weak to [R], so nothing but the aura moves the number.
        vec![PlayedCard::from_id(CardId::A1003Venusaur)],
    );
    hit_opponent_active_for_40(&mut game);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        160 - 60,
        "Fighting Coach should add +20 to a Pokémon that is [F] through Double Type"
    );
}

/// Regression: the aura still ignores a Pokémon that is only [W].
#[test]
fn test_fighting_coach_aura_ignores_a_plain_water_pokemon() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1054Wartortle),
            PlayedCard::from_id(CardId::A2092Lucario),
        ],
        vec![PlayedCard::from_id(CardId::A1003Venusaur)],
    );
    hit_opponent_active_for_40(&mut game);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        160 - 40,
        "Fighting Coach must not boost a Pokémon that is not [F]"
    );
}

/// Baxcalibur: "…you may take a [W] Energy from your Energy Zone and attach it to the [W] Pokémon
/// in the Active Spot." Rapid Strike Urshifu is [W], so the ability is offered.
#[test]
fn test_typed_energy_attach_offers_its_ability_for_water_urshifu() {
    let game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B3051RapidStrikeUrshifu),
            PlayedCard::from_id(CardId::B2a036Baxcalibur),
        ],
        vec![PlayedCard::from_id(CardId::A1003Venusaur)],
    );

    let (_, choices) = game.get_state_clone().generate_possible_actions();
    assert!(
        choices
            .iter()
            .any(|choice| matches!(choice.action, SimpleAction::UseAbility { in_play_idx: 1 })),
        "Baxcalibur should offer its [W] attach with a [W] Active Pokémon"
    );
}

/// …and Single Strike Urshifu ([F] + [D]) is not a [W] Pokémon, so it is not offered. This guards
/// against the type set being widened into "any type matches anything".
#[test]
fn test_typed_energy_attach_does_not_offer_its_ability_for_darkness_urshifu() {
    let game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B3113SingleStrikeUrshifu),
            PlayedCard::from_id(CardId::B2a036Baxcalibur),
        ],
        vec![PlayedCard::from_id(CardId::A1003Venusaur)],
    );

    let (_, choices) = game.get_state_clone().generate_possible_actions();
    assert!(
        !choices
            .iter()
            .any(|choice| matches!(choice.action, SimpleAction::UseAbility { in_play_idx: 1 })),
        "Single Strike Urshifu is [F] + [D], never [W], so Baxcalibur has no target"
    );
}

// ---------------------------------------------------------------------------
// Ability suppression
// ---------------------------------------------------------------------------

fn urshifu_with_abilities_disabled(card_id: CardId) -> PlayedCard {
    let mut urshifu = PlayedCard::from_id(card_id);
    urshifu.add_effect(CardEffect::AbilitiesDisabled, 1);
    urshifu
}

/// Double Type is an Ability, so stripping Abilities reverts Urshifu to its printed [W]: the
/// [F]-weak defender stops taking the bonus.
#[test]
fn test_abilities_disabled_reverts_rapid_strike_urshifu_to_water_only() {
    let hp = remaining_hp_after_40(
        urshifu_with_abilities_disabled(CardId::B3051RapidStrikeUrshifu),
        PlayedCard::from_id(CardId::A1211Snorlax),
    );
    assert_eq!(
        hp,
        150 - 40,
        "with its Abilities disabled Urshifu is only [W], so a [F] Weakness must not trigger"
    );
}

/// …while its printed [W] type keeps working, since that never came from the Ability.
#[test]
fn test_abilities_disabled_keeps_rapid_strike_urshifus_printed_water_type() {
    let hp = remaining_hp_after_40(
        urshifu_with_abilities_disabled(CardId::B3051RapidStrikeUrshifu),
        PlayedCard::from_id(CardId::A1035Charizard),
    );
    assert_eq!(
        hp,
        150 - 60,
        "the printed [W] type survives losing the Ability"
    );
}

/// The same on the defending side: a suppressed Urshifu is no longer a [F] Pokémon for
/// "if your opponent's Active Pokémon is a [F] Pokémon".
#[test]
fn test_abilities_disabled_urshifu_no_longer_counts_as_fighting_for_defender_conditions() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A2044Snover).with_energy(vec![EnergyType::Water])],
        vec![urshifu_with_abilities_disabled(
            CardId::B3051RapidStrikeUrshifu,
        )],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A2044Snover, 0),
        is_stack: false,
    });

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        120 - 10,
        "a suppressed Urshifu is only [W], so Ice Shard should deal its base 10"
    );
}
