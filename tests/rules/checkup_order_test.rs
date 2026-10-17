//! Pokémon Checkup, per the in-app Tips panel (transcribed 2026-09-08):
//!
//! "a step that happens after the end of each turn. During Pokémon Checkup, the state of both
//! players' Pokémon is checked. ... Any Pokémon that has no HP remaining at the end of Pokémon
//! Checkup is Knocked Out. If both players' Pokémon are affected by Special Conditions, the
//! player whose turn just ended checks their Special Conditions first." Order for one Pokémon
//! with several: 1. Poisoned 2. Burned 3. Asleep 4. Paralyzed.

use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{PlayedCard, StatusCondition},
    test_support::get_test_game_with_board,
    Game,
};

fn end_turn(game: &mut Game<'static>) {
    let actor = game.get_state_clone().current_player;
    game.apply_action(&Action {
        actor,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    game.play_until_stable();
}

/// A Pokémon carrying both Poisoned and Burned takes each condition's damage once, in the same
/// Checkup: 10 for the Poison and 20 for the Burn.
#[test]
fn test_poison_then_burn_both_resolve_for_the_same_pokemon_in_one_checkup() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1035Charizard)], // 150 HP, survives comfortably
    );
    let mut state = game.get_state_clone();
    state.apply_status_condition(1, 0, StatusCondition::Poisoned);
    state.apply_status_condition(1, 0, StatusCondition::Burned);
    game.set_state(state);

    end_turn(&mut game);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_remaining_hp(1, 0),
        120,
        "Charizard should take 10 from Poison and 20 from Burn in the same Checkup"
    );
}

/// "During Pokémon Checkup, the state of **both** players' Pokémon is checked." One Checkup
/// resolves the conditions on either side of the board, not just the ending player's.
#[test]
fn test_both_players_conditions_resolve_in_a_single_checkup() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1035Charizard)],
        vec![PlayedCard::from_id(CardId::A1035Charizard)],
    );
    let mut state = game.get_state_clone();
    state.apply_status_condition(0, 0, StatusCondition::Poisoned);
    state.apply_status_condition(1, 0, StatusCondition::Poisoned);
    game.set_state(state);

    end_turn(&mut game);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_remaining_hp(0, 0),
        140,
        "the ending player's own Poisoned Pokémon is checked too"
    );
    assert_eq!(
        state.get_remaining_hp(1, 0),
        140,
        "the opponent's Poisoned Pokémon is checked in the same Checkup"
    );
}

/// "Any Pokémon that has no HP remaining at the end of Pokémon Checkup is Knocked Out." Both
/// sides' Knock Outs are therefore resolved together, and each player scores their point — a
/// Checkup KO on one side does not short-circuit the other.
#[test]
fn test_simultaneous_checkup_knockouts_both_score() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1053Squirtle).with_remaining_hp(10),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![
            PlayedCard::from_id(CardId::A1053Squirtle).with_remaining_hp(10),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );
    let mut state = game.get_state_clone();
    state.apply_status_condition(0, 0, StatusCondition::Poisoned);
    state.apply_status_condition(1, 0, StatusCondition::Poisoned);
    game.set_state(state);

    end_turn(&mut game);

    let state = game.get_state_clone();
    assert_eq!(
        state.points,
        [1, 1],
        "both Poison Knock Outs land at the end of the same Checkup, so both players score"
    );
}

/// Burn's recovery coin is flipped for the Pokémon that is Burned, after its Burn damage — the
/// damage lands whether or not the flip comes up heads.
#[test]
fn test_burn_damage_lands_before_its_recovery_flip() {
    // Sweep seeds so both coin branches are exercised; the damage assertion holds either way.
    let mut saw_recovery = false;
    let mut saw_still_burned = false;
    for seed in 0..30u64 {
        let mut game = deckgym::test_support::get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
            vec![PlayedCard::from_id(CardId::A1035Charizard)],
        );
        let mut state = game.get_state_clone();
        state.apply_status_condition(1, 0, StatusCondition::Burned);
        game.set_state(state);

        end_turn(&mut game);

        let state = game.get_state_clone();
        assert_eq!(
            state.get_remaining_hp(1, 0),
            130,
            "Burn always deals its 20 during Checkup (seed {seed})"
        );
        if state.get_active(1).is_burned() {
            saw_still_burned = true;
        } else {
            saw_recovery = true;
        }
    }
    assert!(saw_recovery, "expected a heads flip to clear the Burn");
    assert!(saw_still_burned, "expected a tails flip to keep the Burn");
}
