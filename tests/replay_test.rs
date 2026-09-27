//! Integration tests for replay recording through the public simulation API (the same path the
//! CLI's `--replay-dir` / `--replay-sample` flags use). See `docs/replay-viewer-plan.md`.

use std::fs;

use deckgym::optimize::{ParallelConfig, SimulationConfig};
use deckgym::players::PlayerCode;
use deckgym::replay::{Replay, REPLAY_FORMAT_VERSION};
use deckgym::simulate;
use uuid::Uuid;

fn temp_replay_dir(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("deckgym-replay-it-{name}-{}", Uuid::new_v4()))
}

fn read_replays(dir: &std::path::Path) -> Vec<Replay> {
    fs::read_dir(dir)
        .expect("replay dir should exist")
        .map(|entry| {
            let path = entry.expect("dir entry").path();
            let contents = fs::read_to_string(&path).expect("replay file should be readable");
            serde_json::from_str::<Replay>(&contents).expect("replay file should deserialize")
        })
        .collect()
}

#[test]
fn simulate_with_replay_dir_writes_one_replay_per_game() {
    let dir = temp_replay_dir("basic");

    simulate(
        "example_decks/venusaur-exeggutor.txt",
        "example_decks/weezing-arbok.txt",
        SimulationConfig {
            num_games: 3,
            players: Some(vec![PlayerCode::R, PlayerCode::R]),
            seed: Some(42),
            data_output: None,
            replay_dir: Some(dir.to_string_lossy().to_string()),
            replay_sample: None,
        },
        ParallelConfig::default(),
    );

    let replays = read_replays(&dir);
    assert_eq!(replays.len(), 3, "one replay file per game");

    for replay in &replays {
        assert_eq!(replay.version, REPLAY_FORMAT_VERSION);
        assert!(!replay.steps.is_empty());
        assert_eq!(replay.players[0].deck.len(), 20);
        assert_eq!(replay.players[1].deck.len(), 20);
        assert!(!replay.cards.is_empty());

        // Steps are in order, each `chosen` index is valid, and every option's `text` matches
        // its action's Display (the recorder derives one from the other).
        for (i, step) in replay.steps.iter().enumerate() {
            assert_eq!(step.ply as usize, i);
            assert!(step.chosen < step.options.len());
            for option in &step.options {
                assert_eq!(option.text, option.action.to_string());
            }
        }

        // The final snapshot's winner matches the recorded outcome.
        let final_state = replay.final_state.as_ref().expect("final_state recorded");
        assert_eq!(final_state.winner, replay.outcome);
        assert!(
            replay.outcome.is_some(),
            "a 30-turn cap always ends with an outcome"
        );
    }

    fs::remove_dir_all(&dir).ok();
}

#[test]
fn replay_sample_caps_how_many_games_get_a_replay_file() {
    let dir = temp_replay_dir("sample");

    simulate(
        "example_decks/venusaur-exeggutor.txt",
        "example_decks/weezing-arbok.txt",
        SimulationConfig {
            num_games: 5,
            players: Some(vec![PlayerCode::R, PlayerCode::R]),
            seed: Some(7),
            data_output: None,
            replay_dir: Some(dir.to_string_lossy().to_string()),
            replay_sample: Some(2),
        },
        ParallelConfig::default(),
    );

    let replays = read_replays(&dir);
    assert_eq!(replays.len(), 2, "--replay-sample caps the written files");

    fs::remove_dir_all(&dir).ok();
}
