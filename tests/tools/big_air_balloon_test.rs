use deckgym::{
    actions::SimpleAction, card_ids::CardId, database::get_card_by_enum, models::PlayedCard,
    test_support::get_test_game_with_board,
};

#[test]
fn test_big_air_balloon_attaches_to_basic_but_only_frees_stage_2_retreat() {
    // The tool can be attached to any Pokemon (verified in-game)...
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let mut state = game.get_state_clone();
    state.hands[0].push(get_card_by_enum(CardId::B2a087BigAirBalloon));
    game.set_state(state);

    let (_, actions) = game.get_state_clone().generate_possible_actions();
    assert!(
        actions.iter().any(|a| matches!(
            &a.action,
            SimpleAction::Play { trainer_card } if trainer_card.name == "Big Air Balloon"
        )),
        "Big Air Balloon should be attachable to non-Stage 2 Pokemon"
    );

    // ...but only a Stage 2 holder gets the free retreat. Bulbasaur (retreat cost 1, no
    // energy) still cannot retreat with the balloon attached.
    let game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_tool(get_card_by_enum(CardId::B2a087BigAirBalloon)),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let (_, actions) = game.get_state_clone().generate_possible_actions();
    assert!(
        !actions
            .iter()
            .any(|a| matches!(a.action, SimpleAction::Retreat(_))),
        "Big Air Balloon should not remove a non-Stage 2 Pokemon's retreat cost"
    );

    // Control: a Stage 2 holder (Venusaur ex, retreat cost 3) retreats for free.
    let game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1004VenusaurEx)
                .with_tool(get_card_by_enum(CardId::B2a087BigAirBalloon)),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let (_, actions) = game.get_state_clone().generate_possible_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a.action, SimpleAction::Retreat(_))),
        "Big Air Balloon should give a Stage 2 Pokemon free retreat"
    );
}
