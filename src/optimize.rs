use std::{fs, time::Duration};

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
pub fn cli_optimize(
    incomplete_deck_path: &str,
    candidate_cards_str: &str,
    enemy_decks_folder: &str,
    num: u32,
    players: Option<Vec<PlayerCode>>,
    seed: Option<u64>,
) {
    let incomplete_deck =
        Deck::from_file(incomplete_deck_path).expect("Failed to parse incomplete deck file");
    let candidate_cards: Vec<String> = candidate_cards_str
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

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
                warn!("Skipping enemy deck {path} since not valid");
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

    optimize(
        &incomplete_deck,
        &candidate_cards,
        &enemy_valid_decks,
        num,
        players,
        seed,
        None::<fn(usize, usize, &[CardId], f32)>,
    );
}

pub fn optimize<F>(
    incomplete_deck: &Deck,
    candidate_cards: &[String],
    enemy_decks: &[Deck],
    num: u32,
    players: Option<Vec<PlayerCode>>,
    seed: Option<u64>,
    progress_callback: Option<F>,
) -> Vec<(Vec<CardId>, f32)>
where
    F: Fn(usize, usize, &[CardId], f32),
{
    if enemy_decks.is_empty() {
        warn!("No valid enemy decks provided. Optimization cannot proceed.");
        return Vec::new();
    }

    // Parse the candidate cards list.
    let candidate_card_ids: Vec<CardId> = candidate_cards
        .iter()
        .map(|s| robustly_parse_card_id_string(s))
        .collect();

    // Read and validate the incomplete deck.
    let current_count = incomplete_deck.cards.len();
    let missing_count = 20 - current_count;
    warn!("Incomplete deck has {current_count} cards, missing {missing_count} cards");
    if missing_count == 0 {
        warn!("Deck is already complete (20 cards). No optimization needed.");
        return Vec::new();
    }

    // Generate all unique k-combinations where k = missing_count
    // This generates all ways to choose missing_count cards from the candidate list
    // (automatically deduplicated)
    let unique_combinations = generate_combinations(&candidate_card_ids, missing_count);
    warn!(
        "Generated {} unique combinations.",
        unique_combinations.len()
    );

    // Filter to only valid combinations by checking deck validity
    let combinations: Vec<Vec<CardId>> = unique_combinations
        .into_iter()
        .filter(|comb| {
            let mut test_deck = incomplete_deck.clone();
            for card_id in comb {
                let card = get_card_by_enum(*card_id);
                test_deck.cards.push(card);
            }
            test_deck.is_valid()
        })
        .collect();

    warn!(
        "Generated {} valid combinations to complete the deck.",
        combinations.len()
    );
    warn!("Valid combinations: {combinations:?}");

    // Estimate the time it will take to run all simulations
    let player_codes = fill_code_array(players.clone());
    let total_games = combinations.len() as u64 * num as u64 * enemy_decks.len() as u64;
    let time_per_game = estimate_time_per_game(&player_codes);
    let total_time = time_per_game.mul_f64(total_games as f64);

    warn!(
        "Estimated time: {} ({} combinations × {} enemy decks × {} games per deck)",
        humantime::format_duration(total_time),
        combinations.len(),
        enemy_decks.len(),
        num
    );
    warn!(
        "Time estimation: {} per game ({} non-R players, {} R players)",
        humantime::format_duration(time_per_game),
        count_player_types(&player_codes, false),
        count_player_types(&player_codes, true)
    );

    // For every valid combination, complete the deck and simulate games.
    let mut best_win_percent = 0.0;
    let mut best_combination = None;
    let mut results = Vec::new();
    let total_combinations = combinations.len();
    for comb in &combinations {
        // Create a completed deck by cloning the incomplete one and adding the candidate cards.
        let mut completed_deck = incomplete_deck.clone();
        for card_id in comb {
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
        for enemy_deck in enemy_decks {
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
        warn!("Combination {comb:?} win percentage: {win_percent:.2}%");

        // Report progress via callback
        if let Some(ref callback) = progress_callback {
            callback(results.len(), total_combinations, comb, win_percent);
        }

        if win_percent > best_win_percent {
            best_win_percent = win_percent;
            best_combination = Some(comb.clone());
        }
    }

    // Report the best combination found.
    match best_combination {
        Some(comb) => {
            warn!("Best combination: {comb:?} with win percentage: {best_win_percent:.2}%");
        }
        None => {
            warn!("No valid combination found.");
        }
    }

    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    results
}

/// Attempts to robustly parse a card ID string, handling various formats and padding the number if needed.
fn robustly_parse_card_id_string(orig: &str) -> CardId {
    let s = orig.trim().replace(' ', "");
    if s.len() < 3 {
        panic!("Card ID '{}' should be at least 3 characters long", orig);
    }
    // Try splitting by space first
    let (prefix, number) = if let Some(idx) = orig.find([' ']) {
        let (pre, num) = orig.split_at(idx);
        let num = num.trim_start_matches([' ']);
        (pre.trim(), num.trim())
    } else {
        // fallback: last 3 chars as number, rest as prefix
        let number = &s[s.len() - 3..];
        let prefix = &s[..s.len() - 3];
        (prefix, number)
    };
    let padded_number = format!("{:0>3}", number);
    let id = format!("{prefix} {padded_number}");
    CardId::from_card_id(id.as_str())
        .unwrap_or_else(|| panic!("Invalid card ID '{}' in candidate cards", orig))
}

/// Estimates time per game based on player types
fn estimate_time_per_game(player_codes: &[PlayerCode]) -> Duration {
    let non_r_count = count_player_types(player_codes, false) as u64;
    let r_count = count_player_types(player_codes, true) as u64;

    // 300ms per non-R player, 15ms per R player
    let non_r_time = Duration::from_millis(non_r_count * 300);
    let r_time = Duration::from_millis(r_count * 15);

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

/// Deduplicates combinations by creating canonical representations based on card counts.
/// This is useful when the candidate pool may have repeated cards.
pub fn deduplicate_combinations(combinations: Vec<Vec<CardId>>) -> Vec<Vec<CardId>> {
    use std::collections::{HashMap, HashSet};
    let mut seen = HashSet::new();
    combinations
        .into_iter()
        .filter(|comb| {
            // Create a canonical representation using card counts
            let mut counts: HashMap<CardId, usize> = HashMap::new();
            for &card_id in comb {
                *counts.entry(card_id).or_insert(0) += 1;
            }
            // Convert to a sorted vector - sort by discriminant value (as usize)
            let mut canonical: Vec<_> = counts.into_iter().collect();
            canonical.sort_by_key(|(id, _)| *id as usize);
            seen.insert(canonical)
        })
        .collect()
}

/// Generates all unique k-combinations from the candidate cards.
/// This is a simple n choose k where we pick k items from the n candidates.
/// Automatically deduplicates combinations (useful when candidate pool has repeated cards).
pub fn generate_combinations(candidates: &[CardId], k: usize) -> Vec<Vec<CardId>> {
    let n = candidates.len();
    if k == 0 {
        return vec![vec![]];
    }
    if n == 0 {
        return vec![];
    }

    let mut all_combinations = Vec::new();
    generate_combinations_recursive(candidates, k, 0, &mut vec![], &mut all_combinations);

    // Deduplicate the combinations
    deduplicate_combinations(all_combinations)
}

/// Recursive helper to generate k-combinations.
fn generate_combinations_recursive(
    candidates: &[CardId],
    k: usize,
    start: usize,
    current: &mut Vec<CardId>,
    result: &mut Vec<Vec<CardId>>,
) {
    if current.len() == k {
        result.push(current.clone());
        return;
    }

    for i in start..candidates.len() {
        current.push(candidates[i]);
        generate_combinations_recursive(candidates, k, i + 1, current, result);
        current.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimize() {
        let incomplete_deck = Deck::from_string(
            "Energy: Grass\n2 Bulbasaur A1 1\n1 Ivysaur A1 2\n2 Venusaur ex A1 4\n2 Snivy A1a 4\n2 Serperior A1a 6\n2 Rocky Helmet A2 148\n2 Rare Candy A3 144\n2 Leaf Cape A3 147\n2 Poké Ball P-A 5\n2 Professor's Research P-A 7",
        )
        .unwrap();
        let candidate_cards: Vec<String> = vec![
            "A1 219".to_string(),
            "A1 219".to_string(),
            "A3 155".to_string(),
            "A3 155".to_string(),
        ];
        let enemy_decks: Vec<Deck> = vec![
    "Energy: Grass\n2 Bulbasaur A1 1\n1 Ivysaur A1 2\n2 Venusaur ex A1 4\n2 Snivy A1a 4\n2 Serperior A1a 6\n1 Erika A1 266\n2 Rocky Helmet A2 148\n2 Rare Candy A3 144\n2 Leaf Cape A3 147\n2 Poké Ball P-A 5\n2 Professor's Research P-A 7",
    "Energy: Water\n2 Froakie A1 87\n2 Greninja A1 89\n1 Giratina ex A2b 35\n2 Suicune ex A4a 20\n1 Giant Cape A2 147\n2 Cyrus A2 150\n1 Mars A2 155\n2 Irida A2a 72\n2 Rare Candy A3 144\n1 Repel A3a 64\n2 Poké Ball P-A 5\n2 Professor's Research P-A 7"
]        .iter()
        .map(|s| Deck::from_string(s).unwrap())
        .collect();
        let num = 1;
        let players = Some(vec![PlayerCode::R, PlayerCode::R]);
        let seed: Option<u64> = None;
        let results = optimize(
            &incomplete_deck,
            &candidate_cards,
            &enemy_decks,
            num,
            players,
            seed,
            None::<fn(usize, usize, &[CardId], f32)>,
        );
        assert!(!results.is_empty());
    }

    #[test]
    fn test_robustly_parse_card_id_string() {
        let cases = vec![
            ("A1 53", CardId::from_card_id("A1 053").unwrap()),
            ("P-A 5", CardId::from_card_id("P-A 005").unwrap()),
            ("A1219", CardId::from_card_id("A1 219").unwrap()),
            ("A2a 072", CardId::from_card_id("A2a 072").unwrap()),
        ];
        for (input, expected) in cases {
            let parsed = robustly_parse_card_id_string(input);
            assert_eq!(parsed, expected, "Failed to parse '{}'", input);
        }
    }
}
