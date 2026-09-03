use deckgym::{
    data_exporter::{DataExporter, ExportedDataPoint, ExportedGameOutcome},
    players::PlayerCode,
    simulate::Simulation,
    test_support::load_test_decks,
};
use std::{fs, path::PathBuf};
use uuid::Uuid;

/// Removes the export directory when dropped, so a failing assertion does not
/// leave files behind in the system temp folder.
struct TempExportDir(PathBuf);

impl TempExportDir {
    fn new() -> Self {
        Self(std::env::temp_dir().join(format!("deckgym-export-test-{}", Uuid::new_v4())))
    }
}

impl Drop for TempExportDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Every simulated game should produce a folder holding its pre-action ply
/// snapshots plus an `outcome.json` whose result matches what the engine reported.
#[test]
fn test_data_exporter_writes_plies_and_outcome_per_game() {
    let temp_dir = TempExportDir::new();
    let output_dir = temp_dir.0.clone();
    let export_dir = output_dir.clone();
    let (deck_a, deck_b) = load_test_decks();
    let num_games = 3;

    let mut simulation = Simulation::new_with_decks(
        deck_a,
        deck_b,
        vec![PlayerCode::R, PlayerCode::R],
        num_games,
        Some(7),
        false,
        None,
    )
    .expect("simulation should build")
    .register_with_closure(move || Box::new(DataExporter::new(export_dir.clone())));

    let outcomes = simulation.run();
    assert_eq!(outcomes.len(), num_games as usize);

    let game_folders: Vec<PathBuf> = fs::read_dir(&output_dir)
        .expect("output folder should exist")
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_dir())
        .collect();
    assert_eq!(game_folders.len(), num_games as usize);

    let mut exported_results = Vec::new();
    for game_folder in &game_folders {
        let game_id = game_folder
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let ply_files: Vec<PathBuf> = fs::read_dir(game_folder)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| {
                path.file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.starts_with("ply_"))
            })
            .collect();
        assert!(!ply_files.is_empty(), "game {game_id} exported no plies");

        let first_ply: ExportedDataPoint =
            serde_json::from_str(&fs::read_to_string(game_folder.join("ply_0000.json")).unwrap())
                .expect("ply file should round-trip through serde");
        assert_eq!(first_ply.game_id, game_id);
        assert_eq!(first_ply.ply, 0);
        assert!(
            first_ply.state.winner.is_none(),
            "pre-action snapshots never hold the winner; that is why outcome.json exists"
        );

        let outcome: ExportedGameOutcome =
            serde_json::from_str(&fs::read_to_string(game_folder.join("outcome.json")).unwrap())
                .expect("outcome file should round-trip through serde");
        assert_eq!(outcome.game_id, game_id);
        exported_results.push(outcome.result);
    }

    // Folder iteration order is arbitrary, so compare as multisets.
    let mut expected: Vec<String> = outcomes.iter().map(|o| format!("{o:?}")).collect();
    let mut actual: Vec<String> = exported_results.iter().map(|o| format!("{o:?}")).collect();
    expected.sort();
    actual.sort();
    assert_eq!(actual, expected);
}
