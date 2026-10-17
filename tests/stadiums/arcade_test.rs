use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, PlayedCard},
    test_support::get_initialized_game,
};

fn game_with_arcade(deck_cards: Vec<Card>, hand_cards: Vec<Card>) -> deckgym::Game<'static> {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    state.active_stadium = Some(get_card_by_enum(CardId::B4a072Arcade));
    state.decks[0].cards = deck_cards;
    state.hands[0] = hand_cards;
    game.set_state(state);
    game
}

fn use_stadium(game: &mut deckgym::Game<'static>, actor: usize) {
    game.apply_action(&Action {
        actor,
        action: SimpleAction::UseStadium,
        is_stack: false,
    });
}

fn has_use_stadium(game: &deckgym::Game<'static>) -> bool {
    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::UseStadium))
}

/// Arcade: "Once during each player's turn, that player may flip 3 coins. If all of them are
/// heads, that player draws cards until they have 7 cards in their hand."
#[test]
fn test_arcade_draws_to_seven_cards_on_all_heads_and_can_only_be_used_once() {
    let deck_cards: Vec<Card> = (0..10)
        .map(|_| get_card_by_enum(CardId::A1001Bulbasaur))
        .collect();
    let hand_cards: Vec<Card> = (0..3)
        .map(|_| get_card_by_enum(CardId::A1033Charmander))
        .collect();

    assert!(has_use_stadium(&game_with_arcade(
        deck_cards.clone(),
        hand_cards.clone()
    )));

    // Try many seeds until we observe both an all-heads (drew to 7) and a not-all-heads
    // (hand size unchanged) outcome.
    let mut saw_draw = false;
    let mut saw_no_draw = false;
    for seed in 0..30 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.set_board(
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        );
        state.current_player = 0;
        state.turn_count = 3;
        state.active_stadium = Some(get_card_by_enum(CardId::B4a072Arcade));
        state.decks[0].cards = deck_cards.clone();
        state.hands[0] = hand_cards.clone();
        game.set_state(state);

        use_stadium(&mut game, 0);
        let state = game.get_state_clone();

        assert!(
            state.has_used_stadium[0],
            "Seed {seed}: Arcade is a once-per-turn stadium effect"
        );
        assert!(
            !has_use_stadium(&game),
            "Seed {seed}: player should not be able to use Arcade twice in one turn"
        );

        match state.hands[0].len() {
            7 => saw_draw = true,
            3 => saw_no_draw = true,
            other => panic!("Seed {seed}: unexpected hand size {other}"),
        }
    }

    assert!(saw_draw, "Arcade should draw to 7 cards for some seed");
    assert!(
        saw_no_draw,
        "Arcade should leave the hand untouched for some seed"
    );
}

#[test]
fn test_arcade_unavailable_when_hand_already_has_seven_cards() {
    let deck_cards: Vec<Card> = (0..10)
        .map(|_| get_card_by_enum(CardId::A1001Bulbasaur))
        .collect();
    let hand_cards: Vec<Card> = (0..7)
        .map(|_| get_card_by_enum(CardId::A1033Charmander))
        .collect();

    let game = game_with_arcade(deck_cards, hand_cards);
    assert!(
        !has_use_stadium(&game),
        "UseStadium should not be offered when the hand already has 7+ cards"
    );
}
