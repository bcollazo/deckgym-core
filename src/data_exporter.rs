use crate::actions::Action;
use crate::simulation_event_handler::SimulationEventHandler;
use crate::state::{GameOutcome, State};
use log::warn;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

/// Struct to hold the exported data point (state, action pair)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportedDataPoint {
    pub game_id: String,
    pub ply: u32,
    pub actor: usize,
    pub state: State,
    pub playable_actions: Vec<Action>,
    pub chosen_action: Action,
}

/// Event handler that exports (state, action) pairs to JSON files
pub struct DataExporter {
    output_folder: PathBuf,
    ply_counter: u32,
    current_game_id: Option<Uuid>,
}

impl DataExporter {
    pub fn new(output_folder: PathBuf) -> Self {
        Self {
            output_folder,
            ply_counter: 0,
            current_game_id: None,
        }
    }
}

impl SimulationEventHandler for DataExporter {
    fn on_game_start(&mut self, game_id: Uuid) {
        self.current_game_id = Some(game_id);
        self.ply_counter = 0;

        // Create folder for this game
        let game_folder = self.output_folder.join(game_id.to_string());
        if let Err(e) = fs::create_dir_all(&game_folder) {
            warn!("Failed to create game folder {:?}: {}", game_folder, e);
        }
    }

    fn on_action(
        &mut self,
        game_id: Uuid,
        state_before_action: &State,
        actor: usize,
        playable_actions: &[Action],
        action: &Action,
    ) {
        let game_folder = self.output_folder.join(game_id.to_string());

        // Create data point
        let data_point = ExportedDataPoint {
            game_id: game_id.to_string(),
            ply: self.ply_counter,
            actor,
            state: state_before_action.clone(),
            playable_actions: playable_actions.to_vec(),
            chosen_action: action.clone(),
        };

        // Write to file
        let file_path = game_folder.join(format!("ply_{:04}.json", self.ply_counter));
        match serde_json::to_string_pretty(&data_point) {
            Ok(json) => {
                if let Err(e) = fs::write(&file_path, json) {
                    warn!("Failed to write ply file {:?}: {}", file_path, e);
                }
            }
            Err(e) => {
                warn!(
                    "Failed to serialize data point for ply {}: {}",
                    self.ply_counter, e
                );
            }
        }

        self.ply_counter += 1;
    }

    fn on_game_end(&mut self, _game_id: Uuid, _state: State, _result: Option<GameOutcome>) {
        // Reset for next game
        self.ply_counter = 0;
        self.current_game_id = None;
    }

    fn on_simulation_end(&mut self) {
        warn!(
            "Data export complete. Data written to: {:?}",
            self.output_folder
        );
    }

    fn merge(&mut self, _other: &dyn SimulationEventHandler) {
        // DataExporter doesn't need to merge data since each thread
        // writes to separate game folders. No aggregation needed.
    }
}
