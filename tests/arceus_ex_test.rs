use common::get_initialized_game;
use deckgym::{
    card_ids::CardId,
    models::{PlayedCard, StatusCondition},
};

mod common;

#[test]
fn test_arceus_ex_fabled_luster_immunity() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Test the original Arceus ex
    let arceus_original =
        PlayedCard::from_id(CardId::A2a071ArceusEx).with_status(StatusCondition::Poisoned);
    assert!(
        !arceus_original.poisoned,
        "Original Arceus ex should be immune to poison"
    );

    // Test the new Arceus ex reprint (A4b 299)
    let arceus_reprint =
        PlayedCard::from_id(CardId::A4b299ArceusEx).with_status(StatusCondition::Confused);
    assert!(
        !arceus_reprint.confused,
        "Reprinted Arceus ex should be immune to confusion"
    );

    // Test A4b 372 Arceus ex
    let arceus_a4b372 =
        PlayedCard::from_id(CardId::A4b372ArceusEx).with_status(StatusCondition::Asleep);
    assert!(
        !arceus_a4b372.asleep,
        "A4b 372 Arceus ex should be immune to sleep"
    );

    // Test B1 328 Arceus ex
    let arceus_b1328 =
        PlayedCard::from_id(CardId::B1328ArceusEx).with_status(StatusCondition::Paralyzed);
    assert!(
        !arceus_b1328.paralyzed,
        "B1 328 Arceus ex should be immune to paralysis"
    );

    // Check in-game state
    state.set_board(
        vec![PlayedCard::from_id(CardId::A4b299ArceusEx)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    game.set_state(state);

    let active = game
        .get_state_clone()
        .get_active(0)
        .clone()
        .with_status(StatusCondition::Burned);

    assert!(
        !active.burned,
        "Arceus ex in game state should be immune to burn"
    );
}
