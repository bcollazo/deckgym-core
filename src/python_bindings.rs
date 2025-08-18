use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::wrap_pyfunction;
use std::collections::HashMap;

use crate::{
    deck::Deck,
    game::Game,
    players::{create_players, fill_code_array, parse_player_code},
    state::{GameOutcome, State},
    types::{Ability, Attack, Card, EnergyType, PlayedCard},
};

/// Python wrapper for EnergyType
#[pyclass]
#[derive(Clone, Copy)]
pub struct PyEnergyType {
    energy_type: EnergyType,
}

#[pymethods]
impl PyEnergyType {
    fn __repr__(&self) -> String {
        format!("{:?}", self.energy_type)
    }

    fn __str__(&self) -> String {
        format!("{}", self.energy_type)
    }

    #[getter]
    fn name(&self) -> String {
        format!("{:?}", self.energy_type)
    }
}

impl From<EnergyType> for PyEnergyType {
    fn from(energy_type: EnergyType) -> Self {
        PyEnergyType { energy_type }
    }
}

/// Python wrapper for Attack
#[pyclass]
#[derive(Clone)]
pub struct PyAttack {
    attack: Attack,
}

#[pymethods]
impl PyAttack {
    #[getter]
    fn title(&self) -> String {
        self.attack.title.clone()
    }

    #[getter]
    fn fixed_damage(&self) -> u32 {
        self.attack.fixed_damage
    }

    #[getter]
    fn effect(&self) -> Option<String> {
        self.attack.effect.clone()
    }

    #[getter]
    fn energy_required(&self) -> Vec<PyEnergyType> {
        self.attack
            .energy_required
            .iter()
            .map(|&e| e.into())
            .collect()
    }

    fn __repr__(&self) -> String {
        format!(
            "Attack(title='{}', damage={}, effect={:?})",
            self.attack.title, self.attack.fixed_damage, self.attack.effect
        )
    }
}

impl From<Attack> for PyAttack {
    fn from(attack: Attack) -> Self {
        PyAttack { attack }
    }
}

/// Python wrapper for Ability
#[pyclass]
#[derive(Clone)]
pub struct PyAbility {
    #[pyo3(get)]
    pub title: String,
    #[pyo3(get)]
    pub effect: String,
}

#[pymethods]
impl PyAbility {
    fn __repr__(&self) -> String {
        format!("Ability(title='{}', effect='{}')", self.title, self.effect)
    }
}

impl From<Ability> for PyAbility {
    fn from(ability: Ability) -> Self {
        PyAbility {
            title: ability.title,
            effect: ability.effect,
        }
    }
}

/// Python wrapper for Card
#[pyclass]
#[derive(Clone)]
pub struct PyCard {
    card: Card,
}

#[pymethods]
impl PyCard {
    #[getter]
    fn id(&self) -> String {
        self.card.get_id()
    }

    #[getter]
    fn name(&self) -> String {
        self.card.get_name()
    }

    #[getter]
    fn is_pokemon(&self) -> bool {
        matches!(self.card, Card::Pokemon(_))
    }

    #[getter]
    fn is_trainer(&self) -> bool {
        matches!(self.card, Card::Trainer(_))
    }

    #[getter]
    fn is_basic(&self) -> bool {
        self.card.is_basic()
    }

    #[getter]
    fn is_ex(&self) -> bool {
        self.card.is_ex()
    }

    #[getter]
    fn energy_type(&self) -> Option<PyEnergyType> {
        self.card.get_type().map(|t| t.into())
    }

    #[getter]
    fn attacks(&self) -> Vec<PyAttack> {
        match &self.card {
            Card::Pokemon(_) => self
                .card
                .get_attacks()
                .iter()
                .map(|a| a.clone().into())
                .collect(),
            _ => Vec::new(),
        }
    }

    #[getter]
    fn ability(&self) -> Option<PyAbility> {
        self.card.get_ability().map(|a| a.into())
    }

    fn __repr__(&self) -> String {
        format!(
            "Card(id='{}', name='{}')",
            self.card.get_id(),
            self.card.get_name()
        )
    }
}

impl From<Card> for PyCard {
    fn from(card: Card) -> Self {
        PyCard { card }
    }
}

/// Python wrapper for PlayedCard
#[pyclass]
#[derive(Clone)]
pub struct PyPlayedCard {
    played_card: PlayedCard,
}

#[pymethods]
impl PyPlayedCard {
    #[getter]
    fn card(&self) -> PyCard {
        self.played_card.card.clone().into()
    }

    #[getter]
    fn remaining_hp(&self) -> u32 {
        self.played_card.remaining_hp
    }

    #[getter]
    fn total_hp(&self) -> u32 {
        self.played_card.total_hp
    }

    #[getter]
    fn attached_energy(&self) -> Vec<PyEnergyType> {
        self.played_card
            .attached_energy
            .iter()
            .map(|&e| e.into())
            .collect()
    }

    #[getter]
    fn played_this_turn(&self) -> bool {
        self.played_card.played_this_turn
    }

    #[getter]
    fn ability_used(&self) -> bool {
        self.played_card.ability_used
    }

    #[getter]
    fn poisoned(&self) -> bool {
        self.played_card.poisoned
    }

    #[getter]
    fn paralyzed(&self) -> bool {
        self.played_card.paralyzed
    }

    #[getter]
    fn asleep(&self) -> bool {
        self.played_card.asleep
    }

    #[getter]
    fn is_damaged(&self) -> bool {
        self.played_card.is_damaged()
    }

    #[getter]
    fn has_tool_attached(&self) -> bool {
        self.played_card.has_tool_attached()
    }

    #[getter]
    fn name(&self) -> String {
        self.played_card.get_name()
    }

    #[getter]
    fn energy_type(&self) -> Option<PyEnergyType> {
        self.played_card.get_energy_type().map(|t| t.into())
    }

    #[getter]
    fn attacks(&self) -> Vec<PyAttack> {
        self.played_card
            .get_attacks()
            .iter()
            .map(|a| a.clone().into())
            .collect()
    }

    #[getter]
    fn ability(&self) -> Option<PyAbility> {
        self.played_card.card.get_ability().map(|a| a.into())
    }

    fn __repr__(&self) -> String {
        format!(
            "PlayedCard(name='{}', hp={}/{}, energy={})",
            self.played_card.get_name(),
            self.played_card.remaining_hp,
            self.played_card.total_hp,
            self.played_card.attached_energy.len()
        )
    }
}

impl From<PlayedCard> for PyPlayedCard {
    fn from(played_card: PlayedCard) -> Self {
        PyPlayedCard { played_card }
    }
}

/// Python wrapper for GameOutcome
#[pyclass]
#[derive(Clone)]
pub struct PyGameOutcome {
    #[pyo3(get)]
    pub winner: Option<usize>,
    #[pyo3(get)]
    pub is_tie: bool,
}

impl From<GameOutcome> for PyGameOutcome {
    fn from(outcome: GameOutcome) -> Self {
        match outcome {
            GameOutcome::Win(player) => PyGameOutcome {
                winner: Some(player),
                is_tie: false,
            },
            GameOutcome::Tie => PyGameOutcome {
                winner: None,
                is_tie: true,
            },
        }
    }
}

#[pymethods]
impl PyGameOutcome {
    fn __repr__(&self) -> String {
        if self.is_tie {
            "GameOutcome::Tie".to_string()
        } else if let Some(winner) = self.winner {
            format!("GameOutcome::Win({})", winner)
        } else {
            panic!("Invalid state: PyGameOutcome has neither a winner nor a tie.");
        }
    }
}

/// Python wrapper for Deck
#[pyclass]
pub struct PyDeck {
    deck: Deck,
}

#[pymethods]
impl PyDeck {
    #[new]
    pub fn new(deck_path: &str) -> PyResult<Self> {
        let deck = Deck::from_file(deck_path).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to load deck: {}", e))
        })?;
        Ok(PyDeck { deck })
    }

    fn __repr__(&self) -> String {
        format!("PyDeck(cards={})", self.deck.cards.len())
    }

    #[getter]
    fn card_count(&self) -> usize {
        self.deck.cards.len()
    }
}

/// Python wrapper for State
#[pyclass]
pub struct PyState {
    state: State,
}

#[pymethods]
impl PyState {
    #[getter]
    fn turn_count(&self) -> u8 {
        self.state.turn_count
    }

    #[getter]
    fn current_player(&self) -> usize {
        self.state.current_player
    }

    #[getter]
    fn points(&self) -> [u8; 2] {
        self.state.points
    }

    #[getter]
    fn winner(&self) -> Option<PyGameOutcome> {
        self.state.winner.map(|outcome| outcome.into())
    }

    #[getter]
    fn current_energy(&self) -> Option<PyEnergyType> {
        self.state.current_energy.map(|e| e.into())
    }

    #[getter]
    fn has_played_support(&self) -> bool {
        self.state.has_played_support
    }

    #[getter]
    fn has_retreated(&self) -> bool {
        self.state.has_retreated
    }

    fn is_game_over(&self) -> bool {
        self.state.is_game_over()
    }

    /// Get the hand of a specific player (0 or 1)
    fn get_hand(&self, player: usize) -> PyResult<Vec<PyCard>> {
        if player > 1 {
            return Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                "Player index must be 0 or 1",
            ));
        }
        Ok(self.state.hands[player]
            .iter()
            .map(|card| card.clone().into())
            .collect())
    }

    /// Get the number of cards remaining in a player's deck
    fn get_deck_size(&self, player: usize) -> PyResult<usize> {
        if player > 1 {
            return Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                "Player index must be 0 or 1",
            ));
        }
        Ok(self.state.decks[player].cards.len())
    }

    /// Get the discard pile of a specific player
    fn get_discard_pile(&self, player: usize) -> PyResult<Vec<PyCard>> {
        if player > 1 {
            return Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                "Player index must be 0 or 1",
            ));
        }
        Ok(self.state.discard_piles[player]
            .iter()
            .map(|card| card.clone().into())
            .collect())
    }

    /// Get all in-play pokemon for a specific player
    fn get_in_play_pokemon(&self, player: usize) -> PyResult<Vec<Option<PyPlayedCard>>> {
        if player > 1 {
            return Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                "Player index must be 0 or 1",
            ));
        }
        Ok(self.state.in_play_pokemon[player]
            .iter()
            .map(|opt_card| opt_card.as_ref().map(|card| card.clone().into()))
            .collect())
    }

    /// Get the active pokemon for a specific player (index 0)
    fn get_active_pokemon(&self, player: usize) -> PyResult<Option<PyPlayedCard>> {
        if player > 1 {
            return Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                "Player index must be 0 or 1",
            ));
        }
        Ok(self.state.in_play_pokemon[player][0]
            .as_ref()
            .map(|card| card.clone().into()))
    }

    /// Get the bench pokemon for a specific player (indices 1-3)
    fn get_bench_pokemon(&self, player: usize) -> PyResult<Vec<Option<PyPlayedCard>>> {
        if player > 1 {
            return Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                "Player index must be 0 or 1",
            ));
        }
        Ok(self.state.in_play_pokemon[player][1..4]
            .iter()
            .map(|opt_card| opt_card.as_ref().map(|card| card.clone().into()))
            .collect())
    }

    /// Get a specific pokemon by player and position
    fn get_pokemon_at_position(
        &self,
        player: usize,
        position: usize,
    ) -> PyResult<Option<PyPlayedCard>> {
        if player > 1 {
            return Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                "Player index must be 0 or 1",
            ));
        }
        if position > 3 {
            return Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                "Position index must be 0-3 (0=active, 1-3=bench)",
            ));
        }
        Ok(self.state.in_play_pokemon[player][position]
            .as_ref()
            .map(|card| card.clone().into()))
    }

    /// Get the remaining HP of a pokemon at a specific position
    fn get_remaining_hp(&self, player: usize, position: usize) -> PyResult<u32> {
        if player > 1 {
            return Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                "Player index must be 0 or 1",
            ));
        }
        if position > 3 {
            return Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                "Position index must be 0-3 (0=active, 1-3=bench)",
            ));
        }
        if let Some(pokemon) = &self.state.in_play_pokemon[player][position] {
            Ok(pokemon.remaining_hp)
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "No pokemon at this position",
            ))
        }
    }

    /// Count the number of in-play pokemon of a specific energy type for a player
    fn count_in_play_of_type(&self, player: usize, energy_type: PyEnergyType) -> PyResult<usize> {
        if player > 1 {
            return Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                "Player index must be 0 or 1",
            ));
        }
        Ok(self
            .state
            .num_in_play_of_type(player, energy_type.energy_type))
    }

    /// Get a list of (position, pokemon) tuples for all in-play pokemon of a player
    fn enumerate_in_play_pokemon(&self, player: usize) -> PyResult<Vec<(usize, PyPlayedCard)>> {
        if player > 1 {
            return Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                "Player index must be 0 or 1",
            ));
        }
        Ok(self
            .state
            .enumerate_in_play_pokemon(player)
            .map(|(pos, card)| (pos, card.clone().into()))
            .collect())
    }

    /// Get a list of (position, pokemon) tuples for all bench pokemon of a player
    fn enumerate_bench_pokemon(&self, player: usize) -> PyResult<Vec<(usize, PyPlayedCard)>> {
        if player > 1 {
            return Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                "Player index must be 0 or 1",
            ));
        }
        Ok(self
            .state
            .enumerate_bench_pokemon(player)
            .map(|(pos, card)| (pos, card.clone().into()))
            .collect())
    }

    /// Get the debug string representation of the state
    fn debug_string(&self) -> String {
        self.state.debug_string()
    }

    /// Get hand size for a player
    fn get_hand_size(&self, player: usize) -> PyResult<usize> {
        if player > 1 {
            return Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                "Player index must be 0 or 1",
            ));
        }
        Ok(self.state.hands[player].len())
    }

    /// Get discard pile size for a player
    fn get_discard_pile_size(&self, player: usize) -> PyResult<usize> {
        if player > 1 {
            return Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                "Player index must be 0 or 1",
            ));
        }
        Ok(self.state.discard_piles[player].len())
    }

    /// Count the number of pokemon in play for a player
    fn count_in_play_pokemon(&self, player: usize) -> PyResult<usize> {
        if player > 1 {
            return Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                "Player index must be 0 or 1",
            ));
        }
        Ok(self.state.in_play_pokemon[player]
            .iter()
            .filter(|pokemon| pokemon.is_some())
            .count())
    }

    /// Check if a player has an active pokemon
    fn has_active_pokemon(&self, player: usize) -> PyResult<bool> {
        if player > 1 {
            return Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                "Player index must be 0 or 1",
            ));
        }
        Ok(self.state.in_play_pokemon[player][0].is_some())
    }

    /// Count the number of bench pokemon for a player
    fn count_bench_pokemon(&self, player: usize) -> PyResult<usize> {
        if player > 1 {
            return Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                "Player index must be 0 or 1",
            ));
        }
        Ok(self.state.in_play_pokemon[player][1..4]
            .iter()
            .filter(|pokemon| pokemon.is_some())
            .count())
    }

    fn __repr__(&self) -> String {
        format!(
            "PyState(turn={}, player={}, points={:?}, game_over={})",
            self.state.turn_count,
            self.state.current_player,
            self.state.points,
            self.state.is_game_over()
        )
    }
}

/// Python wrapper for Game
#[pyclass(unsendable)]
pub struct PyGame {
    game: Game<'static>,
}

#[pymethods]
impl PyGame {
    #[new]
    #[pyo3(signature = (deck_a_path, deck_b_path, players=None, seed=None))]
    pub fn new(
        deck_a_path: &str,
        deck_b_path: &str,
        players: Option<Vec<String>>,
        seed: Option<u64>,
    ) -> PyResult<Self> {
        let deck_a = Deck::from_file(deck_a_path).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to load deck A: {}", e))
        })?;
        let deck_b = Deck::from_file(deck_b_path).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to load deck B: {}", e))
        })?;

        let player_codes = if let Some(player_strs) = players {
            let mut codes = Vec::new();
            for player_str in player_strs {
                let code = parse_player_code(&player_str)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))?;
                codes.push(code);
            }
            Some(codes)
        } else {
            None
        };

        let cli_players = fill_code_array(player_codes);
        let rust_players = create_players(deck_a, deck_b, cli_players);
        let game_seed = seed.unwrap_or_else(|| rand::random::<u64>());
        let game = Game::new(rust_players, game_seed);

        Ok(PyGame { game })
    }

    fn play(&mut self) -> Option<PyGameOutcome> {
        self.game.play().map(|outcome| outcome.into())
    }

    fn get_state(&self) -> PyState {
        PyState {
            state: self.game.get_state_clone(),
        }
    }

    fn play_tick(&mut self) -> String {
        let action = self.game.play_tick();
        format!("{:?}", action.action)
    }

    fn __repr__(&self) -> String {
        let state = self.game.get_state_clone();
        format!(
            "PyGame(turn={}, current_player={}, game_over={})",
            state.turn_count,
            state.current_player,
            state.is_game_over()
        )
    }
}

/// Simulation results
#[pyclass]
pub struct PySimulationResults {
    #[pyo3(get)]
    pub total_games: u32,
    #[pyo3(get)]
    pub player_a_wins: u32,
    #[pyo3(get)]
    pub player_b_wins: u32,
    #[pyo3(get)]
    pub ties: u32,
    #[pyo3(get)]
    pub player_a_win_rate: f32,
    #[pyo3(get)]
    pub player_b_win_rate: f32,
    #[pyo3(get)]
    pub tie_rate: f32,
}

#[pymethods]
impl PySimulationResults {
    fn __repr__(&self) -> String {
        format!(
            "SimulationResults(games={}, A_wins={} ({:.1}%), B_wins={} ({:.1}%), ties={} ({:.1}%))",
            self.total_games,
            self.player_a_wins,
            self.player_a_win_rate * 100.0,
            self.player_b_wins,
            self.player_b_win_rate * 100.0,
            self.ties,
            self.tie_rate * 100.0
        )
    }
}

/// Run multiple game simulations
#[pyfunction]
#[pyo3(signature = (deck_a_path, deck_b_path, players=None, num_simulations=100, seed=None))]
pub fn py_simulate(
    deck_a_path: &str,
    deck_b_path: &str,
    players: Option<Vec<String>>,
    num_simulations: u32,
    seed: Option<u64>,
) -> PyResult<PySimulationResults> {
    let deck_a = Deck::from_file(deck_a_path).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to load deck A: {}", e))
    })?;
    let deck_b = Deck::from_file(deck_b_path).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to load deck B: {}", e))
    })?;

    let player_codes = if let Some(player_strs) = players {
        let mut codes = Vec::new();
        for player_str in player_strs {
            let code = parse_player_code(&player_str)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))?;
            codes.push(code);
        }
        Some(codes)
    } else {
        None
    };

    let cli_players = fill_code_array(player_codes);

    // Run simulations
    let mut wins_per_deck = [0u32, 0u32, 0u32]; // [player_a, player_b, ties]

    for _ in 0..num_simulations {
        let players = create_players(deck_a.clone(), deck_b.clone(), cli_players.clone());
        let game_seed = seed.unwrap_or_else(|| rand::random::<u64>());
        let mut game = Game::new(players, game_seed);
        let outcome = game.play();

        match outcome {
            Some(GameOutcome::Win(winner)) => {
                if winner < 2 {
                    wins_per_deck[winner] += 1;
                } else {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "Invalid winner index: {}",
                        winner
                    )));
                }
            }
            Some(GameOutcome::Tie) | None => {
                wins_per_deck[2] += 1;
            }
        }
    }

    Ok(PySimulationResults {
        total_games: num_simulations,
        player_a_wins: wins_per_deck[0],
        player_b_wins: wins_per_deck[1],
        ties: wins_per_deck[2],
        player_a_win_rate: wins_per_deck[0] as f32 / num_simulations as f32,
        player_b_win_rate: wins_per_deck[1] as f32 / num_simulations as f32,
        tie_rate: wins_per_deck[2] as f32 / num_simulations as f32,
    })
}

/// Get available player types
#[pyfunction]
pub fn get_player_types() -> HashMap<String, String> {
    let mut types = HashMap::new();
    types.insert("r".to_string(), "Random Player".to_string());
    types.insert("aa".to_string(), "Attach-Attack Player".to_string());
    types.insert("et".to_string(), "End Turn Player".to_string());
    types.insert("h".to_string(), "Human Player".to_string());
    types.insert("w".to_string(), "Weighted Random Player".to_string());
    types.insert("m".to_string(), "MCTS Player".to_string());
    types.insert("v".to_string(), "Value Function Player".to_string());
    types.insert("e".to_string(), "Expectiminimax Player".to_string());
    types
}

/// Python module definition
pub fn deckgym(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyEnergyType>()?;
    m.add_class::<PyAttack>()?;
    m.add_class::<PyAbility>()?;
    m.add_class::<PyCard>()?;
    m.add_class::<PyPlayedCard>()?;
    m.add_class::<PyDeck>()?;
    m.add_class::<PyGame>()?;
    m.add_class::<PyState>()?;
    m.add_class::<PyGameOutcome>()?;
    m.add_class::<PySimulationResults>()?;
    m.add_function(wrap_pyfunction!(py_simulate, m)?)?;
    m.add_function(wrap_pyfunction!(get_player_types, m)?)?;
    Ok(())
}
