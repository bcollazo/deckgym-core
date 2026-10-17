//! Winning and losing, per the in-app Tips panel (transcribed 2026-09-08):
//!
//! - "If one player gets the set number of points for that battle or more before the other
//!   player, that player wins the battle."
//! - "If a player doesn't have any Pokémon remaining in play, that player loses the battle
//!   regardless of the number of points each player has."
//! - "A player will not lose even if they run out of cards in their deck."

use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{PlayedCard, StatusCondition},
    state::GameOutcome,
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

/// Both Actives sit at 10 HP and Poisoned, so the Checkup that ends the turn Knocks both out at
/// once and takes both players from 2 points to 3. `player_0_bench` / `player_1_bench` decide who
/// still has a Pokémon in play afterwards.
fn game_with_simultaneous_lethal_poison(
    player_0_bench: bool,
    player_1_bench: bool,
) -> Game<'static> {
    let mut board_0 = vec![PlayedCard::from_id(CardId::A1053Squirtle).with_remaining_hp(10)];
    if player_0_bench {
        board_0.push(PlayedCard::from_id(CardId::A1001Bulbasaur));
    }
    let mut board_1 = vec![PlayedCard::from_id(CardId::A1053Squirtle).with_remaining_hp(10)];
    if player_1_bench {
        board_1.push(PlayedCard::from_id(CardId::A1001Bulbasaur));
    }

    let mut game = get_test_game_with_board(board_0, board_1);
    let mut state = game.get_state_clone();
    state.points = [2, 2];
    state.apply_status_condition(0, 0, StatusCondition::Poisoned);
    state.apply_status_condition(1, 0, StatusCondition::Poisoned);
    game.set_state(state);
    game
}

#[test]
fn test_reaching_three_points_together_with_an_empty_board_is_a_loss_not_a_tie() {
    // Player 1 has nothing left in play; player 0 still has a Benched Bulbasaur.
    let mut game = game_with_simultaneous_lethal_poison(true, false);
    end_turn(&mut game);

    let state = game.get_state_clone();
    assert_eq!(state.points, [3, 3], "both Poison KOs award their point");
    assert_eq!(
        state.winner,
        Some(GameOutcome::Win(0)),
        "in-app Tips: \"If a player doesn't have any Pokémon remaining in play, that player \
         loses the battle regardless of the number of points each player has\" — reaching 3 on \
         the same action does not rescue an empty board into a tie"
    );
}

#[test]
fn test_reaching_three_points_together_with_an_empty_board_loses_for_either_player() {
    // The mirror image: player 0 is the one left with nothing.
    let mut game = game_with_simultaneous_lethal_poison(false, true);
    end_turn(&mut game);

    let state = game.get_state_clone();
    assert_eq!(state.points, [3, 3]);
    assert_eq!(
        state.winner,
        Some(GameOutcome::Win(1)),
        "the empty-board player loses whichever side they are"
    );
}

#[test]
fn test_reaching_three_points_together_is_still_a_tie_when_both_players_have_pokemon() {
    let mut game = game_with_simultaneous_lethal_poison(true, true);
    end_turn(&mut game);

    let state = game.get_state_clone();
    assert_eq!(state.points, [3, 3]);
    assert_eq!(
        state.winner,
        Some(GameOutcome::Tie),
        "with both boards still occupied the no-Pokémon rule cannot separate them, so the \
         simultaneous point win stands as a tie"
    );
}

#[test]
fn test_reaching_three_points_together_with_both_boards_empty_is_a_tie() {
    let mut game = game_with_simultaneous_lethal_poison(false, false);
    end_turn(&mut game);

    let state = game.get_state_clone();
    assert_eq!(state.points, [3, 3]);
    assert_eq!(
        state.winner,
        Some(GameOutcome::Tie),
        "neither player has Pokémon left, so the no-Pokémon rule condemns both equally"
    );
}

/// "A player will not lose even if they run out of cards in their deck."
#[test]
fn test_running_out_of_deck_does_not_lose_the_battle() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1053Squirtle)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let mut state = game.get_state_clone();
    state.decks[1].cards.clear();
    game.set_state(state);

    // Player 0 ends their turn; player 1 starts theirs and tries to draw from an empty deck.
    end_turn(&mut game);

    let state = game.get_state_clone();
    assert_eq!(
        state.current_player, 1,
        "play passes to the deck-less player"
    );
    assert!(
        state.winner.is_none(),
        "drawing from an empty deck must not end the battle"
    );
    assert!(
        state.decks[1].cards.is_empty(),
        "the deck really was empty for that draw step"
    );

    // And they can keep playing: the turn generates actions as normal.
    let (_, actions) = state.generate_possible_actions();
    assert!(
        !actions.is_empty(),
        "the deck-less player still takes their turn"
    );

    // Several more turns pass with the deck still empty and still no loss.
    for _ in 0..4 {
        end_turn(&mut game);
        assert!(
            game.get_state_clone().winner.is_none(),
            "repeated empty-deck draw steps must never decide the battle"
        );
    }
}
