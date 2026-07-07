use deckgym::{
    actions::SimpleAction, card_ids::CardId, database::get_card_by_enum, models::PlayedCard,
    test_support::get_test_game_with_board,
};

#[test]
fn test_elegant_cape_grants_30_hp_to_stage_1() {
    // Ivysaur (A1 002) is a Stage 1 with 90 HP; Elegant Cape should bring it to 120.
    let game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1002Ivysaur)
            .with_tool(get_card_by_enum(CardId::B3b065ElegantCape))],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(0).get_remaining_hp(),
        120,
        "Elegant Cape should grant +30 HP to the Stage 1 it is attached to"
    );
}

#[test]
fn test_elegant_cape_attaches_to_basic_but_grants_no_hp() {
    // The tool can be attached to any Pokemon (verified in-game)...
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let mut state = game.get_state_clone();
    state.hands[0].push(get_card_by_enum(CardId::B3b065ElegantCape));
    game.set_state(state);

    let (_, actions) = game.get_state_clone().generate_possible_actions();
    assert!(
        actions.iter().any(|a| matches!(
            &a.action,
            SimpleAction::Play { trainer_card } if trainer_card.name == "Elegant Cape"
        )),
        "Elegant Cape should be attachable to non-Stage 1 Pokemon"
    );

    // ...but the +30 HP only applies to Stage 1 Pokemon. Bulbasaur stays at 70 HP.
    let game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_tool(get_card_by_enum(CardId::B3b065ElegantCape))],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    assert_eq!(
        game.get_state_clone().get_active(0).get_remaining_hp(),
        70,
        "Elegant Cape should not grant HP to a non-Stage 1 Pokemon"
    );
}
