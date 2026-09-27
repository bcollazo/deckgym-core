//! Evolution timing, per the in-app Tips panel (transcribed 2026-09-08), under "Evolve your
//! Pokémon":
//!
//! - "You can't evolve a Pokémon on its first turn in play."
//! - "Neither player can evolve Pokémon on the first turn, whether they go first or second."

use deckgym::{
    actions::SimpleAction,
    card_ids::CardId,
    database::get_card_by_enum,
    models::PlayedCard,
    test_support::{get_initialized_game_with_board, get_test_game_with_board},
    Game,
};

/// A board where player `current_player` has a Bulbasaur in the Active Spot and an Ivysaur in
/// hand, on turn `turn_count`.
fn game_ready_to_evolve(
    current_player: usize,
    turn_count: u8,
    played_this_turn: bool,
) -> Game<'static> {
    let bulbasaur = PlayedCard::from_id(CardId::A1001Bulbasaur);
    let (board_0, board_1) = if current_player == 0 {
        (
            vec![bulbasaur],
            vec![PlayedCard::from_id(CardId::A1053Squirtle)],
        )
    } else {
        (
            vec![PlayedCard::from_id(CardId::A1053Squirtle)],
            vec![bulbasaur],
        )
    };

    let mut game = get_initialized_game_with_board(0, current_player, turn_count, board_0, board_1);
    let mut state = game.get_state_clone();
    if let Some(pokemon) = state.in_play_pokemon[current_player][0].as_mut() {
        pokemon.played_this_turn = played_this_turn;
    }
    state.hands[current_player].clear();
    state.hands[current_player].push(get_card_by_enum(CardId::A1002Ivysaur));
    game.set_state(state);
    game
}

fn offers_evolution(game: &Game<'static>) -> bool {
    let (_, actions) = game.get_state_clone().generate_possible_actions();
    actions
        .iter()
        .any(|a| matches!(a.action, SimpleAction::Evolve { .. }))
}

/// The control: on a later turn, with a Pokémon that has been in play since before this turn,
/// the evolution is offered. Without this the two blocking tests below would prove nothing.
#[test]
fn test_evolution_is_offered_on_a_later_turn_for_an_established_pokemon() {
    let game = game_ready_to_evolve(0, 3, false);
    assert!(
        offers_evolution(&game),
        "Bulbasaur that has been in play since a previous turn can evolve into Ivysaur"
    );
}

/// "Neither player can evolve Pokémon on the first turn, whether they go first or second."
/// Turn 1 is the first player's first turn, turn 2 the second player's.
#[test]
fn test_no_evolution_on_the_first_turn_for_the_player_who_goes_first() {
    let game = game_ready_to_evolve(0, 1, false);
    assert!(
        !offers_evolution(&game),
        "in-app Tips: no evolution on the first turn for the player who goes first"
    );
}

#[test]
fn test_no_evolution_on_the_first_turn_for_the_player_who_goes_second() {
    let game = game_ready_to_evolve(1, 2, false);
    assert!(
        !offers_evolution(&game),
        "in-app Tips: no evolution on the first turn for the player who goes second either"
    );
}

/// "You can't evolve a Pokémon on its first turn in play."
#[test]
fn test_no_evolution_of_a_pokemon_played_this_turn() {
    let game = game_ready_to_evolve(0, 5, true);
    assert!(
        !offers_evolution(&game),
        "in-app Tips: \"You can't evolve a Pokémon on its first turn in play.\""
    );
}

/// The same rule applies to a Benched Pokémon that was put down this turn, while an established
/// Benched Pokémon may still be evolved.
#[test]
fn test_first_turn_in_play_rule_applies_per_pokemon_not_per_board() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1053Squirtle),
            PlayedCard::from_id(CardId::A1001Bulbasaur), // Bench: just played
        ],
        vec![PlayedCard::from_id(CardId::A1053Squirtle)],
    );
    let mut state = game.get_state_clone();
    state.in_play_pokemon[0][1]
        .as_mut()
        .unwrap()
        .played_this_turn = true;
    state.hands[0].clear();
    state.hands[0].push(get_card_by_enum(CardId::A1002Ivysaur));
    game.set_state(state);

    assert!(
        !offers_evolution(&game),
        "a Bench Pokémon put down this turn cannot be evolved this turn either"
    );

    // Clear the flag: the same board, one turn later in effect, does offer the evolution.
    let mut state = game.get_state_clone();
    state.in_play_pokemon[0][1]
        .as_mut()
        .unwrap()
        .played_this_turn = false;
    game.set_state(state);

    assert!(
        offers_evolution(&game),
        "once it is no longer that Pokémon's first turn in play, the evolution is offered"
    );
}
