use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::PlayedCard,
    test_support::get_test_game_with_board,
};

fn end_turn_and_settle(game: &mut deckgym::Game<'static>, actor: usize) {
    game.apply_action(&Action {
        actor,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    game.play_until_stable();
}

#[test]
fn test_legendary_pulse_draws_before_hiking_trail_tops_up_to_three() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A4a020SuicuneEx)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let mut state = game.get_state_clone();
    state.active_stadium = Some(get_card_by_enum(CardId::B2b069HikingTrail));
    state.hands[0] = vec![
        get_card_by_enum(CardId::A1001Bulbasaur),
        get_card_by_enum(CardId::A1001Bulbasaur),
    ];
    game.set_state(state);

    end_turn_and_settle(&mut game, 0);

    // Legendary Pulse draws first (2 -> 3), so Hiking Trail has nothing left to draw.
    assert_eq!(game.get_state_clone().hands[0].len(), 3);
}

#[test]
fn test_hiking_trail_still_tops_up_without_legendary_pulse() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let mut state = game.get_state_clone();
    state.active_stadium = Some(get_card_by_enum(CardId::B2b069HikingTrail));
    state.hands[0] = vec![get_card_by_enum(CardId::A1001Bulbasaur)];
    game.set_state(state);

    end_turn_and_settle(&mut game, 0);

    assert_eq!(game.get_state_clone().hands[0].len(), 3);
}
