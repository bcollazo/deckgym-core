use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{actions::Action, state::GameOutcome, types::PlayedCard, State};

/// Training data export structures
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrainingData {
    pub games: Vec<GameData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameData {
    pub game_id: String,
    pub winner: Option<usize>, // 0, 1, or None for tie
    pub decisions: Vec<Decision>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Decision {
    pub turn: u8,
    pub acting_player: usize,
    pub state: GameState,
    pub possible_actions: Vec<ActionData>,
    pub action_chosen: ActionData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_scores: Option<Vec<f64>>, // From expectiminimax if available
}

/// Simplified game state for training
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameState {
    pub turn_count: u8,
    pub current_player: usize,
    pub points: [u8; 2],
    pub hands_size: [usize; 2], // Just hand sizes for privacy
    pub in_play_pokemon: [[Option<PokemonState>; 4]; 2],
    pub current_energy: Option<String>, // Energy type as string
    pub has_played_support: bool,
    pub has_retreated: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PokemonState {
    pub card_name: String,
    pub hp: u32,
    pub remaining_hp: u32,
    pub attached_energy: Vec<String>, // Energy types as strings
    pub attached_tool: Option<String>,
    pub played_this_turn: bool,
    pub poisoned: bool,
    pub paralyzed: bool,
    pub asleep: bool,
    pub stage: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActionData {
    pub action_type: String,
    pub description: String,
    pub actor: usize,
}

impl TrainingData {
    pub fn new() -> Self {
        Self { games: Vec::new() }
    }

    pub fn add_game(&mut self, game: GameData) {
        self.games.push(game);
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}

impl GameData {
    pub fn new(game_id: Uuid) -> Self {
        Self {
            game_id: game_id.to_string(),
            winner: None,
            decisions: Vec::new(),
        }
    }

    pub fn add_decision(&mut self, decision: Decision) {
        self.decisions.push(decision);
    }

    pub fn set_winner(&mut self, outcome: Option<GameOutcome>) {
        self.winner = match outcome {
            Some(GameOutcome::Win(player)) => Some(player),
            Some(GameOutcome::Tie) => None,
            None => None,
        };
    }
}

impl From<&State> for GameState {
    fn from(state: &State) -> Self {
        GameState {
            turn_count: state.turn_count,
            current_player: state.current_player,
            points: state.points,
            hands_size: [state.hands[0].len(), state.hands[1].len()],
            in_play_pokemon: [
                [
                    state.in_play_pokemon[0][0].as_ref().map(PokemonState::from),
                    state.in_play_pokemon[0][1].as_ref().map(PokemonState::from),
                    state.in_play_pokemon[0][2].as_ref().map(PokemonState::from),
                    state.in_play_pokemon[0][3].as_ref().map(PokemonState::from),
                ],
                [
                    state.in_play_pokemon[1][0].as_ref().map(PokemonState::from),
                    state.in_play_pokemon[1][1].as_ref().map(PokemonState::from),
                    state.in_play_pokemon[1][2].as_ref().map(PokemonState::from),
                    state.in_play_pokemon[1][3].as_ref().map(PokemonState::from),
                ],
            ],
            current_energy: state.current_energy.map(|e| format!("{:?}", e)),
            has_played_support: state.has_played_support,
            has_retreated: state.has_retreated,
        }
    }
}

impl From<&PlayedCard> for PokemonState {
    fn from(card: &PlayedCard) -> Self {
        PokemonState {
            card_name: card.card.get_name().to_string(),
            hp: card.total_hp,
            remaining_hp: card.remaining_hp,
            attached_energy: card.attached_energy.iter().map(|e| format!("{:?}", e)).collect(),
            attached_tool: card.attached_tool.map(|t| format!("{:?}", t)),
            played_this_turn: card.played_this_turn,
            poisoned: card.poisoned,
            paralyzed: card.paralyzed,
            asleep: card.asleep,
            stage: match &card.card {
                crate::types::Card::Pokemon(pokemon) => pokemon.stage,
                _ => 0, // Shouldn't happen for played cards
            },
        }
    }
}

impl From<&Action> for ActionData {
    fn from(action: &Action) -> Self {
        ActionData {
            action_type: format!("{:?}", std::mem::discriminant(&action.action)),
            description: format!("{}", action.action),
            actor: action.actor,
        }
    }
}

impl Default for TrainingData {
    fn default() -> Self {
        Self::new()
    }
}

/// Event handler for collecting training data
pub struct TrainingDataCollector {
    pub training_data: TrainingData,
    current_game: Option<GameData>,
}

impl TrainingDataCollector {
    pub fn new() -> Self {
        Self {
            training_data: TrainingData::new(),
            current_game: None,
        }
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.training_data.save_to_file(path)
    }
}

impl Default for TrainingDataCollector {
    fn default() -> Self {
        Self::new()
    }
}

use crate::simulation_event_handler::SimulationEventHandler;

impl SimulationEventHandler for TrainingDataCollector {
    fn on_simulation_start(&mut self) {
        // Initialize training data
        self.training_data = TrainingData::new();
    }

    fn on_game_start(&mut self, game_id: Uuid) {
        self.current_game = Some(GameData::new(game_id));
    }

    fn on_action(
        &mut self,
        _game_id: Uuid,
        state_before_action: &State,
        actor: usize,
        playable_actions: &[Action],
        action: &Action,
        _state_after_action: &State,
    ) {
        if let Some(ref mut game) = self.current_game {
            let decision = Decision {
                turn: state_before_action.turn_count,
                acting_player: actor,
                state: GameState::from(state_before_action),
                possible_actions: playable_actions.iter().map(ActionData::from).collect(),
                action_chosen: ActionData::from(action),
                action_scores: None, // TODO: Add expectiminimax scores if available
            };
            game.add_decision(decision);
        }
    }

    fn on_game_end(&mut self, _game_id: Uuid, _state: State, result: Option<GameOutcome>) {
        if let Some(mut game) = self.current_game.take() {
            game.set_winner(result);
            self.training_data.add_game(game);
        }
    }

    fn on_simulation_end(&mut self) {
        // All games have been collected, ready to export
    }
}