use log::{debug, trace};
use rand::{seq::SliceRandom, Rng};
use std::collections::BTreeMap;
use std::hash::Hash;

use crate::{
    actions::SimpleAction,
    deck::Deck,
    effects::TurnEffect,
    types::{Card, EnergyType, PlayedCard},
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum GameOutcome {
    Win(usize),
    Tie,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
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
    // 0 index is the active pokemon, 1..4 are the bench
    pub in_play_pokemon: [[Option<PlayedCard>; 4]; 2],

    // Turn Flags (remember to reset these in reset_turn_states)
    pub(crate) has_played_support: bool,
    pub(crate) has_retreated: bool,
    // Maps turn to a vector of effects (cards) for that turn. Using BTreeMap to keep State hashable.
    turn_effects: BTreeMap<u8, Vec<TurnEffect>>,
}

impl State {
    pub(crate) fn new(deck_a: &Deck, deck_b: &Deck) -> Self {
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
            in_play_pokemon: [[None, None, None, None], [None, None, None, None]],
            has_played_support: false,
            has_retreated: false,
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

    pub(crate) fn discard_card_from_hand(&mut self, current_player: usize, card: &Card) {
        self.remove_card_from_hand(current_player, card);
        self.discard_piles[current_player].push(card.clone());
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
        // Reset .played_this_turn and .ability_used for all in-play pokemon
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
    ///     0 means current turn only,
    ///     1 means current turn and the next turn, etc.
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

    pub(crate) fn get_active(&self, player: usize) -> &PlayedCard {
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
    use crate::{deck::is_basic, test_helpers::load_test_decks};

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
}
