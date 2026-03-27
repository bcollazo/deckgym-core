// Tests for Darkrai (B2b 040) "Bad Dreams" ability:
// "At the end of each turn, if your opponent's Active Pokémon is Asleep, do 20 damage to that Pokémon."

use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{PlayedCard, StatusCondition},
    test_support::get_initialized_game,
};

#[test]
fn test_bad_dreams_deals_damage_when_opponent_asleep() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Darkrai (Bad Dreams) active for player 0; Caterpie (50 HP) asleep for player 1.
    state.set_board(
        vec![PlayedCard::from_id(CardId::B2b040Darkrai)],
        vec![PlayedCard::from_id(CardId::A1005Caterpie)],
    );
    state.apply_status_condition(1, 0, StatusCondition::Asleep);
    state.current_player = 0;
    state.turn_count = 3;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });

    // Bad Dreams fires during on_end_turn, before the sleep coin flip.
    // Caterpie should have taken 20 damage regardless of coin flip outcome (50 - 20 = 30).
    let hp_after = game.get_state_clone().get_active(1).get_remaining_hp();
    assert_eq!(
        hp_after, 30,
        "Bad Dreams should deal 20 damage to the Asleep opponent (50 - 20 = 30)"
    );
}

#[test]
fn test_bad_dreams_no_damage_when_not_asleep() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::B2b040Darkrai)],
        vec![PlayedCard::from_id(CardId::A1005Caterpie)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });

    let hp_after = game.get_state_clone().get_active(1).get_remaining_hp();
    assert_eq!(
        hp_after, 50,
        "Bad Dreams should not deal damage when the opponent is not Asleep"
    );
}

#[test]
fn test_bad_dreams_triggers_at_end_of_each_turn() {
    // Darkrai belongs to player 1; player 0 ends their turn with an Asleep active.
    // The ability says "at the end of EACH turn", so it fires on the opponent's turn too.
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1005Caterpie)],
        vec![PlayedCard::from_id(CardId::B2b040Darkrai)],
    );
    state.apply_status_condition(0, 0, StatusCondition::Asleep);
    state.current_player = 0;
    state.turn_count = 3;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });

    // Player 1's Darkrai should have fired Bad Dreams against player 0's Asleep Caterpie.
    // Caterpie (50 HP) should have 30 HP remaining.
    let hp_after = game.get_state_clone().get_active(0).get_remaining_hp();
    assert_eq!(
        hp_after, 30,
        "Bad Dreams should fire at the end of each turn, including the opponent's turn (50 - 20 = 30)"
    );
}

#[test]
fn test_bad_dreams_ko_during_end_turn_queues_promotion() {
    // Corner case: Bad Dreams KOs the opponent's Active Pokémon during end-turn processing.
    // The opponent must promote a benched Pokémon.
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::B2b040Darkrai)],
        vec![
            // Caterpie with only 10 HP left and Asleep — Bad Dreams (20 dmg) will KO it.
            PlayedCard::from_id(CardId::A1005Caterpie).with_remaining_hp(10),
            // Bench Pokémon that must be promoted after the KO.
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );
    state.apply_status_condition(1, 0, StatusCondition::Asleep);
    state.current_player = 0;
    state.turn_count = 3;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });

    // Check immediate KO effects before resolving remaining forced actions.
    let state_after = game.get_state_clone();
    assert!(
        state_after.in_play_pokemon[1][0].is_none(),
        "Caterpie with 10 HP should be knocked out by Bad Dreams"
    );
    assert_eq!(
        state_after.points[0], 1,
        "Player 0 should earn 1 point for the KO"
    );

    // EndTurn also queues a draw for the next player. Let the engine resolve the draw
    // and the forced promotion (only one bench Pokémon, so no real choice).
    game.play_until_stable();

    // Bulbasaur should have been promoted to active for player 1.
    let active_name = game.get_state_clone().get_active(1).get_name();
    assert_eq!(
        active_name, "Bulbasaur",
        "Bulbasaur should have been promoted after Caterpie was KO'd by Bad Dreams"
    );
}

#[test]
fn test_bad_dreams_multiple_darkrai_each_trigger_independently() {
    // Corner case: Two Darkrai in play for the same player each trigger Bad Dreams.
    // Combined damage should be 40 (2 × 20).
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B2b040Darkrai), // active
            PlayedCard::from_id(CardId::B2b040Darkrai), // bench
        ],
        vec![PlayedCard::from_id(CardId::A1005Caterpie)],
    );
    state.apply_status_condition(1, 0, StatusCondition::Asleep);
    state.current_player = 0;
    state.turn_count = 3;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });

    // Both Darkrai trigger: 2 × 20 = 40 damage. Caterpie (50 HP) → 10 HP remaining.
    let hp_after = game.get_state_clone().get_active(1).get_remaining_hp();
    assert_eq!(
        hp_after, 10,
        "Two Darkrai should each deal 20 damage independently (50 - 40 = 10)"
    );
}
