use crate::deckgym::{
    card_ids::CardId,
    models::{PlayedCard, StatusCondition},
    test_support::get_initialized_game,
};

#[test]
fn test_arceus_ex_fabled_luster_immunity() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Test the original Arceus ex
    state.set_board(
        vec![PlayedCard::from_id(CardId::A2a071ArceusEx)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.apply_status_condition(0, 0, StatusCondition::Poisoned);
    assert!(
        !state.get_active(0).is_poisoned(),
        "Original Arceus ex should be immune to poison"
    );

    // Test the new Arceus ex reprint (A4b 299)
    state.set_board(
        vec![PlayedCard::from_id(CardId::A4b299ArceusEx)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.apply_status_condition(0, 0, StatusCondition::Confused);
    assert!(
        !state.get_active(0).is_confused(),
        "Reprinted Arceus ex should be immune to confusion"
    );

    // Test A4b 372 Arceus ex
    state.set_board(
        vec![PlayedCard::from_id(CardId::A4b372ArceusEx)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.apply_status_condition(0, 0, StatusCondition::Asleep);
    assert!(
        !state.get_active(0).is_asleep(),
        "A4b 372 Arceus ex should be immune to sleep"
    );

    // Test B1 328 Arceus ex
    state.set_board(
        vec![PlayedCard::from_id(CardId::B1328ArceusEx)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.apply_status_condition(0, 0, StatusCondition::Paralyzed);
    assert!(
        !state.get_active(0).is_paralyzed(),
        "B1 328 Arceus ex should be immune to paralysis"
    );

    // Check in-game state
    state.set_board(
        vec![PlayedCard::from_id(CardId::A4b299ArceusEx)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    game.set_state(state);

    let mut state = game.get_state_clone();
    state.apply_status_condition(0, 0, StatusCondition::Burned);
    assert!(
        !state.get_active(0).is_burned(),
        "Arceus ex in game state should be immune to burn"
    );
}
