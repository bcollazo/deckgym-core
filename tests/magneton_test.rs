use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
};

mod common;

#[test]
fn test_magneton_volt_charge_attaches_lightning_energy() {
    // Arrange: Create a game with Magneton in play
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Setup: Put Magneton on the bench (index 1) for player 0
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1098Magneton),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    game.set_state(state);

    // Verify initial energy state
    let state = game.get_state_clone();
    let magneton_before = state.enumerate_bench_pokemon(0).next().unwrap().1;
    assert_eq!(
        magneton_before.attached_energy.len(),
        0,
        "Magneton should start with no energy"
    );

    // Act: Use Magneton's Volt Charge ability
    let action = Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 1 },
        is_stack: false,
    };
    game.apply_action(&action);

    // Assert: Magneton should now have 1 Lightning energy attached
    let state = game.get_state_clone();
    let magneton_after = state.enumerate_bench_pokemon(0).next().unwrap().1;
    assert_eq!(
        magneton_after.attached_energy.len(),
        1,
        "Magneton should have 1 energy after using Volt Charge"
    );
    assert_eq!(
        magneton_after.attached_energy[0],
        EnergyType::Lightning,
        "The attached energy should be Lightning type"
    );
}

#[test]
fn test_magneton_volt_charge_can_only_be_used_once() {
    // Arrange: Create a game with Magneton in play
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Setup: Put Magneton in active spot (index 0) for player 0
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1098Magneton)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    game.set_state(state);

    // Act: Use Magneton's Volt Charge ability first time
    let action = Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    };
    game.apply_action(&action);

    // Assert: ability_used should be set to true
    let state = game.get_state_clone();
    let magneton_after_first_use = state.get_active(0);
    assert!(
        magneton_after_first_use.ability_used,
        "Magneton's ability_used should be true after first use"
    );

    // Verify the ability is not in the available actions anymore
    let (_actor, available_actions) = state.generate_possible_actions();
    let ability_actions: Vec<_> = available_actions
        .iter()
        .filter(|a| matches!(a.action, SimpleAction::UseAbility { in_play_idx: 0 }))
        .collect();
    assert_eq!(
        ability_actions.len(),
        0,
        "Volt Charge should not be available after being used once"
    );
}

#[test]
fn test_magneton_volt_charge_doesnt_end_turn() {
    // Arrange: Create a game with Magneton in play
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Setup: Put Magneton in active spot for player 0
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1098Magneton)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    game.set_state(state);

    // Act: Use Magneton's Volt Charge ability
    let action = Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    };
    game.apply_action(&action);

    // Fetch available actions after ability resolves
    game.play_until_stable();
    let state = game.get_state_clone();

    // Assert: Current player should still be 0 (turn doesn't end)
    assert_eq!(
        state.current_player, 0,
        "Turn should not end after using Volt Charge (unlike Giratina ex)"
    );

    // Verify that other actions are still available (like EndTurn)
    let (_actor, available_actions) = state.generate_possible_actions();
    let has_end_turn = available_actions
        .iter()
        .any(|a| matches!(a.action, SimpleAction::EndTurn));
    assert!(
        has_end_turn,
        "EndTurn should be available after using Volt Charge"
    );
}
