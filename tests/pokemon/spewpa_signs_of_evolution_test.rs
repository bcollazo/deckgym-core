use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

#[test]
fn test_spewpa_signs_of_evolution_puts_vivillon_into_hand() {
    let mut game = get_initialized_game(0);
    game.play_until_stable();

    let mut state = game.get_state_clone();
    state.set_board(
        vec![PlayedCard::from_id(CardId::B2012Spewpa).with_energy(vec![EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    state.current_player = 0;

    // Put a single Vivillon (evolves from Spewpa) into the deck so the outcome is deterministic.
    state.decks[0].cards = vec![get_card_by_enum(CardId::B2013Vivillon)];

    let hand_size_before = state.hands[0].len();
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(state.hands[0].len(), hand_size_before + 1);
    assert!(matches!(&state.hands[0].last(), Some(Card::Pokemon(card)) if card.name == "Vivillon"));
    assert!(state.decks[0].cards.is_empty());
}

#[test]
fn test_spewpa_signs_of_evolution_no_matching_card_just_shuffles() {
    let mut game = get_initialized_game(0);
    game.play_until_stable();

    let mut state = game.get_state_clone();
    state.set_board(
        vec![PlayedCard::from_id(CardId::B2012Spewpa).with_energy(vec![EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    state.current_player = 0;

    // No Vivillon in the deck.
    state.decks[0].cards = vec![get_card_by_enum(CardId::A1033Charmander)];

    let hand_size_before = state.hands[0].len();
    let deck_size_before = state.decks[0].cards.len();
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(state.hands[0].len(), hand_size_before);
    assert_eq!(state.decks[0].cards.len(), deck_size_before);
}
