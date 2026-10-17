//! Integration test for the external-bot protocol (`docs/bot-protocol.md`): plays a full game
//! with one seat driven by `examples/bots/random_bot.py` over stdin/stdout, through the public
//! `Game`/`Simulation` API. Gated on `python3` being available, since CI environments vary.

use std::process::Command;
use std::time::Duration;

use deckgym::players::{create_players_with_bots, PlayerCode};
use deckgym::state::GameOutcome;
use deckgym::test_support::load_test_decks;
use deckgym::Game;

fn python3_available() -> bool {
    Command::new("python3")
        .arg("--version")
        .output()
        .is_ok_and(|o| o.status.success())
}

#[test]
fn plays_a_full_game_against_the_example_random_bot() {
    if !python3_available() {
        eprintln!("Skipping: python3 not available");
        return;
    }

    let (deck_a, deck_b) = load_test_decks();
    let players = create_players_with_bots(
        deck_a,
        deck_b,
        vec![PlayerCode::R, PlayerCode::X],
        [
            None,
            Some("python3 examples/bots/random_bot.py".to_string()),
        ],
        Duration::from_secs(5),
    );

    let mut game = Game::new(players, 1234);
    let outcome = game.play();

    // A 30-turn cap always resolves to Some(...) (Win or Tie) — see State::advance_turn.
    assert!(outcome.is_some(), "game should reach an outcome");
    if let Some(GameOutcome::Win(winner)) = outcome {
        assert!(winner == 0 || winner == 1);
    }
}
