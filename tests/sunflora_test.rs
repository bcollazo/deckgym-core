use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

/// Test Sunflora B1a 008 - Quick-Grow Beam
/// Should deal 30 damage, or 60 if Quick-Grow Extract is in discard pile
#[test]
fn test_sunflora_quick_grow_beam_without_extract() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![PlayedCard::from_id(CardId::B1a008Sunflora).with_energy(vec![EnergyType::Grass])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    // No Quick-Grow Extract in discard pile
    state.discard_piles[0] = vec![];

    game.set_state(state);

    let action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };

    game.apply_action(&action);
    let state = game.get_state_clone();

    // Should deal only 30 damage (no bonus)
    let opponent_active = state.get_active(1);
    assert_eq!(
        opponent_active.get_remaining_hp(),
        40,
        "Opponent should have 40 HP remaining (70 - 30)"
    );
}

#[test]
fn test_sunflora_quick_grow_beam_with_extract() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![PlayedCard::from_id(CardId::B1a008Sunflora).with_energy(vec![EnergyType::Grass])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    // Put Quick-Grow Extract in discard pile
    state.discard_piles[0] = vec![get_card_by_enum(CardId::B1a067QuickGrowExtract)];

    game.set_state(state);

    let action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };

    game.apply_action(&action);
    let state = game.get_state_clone();

    // Should deal 30 + 30 = 60 damage
    let opponent_active = state.get_active(1);
    assert_eq!(
        opponent_active.get_remaining_hp(),
        10,
        "Opponent should have 10 HP remaining (70 - 60)"
    );
}
