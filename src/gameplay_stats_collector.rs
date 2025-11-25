use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use crate::{
    actions::{Action, SimpleAction},
    simulation_event_handler::SimulationEventHandler,
    State,
};

/// Aggregated statistics from all games
#[derive(Debug, Clone)]
pub struct AggregatedStats {
    pub total_games: usize,
    pub avg_game_length: f64,
    /// Average turn when deck became empty, indexed by player (None if never happened)
    pub deck_empty_avg: [Option<f64>; 2],
    /// Average hand size across all turns, indexed by player
    pub hand_sizes: [f64; 2],
    /// Average turn cards first appeared, indexed by player then card_id
    pub cards_seen: [HashMap<String, f64>; 2],
    /// Average turn attacks first used, indexed by player then (card_id, attack_idx)
    pub attacks_used: [HashMap<(String, u8), f64>; 2],
}

/// Collects detailed gameplay statistics during simulations
///
/// Tracks:
/// - When players run out of deck cards
/// - First appearance of cards on the mat (in play)
/// - Hand sizes at end of each turn
/// - First time cards use attacks
pub struct GameplayStatsCollector {
    // Current game tracking
    current_game_id: Option<Uuid>,
    current_turn: u32,
    num_games: u32,

    // Game length statistics (sum and count for computing average)
    game_length_sum: f64,

    // Deck empty statistics per player (sum and count)
    deck_empty_sum: HashMap<usize, f64>,
    deck_empty_count: HashMap<usize, usize>,

    // Hand size statistics per player - sum and count
    hand_size_sum: [f64; 2],
    hand_size_count: [usize; 2],

    // Card first seen statistics per (player, card) - sum and count
    card_seen_sum: HashMap<(usize, String), f64>,
    card_seen_count: HashMap<(usize, String), usize>,

    // Attack first used statistics per (player, card, attack_idx) - sum and count
    attack_used_sum: HashMap<(usize, String, u8), f64>,
    attack_used_count: HashMap<(usize, String, u8), usize>,

    // Temporary tracking for current game
    seen_cards: HashMap<usize, HashSet<String>>,
    used_attacks: HashMap<usize, HashSet<(String, u8)>>,
}

impl Default for GameplayStatsCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl GameplayStatsCollector {
    pub fn new() -> Self {
        Self {
            current_game_id: None,
            current_turn: 0,
            num_games: 0,

            game_length_sum: 0.0,

            deck_empty_sum: HashMap::new(),
            deck_empty_count: HashMap::new(),

            hand_size_sum: [0.0; 2],
            hand_size_count: [0; 2],

            card_seen_sum: HashMap::new(),
            card_seen_count: HashMap::new(),

            attack_used_sum: HashMap::new(),
            attack_used_count: HashMap::new(),

            seen_cards: HashMap::new(),
            used_attacks: HashMap::new(),
        }
    }

    /// Track cards currently on the mat
    fn track_cards_on_mat(&mut self, state: &State) {
        for player in 0..2 {
            for played_card in state.in_play_pokemon[player].iter().flatten() {
                let card_id = played_card.card.get_id();

                // Check if this is the first time we've seen this card in this game
                if !self
                    .seen_cards
                    .entry(player)
                    .or_default()
                    .contains(&card_id)
                {
                    self.seen_cards
                        .get_mut(&player)
                        .unwrap()
                        .insert(card_id.clone());

                    // Add to sum and count for computing average later
                    let key = (player, card_id);
                    *self.card_seen_sum.entry(key.clone()).or_insert(0.0) +=
                        self.current_turn as f64;
                    *self.card_seen_count.entry(key).or_insert(0) += 1;
                }
            }
        }
    }

    /// Track when players run out of deck cards
    fn track_deck_empty(&mut self, state: &State) {
        for player in 0..2 {
            let deck_size = state.decks[player].cards.len();

            // If deck is empty and we haven't recorded it yet for this game
            if deck_size == 0
                && !self
                    .seen_cards
                    .get(&player)
                    .is_some_and(|s| s.contains("__deck_empty__"))
            {
                // Use a special marker to track if we've already recorded deck empty for this player in this game
                self.seen_cards
                    .entry(player)
                    .or_default()
                    .insert("__deck_empty__".to_string());

                // Add to sum and count
                *self.deck_empty_sum.entry(player).or_insert(0.0) += self.current_turn as f64;
                *self.deck_empty_count.entry(player).or_insert(0) += 1;
            }
        }
    }

    /// Track hand sizes at end of turn
    fn track_hand_sizes(&mut self, state: &State) {
        for player in 0..2 {
            let hand_size = state.hands[player].len();

            // Add to sum and count for this player
            self.hand_size_sum[player] += hand_size as f64;
            self.hand_size_count[player] += 1;
        }
    }

    /// Track when a card uses an attack
    fn track_attack_used(&mut self, state: &State, actor: usize, action: &Action) {
        if let SimpleAction::Attack(attack_idx) = action.action {
            // Get the active Pokemon that used the attack
            if let Some(active_pokemon) = &state.in_play_pokemon[actor][0] {
                let card_id = active_pokemon.card.get_id();
                let attack_key = (card_id.clone(), attack_idx as u8);

                // Check if this is the first time this specific attack was used in this game
                if !self
                    .used_attacks
                    .entry(actor)
                    .or_default()
                    .contains(&attack_key)
                {
                    self.used_attacks
                        .get_mut(&actor)
                        .unwrap()
                        .insert(attack_key.clone());

                    // Add to sum and count
                    let key = (actor, card_id, attack_idx as u8);
                    *self.attack_used_sum.entry(key.clone()).or_insert(0.0) +=
                        self.current_turn as f64;
                    *self.attack_used_count.entry(key).or_insert(0) += 1;
                }
            }
        }
    }

    /// Compute aggregated statistics from all collected data
    pub fn compute_stats(&self) -> AggregatedStats {
        // 1. Game length
        let avg_game_length = if self.num_games > 0 {
            self.game_length_sum / self.num_games as f64
        } else {
            0.0
        };

        // 2. Deck Empty Statistics (indexed by player)
        let mut deck_empty_avg = [None; 2];
        for (player, entry) in deck_empty_avg.iter_mut().enumerate() {
            if let Some(&count) = self.deck_empty_count.get(&player) {
                if count > 0 {
                    let sum = self.deck_empty_sum.get(&player).unwrap_or(&0.0);
                    *entry = Some(sum / count as f64);
                }
            }
        }

        // 3. Hand Size Statistics (average across all turns, indexed by player)
        let mut hand_sizes = [0.0; 2];
        for (player, entry) in hand_sizes.iter_mut().enumerate() {
            if self.hand_size_count[player] > 0 {
                *entry = self.hand_size_sum[player] / self.hand_size_count[player] as f64;
            }
        }

        // 4. Cards on Mat Statistics (indexed by player, then card_id)
        let mut cards_seen = [HashMap::new(), HashMap::new()];
        for ((player, card_id), count) in &self.card_seen_count {
            if *count > 0 {
                let sum = self
                    .card_seen_sum
                    .get(&(*player, card_id.clone()))
                    .unwrap_or(&0.0);
                let avg = sum / *count as f64;
                cards_seen[*player].insert(card_id.clone(), avg);
            }
        }

        // 5. Attack Usage Statistics (indexed by player, then (card_id, attack_idx))
        let mut attacks_used = [HashMap::new(), HashMap::new()];
        for ((player, card_id, attack_idx), count) in &self.attack_used_count {
            if *count > 0 {
                let sum = self
                    .attack_used_sum
                    .get(&(*player, card_id.clone(), *attack_idx))
                    .unwrap_or(&0.0);
                let avg = sum / *count as f64;
                attacks_used[*player].insert((card_id.clone(), *attack_idx), avg);
            }
        }

        AggregatedStats {
            total_games: self.num_games as usize,
            avg_game_length,
            deck_empty_avg,
            hand_sizes,
            cards_seen,
            attacks_used,
        }
    }
}

impl SimulationEventHandler for GameplayStatsCollector {
    fn on_game_start(&mut self, game_id: Uuid) {
        self.current_game_id = Some(game_id);
        self.current_turn = 0;

        // Reset per-game tracking
        self.seen_cards.clear();
        self.used_attacks.clear();
    }

    fn on_action(
        &mut self,
        _game_id: Uuid,
        state_before_action: &State,
        actor: usize,
        _playable_actions: &[Action],
        action: &Action,
    ) {
        // Track attack usage before the action is applied
        self.track_attack_used(state_before_action, actor, action);

        // Listen specifically for EndTurn actions
        if matches!(action.action, SimpleAction::EndTurn) {
            // Increment turn counter
            self.current_turn += 1;

            // Track all statistics at the end of turn
            self.track_cards_on_mat(state_before_action);
            self.track_deck_empty(state_before_action);
            self.track_hand_sizes(state_before_action);
        }
    }

    fn on_game_end(
        &mut self,
        _game_id: Uuid,
        state: State,
        _result: Option<crate::state::GameOutcome>,
    ) {
        // Track game length
        self.game_length_sum += state.turn_count as f64;

        self.num_games += 1;
        self.current_game_id = None;
    }

    // Statistics are computed on-demand via compute_stats()

    fn merge(&mut self, other: &dyn SimulationEventHandler) {
        if let Some(other_collector) =
            (other as &dyn std::any::Any).downcast_ref::<GameplayStatsCollector>()
        {
            // Merge game counts
            self.num_games += other_collector.num_games;

            // Merge game length
            self.game_length_sum += other_collector.game_length_sum;

            // Merge deck empty statistics
            for (player, sum) in &other_collector.deck_empty_sum {
                *self.deck_empty_sum.entry(*player).or_insert(0.0) += sum;
            }
            for (player, count) in &other_collector.deck_empty_count {
                *self.deck_empty_count.entry(*player).or_insert(0) += count;
            }

            // Merge hand size statistics
            for player in 0..2 {
                self.hand_size_sum[player] += other_collector.hand_size_sum[player];
                self.hand_size_count[player] += other_collector.hand_size_count[player];
            }

            // Merge card seen statistics
            for (key, sum) in &other_collector.card_seen_sum {
                *self.card_seen_sum.entry(key.clone()).or_insert(0.0) += sum;
            }
            for (key, count) in &other_collector.card_seen_count {
                *self.card_seen_count.entry(key.clone()).or_insert(0) += count;
            }

            // Merge attack used statistics
            for (key, sum) in &other_collector.attack_used_sum {
                *self.attack_used_sum.entry(key.clone()).or_insert(0.0) += sum;
            }
            for (key, count) in &other_collector.attack_used_count {
                *self.attack_used_count.entry(key.clone()).or_insert(0) += count;
            }
        } else {
            panic!("Attempted to merge GameplayStatsCollector with incompatible type");
        }
    }
}
