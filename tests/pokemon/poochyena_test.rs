use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_poochyena_bite_draws_per_poochyena_in_play() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B4093Poochyena).with_energy(vec![EnergyType::Darkness]),
            PlayedCard::from_id(CardId::B4171Poochyena),
            PlayedCard::from_id(CardId::B4171Poochyena),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    let hand_before = state.hands[0].len();
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4093Poochyena, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    // 3 Poochyena in play -> draw 3
    assert_eq!(
        state.hands[0].len(),
        hand_before + 3,
        "should draw one card per Poochyena in play"
    );
}
