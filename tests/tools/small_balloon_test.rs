use deckgym::{
    actions::SimpleAction, card_ids::CardId, database::get_card_by_enum, models::PlayedCard,
    test_support::get_initialized_game,
};

#[test]
fn test_small_balloon_reduces_retreat_cost_of_basic_pokemon() {
    let mut game = get_initialized_game(0);
    game.play_until_stable();

    let mut state = game.get_state_clone();
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_tool(get_card_by_enum(CardId::B3b064SmallBalloon)),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    state.current_player = 0;
    game.set_state(state);

    // Bulbasaur normally has a Retreat Cost of 1, but Small Balloon reduces it
    // by 1, so with no energy attached it should still be able to retreat for free.
    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(choices
        .iter()
        .any(|choice| matches!(choice.action, SimpleAction::Retreat(_))));
}

#[test]
fn test_without_small_balloon_basic_pokemon_cannot_retreat_for_free() {
    let mut game = get_initialized_game(0);
    game.play_until_stable();

    let mut state = game.get_state_clone();
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    state.current_player = 0;
    game.set_state(state);

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(!choices
        .iter()
        .any(|choice| matches!(choice.action, SimpleAction::Retreat(_))));
}
