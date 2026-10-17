//! Replay recording: one JSON file per game, with a full [`ViewState`] snapshot at every
//! decision point plus the legal options and the action that was chosen. See
//! `docs/replay-viewer-plan.md` for the design and `viewer/` for the web app that plays these
//! back.

mod recorder;
pub mod view;

pub use recorder::ReplayRecorder;
pub use view::{PlayerView, SlotView, ViewState};

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{actions::SimpleAction, models::Card, state::GameOutcome};

/// Bumped whenever the replay file shape changes in a way that isn't backward compatible for
/// consumers (the viewer, external tooling). Consumers should check this before assuming field
/// shapes.
pub const REPLAY_FORMAT_VERSION: u32 = 1;

/// One player's identity in a replay: a display name and the deck they played (as a flat list of
/// card ids, one entry per copy — a 20-card deck is 20 entries).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReplayPlayerInfo {
    pub name: String,
    pub deck: Vec<String>,
}

/// One action the acting player could have taken at a decision point: `text` is the `Display` of
/// the `SimpleAction` (human-readable), `action` is the full action, serialized as-is.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReplayOption {
    pub text: String,
    pub action: SimpleAction,
}

/// One decision point in the game: the state right before the action, every legal option, and
/// which one (by index into `options`) was chosen.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReplayStep {
    /// Sequential decision-point number within the game, starting at 0.
    pub ply: u32,
    pub turn: u8,
    pub actor: usize,
    pub state: ViewState,
    pub options: Vec<ReplayOption>,
    pub chosen: usize,
    /// An optional short note the acting player/bot attaches to its decision (see the external
    /// bot protocol in `docs/bot-protocol.md`). `None` for the built-in engine players.
    pub note: Option<String>,
    /// Optional per-option preference values from the acting player/bot, parallel to `options`
    /// (see `scores` in `docs/bot-protocol.md`). Absent for the built-in engine players and for
    /// replays recorded before this field existed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scores: Option<Vec<f64>>,
}

/// A full recorded game: enough to render every state it passed through without re-running the
/// engine.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Replay {
    pub version: u32,
    pub game_id: String,
    pub seed: u64,
    pub players: [ReplayPlayerInfo; 2],
    /// Card id -> full card definition, for every card either player's deck could show. Lets
    /// consumers (the viewer, an external bot) render/reason about cards without needing the
    /// engine's card database.
    pub cards: BTreeMap<String, Card>,
    pub steps: Vec<ReplayStep>,
    /// The state after the very last action of the game. `None` only if the game never started
    /// (should not happen in practice).
    pub final_state: Option<ViewState>,
    pub outcome: Option<GameOutcome>,
}
