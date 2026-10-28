use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, PlayedCard},
    test_support::get_initialized_game,
};

fn game_with_station(seed: u64, deck_size: usize) -> deckgym::Game<'static> {
    let mut game = get_initialized_game(seed);
    let mut state = game.get_state_clone();
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    state.active_stadium = Some(get_card_by_enum(CardId::C1131AbandonedStation));
    state.decks[0].cards = (0..deck_size)
        .map(|_| get_card_by_enum(CardId::A1001Bulbasaur))
        .collect::<Vec<Card>>();
    state.hands[0] = vec![get_card_by_enum(CardId::A1033Charmander)];
    game.set_state(state);
    game
}

fn has_use_stadium(game: &deckgym::Game<'static>) -> bool {
    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::UseStadium))
}

/// Abandoned Station: "Once during each player's turn, that player may use this effect. Flip a
/// coin. If heads, draw a card."
#[test]
fn test_abandoned_station_draws_a_card_on_heads_and_is_once_per_turn() {
    let mut saw_heads = false;
    let mut saw_tails = false;
    for seed in 0..30 {
        let mut game = game_with_station(seed, 10);
        assert!(has_use_stadium(&game));
        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::UseStadium,
            is_stack: false,
        });
        let state = game.get_state_clone();
        assert!(state.has_used_stadium[0]);
        match state.hands[0].len() {
            2 => saw_heads = true,
            1 => saw_tails = true,
            n => panic!("Seed {seed}: unexpected hand size {n}"),
        }
        assert!(!has_use_stadium(&game), "Seed {seed}: only once per turn");
    }
    assert!(saw_heads && saw_tails);
}

#[test]
fn test_abandoned_station_not_offered_with_empty_deck() {
    assert!(!has_use_stadium(&game_with_station(0, 0)));
}
