use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Test Raging Bolt B3a 055 - Baneful Boom: discard all Energy from itself, then Knock Out the
/// opponent's Active Pokémon outright.
#[test]
fn test_raging_bolt_baneful_boom_knocks_out_opponent_active() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B3a055RagingBolt).with_energy(vec![
                EnergyType::Water,
                EnergyType::Water,
                EnergyType::Lightning,
                EnergyType::Lightning,
                EnergyType::Colorless,
            ]),
        ],
        vec![
            PlayedCard::from_id(CardId::B4197WailordEx),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3a055RagingBolt, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    // All Energy was discarded from Raging Bolt.
    assert!(state.get_active(0).attached_energy.is_empty());
    // The opponent's Wailord ex was Knocked Out outright, leaving an empty Active spot.
    assert!(state.in_play_pokemon[1][0].is_none());

    // Player 1 must now choose a new Active Pokémon from the bench.
    let (actor, choices) = state.generate_possible_actions();
    assert_eq!(actor, 1);
    assert!(choices.iter().all(|choice| matches!(
        choice.action,
        SimpleAction::Activate {
            player: 1,
            in_play_idx: _
        }
    )));
}
