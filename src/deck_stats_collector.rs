use log::warn;
use num_format::{Locale, ToFormattedString};
use std::{any, collections::HashMap};
use uuid::Uuid;

use crate::{
    actions::{Action, SimpleAction},
    models::Card,
    simulation_event_handler::SimulationEventHandler,
    state::GameOutcome,
    State,
};

/// Tracks detailed deck statistics across multiple games for deck building analysis
pub struct DeckStatsCollector {
    num_games: u32,

    // Per-game tracking (cleared on game start)
    current_game_pokemon_mat_turns: HashMap<String, u8>,
    current_game_damage_dealt: [u32; 2],
    current_game_total_energy_on_field: u32,
    current_game_total_cards_in_hands: u32,
    current_game_deck_out_turn: [Option<u8>; 2],

    // Accumulators for per-game totals
    current_game_energy_attached: u32,
    current_game_cards_drawn: u32,

    // Aggregate statistics across all games
    // Turn when deck became empty (deck out)
    deck_out_turns: Vec<u8>,

    // Map from Pokemon name to list of turns when it appeared on mat
    pokemon_mat_appearances: HashMap<String, Vec<u8>>,

    // Total damage dealt per player across all games
    total_damage_per_player: [u32; 2],

    // Energy attached per turn distribution
    energy_attached_per_game: Vec<u32>,

    // Cards drawn per game
    cards_drawn_per_game: Vec<u32>,

    // First KO turn distribution
    first_ko_turns: Vec<u8>,

    // Track which Pokemon was played first (most common starting Pokemon)
    first_pokemon_played: HashMap<String, u32>,

    // Average bench size per turn
    bench_sizes: Vec<u8>,

    // Number of prizes taken per game
    prizes_per_game: Vec<u8>,

    // Track damage dealt per game for distribution analysis
    damage_per_game: Vec<u32>,
}

impl Default for DeckStatsCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl DeckStatsCollector {
    pub fn new() -> Self {
        Self {
            num_games: 0,
            current_game_pokemon_mat_turns: HashMap::new(),
            current_game_damage_dealt: [0, 0],
            current_game_total_energy_on_field: 0,
            current_game_total_cards_in_hands: 0,
            current_game_deck_out_turn: [None, None],
            current_game_energy_attached: 0,
            current_game_cards_drawn: 0,
            deck_out_turns: Vec::new(),
            pokemon_mat_appearances: HashMap::new(),
            total_damage_per_player: [0, 0],
            energy_attached_per_game: Vec::new(),
            cards_drawn_per_game: Vec::new(),
            first_ko_turns: Vec::new(),
            first_pokemon_played: HashMap::new(),
            bench_sizes: Vec::new(),
            prizes_per_game: Vec::new(),
            damage_per_game: Vec::new(),
        }
    }

    fn count_energy_on_field(&self, state: &State) -> u32 {
        let mut total = 0u32;
        for player in 0..2 {
            for slot in &state.in_play_pokemon[player] {
                if let Some(pokemon) = slot {
                    total += pokemon.attached_energy.len() as u32;
                }
            }
        }
        total
    }

    fn count_cards_drawn_from_decks(&self, state: &State) -> u32 {
        // Calculate cards drawn by comparing initial deck size to current deck size
        let initial_deck_size: u32 = 20; // Standard Pokemon TCG Pocket deck size
        let cards_remaining = (state.decks[0].cards.len() + state.decks[1].cards.len()) as u32;
        let total_initial = initial_deck_size * 2;
        total_initial.saturating_sub(cards_remaining)
    }

    fn track_pokemon_on_mat(&mut self, state: &State) {
        for player in 0..2 {
            for slot in &state.in_play_pokemon[player] {
                if let Some(played_card) = slot {
                    let pokemon_name = played_card.card.get_name();
                    // Only record the first time this Pokemon appears
                    if !self
                        .current_game_pokemon_mat_turns
                        .contains_key(&pokemon_name)
                    {
                        self.current_game_pokemon_mat_turns
                            .insert(pokemon_name, state.turn_count);
                    }
                }
            }
        }
    }

    fn track_deck_out(&mut self, state: &State) {
        for player in 0..2 {
            if self.current_game_deck_out_turn[player].is_none()
                && state.decks[player].cards.is_empty()
            {
                self.current_game_deck_out_turn[player] = Some(state.turn_count);
            }
        }
    }

    fn track_bench_size(&mut self, state: &State) {
        for player in 0..2 {
            let bench_size = state.in_play_pokemon[player]
                .iter()
                .skip(1) // Skip active Pokemon
                .filter(|slot| slot.is_some())
                .count() as u8;
            self.bench_sizes.push(bench_size);
        }
    }

}

impl SimulationEventHandler for DeckStatsCollector {
    fn on_game_start(&mut self, _game_id: Uuid) {
        // Reset per-game tracking
        self.current_game_pokemon_mat_turns.clear();
        self.current_game_damage_dealt = [0, 0];
        self.current_game_total_energy_on_field = 0;
        self.current_game_total_cards_in_hands = 0; // Will be set on first snapshot
        self.current_game_deck_out_turn = [None, None];
        self.current_game_energy_attached = 0;
        self.current_game_cards_drawn = 0;
    }

    fn on_action(
        &mut self,
        _game_id: Uuid,
        state_before_action: &State,
        _actor: usize,
        _playable_actions: &[Action],
        action: &Action,
    ) {
        // Track first Pokemon played
        if let SimpleAction::Place(card, _idx) = &action.action {
            if let Card::Pokemon(_) = card {
                let pokemon_name = card.get_name();
                if self.current_game_pokemon_mat_turns.is_empty() {
                    *self.first_pokemon_played.entry(pokemon_name).or_insert(0) += 1;
                }
            }
        }

        // Take snapshot at end of turn to capture all state changes
        if matches!(action.action, SimpleAction::EndTurn) {
            // Count current state
            let current_energy = self.count_energy_on_field(state_before_action);
            let current_cards_drawn = self.count_cards_drawn_from_decks(state_before_action);

            // Energy delta since last turn
            let energy_delta = current_energy.saturating_sub(self.current_game_total_energy_on_field);
            self.current_game_energy_attached += energy_delta;

            // Cards drawn is absolute from deck size
            self.current_game_cards_drawn = current_cards_drawn;

            // Update tracking for next turn
            self.current_game_total_energy_on_field = current_energy;

            // Track other per-turn stats
            self.track_bench_size(state_before_action);
        }

        // Always track these
        self.track_pokemon_on_mat(state_before_action);
        self.track_deck_out(state_before_action);
    }

    fn on_game_end(&mut self, _game_id: Uuid, state: State, _outcome: Option<GameOutcome>) {
        self.num_games += 1;

        // Estimate damage based on total HP lost across all Pokemon
        // This includes KO'd Pokemon in discard pile
        for player in 0..2 {
            let mut hp_lost = 0u32;

            // Count HP lost from Pokemon still in play
            for pokemon_slot in &state.in_play_pokemon[player] {
                if let Some(pokemon) = pokemon_slot {
                    hp_lost += pokemon.total_hp - pokemon.remaining_hp;
                }
            }

            // Damage dealt TO this player is credited to the opponent
            let opponent = 1 - player;
            self.current_game_damage_dealt[opponent] += hp_lost;
        }

        // Record deck out turns
        for player in 0..2 {
            if let Some(turn) = self.current_game_deck_out_turn[player] {
                self.deck_out_turns.push(turn);
            }
        }

        // Record Pokemon mat appearances
        for (pokemon_name, turn) in self.current_game_pokemon_mat_turns.drain() {
            self.pokemon_mat_appearances
                .entry(pokemon_name)
                .or_insert_with(Vec::new)
                .push(turn);
        }

        // Record energy attached and cards drawn
        self.energy_attached_per_game
            .push(self.current_game_energy_attached);
        self.cards_drawn_per_game
            .push(self.current_game_cards_drawn);

        // Record prizes
        let total_prizes = state.points[0] + state.points[1];
        self.prizes_per_game.push(total_prizes);

        // Track damage dealt per game
        let total_damage = self.current_game_damage_dealt[0] + self.current_game_damage_dealt[1];
        self.damage_per_game.push(total_damage);
        self.total_damage_per_player[0] += self.current_game_damage_dealt[0];
        self.total_damage_per_player[1] += self.current_game_damage_dealt[1];
    }

    fn merge(&mut self, other: &dyn SimulationEventHandler) {
        if let Some(other_typed) = (other as &dyn any::Any).downcast_ref::<DeckStatsCollector>() {
            self.num_games += other_typed.num_games;
            self.deck_out_turns.extend(&other_typed.deck_out_turns);

            // Merge Pokemon mat appearances
            for (pokemon, turns) in &other_typed.pokemon_mat_appearances {
                self.pokemon_mat_appearances
                    .entry(pokemon.clone())
                    .or_insert_with(Vec::new)
                    .extend(turns);
            }

            self.total_damage_per_player[0] += other_typed.total_damage_per_player[0];
            self.total_damage_per_player[1] += other_typed.total_damage_per_player[1];
            self.energy_attached_per_game
                .extend(&other_typed.energy_attached_per_game);
            self.cards_drawn_per_game
                .extend(&other_typed.cards_drawn_per_game);
            self.first_ko_turns.extend(&other_typed.first_ko_turns);

            // Merge first Pokemon played counts
            for (pokemon, count) in &other_typed.first_pokemon_played {
                *self.first_pokemon_played.entry(pokemon.clone()).or_insert(0) += count;
            }

            self.bench_sizes.extend(&other_typed.bench_sizes);
            self.prizes_per_game.extend(&other_typed.prizes_per_game);
            self.damage_per_game.extend(&other_typed.damage_per_game);
        } else {
            panic!("Attempted to merge DeckStatsCollector with incompatible type");
        }
    }

    fn on_simulation_end(&mut self) {
        warn!("\n========== DECK STATISTICS ==========");

        // Average turn to deck out
        if !self.deck_out_turns.is_empty() {
            let avg_deck_out_turn =
                self.deck_out_turns.iter().map(|&t| t as f32).sum::<f32>()
                    / self.deck_out_turns.len() as f32;
            warn!(
                "Average Turn to Deck Out: {:.2} (occurred in {} / {} games)",
                avg_deck_out_turn,
                self.deck_out_turns.len().to_formatted_string(&Locale::en),
                self.num_games.to_formatted_string(&Locale::en)
            );
        } else {
            warn!("No deck outs occurred in any game");
        }

        // Average turn to have each Pokemon on mat
        warn!("\nAverage Turn for Pokemon to Appear on Mat:");
        let mut pokemon_stats: Vec<_> = self.pokemon_mat_appearances.iter().collect();
        pokemon_stats.sort_by_key(|(name, _)| *name);
        for (pokemon_name, turns) in pokemon_stats {
            let avg_turn = turns.iter().map(|&t| t as f32).sum::<f32>() / turns.len() as f32;
            let appearance_rate = (turns.len() as f32 / self.num_games as f32) * 100.0;
            warn!(
                "  {}: Turn {:.2} (appeared in {:.1}% of games)",
                pokemon_name, avg_turn, appearance_rate
            );
        }

        // Average damage done
        let avg_damage_p0 = self.total_damage_per_player[0] as f32 / self.num_games as f32;
        let avg_damage_p1 = self.total_damage_per_player[1] as f32 / self.num_games as f32;
        warn!("\nAverage Damage Dealt per Game:");
        warn!("  Player 0: {:.2}", avg_damage_p0);
        warn!("  Player 1: {:.2}", avg_damage_p1);

        if !self.damage_per_game.is_empty() {
            let avg_total_damage = self.damage_per_game.iter().sum::<u32>() as f32
                / self.damage_per_game.len() as f32;
            warn!("  Total per game: {:.2}", avg_total_damage);
        }

        // First Pokemon played
        if !self.first_pokemon_played.is_empty() {
            warn!("\nMost Common First Pokemon Played:");
            let mut first_pokemon: Vec<_> = self.first_pokemon_played.iter().collect();
            first_pokemon.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
            for (pokemon, count) in first_pokemon.iter().take(5) {
                let percentage = (**count as f32 / self.num_games as f32) * 100.0;
                warn!("  {}: {} times ({:.1}%)", pokemon, count, percentage);
            }
        }

        // Cards drawn statistics
        if !self.cards_drawn_per_game.is_empty() {
            let avg_cards_drawn = self.cards_drawn_per_game.iter().sum::<u32>() as f32
                / self.cards_drawn_per_game.len() as f32;
            warn!("\nAverage Cards Drawn per Game: {:.2}", avg_cards_drawn);
        }

        // Energy attached statistics
        if !self.energy_attached_per_game.is_empty() {
            let avg_energy_attached = self.energy_attached_per_game.iter().sum::<u32>() as f32
                / self.energy_attached_per_game.len() as f32;
            warn!(
                "Average Energy Attached per Game: {:.2}",
                avg_energy_attached
            );
        }

        // Prizes statistics
        if !self.prizes_per_game.is_empty() {
            let avg_prizes =
                self.prizes_per_game.iter().map(|&p| p as f32).sum::<f32>()
                    / self.prizes_per_game.len() as f32;
            warn!("Average Prizes Taken per Game: {:.2}", avg_prizes);
        }

        // Bench size statistics
        if !self.bench_sizes.is_empty() {
            let avg_bench_size = self.bench_sizes.iter().map(|&s| s as f32).sum::<f32>()
                / self.bench_sizes.len() as f32;
            warn!("Average Bench Size: {:.2}", avg_bench_size);
        }

        warn!("=====================================\n");
    }
}
