use log::{debug, trace};
use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::hash::Hash;

use crate::{
    actions::SimpleAction,
    deck::Deck,
    effects::TurnEffect,
    models::{Card, EnergyType, PlayedCard},
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameOutcome {
    Win(usize),
    Tie,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct State {
    // Turn State
    pub winner: Option<GameOutcome>,
    pub points: [u8; 2],
    pub turn_count: u8, // Global turn count. Matches TCGPocket app.
    // Player that needs to select from playable actions. Might not be aligned
    // with coin toss and the parity, see Sabrina.
    pub current_player: usize,
    pub move_generation_stack: Vec<(usize, Vec<SimpleAction>)>,

    // Core state
    pub(crate) current_energy: Option<EnergyType>,
    pub hands: [Vec<Card>; 2],
    pub decks: [Deck; 2],
    pub discard_piles: [Vec<Card>; 2],
    pub discard_energies: [Vec<EnergyType>; 2],
    // 0 index is the active pokemon, 1..4 are the bench
    pub in_play_pokemon: [[Option<PlayedCard>; 4]; 2],

    // Turn Flags (remember to reset these in reset_turn_states)
    pub(crate) has_played_support: bool,
    pub(crate) has_retreated: bool,
    pub(crate) knocked_out_by_opponent_attack_this_turn: bool,
    pub(crate) knocked_out_by_opponent_attack_last_turn: bool,
    // Maps turn to a vector of effects (cards) for that turn. Using BTreeMap to keep State hashable.
    turn_effects: BTreeMap<u8, Vec<TurnEffect>>,
}

impl State {
    pub fn new(deck_a: &Deck, deck_b: &Deck) -> Self {
        Self {
            winner: None,
            points: [0, 0],
            turn_count: 0,
            current_player: 0,
            move_generation_stack: Vec::new(),
            current_energy: None,
            hands: [Vec::new(), Vec::new()],
            decks: [deck_a.clone(), deck_b.clone()],
            discard_piles: [Vec::new(), Vec::new()],
            discard_energies: [Vec::new(), Vec::new()],
            in_play_pokemon: [[None, None, None, None], [None, None, None, None]],
            has_played_support: false,
            has_retreated: false,

            knocked_out_by_opponent_attack_this_turn: false,
            knocked_out_by_opponent_attack_last_turn: false,
            turn_effects: BTreeMap::new(),
        }
    }

    pub fn debug_string(&self) -> String {
        format!(
            "P1 Hand:\t{:?}\n\
            P1 InPlay:\t{:?}\n\
            P2 InPlay:\t{:?}\n\
            P2 Hand:\t{:?}",
            to_canonical_names(self.hands[0].as_slice()),
            format_cards(&self.in_play_pokemon[0]),
            format_cards(&self.in_play_pokemon[1]),
            to_canonical_names(self.hands[1].as_slice())
        )
    }

    pub fn initialize(deck_a: &Deck, deck_b: &Deck, rng: &mut impl Rng) -> Self {
        let mut state = Self::new(deck_a, deck_b);

        // Shuffle the decks before starting the game and have players
        //  draw 5 cards each to start
        for deck in &mut state.decks {
            deck.shuffle(true, rng);
        }
        for _ in 0..5 {
            state.maybe_draw_card(0);
            state.maybe_draw_card(1);
        }
        // Flip a coin to determine the starting player
        state.current_player = rng.gen_range(0..2);

        state
    }

    pub fn get_remaining_hp(&self, player: usize, index: usize) -> u32 {
        self.in_play_pokemon[player][index]
            .as_ref()
            .unwrap()
            .remaining_hp
    }

    pub(crate) fn remove_card_from_hand(&mut self, current_player: usize, card: &Card) {
        let index = self.hands[current_player]
            .iter()
            .position(|x| x == card)
            .expect("Player hand should contain card to remove");
        self.hands[current_player].swap_remove(index);
    }

    pub(crate) fn remove_card_from_deck(&mut self, player: usize, card: &Card) {
        let pos = self.decks[player]
            .cards
            .iter()
            .position(|c| c == card)
            .expect("Evolution card should be in deck");
        self.decks[player].cards.remove(pos);
    }

    pub(crate) fn discard_card_from_hand(&mut self, current_player: usize, card: &Card) {
        self.remove_card_from_hand(current_player, card);
        self.discard_piles[current_player].push(card.clone());
    }

    /// Returns an iterator over supporter cards in a player's hand
    pub(crate) fn iter_hand_supporters(&self, player: usize) -> impl Iterator<Item = &Card> {
        self.hands[player].iter().filter(|card| card.is_support())
    }

    pub(crate) fn maybe_draw_card(&mut self, player: usize) {
        if let Some(card) = self.decks[player].draw() {
            self.hands[player].push(card.clone());
            debug!(
                "Player {} drew: {:?}, now hand is: {:?} and deck has {} cards",
                player + 1,
                canonical_name(&card),
                to_canonical_names(&self.hands[player]),
                self.decks[player].cards.len()
            );
        } else {
            debug!("Player {} cannot draw a card, deck is empty", player + 1);
        }
    }

    pub(crate) fn transfer_card_from_deck_to_hand(&mut self, player: usize, card: &Card) {
        // Remove from deck and add to hand
        let pos = self.decks[player]
            .cards
            .iter()
            .position(|c| c == card)
            .expect("Card must exist in deck to transfer to hand");
        self.decks[player].cards.remove(pos);
        self.hands[player].push(card.clone());
    }

    pub(crate) fn transfer_card_from_hand_to_deck(&mut self, player: usize, card: &Card) {
        // Remove from hand and add to deck
        let pos = self.hands[player]
            .iter()
            .position(|c| c == card)
            .expect("Card must exist in hand to transfer to deck");
        self.hands[player].remove(pos);
        self.decks[player].cards.push(card.clone());
    }

    pub(crate) fn iter_deck_pokemon(&self, player: usize) -> impl Iterator<Item = &Card> {
        self.decks[player]
            .cards
            .iter()
            .filter(|card| matches!(card, Card::Pokemon(_)))
    }

    pub fn iter_hand_pokemon(&self, player: usize) -> impl Iterator<Item = &Card> {
        self.hands[player]
            .iter()
            .filter(|card| matches!(card, Card::Pokemon(_)))
    }

    pub(crate) fn generate_energy(&mut self) {
        if self.decks[self.current_player].energy_types.len() == 1 {
            self.current_energy = Some(self.decks[self.current_player].energy_types[0]);
        }

        let deck_energies = &self.decks[self.current_player].energy_types;
        let mut rng = rand::thread_rng();
        let generated = deck_energies
            .choose(&mut rng)
            .expect("Decks should have at least 1 energy");
        self.current_energy = Some(*generated);
    }

    pub(crate) fn end_turn_maintenance(&mut self) {
        // Maintain PlayedCard state for _all_ players
        for i in 0..2 {
            self.in_play_pokemon[i].iter_mut().for_each(|x| {
                if let Some(played_card) = x {
                    played_card.end_turn_maintenance();
                }
            });
        }

        self.has_played_support = false;
        self.has_retreated = false;
    }

    /// Adds an effect card that will remain active for a specified number of turns.
    ///
    /// # Arguments
    ///
    /// * `effect` - The effect to be added.
    /// * `duration` - The number of turns the effect should remain active.
    ///   0 means current turn only,
    ///   1 means current turn and the next turn, etc.
    pub(crate) fn add_turn_effect(&mut self, effect: TurnEffect, duration: u8) {
        for turn_offset in 0..(duration + 1) {
            let target_turn = self.turn_count + turn_offset;
            self.turn_effects
                .entry(target_turn)
                .or_default()
                .push(effect.clone());
            trace!(
                "Adding effect {:?} for {} turns, current turn: {}, target turn: {}",
                effect,
                duration,
                self.turn_count,
                target_turn
            );
        }
    }

    /// Retrieves all effects scheduled for the current turn
    pub(crate) fn get_current_turn_effects(&self) -> Vec<TurnEffect> {
        self.turn_effects
            .get(&self.turn_count)
            .cloned()
            .unwrap_or_default()
    }

    pub fn enumerate_in_play_pokemon(
        &self,
        player: usize,
    ) -> impl Iterator<Item = (usize, &PlayedCard)> {
        self.in_play_pokemon[player]
            .iter()
            .enumerate()
            .filter(|(_, x)| x.is_some())
            .map(|(i, x)| (i, x.as_ref().unwrap()))
    }

    // e.g. returns (1, Weezing) if player 1 has Weezing in 1st bench slot
    pub fn enumerate_bench_pokemon(
        &self,
        player: usize,
    ) -> impl Iterator<Item = (usize, &PlayedCard)> {
        self.enumerate_in_play_pokemon(player)
            .filter(|(i, _)| *i != 0)
    }

    pub(crate) fn queue_draw_action(&mut self, actor: usize, amount: u8) {
        self.move_generation_stack
            .push((actor, vec![SimpleAction::DrawCard { amount }]));
    }

    pub fn maybe_get_active(&self, player: usize) -> Option<&PlayedCard> {
        self.in_play_pokemon[player][0].as_ref()
    }

    pub fn get_active(&self, player: usize) -> &PlayedCard {
        self.in_play_pokemon[player][0]
            .as_ref()
            .expect("Active Pokemon should be there")
    }

    pub(crate) fn get_active_mut(&mut self, player: usize) -> &mut PlayedCard {
        self.in_play_pokemon[player][0]
            .as_mut()
            .expect("Active Pokemon should be there")
    }

    // This function should be called only from turn 1 onwards
    pub(crate) fn advance_turn(&mut self) {
        debug!(
            "Ending turn moving from player {} to player {}",
            self.current_player,
            (self.current_player + 1) % 2
        );
        self.current_player = (self.current_player + 1) % 2;
        self.turn_count += 1;
        self.end_turn_maintenance();
        self.queue_draw_action(self.current_player, 1);
        self.generate_energy();
    }

    pub(crate) fn is_game_over(&self) -> bool {
        self.winner.is_some() || self.turn_count >= 100
    }

    pub(crate) fn num_in_play_of_type(&self, player: usize, energy: EnergyType) -> usize {
        self.enumerate_in_play_pokemon(player)
            .filter(|(_, x)| x.get_energy_type() == Some(energy))
            .count()
    }

    pub(crate) fn is_users_first_turn(&self) -> bool {
        self.turn_count <= 2
    }

    /// Attaches energies from the discard pile to a Pokemon in play.
    /// Removes the specified energies from discard_energies and attaches them to the Pokemon.
    pub(crate) fn attach_energies_from_discard(
        &mut self,
        player: usize,
        in_play_idx: usize,
        energies: &[EnergyType],
    ) {
        // Remove energies from discard pile
        for energy in energies {
            let pos = self.discard_energies[player]
                .iter()
                .position(|e| e == energy)
                .expect("Energy should be in discard pile");
            self.discard_energies[player].remove(pos);
        }

        // Attach energies to Pokemon
        self.in_play_pokemon[player][in_play_idx]
            .as_mut()
            .expect("Pokemon should be there if attaching energy to it")
            .attached_energy
            .extend(energies.iter().cloned());
    }

    /// Discards a Pokemon from play, moving it, its evolution chain, and its energies
    ///  to the discard pile.
    pub(crate) fn discard_from_play(&mut self, ko_receiver: usize, ko_pokemon_idx: usize) {
        let ko_pokemon = self.in_play_pokemon[ko_receiver][ko_pokemon_idx]
            .as_ref()
            .expect("There should be a Pokemon to discard");
        let mut cards_to_discard = ko_pokemon.cards_behind.clone();
        // TODO: Include attached Tools
        cards_to_discard.push(ko_pokemon.card.clone());
        debug!("Discarding: {cards_to_discard:?}");
        self.discard_piles[ko_receiver].extend(cards_to_discard);
        self.discard_energies[ko_receiver].extend(ko_pokemon.attached_energy.iter().cloned());
        self.in_play_pokemon[ko_receiver][ko_pokemon_idx] = None;
    }

    pub(crate) fn discard_from_active(&mut self, actor: usize, to_discard: &[EnergyType]) {
        self.discard_energy_from_in_play(actor, 0, to_discard);
    }

    pub(crate) fn discard_energy_from_in_play(
        &mut self,
        actor: usize,
        in_play_idx: usize,
        to_discard: &[EnergyType],
    ) {
        let pokemon = self.in_play_pokemon[actor][in_play_idx]
            .as_mut()
            .expect("Pokemon should be there if discarding energy");
        let mut discarded: Vec<EnergyType> = Vec::new();
        for energy in to_discard {
            if let Some(pos) = pokemon.attached_energy.iter().position(|e| *e == *energy) {
                pokemon.attached_energy.swap_remove(pos);
                discarded.push(*energy);
            } else {
                panic!("Pokemon does not have energy to discard");
            }
        }
        if !discarded.is_empty() {
            self.discard_energies[actor].extend(discarded);
        }
    }

    /// Triggers promotion from bench or declares winner if no bench pokemon available.
    /// This should be called when the active spot becomes empty (e.g., after KO or discard).
    pub(crate) fn trigger_promotion_or_declare_winner(&mut self, player_with_empty_active: usize) {
        let enumerated_bench_pokemon = self
            .enumerate_bench_pokemon(player_with_empty_active)
            .collect::<Vec<_>>();

        if enumerated_bench_pokemon.is_empty() {
            // If no bench pokemon, opponent wins
            let opponent = (player_with_empty_active + 1) % 2;
            self.winner = Some(GameOutcome::Win(opponent));
            debug!("Player {player_with_empty_active} lost due to no bench pokemon");
        } else {
            // Queue up promotion actions
            let possible_moves = self
                .enumerate_bench_pokemon(player_with_empty_active)
                .map(|(i, _)| SimpleAction::Activate {
                    player: player_with_empty_active,
                    in_play_idx: i,
                })
                .collect::<Vec<_>>();
            debug!("Triggering Activate moves: {possible_moves:?} to player {player_with_empty_active}");

            // Insert right next to EndTurn, so that if this was triggered by an attack,
            // we resolve any move_generation_stack effects from that attack first.
            // If no EndTurn, just append to end (we could be coming through pokemon checkup poison).
            let index_of_end_turn = self
                .move_generation_stack
                .iter()
                .rposition(|(_, actions)| actions.contains(&SimpleAction::EndTurn));

            if let Some(index_of_end_turn) = index_of_end_turn {
                self.move_generation_stack.insert(
                    index_of_end_turn + 1,
                    (player_with_empty_active, possible_moves),
                );
            } else {
                self.move_generation_stack
                    .push((player_with_empty_active, possible_moves));
            }
        }
    }

    // =========================================================================
    // Test Helper Methods
    // These methods are public for integration tests but should be used carefully
    // =========================================================================

    /// Set the flag indicating a Pokemon was KO'd by opponent's attack last turn.
    /// Used for testing Marshadow's Revenge attack and similar mechanics.
    pub fn set_knocked_out_by_opponent_attack_last_turn(&mut self, value: bool) {
        self.knocked_out_by_opponent_attack_last_turn = value;
    }

    /// Get the flag indicating a Pokemon was KO'd by opponent's attack last turn.
    pub fn get_knocked_out_by_opponent_attack_last_turn(&self) -> bool {
        self.knocked_out_by_opponent_attack_last_turn
    }
}

fn format_cards(played_cards: &[Option<PlayedCard>]) -> Vec<String> {
    played_cards.iter().map(format_card).collect()
}

fn format_card(x: &Option<PlayedCard>) -> String {
    match x {
        Some(played_card) => format!(
            "{}({}hp,{:?})",
            played_card.get_name(),
            played_card.remaining_hp,
            played_card.attached_energy.len(),
        ),
        None => "".to_string(),
    }
}

fn canonical_name(card: &Card) -> &String {
    match card {
        Card::Pokemon(pokemon_card) => &pokemon_card.name,
        Card::Trainer(trainer_card) => &trainer_card.name,
    }
}

fn to_canonical_names(cards: &[Card]) -> Vec<&String> {
    cards.iter().map(canonical_name).collect()
}

#[cfg(test)]
mod tests {
    use crate::{
        card_ids::CardId, database::get_card_by_enum, deck::is_basic, hooks::to_playable_card,
        test_helpers::load_test_decks,
    };

    use super::*;

    #[test]
    fn test_draw_transfers_to_hand() {
        let (deck_a, deck_b) = load_test_decks();
        let mut state = State::new(&deck_a, &deck_b);

        assert_eq!(state.decks[0].cards.len(), 20);
        assert_eq!(state.hands[0].len(), 0);

        state.maybe_draw_card(0);

        assert_eq!(state.decks[0].cards.len(), 19);
        assert_eq!(state.hands[0].len(), 1);
    }

    #[test]
    fn test_players_start_with_five_cards_one_of_which_is_basic() {
        let (deck_a, deck_b) = load_test_decks();
        let state = State::initialize(&deck_a, &deck_b, &mut rand::thread_rng());

        assert_eq!(state.hands[0].len(), 5);
        assert_eq!(state.hands[1].len(), 5);
        assert_eq!(state.decks[0].cards.len(), 15);
        assert_eq!(state.decks[1].cards.len(), 15);
        assert!(state.hands[0].iter().any(is_basic));
        assert!(state.hands[1].iter().any(is_basic));
    }

    #[test]
    fn test_discard_from_play_basic_pokemon() {
        // Arrange: Create a state with a basic Pokemon in play
        let (deck_a, deck_b) = load_test_decks();
        let mut state = State::new(&deck_a, &deck_b);

        let bulbasaur_card = get_card_by_enum(CardId::A1001Bulbasaur);
        let mut played_bulbasaur = to_playable_card(&bulbasaur_card, false);

        // Attach some energy to test energy discard
        played_bulbasaur.attach_energy(&EnergyType::Grass, 2);

        // Place Bulbasaur in active slot for player 0
        state.in_play_pokemon[0][0] = Some(played_bulbasaur.clone());

        // Verify initial state
        assert!(state.in_play_pokemon[0][0].is_some());
        assert_eq!(state.discard_piles[0].len(), 0);
        assert_eq!(state.discard_energies[0].len(), 0);

        // Act: Discard the Pokemon from play
        state.discard_from_play(0, 0);

        // Assert: Pokemon slot is now empty
        assert!(state.in_play_pokemon[0][0].is_none());

        // Assert: Card is in discard pile
        assert_eq!(state.discard_piles[0].len(), 1);
        assert_eq!(state.discard_piles[0][0], bulbasaur_card);

        // Assert: Energy is in discard energy pile
        assert_eq!(state.discard_energies[0].len(), 2);
        assert_eq!(state.discard_energies[0][0], EnergyType::Grass);
        assert_eq!(state.discard_energies[0][1], EnergyType::Grass);
    }
}
