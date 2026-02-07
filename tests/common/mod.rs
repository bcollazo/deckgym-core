use deckgym::{
    players::{Player, RandomPlayer},
    test_helpers::{load_test_deck, load_test_decks},
    Game,
};

// Don't know a better way to eliminate the warning...
#[allow(dead_code)]
pub fn init_random_players() -> Vec<Box<dyn Player>> {
    let (deck_a, deck_b) = load_test_decks();
    let player_a = Box::new(RandomPlayer { deck: deck_a });
    let player_b = Box::new(RandomPlayer { deck: deck_b });
    vec![player_a, player_b]
}

// Don't know a better way to eliminate the warning...
#[allow(dead_code)]
pub fn init_decks(deck_a_path: &str, deck_b_path: &str) -> Vec<Box<dyn Player>> {
    let deck_a = load_test_deck(deck_a_path);
    let deck_b = load_test_deck(deck_b_path);
    let player_a = Box::new(RandomPlayer { deck: deck_a });
    let player_b = Box::new(RandomPlayer { deck: deck_b });
    vec![player_a, player_b]
}

// Don't know a better way to eliminate the warning...
#[allow(dead_code)]
pub fn get_initialized_game(seed: u64) -> Game<'static> {
    let players = init_random_players();
    let mut game = deckgym::Game::new(players, seed);
    game.play_until_stable();
    game
}
