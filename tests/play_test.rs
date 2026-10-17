//! Integration test for the `play` public API (the same path the CLI's `play` subcommand uses) —
//! see `docs/replay-viewer-plan.md`'s "Round 4" notes and `tests/replay_test.rs` for the sibling
//! test covering `simulate`'s replay recording.

use std::fs;

use deckgym::players::PlayerCode;
use deckgym::replay::{Replay, REPLAY_FORMAT_VERSION};
use deckgym::{play, PlayConfig};
use uuid::Uuid;

fn temp_replay_dir(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("deckgym-play-it-{name}-{}", Uuid::new_v4()))
}

#[test]
fn play_writes_exactly_one_replay_file_matching_its_result() {
    let dir = temp_replay_dir("basic");

    let result = play(
        "example_decks/venusaur-exeggutor.txt",
        "example_decks/weezing-arbok.txt",
        PlayConfig {
            players: Some(vec![PlayerCode::R, PlayerCode::R]),
            seed: Some(42),
            bot_a: None,
            bot_b: None,
            bot_timeout_ms: deckgym::simulate::DEFAULT_BOT_TIMEOUT_MS,
            replay_dir: dir.to_string_lossy().to_string(),
        },
    );

    // Exactly one replay file, named after the result's own game id.
    let entries: Vec<_> = fs::read_dir(&dir)
        .expect("replay dir should exist")
        .map(|e| e.expect("dir entry").path())
        .collect();
    assert_eq!(entries.len(), 1, "play() writes exactly one replay file");
    assert_eq!(entries[0], result.replay_path);
    assert_eq!(
        result.replay_path,
        dir.join(format!("{}.json", result.game_id))
    );

    // The file itself is a well-formed replay, consistent with the result play() returned.
    let contents = fs::read_to_string(&result.replay_path).expect("replay file should be readable");
    let replay: Replay = serde_json::from_str(&contents).expect("replay file should deserialize");
    assert_eq!(replay.version, REPLAY_FORMAT_VERSION);
    assert_eq!(replay.game_id, result.game_id.to_string());
    assert!(!replay.steps.is_empty());
    assert_eq!(replay.players[0].deck.len(), 20);
    assert_eq!(replay.players[1].deck.len(), 20);

    let final_state = replay.final_state.as_ref().expect("final_state recorded");
    assert_eq!(final_state.winner, result.outcome);
    assert_eq!(final_state.points, result.points);
    assert_eq!(
        result.player_names,
        [
            replay.players[0].name.clone(),
            replay.players[1].name.clone()
        ]
    );

    fs::remove_dir_all(&dir).ok();
}

#[test]
fn play_defaults_to_two_random_players_when_none_given() {
    let dir = temp_replay_dir("default-players");

    let result = play(
        "example_decks/venusaur-exeggutor.txt",
        "example_decks/weezing-arbok.txt",
        PlayConfig {
            players: None,
            seed: Some(7),
            bot_a: None,
            bot_b: None,
            bot_timeout_ms: deckgym::simulate::DEFAULT_BOT_TIMEOUT_MS,
            replay_dir: dir.to_string_lossy().to_string(),
        },
    );

    assert!(result.replay_path.exists());
    // A 30-turn cap always ends with an outcome for two built-in players.
    assert!(result.outcome.is_some());

    fs::remove_dir_all(&dir).ok();
}
