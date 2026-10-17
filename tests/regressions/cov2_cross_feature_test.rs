//! Cross-feature integration: Revavroom's multi-Tool storage (cov2/two-tools), the in-play type
//! sets that back Urshifu's Double Type (cov2/dual-type), and Alolan Muk's Power of Alchemy
//! (cov2/ability-gating) all on one board.
//!
//! The three features meet at `PlayedCard::ability_mechanic()`: `tool_capacity()` and
//! `get_energy_types()` both read it, and Power of Alchemy is what can make it return `None`.
//! Revavroom, Single Strike Urshifu and Alolan Muk are all Stage 1, so Power of Alchemy ("Basic
//! Pokémon in play ... have no Abilities") must leave every one of them alone — including Muk's
//! own. These tests pin that down, plus the typed-Tool gates that read the *holder's* type set
//! while `has_tool` scans every attached Tool.

use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
    Game,
};

/// Chansey's Gentle Slap (attack 0, 60 damage) fired by player 0 into player 1's Active.
fn gentle_slap(game: &mut Game<'static>) {
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1202Chansey, 0),
        is_stack: false,
    });
    game.play_until_stable();
}

fn attacking_chansey() -> PlayedCard {
    PlayedCard::from_id(CardId::A1202Chansey).with_energy(vec![
        EnergyType::Colorless,
        EnergyType::Colorless,
        EnergyType::Colorless,
    ])
}

/// Revavroom (Stage 1, [M], 120 HP) holding Giant Cape and Steel Apron, with `bench` behind the
/// attacking Chansey.
fn game_with_two_tool_revavroom(bench: Vec<PlayedCard>) -> Game<'static> {
    let mut player_zero = vec![attacking_chansey()];
    player_zero.extend(bench);
    get_test_game_with_board(
        player_zero,
        vec![PlayedCard::from_id(CardId::B4115Revavroom)
            .with_tool(get_card_by_enum(CardId::A2147GiantCape))
            .with_tool(get_card_by_enum(CardId::A4153SteelApron))],
    )
}

/// Revavroom is a Stage 1, so Power of Alchemy never reaches Dual Customization: it keeps both
/// Tool slots, and both Tools keep working. Steel Apron's −10 is gated on the *holder's* in-play
/// type set ([M]), so this exercises the dual-type type check, the two-Tool storage and the
/// ability-gating board scan in a single damage number.
#[test]
fn test_revavroom_keeps_both_tools_working_next_to_alolan_muk() {
    let expected_remaining_hp = |bench: Vec<PlayedCard>| {
        let mut game = game_with_two_tool_revavroom(bench);
        gentle_slap(&mut game);
        game.get_state_clone().get_active(1).get_remaining_hp()
    };

    // 120 printed HP + 20 Giant Cape = 140; Gentle Slap's 60 reduced to 50 by Steel Apron.
    assert_eq!(
        expected_remaining_hp(vec![]),
        90,
        "Giant Cape and Steel Apron should both apply to a two-slot Revavroom"
    );
    assert_eq!(
        expected_remaining_hp(vec![PlayedCard::from_id(CardId::B2097AlolanMuk)]),
        90,
        "Power of Alchemy only touches Basics, so a Stage 1 Revavroom keeps Dual Customization \
         and both of its Tools keep working"
    );
}

/// The capacity itself, not just the effects: `tool_capacity()` reads `ability_mechanic()`, which
/// is exactly what Power of Alchemy switches off — for Basics only.
#[test]
fn test_alolan_muk_does_not_take_revavrooms_second_tool_slot() {
    let game = game_with_two_tool_revavroom(vec![PlayedCard::from_id(CardId::B2097AlolanMuk)]);
    let state = game.get_state_clone();
    let revavroom = state.get_active(1);

    assert_eq!(
        revavroom.tool_capacity(),
        2,
        "Dual Customization survives Power of Alchemy on a Stage 1"
    );
    assert_eq!(revavroom.attached_tools.len(), 2);
}

/// Single Strike Urshifu's Double Type is a Stage 1 Ability too, so an Alolan Muk in play must not
/// revert it to its printed [D]. (The `AbilitiesDisabled` reversion — Budew's Prickly Powder — is
/// covered in `tests/pokemon/urshifu_double_type_test.rs`; this is the board-scan counterpart.)
#[test]
fn test_alolan_muk_does_not_revert_urshifus_granted_fighting_type() {
    let game = get_test_game_with_board(
        vec![
            attacking_chansey(),
            PlayedCard::from_id(CardId::B2097AlolanMuk),
        ],
        vec![PlayedCard::from_id(CardId::B3113SingleStrikeUrshifu)],
    );
    let state = game.get_state_clone();
    let urshifu = state.get_active(1);

    assert!(
        urshifu.is_type(EnergyType::Fighting),
        "Double Type is on a Stage 1, so Power of Alchemy should not strip the granted [F] type"
    );
    assert!(
        urshifu.is_type(EnergyType::Darkness),
        "printed type remains"
    );
    assert_eq!(
        urshifu.get_energy_types(),
        vec![EnergyType::Darkness, EnergyType::Fighting]
    );
}

/// The HP-cape chain is a series of independent `if`s precisely so a two-slot Pokémon can stack
/// two of them. Revavroom is a Stage 1, so Giant Cape (+20, any Pokémon) and Elegant Cape (+30,
/// Stage 1) both land — 120 + 20 + 30 = 170 — and Power of Alchemy does not change that.
#[test]
fn test_two_hp_capes_stack_on_revavroom_with_alolan_muk_in_play() {
    let game = get_test_game_with_board(
        vec![
            attacking_chansey(),
            PlayedCard::from_id(CardId::B2097AlolanMuk),
        ],
        vec![PlayedCard::from_id(CardId::B4115Revavroom)
            .with_tool(get_card_by_enum(CardId::A2147GiantCape))
            .with_tool(get_card_by_enum(CardId::B3b065ElegantCape))],
    );

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        170,
        "Giant Cape and Elegant Cape are independent bonuses, not an either/or chain"
    );
}
