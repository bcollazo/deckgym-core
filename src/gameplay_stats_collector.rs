use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use crate::{
    actions::{Action, SimpleAction},
    simulation_event_handler::SimulationEventHandler,
    state::GameOutcome,
    State,
};

/// Statistics for when a player's deck became empty
#[derive(Debug, Clone)]
pub struct DeckEmptyStats {
    pub player: usize,
    pub avg_turn: f64,
    pub games_empty: usize,
}

/// Statistics for when a card first appeared on the mat
#[derive(Debug, Clone)]
pub struct CardSeenStats {
    pub player: usize,
    pub card_id: String,
    pub avg_turn: f64,
}

/// Statistics for when a card first used an attack
#[derive(Debug, Clone)]
pub struct AttackUsedStats {
    pub player: usize,
    pub card_id: String,
    pub avg_turn: f64,
}

/// Statistics for hand sizes per turn
#[derive(Debug, Clone)]
pub struct HandSizeStats {
    pub player: usize,
    pub turn: u32,
    pub avg_size: f64,
}

/// Aggregated statistics from all games
#[derive(Debug, Clone)]
pub struct AggregatedStats {
    pub total_games: usize,
    pub avg_game_length: f64,
    pub player_0_wins: usize,
    pub player_1_wins: usize,
    pub ties: usize,
    pub deck_empty: Vec<DeckEmptyStats>,
    pub hand_sizes: Vec<HandSizeStats>,
    pub cards_seen: Vec<CardSeenStats>,
    pub attacks_used: Vec<AttackUsedStats>,
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

    // Game outcomes
    player_0_wins: u32,
    player_1_wins: u32,
    ties: u32,

    // Game length statistics (sum and count for computing average)
    game_length_sum: f64,

    // Deck empty statistics per player (sum and count)
    deck_empty_sum: HashMap<usize, f64>,
    deck_empty_count: HashMap<usize, usize>,

    // Hand size statistics per (player, turn) - sum and count
    hand_size_sum: HashMap<(usize, u32), f64>,
    hand_size_count: HashMap<(usize, u32), usize>,

    // Card first seen statistics per (player, card) - sum and count
    card_seen_sum: HashMap<(usize, String), f64>,
    card_seen_count: HashMap<(usize, String), usize>,

    // Attack first used statistics per (player, card) - sum and count
    attack_used_sum: HashMap<(usize, String), f64>,
    attack_used_count: HashMap<(usize, String), usize>,

    // Temporary tracking for current game
    seen_cards: HashMap<usize, HashSet<String>>,
    used_attacks: HashMap<usize, HashSet<String>>,
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

            player_0_wins: 0,
            player_1_wins: 0,
            ties: 0,

            game_length_sum: 0.0,

            deck_empty_sum: HashMap::new(),
            deck_empty_count: HashMap::new(),

            hand_size_sum: HashMap::new(),
            hand_size_count: HashMap::new(),

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

            // Add to sum and count for this (player, turn) combination
            let key = (player, self.current_turn);
            *self.hand_size_sum.entry(key).or_insert(0.0) += hand_size as f64;
            *self.hand_size_count.entry(key).or_insert(0) += 1;
        }
    }

    /// Track when a card uses an attack
    fn track_attack_used(&mut self, state: &State, actor: usize, action: &Action) {
        if let SimpleAction::Attack(_attack_idx) = action.action {
            // Get the active Pokemon that used the attack
            if let Some(active_pokemon) = &state.in_play_pokemon[actor][0] {
                let card_id = active_pokemon.card.get_id();

                // Check if this is the first time this card used an attack in this game
                if !self
                    .used_attacks
                    .entry(actor)
                    .or_default()
                    .contains(&card_id)
                {
                    self.used_attacks
                        .get_mut(&actor)
                        .unwrap()
                        .insert(card_id.clone());

                    // Add to sum and count
                    let key = (actor, card_id);
                    *self.attack_used_sum.entry(key.clone()).or_insert(0.0) +=
                        self.current_turn as f64;
                    *self.attack_used_count.entry(key).or_insert(0) += 1;
                }
            }
        }
    }

    /// Compute aggregated statistics from all collected data
    pub fn compute_stats(&self) -> AggregatedStats {
        // 1. Game length and outcomes
        let avg_game_length = if self.num_games > 0 {
            self.game_length_sum / self.num_games as f64
        } else {
            0.0
        };

        // 2. Deck Empty Statistics
        let mut deck_empty = Vec::new();
        for player in 0..2 {
            if let Some(&count) = self.deck_empty_count.get(&player) {
                if count > 0 {
                    let sum = self.deck_empty_sum.get(&player).unwrap_or(&0.0);
                    deck_empty.push(DeckEmptyStats {
                        player,
                        avg_turn: sum / count as f64,
                        games_empty: count,
                    });
                }
            }
        }

        // 3. Hand Size Statistics
        let mut hand_sizes = Vec::new();
        let mut turn_player_keys: Vec<_> = self.hand_size_sum.keys().collect();
        turn_player_keys.sort();

        for &key in &turn_player_keys {
            let count = self.hand_size_count.get(key).unwrap_or(&0);
            if *count > 0 {
                let sum = self.hand_size_sum.get(key).unwrap_or(&0.0);
                hand_sizes.push(HandSizeStats {
                    player: key.0,
                    turn: key.1,
                    avg_size: sum / *count as f64,
                });
            }
        }

        // 4. Cards on Mat Statistics
        let mut cards_seen = Vec::new();
        for (key, count) in &self.card_seen_count {
            if *count > 0 {
                let sum = self.card_seen_sum.get(key).unwrap_or(&0.0);
                cards_seen.push(CardSeenStats {
                    player: key.0,
                    card_id: key.1.clone(),
                    avg_turn: sum / *count as f64,
                });
            }
        }

        // 5. Attack Usage Statistics
        let mut attacks_used = Vec::new();
        for (key, count) in &self.attack_used_count {
            if *count > 0 {
                let sum = self.attack_used_sum.get(key).unwrap_or(&0.0);
                attacks_used.push(AttackUsedStats {
                    player: key.0,
                    card_id: key.1.clone(),
                    avg_turn: sum / *count as f64,
                });
            }
        }

        AggregatedStats {
            total_games: self.num_games as usize,
            avg_game_length,
            player_0_wins: self.player_0_wins as usize,
            player_1_wins: self.player_1_wins as usize,
            ties: self.ties as usize,
            deck_empty,
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

    fn on_game_end(&mut self, _game_id: Uuid, state: State, result: Option<GameOutcome>) {
        // Track game length
        self.game_length_sum += state.turn_count as f64;

        // Track game outcome
        match result {
            Some(GameOutcome::Win(0)) => self.player_0_wins += 1,
            Some(GameOutcome::Win(1)) => self.player_1_wins += 1,
            Some(GameOutcome::Tie) => self.ties += 1,
            _ => {}
        }

        self.num_games += 1;
        self.current_game_id = None;
    }

    // Statistics are computed on-demand via compute_stats()

    fn merge(&mut self, other: &dyn SimulationEventHandler) {
        if let Some(other_collector) =
            (other as &dyn std::any::Any).downcast_ref::<GameplayStatsCollector>()
        {
            // Merge game counts and outcomes
            self.num_games += other_collector.num_games;
            self.player_0_wins += other_collector.player_0_wins;
            self.player_1_wins += other_collector.player_1_wins;
            self.ties += other_collector.ties;

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
            for (key, sum) in &other_collector.hand_size_sum {
                *self.hand_size_sum.entry(*key).or_insert(0.0) += sum;
            }
            for (key, count) in &other_collector.hand_size_count {
                *self.hand_size_count.entry(*key).or_insert(0) += count;
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
