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
    pub games_empty: usize,
    pub total_games: usize,
    pub avg_turn: f64,
    pub min_turn: u32,
    pub max_turn: u32,
}

/// Statistics for when a card first appeared on the mat
#[derive(Debug, Clone)]
pub struct CardSeenStats {
    pub card_id: String,
    pub player: usize,
    pub games_seen: usize,
    pub total_games: usize,
    pub avg_turn: f64,
}

/// Statistics for when a card first used an attack
#[derive(Debug, Clone)]
pub struct AttackUsedStats {
    pub card_id: String,
    pub player: usize,
    pub games_used: usize,
    pub total_games: usize,
    pub avg_turn: f64,
}

/// Statistics for game endings
#[derive(Debug, Clone)]
pub struct GameEndStats {
    pub total_games: usize,
    pub avg_length: f64,
    pub min_length: u32,
    pub max_length: u32,
    pub player_0_wins: usize,
    pub player_1_wins: usize,
    pub ties: usize,
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
    pub game_end: GameEndStats,
    pub deck_empty: Vec<DeckEmptyStats>,
    pub hand_sizes: Vec<HandSizeStats>,
    pub cards_seen: Vec<CardSeenStats>,
    pub attacks_used: Vec<AttackUsedStats>,
}

/// Simple structure for per-player data with consistent naming
#[derive(Debug, Clone, Default)]
struct PerPlayerData<T> {
    players: [T; 2],
}

impl<T> PerPlayerData<T> {
    fn get(&self, player: usize) -> &T {
        &self.players[player]
    }

    fn get_mut(&mut self, player: usize) -> &mut T {
        &mut self.players[player]
    }
}

impl<T: Default> PerPlayerData<T> {
    fn default_new() -> Self {
        Self {
            players: [T::default(), T::default()],
        }
    }
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

    // Per-game, per-player statistics
    /// Turn when each player's deck became empty (per game, per player)
    deck_empty_turn: HashMap<Uuid, PerPlayerData<Option<u32>>>,

    /// Turn when each card first appeared on mat (per game, per player, per card)
    card_first_seen: HashMap<Uuid, PerPlayerData<HashMap<String, u32>>>,

    /// Turn when each card first used attack (per game, per player, per card)
    attack_first_used: HashMap<Uuid, PerPlayerData<HashMap<String, u32>>>,

    // Per-game statistics
    /// Turn when each game ended (per game)
    game_end_turn: HashMap<Uuid, u32>,

    /// Outcome for each game (per game)
    game_outcome: HashMap<Uuid, Option<GameOutcome>>,

    // Aggregated statistics across all games
    /// Hand sizes at end of each turn (per player, aggregated across all games)
    hand_sizes: PerPlayerData<HashMap<u32, Vec<usize>>>,

    // Temporary tracking for current game
    seen_cards: PerPlayerData<HashSet<String>>,
    used_attacks: PerPlayerData<HashSet<String>>,
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

            deck_empty_turn: HashMap::new(),
            card_first_seen: HashMap::new(),
            attack_first_used: HashMap::new(),
            game_end_turn: HashMap::new(),
            game_outcome: HashMap::new(),
            hand_sizes: PerPlayerData::default_new(),

            seen_cards: PerPlayerData::default_new(),
            used_attacks: PerPlayerData::default_new(),
        }
    }

    /// Track cards currently on the mat
    fn track_cards_on_mat(&mut self, state: &State) {
        let game_id = self.current_game_id.expect("No current game");

        for player in 0..2 {
            for played_card in state.in_play_pokemon[player].iter().flatten() {
                let card_id = played_card.card.get_id();

                // Check if this is the first time we've seen this card
                if !self.seen_cards.get(player).contains(&card_id) {
                    self.seen_cards.get_mut(player).insert(card_id.clone());

                    // Record the turn this card first appeared
                    self.card_first_seen
                        .entry(game_id)
                        .or_insert_with(PerPlayerData::default_new)
                        .get_mut(player)
                        .insert(card_id, self.current_turn);
                }
            }
        }
    }

    /// Track when players run out of deck cards
    fn track_deck_empty(&mut self, state: &State) {
        let game_id = self.current_game_id.expect("No current game");

        for player in 0..2 {
            let deck_size = state.decks[player].cards.len();

            // If deck is empty and we haven't recorded it yet
            if deck_size == 0 {
                let entry = self
                    .deck_empty_turn
                    .entry(game_id)
                    .or_insert_with(PerPlayerData::default_new);

                if entry.get(player).is_none() {
                    *entry.get_mut(player) = Some(self.current_turn);
                }
            }
        }
    }

    /// Track hand sizes at end of turn
    fn track_hand_sizes(&mut self, state: &State) {
        for player in 0..2 {
            let hand_size = state.hands[player].len();

            self.hand_sizes
                .get_mut(player)
                .entry(self.current_turn)
                .or_default()
                .push(hand_size);
        }
    }

    /// Track when a card uses an attack
    fn track_attack_used(&mut self, state: &State, actor: usize, action: &Action) {
        if let SimpleAction::Attack(_attack_idx) = action.action {
            // Get the active Pokemon that used the attack
            if let Some(active_pokemon) = &state.in_play_pokemon[actor][0] {
                let card_id = active_pokemon.card.get_id();

                // Check if this is the first time this card used an attack
                if !self.used_attacks.get(actor).contains(&card_id) {
                    self.used_attacks.get_mut(actor).insert(card_id.clone());

                    let game_id = self.current_game_id.expect("No current game");

                    // Record the turn this card first used an attack
                    self.attack_first_used
                        .entry(game_id)
                        .or_insert_with(PerPlayerData::default_new)
                        .get_mut(actor)
                        .insert(card_id, self.current_turn);
                }
            }
        }
    }

    /// Compute aggregated statistics from all collected data
    pub fn compute_stats(&self) -> AggregatedStats {
        // 1. Game Ending Statistics
        let end_turns: Vec<u32> = self.game_end_turn.values().copied().collect();
        let (avg_length, min_length, max_length) = if !end_turns.is_empty() {
            let avg = end_turns.iter().sum::<u32>() as f64 / end_turns.len() as f64;
            let min = *end_turns.iter().min().unwrap();
            let max = *end_turns.iter().max().unwrap();
            (avg, min, max)
        } else {
            (0.0, 0, 0)
        };

        // Count outcomes
        let mut player_0_wins = 0;
        let mut player_1_wins = 0;
        let mut ties = 0;
        for outcome in self.game_outcome.values() {
            match outcome {
                Some(GameOutcome::Win(0)) => player_0_wins += 1,
                Some(GameOutcome::Win(1)) => player_1_wins += 1,
                Some(GameOutcome::Tie) => ties += 1,
                _ => {}
            }
        }

        let game_end = GameEndStats {
            total_games: self.num_games as usize,
            avg_length,
            min_length,
            max_length,
            player_0_wins,
            player_1_wins,
            ties,
        };

        // 2. Deck Empty Statistics
        let mut deck_empty = Vec::new();
        for player in 0..2 {
            let mut empty_turns = Vec::new();
            for turns in self.deck_empty_turn.values() {
                if let Some(turn) = turns.get(player) {
                    empty_turns.push(*turn);
                }
            }

            if !empty_turns.is_empty() {
                let avg = empty_turns.iter().sum::<u32>() as f64 / empty_turns.len() as f64;
                let min = *empty_turns.iter().min().unwrap();
                let max = *empty_turns.iter().max().unwrap();
                deck_empty.push(DeckEmptyStats {
                    player,
                    games_empty: empty_turns.len(),
                    total_games: self.num_games as usize,
                    avg_turn: avg,
                    min_turn: min,
                    max_turn: max,
                });
            }
        }

        // 3. Hand Size Statistics
        let mut hand_sizes = Vec::new();
        for player in 0..2 {
            let mut turns: Vec<_> = self.hand_sizes.get(player).keys().collect();
            turns.sort();

            for &turn in turns.iter() {
                let sizes = &self.hand_sizes.get(player)[turn];
                let avg = sizes.iter().sum::<usize>() as f64 / sizes.len() as f64;
                hand_sizes.push(HandSizeStats {
                    player,
                    turn: *turn,
                    avg_size: avg,
                });
            }
        }

        // 4. Cards on Mat Statistics
        let mut cards_seen = Vec::new();
        for player in 0..2 {
            let mut all_cards = HashSet::new();
            for cards in self.card_first_seen.values() {
                for card_id in cards.get(player).keys() {
                    all_cards.insert(card_id.clone());
                }
            }

            for card_id in all_cards.iter() {
                let mut turns = Vec::new();
                for cards in self.card_first_seen.values() {
                    if let Some(turn) = cards.get(player).get(card_id) {
                        turns.push(*turn);
                    }
                }
                let avg = turns.iter().sum::<u32>() as f64 / turns.len() as f64;
                cards_seen.push(CardSeenStats {
                    card_id: card_id.clone(),
                    player,
                    games_seen: turns.len(),
                    total_games: self.num_games as usize,
                    avg_turn: avg,
                });
            }
        }

        // 5. Attack Usage Statistics
        let mut attacks_used = Vec::new();
        for player in 0..2 {
            let mut all_cards = HashSet::new();
            for attacks in self.attack_first_used.values() {
                for card_id in attacks.get(player).keys() {
                    all_cards.insert(card_id.clone());
                }
            }

            for card_id in all_cards.iter() {
                let mut turns = Vec::new();
                for attacks in self.attack_first_used.values() {
                    if let Some(turn) = attacks.get(player).get(card_id) {
                        turns.push(*turn);
                    }
                }
                let avg = turns.iter().sum::<u32>() as f64 / turns.len() as f64;
                attacks_used.push(AttackUsedStats {
                    card_id: card_id.clone(),
                    player,
                    games_used: turns.len(),
                    total_games: self.num_games as usize,
                    avg_turn: avg,
                });
            }
        }

        AggregatedStats {
            game_end,
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
        self.seen_cards = PerPlayerData::default_new();
        self.used_attacks = PerPlayerData::default_new();
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

    fn on_game_end(&mut self, game_id: Uuid, state: State, result: Option<GameOutcome>) {
        // Record the turn when this game ended
        self.game_end_turn.insert(game_id, state.turn_count as u32);

        // Record the game outcome
        self.game_outcome.insert(game_id, result);

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

            // Merge per-game statistics
            self.deck_empty_turn
                .extend(other_collector.deck_empty_turn.clone());
            self.card_first_seen
                .extend(other_collector.card_first_seen.clone());
            self.attack_first_used
                .extend(other_collector.attack_first_used.clone());
            self.game_end_turn
                .extend(other_collector.game_end_turn.clone());
            self.game_outcome
                .extend(other_collector.game_outcome.clone());

            // Merge aggregated hand sizes
            for player in 0..2 {
                for (turn, sizes) in other_collector.hand_sizes.get(player).iter() {
                    self.hand_sizes
                        .get_mut(player)
                        .entry(*turn)
                        .or_default()
                        .extend(sizes);
                }
            }
        } else {
            panic!("Attempted to merge GameplayStatsCollector with incompatible type");
        }
    }
}
