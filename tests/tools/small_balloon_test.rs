use deckgym::{
    actions::SimpleAction, card_ids::CardId, database::get_card_by_enum, models::PlayedCard,
    test_support::get_test_game_with_board,
};

#[test]
fn test_small_balloon_lets_basic_retreat_for_free() {
    // Bulbasaur (A1 001) is a Basic with a retreat cost of 1. With Small Balloon attached and no
    // energy, it should be able to retreat for free.
    let game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_tool(get_card_by_enum(CardId::B3b064SmallBalloon)),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let (_, actions) = game.get_state_clone().generate_possible_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a.action, SimpleAction::Retreat(_))),
        "Small Balloon should reduce a Basic's retreat cost of 1 to 0"
    );

    // Control: without the tool, the same Bulbasaur cannot pay its retreat cost.
    let game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let (_, actions) = game.get_state_clone().generate_possible_actions();
    assert!(
        !actions
            .iter()
            .any(|a| matches!(a.action, SimpleAction::Retreat(_))),
        "Without Small Balloon, Caterpie should not be able to retreat with no energy"
    );
}

#[test]
fn test_small_balloon_attaches_to_evolved_pokemon_but_does_not_reduce_their_cost() {
    // The tool can be attached to any Pokemon (verified in-game)...
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1002Ivysaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let mut state = game.get_state_clone();
    state.hands[0].push(get_card_by_enum(CardId::B3b064SmallBalloon));
    game.set_state(state);

    let (_, actions) = game.get_state_clone().generate_possible_actions();
    assert!(
        actions.iter().any(|a| matches!(
            &a.action,
            SimpleAction::Play { trainer_card } if trainer_card.name == "Small Balloon"
        )),
        "Small Balloon should be attachable to non-Basic Pokemon"
    );

    // ...but the retreat reduction only applies to Basic Pokemon. Ivysaur has a retreat cost
    // of 2; with the balloon and 1 energy it still cannot retreat.
    let game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1002Ivysaur)
                .with_tool(get_card_by_enum(CardId::B3b064SmallBalloon))
                .with_energy(vec![deckgym::models::EnergyType::Grass]),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let (_, actions) = game.get_state_clone().generate_possible_actions();
    assert!(
        !actions
            .iter()
            .any(|a| matches!(a.action, SimpleAction::Retreat(_))),
        "Small Balloon should not reduce a non-Basic Pokemon's retreat cost"
    );
}
