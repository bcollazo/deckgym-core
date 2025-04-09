use std::{collections::HashMap, fs, time::Duration};

use log::warn;
use num_format::{Locale, ToFormattedString};

use crate::{
    card_ids::CardId,
    database::get_card_by_enum,
    players::{create_players, fill_code_array, PlayerCode},
    state::GameOutcome,
    Deck, Game,
};

/// Optimizes a deck by simulating games with different combinations of candidate cards.
pub fn optimize(
    incomplete_deck_path: &str,
    candidate_cards_str: &str,
    enemy_decks_folder: &str,
    num: u32,
    players: Option<Vec<PlayerCode>>,
    seed: Option<u64>,
) {
    // Parse the candidate cards list.
    let candidate_cards: Vec<CardId> = candidate_cards_str
        .split(',')
        .map(|s| {
            // take last 3 to be id, then the rest of prefix will be set
            let s = s.trim();
            if s.len() < 3 {
                panic!("Card ID should be at least 3 characters long");
            }
            let number = &s[s.len() - 3..];
            let prefix = &s[..s.len() - 3];
            let id = format!("{} {}", prefix, number);
            CardId::from_card_id(id.as_str()).expect("Card ID should be valid")
        })
        .collect();

    // Read and validate the incomplete deck.
    let incomplete_deck =
        Deck::from_file(incomplete_deck_path).expect("Failed to parse incomplete deck file");
    let current_count = incomplete_deck.cards.len();
    let missing_count = 20 - current_count;
    warn!(
        "Incomplete deck has {} cards, missing {} cards",
        current_count, missing_count
    );
    if missing_count == 0 {
        warn!("Deck is already complete (20 cards). No optimization needed.");
        return;
    }

    // For each candidate card, determine how many additional copies are allowed.
    // A card cannot appear more than twice in the deck.
    let mut allowed_map: HashMap<CardId, u32> = HashMap::new();
    for card in &candidate_cards {
        let count = incomplete_deck
            .cards
            .iter()
            .filter(|c| c.get_card_id() == *card)
            .count();
        let allowed = if count >= 2 { 0 } else { 2 - count };
        allowed_map.insert(*card, allowed as u32);
    }

    // Read enemy decks from the specified folder.
    let enemy_deck_paths: Vec<String> = fs::read_dir(enemy_decks_folder)
        .expect("Failed to read enemy decks folder")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.path().is_file() {
                Some(entry.path().to_str()?.to_string())
            } else {
                None
            }
        })
        .collect();
    let enemy_valid_decks: Vec<Deck> = enemy_deck_paths
        .iter()
        .filter_map(|path| {
            let deck = Deck::from_file(path).ok()?;
            if deck.cards.len() == 20 {
                Some(deck)
            } else {
                warn!("Skipping enemy deck {} since not valid", path);
                None
            }
        })
        .collect();
    warn!(
        "Found {} enemy deck files ({} valid). {:?}",
        enemy_deck_paths.len().to_formatted_string(&Locale::en),
        enemy_valid_decks.len(),
        enemy_deck_paths
            .iter()
            .map(|s| s.split('/').next_back().unwrap())
            .collect::<Vec<_>>()
    );

    // Generate all valid combinations (multiset selections) of candidate cards that sum to missing_count.
    let combinations = generate_combinations(&candidate_cards, &allowed_map, missing_count as u32);
    warn!(
        "Generated {} possible combinations to complete the deck.",
        combinations.len()
    );
    warn!("Combinations: {:?}", combinations);

    // Estimate the time it will take to run all simulations
    let player_codes = fill_code_array(players.clone());
    let total_games = combinations.len() as u64 * num as u64 * enemy_valid_decks.len() as u64;
    let time_per_game = estimate_time_per_game(&player_codes);
    let total_time = time_per_game.mul_f64(total_games as f64);

    warn!(
        "Estimated time: {} ({} combinations × {} enemy decks × {} games per deck)",
        humantime::format_duration(total_time).to_string(),
        combinations.len(),
        enemy_valid_decks.len(),
        num
    );
    warn!(
        "Time estimation: {} per game ({} non-R players, {} R players)",
        humantime::format_duration(time_per_game).to_string(),
        count_player_types(&player_codes, false),
        count_player_types(&player_codes, true)
    );

    // For every valid combination, complete the deck and simulate games.
    let mut best_win_percent = 0.0;
    let mut best_combination = None;
    let mut results = Vec::new();
    for comb in combinations {
        // Create a completed deck by cloning the incomplete one and adding the candidate cards.
        let mut completed_deck = incomplete_deck.clone();
        for card_id in &comb {
            let card = get_card_by_enum(*card_id);
            completed_deck.cards.push(card);
        }
        if !completed_deck.is_valid() {
            warn!(
                "Completed deck is invalid. Num cards: {}, num basics: {}",
                completed_deck.cards.len(),
                completed_deck.cards.iter().filter(|x| x.is_basic()).count()
            );
            continue;
        }

        // Simulate games for each enemy deck.
        let mut total_wins = 0;
        let mut total_games = 0;
        for enemy_deck in &enemy_valid_decks {
            for _ in 0..num {
                let players = create_players(
                    completed_deck.clone(),
                    enemy_deck.clone(),
                    fill_code_array(players.clone()),
                );
                let seed = seed.unwrap_or(rand::random::<u64>());
                let mut game = Game::new(players, seed);
                let outcome = game.play();

                // Assume that if outcome is a win and the first player (our deck) wins, it counts as a win.
                if let Some(GameOutcome::Win(winner)) = outcome {
                    if winner == 0 {
                        total_wins += 1;
                    }
                }
                total_games += 1;
            }
        }

        let win_percent = (total_wins as f32 / total_games as f32) * 100.0;
        results.push((comb.clone(), win_percent));
        warn!("Combination {:?} win percentage: {:.2}%", comb, win_percent);
        if win_percent > best_win_percent {
            best_win_percent = win_percent;
            best_combination = Some(comb.clone());
        }
    }

    // Report the best combination found.
    match best_combination {
        Some(comb) => {
            warn!(
                "Best combination: {:?} with win percentage: {:.2}%",
                comb, best_win_percent
            );
        }
        None => {
            warn!("No valid combination found.");
        }
    }
}

/// Estimates time per game based on player types
fn estimate_time_per_game(player_codes: &[PlayerCode]) -> Duration {
    let non_r_count = count_player_types(player_codes, false) as u64;
    let r_count = count_player_types(player_codes, true) as u64;

    // 15ms per non-R player, 150µs per R player
    let non_r_time = Duration::from_millis(non_r_count * 15);
    let r_time = Duration::from_micros(r_count * 150);

    non_r_time.checked_add(r_time).unwrap_or(non_r_time)
}

/// Counts the number of players of a specific type (R or non-R)
fn count_player_types(player_codes: &[PlayerCode], is_r: bool) -> usize {
    player_codes
        .iter()
        .filter(|&code| {
            if is_r {
                matches!(code, PlayerCode::R)
            } else {
                !matches!(code, PlayerCode::R)
            }
        })
        .count()
}

/// Generates all valid multisets of candidate cards (as vectors of strings) whose total count is `remaining`.
/// Each candidate card cannot be used more than allowed_map[card] times.
fn generate_combinations(
    candidates: &Vec<CardId>,
    allowed_map: &HashMap<CardId, u32>,
    remaining: u32,
) -> Vec<Vec<CardId>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    generate_combinations_recursive(
        candidates,
        allowed_map,
        remaining,
        0,
        &mut current,
        &mut result,
    );
    result
}

/// Helper recursive function to generate combinations.
fn generate_combinations_recursive(
    candidates: &Vec<CardId>,
    allowed_map: &HashMap<CardId, u32>,
    remaining: u32,
    index: usize,
    current: &mut Vec<CardId>,
    result: &mut Vec<Vec<CardId>>,
) {
    if remaining == 0 {
        result.push(current.clone());
        return;
    }
    if index >= candidates.len() {
        return;
    }
    let candidate = &candidates[index];
    let max_allowed = *allowed_map.get(candidate).unwrap_or(&2);
    // Try using this candidate 0 up to min(max_allowed, remaining) times.
    for count in 0..=std::cmp::min(max_allowed, remaining) {
        for _ in 0..count {
            current.push(*candidate);
        }
        generate_combinations_recursive(
            candidates,
            allowed_map,
            remaining - count,
            index + 1,
            current,
            result,
        );
        for _ in 0..count {
            current.pop();
        }
    }
}
