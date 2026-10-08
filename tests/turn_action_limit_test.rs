use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::PlayedCard,
    move_generation::MAX_ACTIONS_PER_TURN,
    test_support::get_test_game_with_board,
};

fn is_shadow_void(action: &Action) -> bool {
    matches!(action.action, SimpleAction::UseAbility { .. })
}

/// Two Dusknoir can move the same damage back and forth with Shadow Void ("As often as you
/// like during your turn") forever. A player that keeps choosing the ability must still be
/// forced to end its turn after MAX_ACTIONS_PER_TURN actions, like the app's turn timer.
#[test]
fn two_dusknoir_shadow_void_loop_is_cut_off_by_the_turn_action_limit() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A2072Dusknoir),
            PlayedCard::from_id(CardId::A2072Dusknoir).with_damage(20),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let player = game.get_state_clone().current_player;
    let mut uses = 0;
    loop {
        let state = game.get_state_clone();
        assert_eq!(state.current_player, player, "still the same turn");
        let (_, actions) = state.generate_possible_actions();
        // Prefer the ability whenever it is offered; otherwise resolve the pending choice.
        let action = match actions.iter().find(|a| is_shadow_void(a)) {
            Some(ability) => {
                uses += 1;
                ability.clone()
            }
            None if actions.iter().all(|a| a.is_stack) => actions[0].clone(),
            None => break,
        };
        game.apply_action(&action);
        assert!(
            uses <= MAX_ACTIONS_PER_TURN as usize,
            "the ability kept being offered"
        );
    }
    let state = game.get_state_clone();
    assert!(
        uses >= 10,
        "the ping-pong loop ran ({uses} uses) before being cut off"
    );
    assert!(state.actions_this_turn >= MAX_ACTIONS_PER_TURN);
    let (_, actions) = state.generate_possible_actions();
    assert!(actions
        .iter()
        .all(|a| matches!(a.action, SimpleAction::EndTurn | SimpleAction::Attack(_))));

    // Ending the turn resets the count for the next turn.
    game.apply_action(&Action {
        actor: player,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    assert_eq!(game.get_state_clone().actions_this_turn, 0);
}
