use std::{
    collections::BTreeMap,
    fs,
    path::PathBuf,
    sync::{atomic::AtomicUsize, atomic::Ordering, Arc},
};

use log::warn;
use uuid::Uuid;

use crate::{
    actions::Action,
    models::Card,
    replay::{
        Replay, ReplayOption, ReplayPlayerInfo, ReplayStep, ViewState, REPLAY_FORMAT_VERSION,
    },
    simulation_event_handler::{GameStartMetadata, SimulationEventHandler},
    state::GameOutcome,
    State,
};

/// A `SimulationEventHandler` that writes one JSON replay file per game to `output_dir`, following
/// the `DataExporter` pattern. See `crate::replay` for the file format.
///
/// One instance is created per game (per the `SimulationEventHandler` contract), so the recorder
/// itself holds no cross-game state; `recorded_so_far` is the one exception, shared (via `Arc`)
/// across every instance so `--replay-sample N` can cap the whole run rather than just one game.
pub struct ReplayRecorder {
    output_dir: PathBuf,
    recorded_so_far: Arc<AtomicUsize>,
    sample_limit: Option<usize>,
    /// Whether this particular game was selected to be recorded (always true when there's no
    /// sample limit).
    recording: bool,
    current: Option<Replay>,
}

impl ReplayRecorder {
    pub fn new(
        output_dir: PathBuf,
        recorded_so_far: Arc<AtomicUsize>,
        sample_limit: Option<usize>,
    ) -> Self {
        Self {
            output_dir,
            recorded_so_far,
            sample_limit,
            recording: false,
            current: None,
        }
    }
}

impl SimulationEventHandler for ReplayRecorder {
    fn on_game_start(&mut self, _game_id: Uuid) {
        // Metadata (decks, player names, seed) is required to start a replay; a handler wired up
        // through the plain `on_game_start` hook (bypassing `Simulation::run`) can't record.
        warn!("ReplayRecorder needs on_game_start_with_metadata; this game will not be recorded");
    }

    fn on_game_start_with_metadata(&mut self, game_id: Uuid, metadata: &GameStartMetadata) {
        self.recording = match self.sample_limit {
            None => true,
            Some(limit) => self.recorded_so_far.fetch_add(1, Ordering::SeqCst) < limit,
        };
        if !self.recording {
            return;
        }

        let mut cards: BTreeMap<String, Card> = BTreeMap::new();
        for deck in &metadata.decks {
            for card in &deck.cards {
                cards.entry(card.get_id()).or_insert_with(|| card.clone());
            }
        }

        let players = std::array::from_fn(|i| ReplayPlayerInfo {
            name: metadata.player_names[i].clone(),
            deck: metadata.decks[i].cards.iter().map(Card::get_id).collect(),
        });

        self.current = Some(Replay {
            version: REPLAY_FORMAT_VERSION,
            game_id: game_id.to_string(),
            seed: metadata.seed,
            players,
            cards,
            steps: Vec::new(),
            final_state: None,
            outcome: None,
        });
    }

    fn on_action(
        &mut self,
        _game_id: Uuid,
        state_before_action: &State,
        actor: usize,
        playable_actions: &[Action],
        action: &Action,
    ) {
        let Some(replay) = self.current.as_mut() else {
            return;
        };

        let options: Vec<ReplayOption> = playable_actions
            .iter()
            .map(|a| ReplayOption {
                text: a.action.to_string(),
                action: a.action.clone(),
            })
            .collect();
        let chosen = playable_actions
            .iter()
            .position(|a| a == action)
            .unwrap_or(0);

        replay.steps.push(ReplayStep {
            ply: replay.steps.len() as u32,
            turn: state_before_action.turn_count,
            actor,
            state: ViewState::omniscient(state_before_action),
            options,
            chosen,
            note: None,
        });
    }

    fn on_action_note(&mut self, _game_id: Uuid, note: Option<String>) {
        if let Some(step) = self.current.as_mut().and_then(|r| r.steps.last_mut()) {
            step.note = note;
        }
    }

    fn on_game_end(&mut self, _game_id: Uuid, state: State, result: Option<GameOutcome>) {
        let Some(mut replay) = self.current.take() else {
            return;
        };
        replay.final_state = Some(ViewState::omniscient(&state));
        replay.outcome = result;

        if let Err(e) = fs::create_dir_all(&self.output_dir) {
            warn!(
                "Failed to create replay folder {:?}: {}",
                self.output_dir, e
            );
            return;
        }
        let file_path = self.output_dir.join(format!("{}.json", replay.game_id));
        match serde_json::to_string(&replay) {
            Ok(json) => {
                if let Err(e) = fs::write(&file_path, json) {
                    warn!("Failed to write replay file {:?}: {}", file_path, e);
                }
            }
            Err(e) => warn!("Failed to serialize replay {}: {}", replay.game_id, e),
        }
    }

    fn on_simulation_end(&mut self) {
        warn!(
            "Replay recording complete. Replays written to: {:?}",
            self.output_dir
        );
    }

    fn merge(&mut self, _other: &dyn SimulationEventHandler) {
        // Each per-game instance writes its own file directly in `on_game_end`; there's nothing
        // to aggregate across instances.
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        players::{EndTurnPlayer, Player},
        simulation_event_handler::CompositeSimulationEventHandler,
        state::GameOutcome,
        test_support::load_test_decks,
        Deck, Game,
    };
    use std::sync::atomic::AtomicUsize;

    fn record_one_game(
        seed: u64,
        output_dir: PathBuf,
        recorded_so_far: Arc<AtomicUsize>,
        sample_limit: Option<usize>,
    ) -> (Uuid, Option<GameOutcome>) {
        let (deck_a, deck_b): (Deck, Deck) = load_test_decks();
        let player_a: Box<dyn Player> = Box::new(EndTurnPlayer {
            deck: deck_a.clone(),
        });
        let player_b: Box<dyn Player> = Box::new(EndTurnPlayer {
            deck: deck_b.clone(),
        });
        let players = vec![player_a, player_b];

        let mut event_handler = CompositeSimulationEventHandler::new(vec![Box::new(
            ReplayRecorder::new(output_dir, recorded_so_far, sample_limit),
        )]);
        let game_id = Uuid::new_v4();
        let metadata = GameStartMetadata {
            seed,
            player_names: ["EndTurnPlayer".to_string(), "EndTurnPlayer".to_string()],
            decks: [&deck_a, &deck_b],
        };
        event_handler.on_game_start_with_metadata(game_id, &metadata);

        let mut game = Game::new_with_event_handlers(game_id, players, seed, &mut event_handler);
        let outcome = game.play();
        let final_state = game.get_state_clone();
        event_handler.on_game_end(game_id, final_state, outcome);
        (game_id, outcome)
    }

    #[test]
    fn records_a_full_game_that_round_trips() {
        let dir = std::env::temp_dir().join(format!("deckgym-replay-test-{}", Uuid::new_v4()));
        let counter = Arc::new(AtomicUsize::new(0));
        let (game_id, outcome) = record_one_game(1, dir.clone(), counter, None);

        let file_path = dir.join(format!("{}.json", game_id));
        let contents = fs::read_to_string(&file_path).expect("replay file should exist");
        let replay: Replay = serde_json::from_str(&contents).expect("replay should deserialize");

        assert_eq!(replay.version, REPLAY_FORMAT_VERSION);
        assert_eq!(replay.game_id, game_id.to_string());
        assert!(!replay.steps.is_empty());
        // Every step's `ply` matches its position.
        for (i, step) in replay.steps.iter().enumerate() {
            assert_eq!(step.ply as usize, i);
        }
        assert_eq!(replay.outcome, outcome);
        assert_eq!(replay.final_state.unwrap().winner, outcome);

        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn replay_sample_limits_how_many_games_are_recorded() {
        let dir =
            std::env::temp_dir().join(format!("deckgym-replay-sample-test-{}", Uuid::new_v4()));
        let counter = Arc::new(AtomicUsize::new(0));

        let (game_id_1, _) = record_one_game(1, dir.clone(), counter.clone(), Some(1));
        let (game_id_2, _) = record_one_game(2, dir.clone(), counter, Some(1));

        assert!(dir.join(format!("{}.json", game_id_1)).exists());
        assert!(!dir.join(format!("{}.json", game_id_2)).exists());

        fs::remove_dir_all(&dir).ok();
    }
}
