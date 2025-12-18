/// Utilities for example scripts
use std::fs;
use std::path::{Path, PathBuf};

/// Discover all .txt files in a directory
pub fn discover_deck_files(folder: &str) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let path = Path::new(folder);

    if !path.exists() {
        return Err(format!("Folder does not exist: {}", folder).into());
    }

    if !path.is_dir() {
        return Err(format!("Path is not a directory: {}", folder).into());
    }

    let mut deck_files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "txt" {
                    deck_files.push(path);
                }
            }
        }
    }

    if deck_files.is_empty() {
        return Err(format!("No .txt deck files found in folder: {}", folder).into());
    }

    // Sort for consistent ordering
    deck_files.sort();

    Ok(deck_files)
}
