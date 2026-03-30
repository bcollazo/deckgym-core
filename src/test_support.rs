use crate::{
    models::PlayedCard,
    players::{Player, RandomPlayer},
    Deck, Game,
};
use lazy_static::lazy_static;

pub fn load_test_decks() -> (Deck, Deck) {
    let deck_a_filename = "venusaur-exeggutor.txt";
    let deck_b_filename = "weezing-arbok.txt";

    let deck_a = load_test_deck(deck_a_filename);
    let deck_b = load_test_deck(deck_b_filename);

    (deck_a, deck_b)
}

pub fn load_test_deck(filename: &str) -> Deck {
    let deck_path = format!("example_decks/{filename}");
    Deck::from_file(&deck_path).expect("Valid Deck Format")
}

pub fn init_random_players() -> Vec<Box<dyn Player>> {
    let (deck_a, deck_b) = load_test_decks();
    let player_a = Box::new(RandomPlayer { deck: deck_a });
    let player_b = Box::new(RandomPlayer { deck: deck_b });
    vec![player_a, player_b]
}

pub fn init_decks(deck_a_path: &str, deck_b_path: &str) -> Vec<Box<dyn Player>> {
    let deck_a = load_test_deck(deck_a_path);
    let deck_b = load_test_deck(deck_b_path);
    let player_a = Box::new(RandomPlayer { deck: deck_a });
    let player_b = Box::new(RandomPlayer { deck: deck_b });
    vec![player_a, player_b]
}

pub fn get_initialized_game(seed: u64) -> Game<'static> {
    let players = init_random_players();
    let mut game = crate::Game::new(players, seed);
    game.play_until_stable();
    game
}

pub fn get_initialized_game_with_board(
    seed: u64,
    current_player: usize,
    turn_count: u8,
    player_board: Vec<PlayedCard>,
    opponent_board: Vec<PlayedCard>,
) -> Game<'static> {
    let mut game = get_initialized_game(seed);
    let mut state = game.get_state_clone();
    state.set_board(player_board, opponent_board);
    state.current_player = current_player;
    state.turn_count = turn_count;
    game.set_state(state);
    game
}

pub fn get_test_game_with_board(
    player_board: Vec<PlayedCard>,
    opponent_board: Vec<PlayedCard>,
) -> Game<'static> {
    get_initialized_game_with_board(0, 0, 3, player_board, opponent_board)
}

lazy_static! {
    pub static ref DECK_A: Deck =
        Deck::from_file("example_decks/venusaur-exeggutor.txt").expect("Valid Deck Format");
    pub static ref DECK_B: Deck =
        Deck::from_file("example_decks/weezing-arbok.txt").expect("Valid Deck Format");
}
