use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
    Game,
};

#[test]
fn test_alcremie_sweets_overload_deals_no_damage_without_prior_sweets_relay_use() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.set_board(
        vec![PlayedCard::from_id(CardId::A3b037Alcremie).with_energy(vec![EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A3b037Alcremie, 0),
        is_stack: false,
    });

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        70,
        "Sweets Overload should deal no damage when Sweets Relay was never used this game"
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
/// particular, doesn't touch the Sweets Relay usage count tracked on state).
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
fn test_alcremie_sweets_overload_scales_with_sweets_relay_uses_this_game() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B2039Vanilluxe)
            .with_energy(vec![EnergyType::Water, EnergyType::Water])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    game.set_state(state);

    // Use Sweets Relay on two separate own-turns, so it has been used twice this game.
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2039Vanilluxe, 0),
        is_stack: false,
    });
    advance_to_players_next_turn(&mut game, 0);
    heal_active_to_full(&mut game, 1, 150);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2039Vanilluxe, 0),
        is_stack: false,
    });
    advance_to_players_next_turn(&mut game, 0);

    // Swap Alcremie into the active slot (preserving the Sweets Relay usage count tracked on
    // state) and use Sweets Overload.
    let mut state = game.get_state_clone();
    state.set_board(
        vec![PlayedCard::from_id(CardId::A3b037Alcremie).with_energy(vec![EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A3b037Alcremie, 0),
        is_stack: false,
    });

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        70,
        "Sweets Overload should deal 40 damage for each of the 2 prior Sweets Relay uses this game"
    );
}
