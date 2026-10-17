//! Promotion order after a double Knock Out. No card text covers this; it is a game rule seen in
//! play: when an attack leaves both Active Spots empty, the player whose turn it is (the
//! attacker) chooses their new Active Pokémon first, then the opponent, whichever seat went first.

use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game_with_board},
};

/// The attacker's Squirtle (20 HP left) uses Water Gun (20) on a Squirtle (20 HP left) holding
/// Rocky Helmet, whose 20 recoil Knocks the attacker out too. Both sides have a Benched
/// Bulbasaur. Returns the player who is asked to promote first.
fn first_to_promote(attacking_player: usize) -> usize {
    let attacker = vec![
        PlayedCard::from_id(CardId::A1053Squirtle)
            .with_energy(vec![EnergyType::Water])
            .with_remaining_hp(20),
        PlayedCard::from_id(CardId::A1001Bulbasaur),
    ];
    let defender = vec![
        PlayedCard::from_id(CardId::A1053Squirtle)
            .with_remaining_hp(20)
            .with_tool(get_card_by_enum(CardId::A2148RockyHelmet)),
        PlayedCard::from_id(CardId::A1001Bulbasaur),
    ];
    let (board_0, board_1) = if attacking_player == 0 {
        (attacker, defender)
    } else {
        (defender, attacker)
    };
    let mut game = get_initialized_game_with_board(0, attacking_player, 3, board_0, board_1);

    game.apply_action(&Action {
        actor: attacking_player,
        action: attack_action(CardId::A1053Squirtle, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert!(state.winner.is_none(), "both players still have a Bench");
    assert!(state.in_play_pokemon[0][0].is_none());
    assert!(state.in_play_pokemon[1][0].is_none());
    let (actor, actions) = deckgym::move_generation::generate_possible_actions(&state);
    assert!(actions
        .iter()
        .all(|a| matches!(a.action, SimpleAction::Activate { .. })));
    actor
}

#[test]
fn test_double_knockout_player_0_attacker_promotes_first() {
    assert_eq!(first_to_promote(0), 0);
}

#[test]
fn test_double_knockout_player_1_attacker_promotes_first() {
    assert_eq!(first_to_promote(1), 1);
}
