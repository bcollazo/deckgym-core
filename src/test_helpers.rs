use crate::Deck;
use lazy_static::lazy_static;

// Thinking of making this public and part of production code, just like
//  numpy has some datasets for testing purposes. Tried #[cfg(test)] but
//  then it cannot be used in integration tests.
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

lazy_static! {
    pub static ref DECK_A: Deck =
        Deck::from_file("example_decks/venusaur-exeggutor.txt").expect("Valid Deck Format");
    pub static ref DECK_B: Deck =
        Deck::from_file("example_decks/weezing-arbok.txt").expect("Valid Deck Format");
}
