use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::PlayedCard,
    test_support::get_test_game_with_board,
    Game,
};

fn find_action<F>(game: &Game, predicate: F) -> Action
where
    F: Fn(&Action) -> bool,
{
    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    actions
        .into_iter()
        .find(predicate)
        .expect("expected action to be available")
}

/// When Roaring Moon is placed on the bench, the player is offered a choice to use
/// Ancient Roar (force opponent's active to bench) or pass. If used, opponent chooses
/// which bench pokemon becomes the new active.
#[test]
fn test_ancient_roar_forces_opponent_active_to_bench() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![
            PlayedCard::from_id(CardId::A1053Squirtle),
            PlayedCard::from_id(CardId::A1057Psyduck),
        ],
    );
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.hands[0].clear();
    state.hands[0].push(get_card_by_enum(CardId::B3a047RoaringMoon));
    game.set_state(state);

    // Place Roaring Moon on bench
    let place_action = find_action(
        &game,
        |a| matches!(a.action, SimpleAction::Place(ref c, _) if c.get_name() == "Roaring Moon"),
    );
    game.apply_action(&place_action);

    // Player 0 should be offered UseAbility (Ancient Roar) or Noop
    let state = game.get_state_clone();
    let (_actor, actions) = state.generate_possible_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a.action, SimpleAction::UseAbility { .. })),
        "Ancient Roar should be offered as UseAbility after placing Roaring Moon on bench"
    );
    assert!(
        actions
            .iter()
            .any(|a| matches!(a.action, SimpleAction::Noop)),
        "Noop should be offered alongside Ancient Roar"
    );

    // Record the original opponent's active pokemon name
    let original_active_name = game.get_state_clone().get_active(1).get_name().to_string();

    // Use Ancient Roar (find the UseAbility action)
    let use_ability_action = actions
        .into_iter()
        .find(|a| matches!(a.action, SimpleAction::UseAbility { .. }))
        .unwrap();
    game.apply_action(&use_ability_action);

    // Now it's the OPPONENT's turn to choose a new active from their bench
    let state = game.get_state_clone();
    let (_actor, actions) = state.generate_possible_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a.action, SimpleAction::Activate { player: 1, .. })),
        "Opponent should be asked to choose a new active pokemon after Ancient Roar"
    );

    // Opponent promotes Psyduck (index 2) as new active
    let activate_action = actions
        .into_iter()
        .find(|a| matches!(a.action, SimpleAction::Activate { player: 1, .. }))
        .unwrap();
    game.apply_action(&activate_action);

    let final_state = game.get_state_clone();

    // The opponent's original active should now be on the bench
    let opponent_bench: Vec<_> = final_state.enumerate_bench_pokemon(1).collect();
    assert!(
        opponent_bench
            .iter()
            .any(|(_, p)| p.get_name() == original_active_name),
        "Opponent's original active ({original_active_name}) should now be on the bench"
    );
}

/// When Noop is chosen for Ancient Roar, opponent's active stays in place.
#[test]
fn test_ancient_roar_noop_leaves_state_unchanged() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![
            PlayedCard::from_id(CardId::A1053Squirtle),
            PlayedCard::from_id(CardId::A1057Psyduck),
        ],
    );
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.hands[0].clear();
    state.hands[0].push(get_card_by_enum(CardId::B3a047RoaringMoon));
    game.set_state(state);

    let original_active_name = game.get_state_clone().get_active(1).get_name().to_string();

    // Place Roaring Moon on bench
    let place_action = find_action(
        &game,
        |a| matches!(a.action, SimpleAction::Place(ref c, _) if c.get_name() == "Roaring Moon"),
    );
    game.apply_action(&place_action);

    // Choose Noop (don't use Ancient Roar)
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Noop,
        is_stack: true,
    });

    let final_state = game.get_state_clone();

    // Opponent's active should still be the same pokemon
    assert_eq!(
        final_state.get_active(1).get_name(),
        original_active_name,
        "Opponent's active should be unchanged when Ancient Roar is not used"
    );
}
