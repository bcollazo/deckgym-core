use deckgym::{
    actions::SimpleAction, card_ids::CardId, database::get_card_by_enum, models::PlayedCard,
    test_support::get_test_game_with_board,
};

#[test]
fn test_massive_body_blocks_opponent_stadiums_only_while_active() {
    // Opponent's active Snorlax blocks the current player from playing Stadium cards.
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::B3b055Snorlax)],
    );
    let mut state = game.get_state_clone();
    state.hands[0].push(get_card_by_enum(CardId::B2153TrainingArea));
    game.set_state(state);

    let (_, actions) = game.get_state_clone().generate_possible_actions();
    assert!(
        !actions.iter().any(|a| matches!(
            &a.action,
            SimpleAction::Play { trainer_card } if trainer_card.name == "Training Area"
        )),
        "Massive Body should block the opponent from playing Stadium cards"
    );

    // Control: with Snorlax on the opponent's Bench instead, the Stadium is playable.
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::B3b055Snorlax),
        ],
    );
    let mut state = game.get_state_clone();
    state.hands[0].push(get_card_by_enum(CardId::B2153TrainingArea));
    game.set_state(state);

    let (_, actions) = game.get_state_clone().generate_possible_actions();
    assert!(
        actions.iter().any(|a| matches!(
            &a.action,
            SimpleAction::Play { trainer_card } if trainer_card.name == "Training Area"
        )),
        "Massive Body should not apply from the Bench"
    );
}
