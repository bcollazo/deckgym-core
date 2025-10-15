use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    generate_possible_actions,
    models::{EnergyType, PlayedCard},
};

mod common;

#[test]
fn test_magneton_volt_charge_attaches_lightning_energy() {
    // Arrange: Create a game with Magneton in play
    let magneton_card = get_card_by_enum(CardId::A1098Magneton);
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    let current_player = state.current_player;

    // Setup: Put Magneton on the bench (index 1) for current player
    let magneton = PlayedCard::new(
        magneton_card.clone(),
        100,    // remaining_hp
        100,    // total_hp
        vec![], // No energy initially
        false,
        vec![],
    );
    state.in_play_pokemon[current_player][1] = Some(magneton);
    game.set_state(state);

    // Verify initial energy state
    let state = game.get_state_clone();
    let magneton_before = state.in_play_pokemon[current_player][1]
        .as_ref()
        .expect("Magneton should be in play");
    assert_eq!(
        magneton_before.attached_energy.len(),
        0,
        "Magneton should start with no energy"
    );

    // Act: Use Magneton's Volt Charge ability
    let action = Action {
        actor: current_player,
        action: SimpleAction::UseAbility(1),
        is_stack: false,
    };
    game.apply_action(&action);

    // Assert: Magneton should now have 1 Lightning energy attached
    let state = game.get_state_clone();
    let magneton_after = state.in_play_pokemon[current_player][1]
        .as_ref()
        .expect("Magneton should still be in play");
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
    let magneton_card = get_card_by_enum(CardId::A1098Magneton);
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    let current_player = state.current_player;

    // Setup: Put Magneton in active spot (index 0) for current player
    let magneton = PlayedCard::new(
        magneton_card.clone(),
        100,    // remaining_hp
        100,    // total_hp
        vec![], // No energy initially
        false,
        vec![],
    );
    state.in_play_pokemon[current_player][0] = Some(magneton);
    game.set_state(state);

    // Act: Use Magneton's Volt Charge ability first time
    let action = Action {
        actor: current_player,
        action: SimpleAction::UseAbility(0),
        is_stack: false,
    };
    game.apply_action(&action);

    // Assert: ability_used should be set to true
    let state = game.get_state_clone();
    let magneton_after_first_use = state.in_play_pokemon[current_player][0]
        .as_ref()
        .expect("Magneton should be in play");
    assert!(
        magneton_after_first_use.ability_used,
        "Magneton's ability_used should be true after first use"
    );

    // Verify the ability is not in the available actions anymore
    let (_actor, available_actions) = generate_possible_actions(&state);
    let ability_actions: Vec<_> = available_actions
        .iter()
        .filter(|a| matches!(a.action, SimpleAction::UseAbility(0)))
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
    let magneton_card = get_card_by_enum(CardId::A1098Magneton);
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    let current_player = state.current_player;

    // Setup: Put Magneton in active spot for current player
    let magneton = PlayedCard::new(
        magneton_card.clone(),
        100,    // remaining_hp
        100,    // total_hp
        vec![], // No energy initially
        false,
        vec![],
    );
    state.in_play_pokemon[current_player][0] = Some(magneton);
    game.set_state(state);

    // Act: Use Magneton's Volt Charge ability
    let action = Action {
        actor: current_player,
        action: SimpleAction::UseAbility(0),
        is_stack: false,
    };
    game.apply_action(&action);

    // Process any stack items from the ability
    let mut state = game.get_state_clone();
    while !state.move_generation_stack.is_empty() {
        let (_actor, actions) = generate_possible_actions(&state);
        if !actions.is_empty() {
            game.apply_action(&actions[0]);
            state = game.get_state_clone();
        } else {
            break;
        }
    }

    // Assert: Current player should still be the same (turn doesn't end)
    assert_eq!(
        state.current_player, current_player,
        "Turn should not end after using Volt Charge (unlike Giratina ex)"
    );

    // Verify that other actions are still available (like EndTurn)
    let (_actor, available_actions) = generate_possible_actions(&state);
    let has_end_turn = available_actions
        .iter()
        .any(|a| matches!(a.action, SimpleAction::EndTurn));
    assert!(
        has_end_turn,
        "EndTurn should be available after using Volt Charge"
    );
}
