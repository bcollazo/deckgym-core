use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Team Rocket's Slowking ex's Evil Inspiration: "Once during your turn, if this Pokémon is in
/// the Active Spot, you may draw a card." Using it once draws a card and can't be used again
/// this turn.
#[test]
fn test_slowking_ex_evil_inspiration_draws_once_per_turn() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4a026TeamRocketsSlowkingEx)],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    let hand_size_before = game.get_state_clone().hands[0].len();

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.hands[0].len(), hand_size_before + 1);

    // The ability can't be used again this turn.
    let (actor, choices) = state.generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(!choices
        .iter()
        .any(|choice| matches!(choice.action, SimpleAction::UseAbility { in_play_idx: 0 })));
}

/// Team Rocket's Slowking ex's Hand Kinesis: 20 damage for each card in the attacker's hand.
#[test]
fn test_slowking_ex_hand_kinesis_damage_scales_with_hand_size() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4a026TeamRocketsSlowkingEx)
            .with_energy(vec![EnergyType::Psychic, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    let hand_size = game.get_state_clone().hands[0].len() as u32;
    let opponent_max_hp = game.get_state_clone().get_active(1).get_remaining_hp();

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4a026TeamRocketsSlowkingEx, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        opponent_max_hp - 20 * hand_size
    );
}
