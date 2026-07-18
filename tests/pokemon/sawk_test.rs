use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    effects::CardEffect,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game_with_board, get_test_game_with_board},
};

// Sawk's "Brick Break": "This attack's damage isn't affected by any effects on your opponent's
// Active Pokémon." Brick Break does a fixed 30 damage.
//
// These tests drive the attack through the public `Game` API and assert on the resulting HP,
// checking that every flavor of "effect on the opponent's Active Pokémon" (ability-derived
// damage reduction, a stored CardEffect, a damage-reducing Tool, and a coin-flip prevention
// ability) is ignored — while Weakness (a card property, not an effect) still applies.

fn sawk_active() -> PlayedCard {
    PlayedCard::from_id(CardId::B3086Sawk).with_energy(vec![EnergyType::Fighting])
}

fn brick_break() -> Action {
    Action {
        actor: 0,
        action: attack_action(CardId::B3086Sawk, 0),
        is_stack: false,
    }
}

/// Cloyster's "Shell Armor" (`ReduceDamageFromAttacks{10}`) is bypassed: Cloyster takes the full 30.
#[test]
fn test_sawk_bypasses_cloyster_shell_armor() {
    let mut game = get_test_game_with_board(
        vec![sawk_active()],
        vec![PlayedCard::from_id(CardId::A1067Cloyster)],
    );

    game.apply_action(&brick_break());

    let state = game.get_state_clone();
    // Cloyster has 120 HP; Shell Armor would leave it at 110. Brick Break ignores it → 90.
    assert_eq!(state.get_active(1).get_remaining_hp(), 90);
}

/// Control: a vanilla Basic Fighting attacker (Mankey, Low Kick 20) IS reduced by Shell Armor,
/// proving the ability is real and the bypass — not a broken ability — is responsible above.
#[test]
fn test_cloyster_shell_armor_reduces_normal_attacker() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1141Mankey).with_energy(vec![EnergyType::Fighting])],
        vec![PlayedCard::from_id(CardId::A1067Cloyster)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1141Mankey, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    // 20 damage - 10 Shell Armor = 10 dealt; 120 - 10 = 110.
    assert_eq!(state.get_active(1).get_remaining_hp(), 110);
}

/// A stored `CardEffect::PreventDamageFromBasic` (Carracosta's Blocking Shell) on the opponent's
/// active is bypassed: Sawk is Basic, yet Brick Break still deals its full 30.
#[test]
fn test_sawk_bypasses_prevent_damage_from_basic() {
    let mut defender = PlayedCard::from_id(CardId::A1004VenusaurEx);
    defender.add_effect(CardEffect::PreventDamageFromBasic, 1);

    let mut game = get_test_game_with_board(vec![sawk_active()], vec![defender]);

    game.apply_action(&brick_break());

    let state = game.get_state_clone();
    // Venusaur ex has 190 HP; without the bypass a Basic attacker deals 0. With bypass → 30.
    assert_eq!(state.get_active(1).get_remaining_hp(), 160);
}

/// Control: a vanilla Basic attacker IS blocked by PreventDamageFromBasic (0 damage dealt).
#[test]
fn test_prevent_damage_from_basic_blocks_normal_basic_attacker() {
    let mut defender = PlayedCard::from_id(CardId::A1004VenusaurEx);
    defender.add_effect(CardEffect::PreventDamageFromBasic, 1);

    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1141Mankey).with_energy(vec![EnergyType::Fighting])],
        vec![defender],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1141Mankey, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 190);
}

/// A damage-reducing Tool (Heavy Helmet, -20 to a Pokémon with retreat cost ≥ 3) on the
/// opponent's active is bypassed. Cloyster has a retreat cost of 3 and (with the helmet) would
/// otherwise stack -10 (Shell Armor) and -20 (Heavy Helmet); Brick Break ignores both.
#[test]
fn test_sawk_bypasses_heavy_helmet_and_ability() {
    let cloyster = PlayedCard::from_id(CardId::A1067Cloyster)
        .with_tool(get_card_by_enum(CardId::B1219HeavyHelmet));

    let mut game = get_test_game_with_board(vec![sawk_active()], vec![cloyster]);

    game.apply_action(&brick_break());

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 90);
}

/// Meowth's "Carefree Steps" (`CoinFlipToPreventDamage`) is bypassed: no prevention coin is
/// flipped against Brick Break. Meowth (50 HP, Weak to Fighting) always takes 30 + 20 Weakness =
/// 50 and is Knocked Out on every RNG seed. Without the bypass, a heads flip would prevent the
/// damage and leave Meowth alive at 50 HP.
#[test]
fn test_sawk_bypasses_meowth_carefree_steps() {
    for seed in 0..30u64 {
        let mut game = get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![sawk_active()],
            // Bench a second Pokémon so the Knock Out doesn't end the game.
            vec![
                PlayedCard::from_id(CardId::B2124Meowth),
                PlayedCard::from_id(CardId::A1001Bulbasaur),
            ],
        );

        game.apply_action(&brick_break());

        let state = game.get_state_clone();
        // If Carefree Steps had prevented the damage, Meowth would still be Active at full HP.
        let prevented = state.in_play_pokemon[1][0]
            .as_ref()
            .is_some_and(|p| p.get_name() == "Meowth" && p.get_remaining_hp() == 50);
        assert!(
            !prevented,
            "seed {seed}: Carefree Steps must not prevent Brick Break's damage"
        );
    }
}

/// Regression: Weakness is a card property, not an "effect on the Pokémon", so Brick Break is
/// still affected by it. Meowth (A1 196) is Weak to Fighting: 30 + 20 = 50.
#[test]
fn test_sawk_still_affected_by_weakness() {
    let mut game = get_test_game_with_board(
        vec![sawk_active()],
        vec![PlayedCard::from_id(CardId::A1196Meowth)],
    );

    game.apply_action(&brick_break());

    let state = game.get_state_clone();
    // Meowth has 60 HP; 30 + 20 Weakness = 50 dealt → 10 remaining.
    assert_eq!(state.get_active(1).get_remaining_hp(), 10);
}
