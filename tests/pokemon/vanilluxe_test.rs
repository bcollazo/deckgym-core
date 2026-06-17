use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
    Game,
};

#[test]
fn test_vanilluxe_sweets_relay_no_bonus_without_prior_use() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B2039Vanilluxe)
            .with_energy(vec![EnergyType::Water, EnergyType::Water])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2039Vanilluxe, 0),
        is_stack: false,
    });

    let final_state = game.get_state_clone();
    assert_eq!(
        final_state.get_active(1).get_remaining_hp(),
        10,
        "Sweets Relay should only deal its base 60 damage with no prior Sweets Relay use"
    );
}

/// Drives the game forward (auto-ending every other player's turn, and resolving any forced
/// single-choice actions like the post-turn draw) until it's `target_player`'s turn again with a
/// clean decision point (no pending move-generation stack).
fn advance_to_players_next_turn(game: &mut Game, target_player: usize) {
    loop {
        let state = game.get_state_clone();
        let (_actor, actions) = state.generate_possible_actions();
        let only_forced_end_turn =
            actions.len() == 1 && matches!(actions[0].action, SimpleAction::EndTurn);
        if state.current_player == target_player
            && state.move_generation_stack.is_empty()
            && !only_forced_end_turn
        {
            break;
        }
        let action = actions
            .iter()
            .find(|action| matches!(action.action, SimpleAction::EndTurn))
            .unwrap_or(&actions[0])
            .clone();
        game.apply_action(&action);
    }
}

/// Resets a player's active Pokemon back to full HP, without touching any other state (in
/// particular, doesn't touch the "attack used during last turn" bookkeeping).
fn heal_active_to_full(game: &mut Game, player: usize, total_hp: u32) {
    let mut state = game.get_state_clone();
    let healed = state.in_play_pokemon[player][0]
        .take()
        .expect("active Pokemon should be there")
        .with_remaining_hp(total_hp);
    state.in_play_pokemon[player][0] = Some(healed);
    game.set_state(state);
}

#[test]
fn test_vanilluxe_sweets_relay_gets_bonus_after_own_last_turn_use_multi_turn() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B2039Vanilluxe)
            .with_energy(vec![EnergyType::Water, EnergyType::Water])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    game.set_state(state);

    // Turn 1: no Sweets Relay used yet, so only the base 60 damage applies.
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2039Vanilluxe, 0),
        is_stack: false,
    });
    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        90,
        "First Sweets Relay use should deal only the base 60 damage"
    );

    // Play through the opponent's turn (no Sweets Relay use happens for them) and get back to
    // player 0's turn.
    advance_to_players_next_turn(&mut game, 0);
    heal_active_to_full(&mut game, 1, 150);

    // Turn 2 (player 0's next own turn): Sweets Relay was used during their last turn, so this
    // attack should now deal the base 60 + 60 bonus damage.
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2039Vanilluxe, 0),
        is_stack: false,
    });
    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        30,
        "Sweets Relay should deal 60 base + 60 bonus damage after a prior own-turn use"
    );
}

#[test]
fn test_vanilluxe_sweets_relay_bonus_does_not_stack_across_turns() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B2039Vanilluxe)
            .with_energy(vec![EnergyType::Water, EnergyType::Water])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    game.set_state(state);

    // Turn 1: base damage only.
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2039Vanilluxe, 0),
        is_stack: false,
    });
    advance_to_players_next_turn(&mut game, 0);
    heal_active_to_full(&mut game, 1, 150);

    // Turn 2: gets the bonus from turn 1's use.
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2039Vanilluxe, 0),
        is_stack: false,
    });
    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        30,
        "Second consecutive Sweets Relay use should get the bonus"
    );
    advance_to_players_next_turn(&mut game, 0);
    heal_active_to_full(&mut game, 1, 150);

    // Turn 3: still only a single +60 bonus, not a stacked +120, even though Sweets Relay has
    // now been used on every prior own turn.
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2039Vanilluxe, 0),
        is_stack: false,
    });
    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        30,
        "Sweets Relay bonus should not stack across repeated own-turn uses"
    );
}
