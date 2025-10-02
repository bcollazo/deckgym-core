use crate::{
    actions::Action,
    players::{create_players, Player, PlayerCode},
    Deck, Game, State,
};
use rand::{thread_rng, Rng};
use std::error::Error;

pub struct App {
    pub states: Vec<State>,
    pub actions: Vec<Action>, // actions[i] is the action that led from states[i] to states[i+1]
    pub current_state_index: usize,
    pub scroll_offset: u16,
    pub player_hand_scroll: usize,
    pub opponent_hand_scroll: usize,
}

impl App {
    pub fn new(
        deck_a_path: &str,
        deck_b_path: &str,
        player_codes: Vec<PlayerCode>,
    ) -> Result<App, Box<dyn Error>> {
        // Load decks from files
        let deck_a = Deck::from_file(deck_a_path)?;
        let deck_b = Deck::from_file(deck_b_path)?;

        // Create players based on player codes
        let players: Vec<Box<dyn Player>> = create_players(deck_a, deck_b, player_codes);

        // Initialize game with a random seed
        let mut rng = thread_rng();
        let seed = rng.gen::<u64>();
        let mut game = Game::new(players, seed);

        // Play the full game and collect all states and actions
        let mut states = Vec::new();
        let mut actions = Vec::new();
        states.push(game.get_state_clone()); // Initial state

        while game.get_state_clone().winner.is_none() {
            let action = game.play_tick();
            actions.push(action);
            states.push(game.get_state_clone());
        }

        Ok(App {
            states,
            actions,
            current_state_index: 0,
            scroll_offset: 0,
            player_hand_scroll: 0,
            opponent_hand_scroll: 0,
        })
    }

    pub fn get_state(&self) -> &State {
        &self.states[self.current_state_index]
    }

    pub fn next_state(&mut self) {
        if self.current_state_index < self.states.len() - 1 {
            self.current_state_index += 1;
        }
    }

    pub fn prev_state(&mut self) {
        if self.current_state_index > 0 {
            self.current_state_index -= 1;
        }
    }

    pub fn scroll_page_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(10);
    }

    pub fn scroll_page_down(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_add(10);
    }

    pub fn scroll_player_hand_left(&mut self) {
        self.player_hand_scroll = self.player_hand_scroll.saturating_sub(1);
    }

    pub fn scroll_player_hand_right(&mut self) {
        let player_hand_size = self.get_state().hands[1].len();
        if self.player_hand_scroll < player_hand_size.saturating_sub(5) {
            self.player_hand_scroll += 1;
        }
    }

    pub fn scroll_opponent_hand_left(&mut self) {
        self.opponent_hand_scroll = self.opponent_hand_scroll.saturating_sub(1);
    }

    pub fn scroll_opponent_hand_right(&mut self) {
        let opponent_hand_size = self.get_state().hands[0].len();
        if self.opponent_hand_scroll < opponent_hand_size.saturating_sub(5) {
            self.opponent_hand_scroll += 1;
        }
    }
}
