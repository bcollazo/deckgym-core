use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{Card, EnergyType, PlayedCard},
    state::GameOutcome,
    test_support::{attack_action, get_test_game_with_board},
};

/// Mega Blaziken ex's Mega Burning does 120 damage and burns the opponent's Active Pokémon.
/// Between turns, burn deals another 20 damage during Pokémon Checkup, and (if the burned
/// Pokémon's owner has a Garganacl in play) Garganacl's Blessed Salt heals 10 damage from each of
/// its owner's Pokémon during that same checkup. The knockout check must not run until *all* of
/// these checkup effects have applied, not immediately after burn's damage alone — otherwise a
/// Pokémon that would have survived the net damage (attack + burn - Blessed Salt heal) gets
/// discarded prematurely, before Garganacl's heal ever gets a chance to apply.
///
/// This test: a Mega Lucario ex (190 HP) that took the full net damage (120 + 20 - 10 = 130)
/// still ends up exactly at 0 HP and knocked out — the point being that the KO is only decided
/// after Garganacl's heal has been applied, not right after burn.
#[test]
fn test_burn_ko_is_decided_only_after_garganacl_heal_applies() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B1036MegaBlazikenEx)
            .with_energy(vec![EnergyType::Fire, EnergyType::Fire])],
        vec![
            PlayedCard::from_id(CardId::B3081MegaLucarioEx).with_remaining_hp(130),
            PlayedCard::from_id(CardId::B3a033Garganacl),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B1036MegaBlazikenEx, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(
        state.get_remaining_hp(1, 0),
        10,
        "Mega Burning should leave Mega Lucario ex (130 HP remaining) at 10 HP and Burned"
    );

    // End Mega Blaziken ex's turn: burn (20) then Garganacl's heal (10) both apply during the
    // between-turns Pokémon Checkup. Net: 130 HP remaining - 20 + 10 = 0 -> knocked out.
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(
        state.points[0], 3,
        "Knocking out a Mega ex Pokémon should award 3 points"
    );
    // Mega Lucario ex is worth 3 points on its own, so this KO immediately wins the game for
    // player 0 (short-circuiting the promotion logic that would otherwise bring Garganacl up).
    assert_eq!(
        state.winner,
        Some(GameOutcome::Win(0)),
        "Knocking out Mega Lucario ex should immediately win the game for player 0"
    );
}

/// Same interaction as above, but with a smaller net burn that Garganacl's heal actually saves:
/// a Koraidon ex (150 HP) at 140 HP remaining takes 120 (Mega Burning) + 20 (burn) damage during
/// the same turn/checkup sequence, which alone would be lethal (140 - 120 - 20 = -20). But
/// Koraidon ex's owner also has a Garganacl in play, whose Blessed Salt heals 10 damage from each
/// of their Pokémon during that same Pokémon Checkup — applied before the knockout check, this
/// leaves Koraidon ex alive at 10 HP (140 - 120 - 20 + 10 = 10) instead of knocked out.
#[test]
fn test_garganacl_heal_saves_koraidon_ex_from_otherwise_lethal_burn() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B1036MegaBlazikenEx)
            .with_energy(vec![EnergyType::Fire, EnergyType::Fire])],
        vec![
            PlayedCard::from_id(CardId::B3a036KoraidonEx).with_remaining_hp(140),
            PlayedCard::from_id(CardId::B3a033Garganacl),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B1036MegaBlazikenEx, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(
        state.get_remaining_hp(1, 0),
        20,
        "Mega Burning should leave Koraidon ex (140 HP remaining) at 20 HP and Burned"
    );

    // End Mega Blaziken ex's turn: burn (20) would be lethal on its own (20 - 20 == 0), but
    // Garganacl's heal (10) applies in the same checkup before the knockout is decided.
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(
        state.get_remaining_hp(1, 0),
        10,
        "Garganacl's Blessed Salt heal should save Koraidon ex from otherwise-lethal burn \
         damage, leaving it at 10 HP (140 - 120 - 20 + 10)"
    );
    assert!(
        matches!(&state.get_active(1).card, Card::Pokemon(p) if p.name == "Koraidon ex"),
        "Koraidon ex should still be the Active Pokemon"
    );
    assert_eq!(
        state.points[0], 0,
        "Player 0 should not have scored any knockout points"
    );
}
