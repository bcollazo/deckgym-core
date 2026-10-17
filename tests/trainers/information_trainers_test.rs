use deckgym::{
    actions::SimpleAction,
    card_ids::CardId,
    models::{Card, PlayedCard},
    test_support::{get_test_game_with_board, play_trainer, trainer_from_id},
};

/// Pokédex, Hand Scope, Looker, Hiker and Morty only *look at* cards — nothing changes zones, so
/// they resolve as no-ops. What must hold is that they are playable and do not disturb the board.
#[test]
fn test_information_only_trainers_are_playable_and_change_nothing() {
    for card_id in [
        CardId::PA003HandScope,
        CardId::PA004PokedEx,
        CardId::PA008PokedEx,
        CardId::A3a068Looker,
        CardId::A4161Hiker,
        CardId::A4a071Morty,
    ] {
        let mut game = get_test_game_with_board(
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
            vec![PlayedCard::from_id(CardId::A1033Charmander)],
        );
        let mut state = game.get_state_clone();
        let trainer = trainer_from_id(card_id);
        state.hands[0] = vec![Card::Trainer(trainer.clone())];
        let deck_before = state.decks[0].cards.len();
        let opponent_hand_before = state.hands[1].len();
        game.set_state(state);

        play_trainer(&mut game, 0, trainer);

        let state = game.get_state_clone();
        assert_eq!(
            state.decks[0].cards.len(),
            deck_before,
            "{card_id:?} should not move any card out of the deck"
        );
        assert!(
            state.hands[0].is_empty(),
            "{card_id:?} should leave the hand empty after being played"
        );
        assert_eq!(
            state.hands[1].len(),
            opponent_hand_before,
            "{card_id:?} should not touch the opponent's hand"
        );
        assert_eq!(state.get_active(0).get_name(), "Bulbasaur");
    }
}

/// Rotom Dex: "Look at the top card of your deck. Then, you may shuffle your deck." The optional
/// shuffle is offered as a real choice (decline = Noop).
#[test]
fn test_rotom_dex_offers_optional_shuffle() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    let mut state = game.get_state_clone();
    let rotom_dex = trainer_from_id(CardId::A3145RotomDEx);
    state.hands[0] = vec![Card::Trainer(rotom_dex.clone())];
    let deck_size_before = state.decks[0].cards.len();
    game.set_state(state);

    play_trainer(&mut game, 0, rotom_dex);

    let (actor, actions) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(
        actions
            .iter()
            .any(|a| matches!(a.action, SimpleAction::ShuffleOwnDeck)),
        "Rotom Dex should offer to shuffle the deck"
    );
    assert!(
        actions
            .iter()
            .any(|a| matches!(a.action, SimpleAction::Noop)),
        "Rotom Dex should offer to decline the shuffle"
    );

    let shuffle = actions
        .iter()
        .find(|a| matches!(a.action, SimpleAction::ShuffleOwnDeck))
        .expect("shuffle option")
        .clone();
    game.apply_action(&shuffle);

    assert_eq!(
        game.get_state_clone().decks[0].cards.len(),
        deck_size_before,
        "Shuffling should not change the number of cards in the deck"
    );
}
