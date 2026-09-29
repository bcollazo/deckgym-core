use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Mega Sableye ex's Cursed Jewel: "During your opponent's next turn, if this Pokémon is
/// damaged by an attack, do 40 damage to the Attacking Pokémon." When that delayed 40 damage
/// lands on the attacker, if the attacker is holding its own Rocky Helmet, that hit should
/// also bounce 20 damage from the Rocky Helmet back onto Mega Sableye ex.
///
/// Mega Blastoise ex holds a Rocky Helmet the whole game, so the ordinary "Rocky Helmet
/// punishes whoever attacks its holder" case fires on turn 1 too, when Mega Sableye ex attacks
/// into it. Mega Sableye ex (170 HP) ends up taking three hits across both turns:
///   - Turn 1: 20 from Mega Blastoise ex's Rocky Helmet, for attacking into it with Cursed Jewel.
///   - Turn 2: 130 from Mega Blastoise ex's Triple Bombardment.
///   - Turn 2: 20 more from that same Rocky Helmet, reflected off Cursed Jewel's own delayed
///     counterattack landing on Mega Blastoise ex (the interaction under test).
/// 20 + 130 + 20 = 170, exactly knocking out a 170 HP Mega Sableye ex.
#[test]
fn test_rocky_helmet_reflects_mega_sableye_ex_cursed_jewel_counterattack() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B3b041MegaSableyeEx)
            .with_energy(vec![EnergyType::Darkness, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::B1a020MegaBlastoiseEx)
            .with_energy(vec![
                EnergyType::Water,
                EnergyType::Water,
                EnergyType::Colorless,
            ])
            .with_tool(get_card_by_enum(CardId::A2148RockyHelmet))],
    );

    // Turn 1: Mega Sableye ex uses Cursed Jewel on Mega Blastoise ex (80 damage), and arms the
    // delayed counterattack for the opponent's next turn. Attacking into Mega Blastoise ex's
    // Rocky Helmet also immediately reflects 20 damage back onto Mega Sableye ex.
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3b041MegaSableyeEx, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        150,
        "Mega Blastoise ex should take 80 damage from Cursed Jewel (230 - 80 = 150)"
    );
    assert_eq!(
        state.get_active(0).get_remaining_hp(),
        150,
        "Mega Sableye ex should take 20 Rocky Helmet damage for attacking into it (170 - 20 = 150)"
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });

    // Turn 2: Mega Blastoise ex attacks Mega Sableye ex with Triple Bombardment (130 damage).
    // This triggers Cursed Jewel's delayed 40 damage back onto Mega Blastoise ex, which in turn
    // should trigger Mega Blastoise ex's own Rocky Helmet, reflecting 20 damage back onto Mega
    // Sableye ex.
    game.apply_action(&Action {
        actor: 1,
        action: attack_action(CardId::B1a020MegaBlastoiseEx, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert!(
        state.in_play_pokemon[0][0].is_none(),
        "Mega Sableye ex should be knocked out: 20 (turn 1 Rocky Helmet) + 130 (Triple \
         Bombardment) + 20 (Rocky Helmet again, reflected off Cursed Jewel's counterattack) = \
         170 damage on a 170 HP Mega Sableye ex"
    );
    assert_eq!(
        state.points[1], 3,
        "Player 1 should have won 3 points for knocking out a Mega Pokemon"
    );
}
