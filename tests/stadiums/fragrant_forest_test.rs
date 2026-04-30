use crate::deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

fn trainer_from_id(card_id: CardId) -> crate::deckgym::models::TrainerCard {
    match get_card_by_enum(card_id) {
        Card::Trainer(trainer_card) => trainer_card,
        _ => panic!("Expected trainer card"),
    }
}

#[test]
fn test_fragrant_forest_use_stadium_available_with_basic_grass_in_deck() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 2;
    state.hands[0] = vec![get_card_by_enum(CardId::B3153FragrantForest)];
    // Put a Basic Grass pokemon in the deck
    state.decks[0].cards = vec![get_card_by_enum(CardId::A4008Chikorita)];
    game.set_state(state);

    // Play Fragrant Forest stadium
    let trainer_card = trainer_from_id(CardId::B3153FragrantForest);
    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let state = game.get_state_clone();
    assert!(
        state.active_stadium.is_some(),
        "Fragrant Forest should be active"
    );

    let (_actor, actions) = state.generate_possible_actions();
    let has_use_stadium = actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::UseStadium));

    assert!(
        has_use_stadium,
        "UseStadium action should be available when Fragrant Forest is active and deck has Basic Grass Pokemon"
    );
}

#[test]
fn test_fragrant_forest_draws_basic_grass_to_hand() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 2;
    state.hands[0] = vec![get_card_by_enum(CardId::B3153FragrantForest)];
    let chikorita = get_card_by_enum(CardId::A4008Chikorita);
    state.decks[0].cards = vec![chikorita.clone()];
    game.set_state(state);

    // Play Fragrant Forest
    let trainer_card = trainer_from_id(CardId::B3153FragrantForest);
    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    };
    game.apply_action(&play_action);

    // Use the stadium effect
    let use_stadium_action = Action {
        actor: 0,
        action: SimpleAction::UseStadium,
        is_stack: false,
    };
    game.apply_action(&use_stadium_action);

    let state = game.get_state_clone();

    // Chikorita should now be in player 0's hand
    let has_chikorita_in_hand = state.hands[0].contains(&chikorita);
    assert!(
        has_chikorita_in_hand,
        "Fragrant Forest should have moved Chikorita (Basic Grass) from deck to hand"
    );

    // Deck should now be empty
    assert!(
        state.decks[0].cards.is_empty(),
        "Deck should be empty after Fragrant Forest moved the only card"
    );

    // has_used_stadium should be true for player 0
    assert!(
        state.has_used_stadium[0],
        "has_used_stadium[0] should be true after using Fragrant Forest"
    );
}

#[test]
fn test_fragrant_forest_use_stadium_not_available_without_basic_grass() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 2;
    state.hands[0] = vec![get_card_by_enum(CardId::B3153FragrantForest)];
    // Deck has no Basic Grass pokemon (only a Trainer card)
    state.decks[0].cards = vec![get_card_by_enum(CardId::PA001Potion)];
    game.set_state(state);

    let trainer_card = trainer_from_id(CardId::B3153FragrantForest);
    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let state = game.get_state_clone();
    let (_actor, actions) = state.generate_possible_actions();
    let has_use_stadium = actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::UseStadium));

    assert!(
        !has_use_stadium,
        "UseStadium should NOT be available when deck has no Basic Grass Pokemon"
    );
}

#[test]
fn test_fragrant_forest_each_player_can_use_once_per_turn() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    let chikorita = get_card_by_enum(CardId::A4008Chikorita);
    let bulbasaur = get_card_by_enum(CardId::A1001Bulbasaur);

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1033Charmander).with_energy(vec![EnergyType::Fire])],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    state.current_player = 0;
    state.turn_count = 2;
    state.hands[0] = vec![get_card_by_enum(CardId::B3153FragrantForest)];
    state.decks[0].cards = vec![chikorita.clone()];
    state.decks[1].cards = vec![bulbasaur.clone()];
    game.set_state(state);

    // Player 0 plays Fragrant Forest
    let trainer_card = trainer_from_id(CardId::B3153FragrantForest);
    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    };
    game.apply_action(&play_action);

    // Player 0 uses it
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseStadium,
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert!(
        state.has_used_stadium[0],
        "Player 0 should have used stadium"
    );
    assert!(
        !state.has_used_stadium[1],
        "Player 1 should NOT have used stadium yet"
    );

    // Player 0 should no longer have UseStadium as an option this turn
    let (_actor, actions) = state.generate_possible_actions();
    let has_use_stadium = actions
        .iter()
        .any(|a| matches!(a.action, SimpleAction::UseStadium));
    assert!(
        !has_use_stadium,
        "Player 0 should not be able to use Fragrant Forest a second time this turn"
    );
}
