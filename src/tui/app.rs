use crate::{
    actions::Action,
    generate_possible_actions,
    players::{create_players, Player, PlayerCode},
    Deck, Game, State,
};
use rand::{thread_rng, Rng};
use std::error::Error;

pub enum AppMode {
    Replay {
        states: Vec<State>,
        actions: Vec<Action>,
        current_index: usize,
    },
    Interactive {
        game: Box<Game<'static>>,
        current_actor: usize,
        possible_actions: Vec<Action>,
        action_history: Vec<Action>, // Track actions as they happen
        turn_history: Vec<u8>,       // Track turn number when each action was taken
    },
}

pub enum SelectionState {
    AwaitingActionSelection,
    ActionSelected { action_index: usize },
}

pub struct App {
    pub mode: AppMode,
    pub selection_state: SelectionState,
    pub scroll_offset: u16,
    pub player_hand_scroll: usize,
    pub opponent_hand_scroll: usize,
    pub lock_actions_center: bool,
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

        // Detect if any player is human
        let has_human = player_codes.contains(&PlayerCode::H);

        let mode = if has_human {
            // Interactive mode - create live game
            let players: Vec<Box<dyn Player>> = create_players(deck_a, deck_b, player_codes);
            let mut rng = thread_rng();
            let seed = rng.gen::<u64>();
            let game = Box::new(Game::new(players, seed));

            // Get initial state and possible actions
            let (current_actor, possible_actions) =
                generate_possible_actions(&game.get_state_clone());

            AppMode::Interactive {
                game,
                current_actor,
                possible_actions,
                action_history: vec![],
                turn_history: vec![],
            }
        } else {
            // Replay mode - pre-compute entire game
            let players: Vec<Box<dyn Player>> = create_players(deck_a, deck_b, player_codes);
            let mut rng = thread_rng();
            let seed = rng.gen::<u64>();
            let mut game = Game::new(players, seed);

            let mut states = Vec::new();
            let mut actions = Vec::new();
            states.push(game.get_state_clone());

            while !game.is_game_over() {
                let action = game.play_tick();
                actions.push(action);
                states.push(game.get_state_clone());
            }

            AppMode::Replay {
                states,
                actions,
                current_index: 0,
            }
        };

        Ok(App {
            mode,
            selection_state: SelectionState::AwaitingActionSelection,
            scroll_offset: 0,
            player_hand_scroll: 0,
            opponent_hand_scroll: 0,
            lock_actions_center: false,
        })
    }

    pub fn get_state(&self) -> State {
        match &self.mode {
            AppMode::Replay {
                states,
                current_index,
                ..
            } => states[*current_index].clone(),
            AppMode::Interactive { game, .. } => game.get_state_clone(),
        }
    }

    // Helper method to calculate turn boundaries in the battle log
    // Returns the scroll offset (line number) where each turn header appears
    fn calculate_turn_boundaries(&self) -> Vec<usize> {
        let mut boundaries = Vec::new();
        let mut line_count = 0;

        match &self.mode {
            AppMode::Interactive {
                action_history,
                turn_history,
                ..
            } => {
                // Even if there are no recorded actions yet, we should at least
                // expose the initial turn header so "jump" can move the battle
                // log to the start of a turn in interactive mode.
                let mut current_turn: u8 = if !turn_history.is_empty() {
                    turn_history[0]
                } else {
                    // No actions yet - use the game's current turn number as the initial header
                    self.get_state().turn_count
                };

                // Initial turn header
                boundaries.push(line_count);
                line_count += 1;

                // For each recorded action add its line and detect turn changes
                for i in 0..action_history.len() {
                    // Each action occupies a single line
                    line_count += 1;

                    // If next action has different turn, add header boundary
                    if i + 1 < turn_history.len() {
                        let next_turn = turn_history[i + 1];
                        if next_turn != current_turn {
                            line_count += 1; // empty line before header
                            boundaries.push(line_count);
                            line_count += 1; // header line
                            current_turn = next_turn;
                        }
                    }
                }
            }
            AppMode::Replay {
                states, actions, current_index, ..
            } => {
                if states.is_empty() {
                    return boundaries;
                }

                let mut current_turn = states[0].turn_count;
                boundaries.push(line_count); // Initial turn header
                line_count += 1;

                for i in 0..actions.len() {
                    // Add cursor marker line if this is the current action
                    if i == *current_index && i < actions.len() {
                        line_count += 1; // Cursor marker ">>> CURRENT <<<"
                    }
                    
                    // Each action takes exactly 1 line
                    line_count += 1;

                    // Check if turn changed after this action
                    if i + 1 < states.len() {
                        let next_turn = states[i + 1].turn_count;
                        if next_turn != current_turn && i + 1 < actions.len() {
                            line_count += 1; // Empty line
                            boundaries.push(line_count);
                            line_count += 1; // Turn header
                            current_turn = next_turn;
                        }
                    }
                }
            }
        }

        boundaries
    }

    pub fn next_state(&mut self) {
        if let AppMode::Replay {
            current_index,
            states,
            ..
        } = &mut self.mode
        {
            if *current_index < states.len() - 1 {
                *current_index += 1;
            }
        }
    }

    pub fn prev_state(&mut self) {
        if let AppMode::Replay { current_index, .. } = &mut self.mode {
            if *current_index > 0 {
                *current_index -= 1;
            }
        }
    }

    pub fn toggle_lock_actions_center(&mut self) {
        self.lock_actions_center = !self.lock_actions_center;
    }

    fn jump_turn(&mut self, forward: bool) {
        if self.lock_actions_center {
            // Center lock on: jump state to beginning of next/previous turn
            match &mut self.mode {
                AppMode::Replay {
                    states,
                    current_index,
                    ..
                } => {
                    let valid_range = if forward {
                        *current_index < states.len()
                    } else {
                        *current_index > 0
                    };

                    if valid_range {
                        let current_turn = states[*current_index].turn_count;
                        
                        // Find a state with different turn number
                        let mut target_turn = None;
                        if forward {
                            for state in states.iter().skip(*current_index + 1) {
                                if state.turn_count != current_turn {
                                    target_turn = Some(state.turn_count);
                                    break;
                                }
                            }
                        } else {
                            for state in states.iter().take(*current_index).rev() {
                                if state.turn_count != current_turn {
                                    target_turn = Some(state.turn_count);
                                    break;
                                }
                            }
                        }
                        
                        // If we found a different turn, find the FIRST state of that turn
                        if let Some(turn) = target_turn {
                            for (i, state) in states.iter().enumerate() {
                                if state.turn_count == turn {
                                    *current_index = i;
                                    return;
                                }
                            }
                        }
                    }
                }
                AppMode::Interactive { .. } => {
                    // In interactive mode we don't have a precomputed states vector,
                    // but we can still move the battle log view to the next/previous
                    // turn header. Compute turn boundaries and adjust the scroll
                    // offset similarly to the non-center-lock branch.
                    let boundaries = self.calculate_turn_boundaries();
                    if boundaries.is_empty() {
                        return;
                    }

                    let current_line = self.scroll_offset as usize;
                    if forward {
                        if let Some(&next_line) = boundaries.iter().find(|&&line| line > current_line) {
                            self.scroll_offset = next_line as u16;
                        }
                    } else if let Some(&prev_line) = boundaries.iter().rev().find(|&&line| line < current_line) {
                        self.scroll_offset = prev_line as u16;
                    }
                }
            }
        } else {
            // Center lock off: just scroll the battle log to next/previous turn header
            let boundaries = self.calculate_turn_boundaries();
            let current_line = self.scroll_offset as usize;
            
            if forward {
                if let Some(&next_line) = boundaries.iter().find(|&&line| line > current_line) {
                    self.scroll_offset = next_line as u16;
                }
            } else if let Some(&prev_line) = boundaries.iter().rev().find(|&&line| line < current_line) {
                self.scroll_offset = prev_line as u16;
            }
        }
    }

    pub fn jump_to_next_turn(&mut self) {
        self.jump_turn(true);
    }

    pub fn jump_to_prev_turn(&mut self) {
        self.jump_turn(false);
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

    // Interactive mode methods
    pub fn handle_action_selection(&mut self, index: usize) {
        if let AppMode::Interactive {
            possible_actions, ..
        } = &self.mode
        {
            if index < possible_actions.len() {
                self.selection_state = SelectionState::ActionSelected {
                    action_index: index,
                };
            }
        }
    }

    pub fn tick_game(&mut self) {
        if let AppMode::Interactive {
            game,
            current_actor,
            possible_actions,
            action_history,
            turn_history,
        } = &mut self.mode
        {
            match &self.selection_state {
                SelectionState::ActionSelected { action_index } => {
                    // Record current turn before applying action
                    let current_turn = game.get_state_clone().turn_count;

                    // Apply the selected action
                    let action = possible_actions[*action_index].clone();
                    action_history.push(action.clone());
                    turn_history.push(current_turn);
                    game.apply_action(&action);

                    // Reset selection state
                    self.selection_state = SelectionState::AwaitingActionSelection;

                    // Refresh game state and possible actions for next turn
                    let (new_actor, new_actions) =
                        generate_possible_actions(&game.get_state_clone());
                    *current_actor = new_actor;
                    *possible_actions = new_actions;
                }
                SelectionState::AwaitingActionSelection => {
                    // If it's AI's turn, play automatically
                    if *current_actor == 0 {
                        // Record current turn before AI plays
                        let current_turn = game.get_state_clone().turn_count;

                        // AI turn (player 0)
                        let action = game.play_tick();
                        action_history.push(action);
                        turn_history.push(current_turn);

                        // Refresh for next turn
                        let (new_actor, new_actions) =
                            generate_possible_actions(&game.get_state_clone());
                        *current_actor = new_actor;
                        *possible_actions = new_actions;
                    }
                    // Otherwise wait for human input
                }
            }
        }
    }

    pub fn is_game_over(&self) -> bool {
        match &self.mode {
            AppMode::Replay { .. } => false, // Replay never "ends" automatically
            AppMode::Interactive { game, .. } => game.is_game_over(),
        }
    }

    pub fn get_possible_actions(&self) -> Vec<Action> {
        match &self.mode {
            AppMode::Replay {
                states,
                current_index,
                ..
            } => generate_possible_actions(&states[*current_index]).1,
            AppMode::Interactive {
                possible_actions, ..
            } => possible_actions.clone(),
        }
    }

    pub fn get_current_actor(&self) -> usize {
        match &self.mode {
            AppMode::Replay {
                states,
                current_index,
                ..
            } => generate_possible_actions(&states[*current_index]).0,
            AppMode::Interactive { current_actor, .. } => *current_actor,
        }
    }

    pub fn get_current_state_index(&self) -> usize {
        match &self.mode {
            AppMode::Replay { current_index, .. } => *current_index,
            AppMode::Interactive { .. } => 0, // Not really meaningful in interactive mode
        }
    }

    pub fn get_states_len(&self) -> usize {
        match &self.mode {
            AppMode::Replay { states, .. } => states.len(),
            AppMode::Interactive { .. } => 1, // Only current state
        }
    }

    pub fn get_actions(&self) -> Vec<Action> {
        match &self.mode {
            AppMode::Replay { actions, .. } => actions.clone(),
            AppMode::Interactive { action_history, .. } => action_history.clone(),
        }
    }

    pub fn get_turn_history(&self) -> Option<Vec<u8>> {
        match &self.mode {
            AppMode::Interactive { turn_history, .. } => Some(turn_history.clone()),
            _ => None,
        }
    }
}
