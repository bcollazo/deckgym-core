use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Chatot's Mimic shuffles your hand into your deck and then draws a card for each card in your
/// opponent's hand.
#[test]
fn test_mimic_redraws_to_the_opponents_hand_size() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1a062Chatot).with_energy(vec![EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    let mut state = game.get_state_clone();
    state.hands[0] = vec![
        get_card_by_enum(CardId::A1001Bulbasaur),
        get_card_by_enum(CardId::A1001Bulbasaur),
        get_card_by_enum(CardId::A1001Bulbasaur),
        get_card_by_enum(CardId::A1001Bulbasaur),
    ];
    state.hands[1] = vec![
        get_card_by_enum(CardId::A1033Charmander),
        get_card_by_enum(CardId::A1033Charmander),
    ];
    let deck_size_before = state.decks[0].cards.len();
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1a062Chatot, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.hands[0].len(), 2);
    // 4 cards went back into the deck and 2 came back out.
    assert_eq!(state.decks[0].cards.len(), deck_size_before + 4 - 2);
}

/// Mime Jr.'s Mime-y Shuffle is the same effect; with an empty opponent hand it just discards
/// your own hand into the deck.
#[test]
fn test_mimey_shuffle_with_empty_opponent_hand_draws_nothing() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4068MimeJr)],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    let mut state = game.get_state_clone();
    state.hands[0] = vec![get_card_by_enum(CardId::A1001Bulbasaur)];
    state.hands[1].clear();
    let deck_size_before = state.decks[0].cards.len();
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4068MimeJr, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert!(state.hands[0].is_empty());
    assert_eq!(state.decks[0].cards.len(), deck_size_before + 1);
}
