use deckgym::Deck;

pub fn load_test_decks() -> (Deck, Deck) {
    (
        load_test_deck("venusaur-exeggutor.txt"),
        load_test_deck("weezing-arbok.txt"),
    )
}

fn load_test_deck(filename: &str) -> Deck {
    let deck_path = format!("example_decks/{filename}");
    Deck::from_file(&deck_path).expect("Valid Deck Format")
}
