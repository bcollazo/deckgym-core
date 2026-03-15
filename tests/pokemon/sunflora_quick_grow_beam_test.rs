use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, PlayedCard},
};

mod common;

/// Test Quick-Grow Extract B1a 067 - Evolution from deck
/// Should evolve a Grass Pokemon in play with a random Grass evolution from deck
#[test]
fn test_quick_grow_extract_evolves_from_deck() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    // Clear the hand and deck to have a controlled test environment
    state.hands[0].clear();
    state.decks[0].cards.clear();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    // Put exactly ONE Ivysaur (evolution of Bulbasaur, Grass type) in the deck
    let ivysaur = get_card_by_enum(CardId::A1002Ivysaur);
    state.decks[0].cards.push(ivysaur.clone());

    // Add some other cards to the deck so it's not empty
    state.decks[0]
        .cards
        .push(get_card_by_enum(CardId::A1011Oddish));

    // Put Quick-Grow Extract in hand
    let extract = get_card_by_enum(CardId::B1a067QuickGrowExtract);
    state.hands[0].push(extract.clone());

    game.set_state(state);

    // Play Quick-Grow Extract
    let play_extract = Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: if let deckgym::models::Card::Trainer(tc) = extract {
                tc
            } else {
                panic!("Expected trainer card")
            },
        },
        is_stack: false,
    };

    game.apply_action(&play_extract);
    let state = game.get_state_clone();

    // Verify that Bulbasaur evolved into Ivysaur
    let active = state.get_active(0);
    if let Card::Pokemon(pokemon) = &active.card {
        assert_eq!(
            pokemon.name, "Ivysaur",
            "Bulbasaur should have evolved into Ivysaur"
        );
    } else {
        panic!("Expected Pokemon card");
    }
}
