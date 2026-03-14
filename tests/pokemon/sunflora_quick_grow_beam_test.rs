use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    effects::CardEffect,
    models::{Card, EnergyType, PlayedCard},
};

#[path = "../common/mod.rs"]
mod common;

fn played_card_with_base_hp(card_id: CardId, base_hp: u32) -> PlayedCard {
    let card = get_card_by_enum(card_id);
    PlayedCard::new(card, 0, base_hp, vec![], false, vec![])
}

/// Test Sunflora B1a 008 - Quick-Grow Beam
/// Should deal 30 damage, or 60 if Quick-Grow Extract is in discard pile
#[test]
fn test_sunflora_quick_grow_beam_without_extract() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![PlayedCard::from_id(CardId::B1a008Sunflora).with_energy(vec![EnergyType::Grass])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    // No Quick-Grow Extract in discard pile
    state.discard_piles[0] = vec![];

    game.set_state(state);

    let action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };

    game.apply_action(&action);
    let state = game.get_state_clone();

    // Should deal only 30 damage (no bonus)
    let opponent_active = state.get_active(1);
    assert_eq!(
        opponent_active.get_remaining_hp(),
        40,
        "Opponent should have 40 HP remaining (70 - 30)"
    );
}


#[test]
fn test_sunflora_quick_grow_beam_with_extract() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![PlayedCard::from_id(CardId::B1a008Sunflora).with_energy(vec![EnergyType::Grass])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    // Put Quick-Grow Extract in discard pile
    state.discard_piles[0] = vec![get_card_by_enum(CardId::B1a067QuickGrowExtract)];

    game.set_state(state);

    let action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };

    game.apply_action(&action);
    let state = game.get_state_clone();

    // Should deal 30 + 30 = 60 damage
    let opponent_active = state.get_active(1);
    assert_eq!(
        opponent_active.get_remaining_hp(),
        10,
        "Opponent should have 10 HP remaining (70 - 60)"
    );
}


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
