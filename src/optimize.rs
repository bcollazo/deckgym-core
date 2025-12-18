use std::fs;

use log::warn;
use num_format::{Locale, ToFormattedString};
use rayon::prelude::*;

use crate::{
    card_ids::CardId,
    database::get_card_by_enum,
    players::{create_players, fill_code_array, PlayerCode},
    simulate::create_progress_bar,
    state::GameOutcome,
    Deck, Game,
};

/// Configuration for a single enemy deck in optimization
#[derive(Clone)]
pub struct EnemyDeckConfig {
    pub deck: Deck,
    pub num_games: u32,
}

/// Configuration for optimization runs
#[derive(Clone)]
pub struct OptimizationConfig {
    pub enemy_deck_configs: Vec<EnemyDeckConfig>,
    pub players: Option<Vec<PlayerCode>>,
    pub seed: Option<u64>,
}

/// Configuration for running simulations
#[derive(Clone)]
pub struct SimulationConfig {
    pub num_games: u32,
    pub players: Option<Vec<PlayerCode>>,
    pub seed: Option<u64>,
    pub data_output: Option<String>,
}

/// Configuration for parallelism
#[derive(Clone, Default)]
pub struct ParallelConfig {
    pub enabled: bool,
    pub num_threads: Option<usize>,
}

/// Callbacks for optimization progress tracking
pub struct OptimizationCallbacks<F, G>
where
    F: Fn(usize, usize, &[CardId], f32),
    G: Fn() + Sync,
{
    pub on_combination_complete: Option<F>,
    pub on_game_complete: Option<G>,
}

impl<F, G> Default for OptimizationCallbacks<F, G>
where
    F: Fn(usize, usize, &[CardId], f32),
    G: Fn() + Sync,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<F, G> OptimizationCallbacks<F, G>
where
    F: Fn(usize, usize, &[CardId], f32),
    G: Fn() + Sync,
{
    pub fn new() -> Self {
        Self {
            on_combination_complete: None,
            on_game_complete: None,
        }
    }

    pub fn with_combination_callback(mut self, callback: F) -> Self {
        self.on_combination_complete = Some(callback);
        self
    }

    pub fn with_game_callback(mut self, callback: G) -> Self {
        self.on_game_complete = Some(callback);
        self
    }
}

/// Optimizes a deck by simulating games with different combinations of candidate cards.
pub fn cli_optimize(
    incomplete_deck_path: &str,
    candidate_cards_str: &str,
    enemy_decks_folder: &str,
    sim_config: SimulationConfig,
    parallel_config: ParallelConfig,
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

    // Create progress bar for total games (not combinations)
    let candidate_card_ids: Vec<CardId> = candidate_cards
        .iter()
        .map(|s| robustly_parse_card_id_string(s))
        .collect();
    let missing_count = 20 - incomplete_deck.cards.len();
    let combinations_count =
        count_valid_combinations(&incomplete_deck, &candidate_card_ids, missing_count);
    let total_games =
        (combinations_count * enemy_valid_decks.len() * sim_config.num_games as usize) as u64;
    let pb = create_progress_bar(total_games);
    pb.tick(); // Ensure progress bar is drawn immediately

    type NeverCalled = fn(usize, usize, &[CardId], f32);
    let callbacks: OptimizationCallbacks<NeverCalled, _> =
        OptimizationCallbacks::new().with_game_callback(|| pb.inc(1));

    optimize(
        &incomplete_deck,
        &candidate_cards,
        &enemy_valid_decks,
        sim_config,
        parallel_config,
        Some(callbacks),
    );

    pb.finish_with_message("Optimization complete!");
}

/// Wrapper function that accepts simple Deck slice for backward compatibility
pub fn optimize<F, G>(
    incomplete_deck: &Deck,
    candidate_cards: &[String],
    enemy_decks: &[Deck],
    sim_config: SimulationConfig,
    parallel_config: ParallelConfig,
    callbacks: Option<OptimizationCallbacks<F, G>>,
) -> Vec<(Vec<CardId>, f32)>
where
    F: Fn(usize, usize, &[CardId], f32),
    G: Fn() + Sync,
{
    // Convert to OptimizationConfig format
    let enemy_deck_configs: Vec<EnemyDeckConfig> = enemy_decks
        .iter()
        .map(|deck| EnemyDeckConfig {
            deck: deck.clone(),
            num_games: sim_config.num_games,
        })
        .collect();
    let opt_config = OptimizationConfig {
        enemy_deck_configs,
        players: sim_config.players,
        seed: sim_config.seed,
    };

    optimize_with_configs(
        incomplete_deck,
        candidate_cards,
        opt_config,
        parallel_config,
        callbacks,
    )
}

/// Optimizes with per-deck game configuration
pub fn optimize_with_configs<F, G>(
    incomplete_deck: &Deck,
    candidate_cards: &[String],
    opt_config: OptimizationConfig,
    parallel_config: ParallelConfig,
    callbacks: Option<OptimizationCallbacks<F, G>>,
) -> Vec<(Vec<CardId>, f32)>
where
    F: Fn(usize, usize, &[CardId], f32),
    G: Fn() + Sync,
{
    if opt_config.enemy_deck_configs.is_empty() {
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

    // Generate all valid combinations for the incomplete deck
    let combinations =
        generate_valid_combinations(incomplete_deck, &candidate_card_ids, missing_count);

    warn!(
        "Valid combinations ({}): {combinations:?}",
        combinations.len()
    );
    let total_games_per_combination: u32 = opt_config
        .enemy_deck_configs
        .iter()
        .map(|config| config.num_games)
        .sum();
    warn!(
        "Games to Play: {} combinations × {} total games per combination",
        combinations.len(),
        total_games_per_combination
    );

    // Configure rayon thread pool if specified
    if let Some(num_threads) = parallel_config.num_threads {
        rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build_global()
            .ok(); // Ignore error if pool is already initialized
    }

    // For every valid combination, complete the deck and simulate games.
    let mut results = Vec::new();

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

        // Generate all games to simulate for this combination
        let games_to_simulate: Vec<(Deck, Deck, Vec<PlayerCode>)> = opt_config
            .enemy_deck_configs
            .iter()
            .flat_map(|enemy_config| {
                let deck_clone = completed_deck.clone();
                let players_clone = opt_config.players.clone();
                let enemy_deck_clone = enemy_config.deck.clone();
                (0..enemy_config.num_games).map(move |_| {
                    (
                        deck_clone.clone(),
                        enemy_deck_clone.clone(),
                        fill_code_array(players_clone.clone()),
                    )
                })
            })
            .collect();

        // Extract the game callback to avoid capturing the entire callbacks struct in parallel closures
        let game_callback = callbacks
            .as_ref()
            .and_then(|cbs| cbs.on_game_complete.as_ref());

        // Run games either in parallel or sequentially
        let wins: usize = if parallel_config.enabled {
            games_to_simulate
                .par_iter()
                .map(|(deck_a, deck_b, player_codes)| {
                    let players =
                        create_players(deck_a.clone(), deck_b.clone(), player_codes.clone());
                    let seed = opt_config.seed.unwrap_or(rand::random::<u64>());
                    let mut game = Game::new(players, seed);
                    let outcome = game.play();

                    if let Some(callback) = game_callback {
                        callback();
                    }

                    // Count as win if first player (our deck) wins
                    if let Some(GameOutcome::Win(winner)) = outcome {
                        if winner == 0 {
                            return 1;
                        }
                    }
                    0
                })
                .sum()
        } else {
            games_to_simulate
                .iter()
                .map(|(deck_a, deck_b, player_codes)| {
                    let players =
                        create_players(deck_a.clone(), deck_b.clone(), player_codes.clone());
                    let seed = opt_config.seed.unwrap_or(rand::random::<u64>());
                    let mut game = Game::new(players, seed);
                    let outcome = game.play();

                    if let Some(callback) = game_callback {
                        callback();
                    }

                    // Count as win if first player (our deck) wins
                    if let Some(GameOutcome::Win(winner)) = outcome {
                        if winner == 0 {
                            return 1;
                        }
                    }
                    0
                })
                .sum()
        };

        let total_games = games_to_simulate.len();
        let win_percent = (wins as f32 / total_games as f32) * 100.0;
        results.push((comb.clone(), win_percent));

        warn!("Combination {comb:?} win percentage: {win_percent:.2}%");

        // Report progress via callback
        if let Some(ref cbs) = callbacks {
            if let Some(ref callback) = cbs.on_combination_complete {
                let total_combinations = combinations.len();
                callback(results.len(), total_combinations, comb, win_percent);
            }
        }
    }

    // Find the best combination
    let mut best_win_percent = 0.0;
    let mut best_combination = None;
    for (comb, win_percent) in &results {
        if *win_percent > best_win_percent {
            best_win_percent = *win_percent;
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

/// Generates all valid combinations for the incomplete deck.
/// Returns a vector of card combinations that result in valid decks when added.
pub fn generate_valid_combinations(
    incomplete_deck: &Deck,
    candidates: &[CardId],
    missing_count: usize,
) -> Vec<Vec<CardId>> {
    let combinations = generate_combinations(candidates, missing_count);
    combinations
        .into_iter()
        .filter(|comb| {
            let mut test_deck = incomplete_deck.clone();
            for card_id in comb {
                let card = get_card_by_enum(*card_id);
                test_deck.cards.push(card);
            }
            test_deck.is_valid()
        })
        .collect()
}

/// Counts how many valid combinations will be generated.
/// This is useful for setting up progress bars before running the optimization.
fn count_valid_combinations(
    incomplete_deck: &Deck,
    candidates: &[CardId],
    missing_count: usize,
) -> usize {
    generate_valid_combinations(incomplete_deck, candidates, missing_count).len()
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
        let enemy_decks: Vec<Deck> = ["Energy: Grass\n2 Bulbasaur A1 1\n1 Ivysaur A1 2\n2 Venusaur ex A1 4\n2 Snivy A1a 4\n2 Serperior A1a 6\n1 Erika A1 266\n2 Rocky Helmet A2 148\n2 Rare Candy A3 144\n2 Leaf Cape A3 147\n2 Poké Ball P-A 5\n2 Professor's Research P-A 7",
    "Energy: Water\n2 Froakie A1 87\n2 Greninja A1 89\n1 Giratina ex A2b 35\n2 Suicune ex A4a 20\n1 Giant Cape A2 147\n2 Cyrus A2 150\n1 Mars A2 155\n2 Irida A2a 72\n2 Rare Candy A3 144\n1 Repel A3a 64\n2 Poké Ball P-A 5\n2 Professor's Research P-A 7"]        .iter()
        .map(|s| Deck::from_string(s).unwrap())
        .collect();
        let sim_config = SimulationConfig {
            num_games: 1,
            players: Some(vec![PlayerCode::R, PlayerCode::R]),
            seed: None,
            data_output: None,
        };
        let parallel_config = ParallelConfig {
            enabled: false,
            num_threads: None,
        };
        let results = optimize(
            &incomplete_deck,
            &candidate_cards,
            &enemy_decks,
            sim_config,
            parallel_config,
            None::<OptimizationCallbacks<fn(usize, usize, &[CardId], f32), fn()>>,
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
