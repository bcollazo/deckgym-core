use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, PlayedCard},
    test_support::get_initialized_game,
};

fn make_trainer_card(card_id: CardId) -> deckgym::models::TrainerCard {
    get_card_by_enum(card_id).as_trainer()
}

#[test]
fn test_juliana_puts_stage2_from_deck_into_hand() {
    // Juliana: "Put a random Stage 2 Pokémon from your deck into your hand."
    // Deck has one Stage 2 (Venusaur); after playing Juliana it should appear in hand.
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let venusaur_card = get_card_by_enum(CardId::A1003Venusaur);
    state.decks[0].cards = vec![venusaur_card.clone()];

    let juliana = make_trainer_card(CardId::B3a071Juliana);
    state.hands[0] = vec![Card::Trainer(juliana.clone())];
    game.set_state(state);

    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: juliana,
        },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let state = game.get_state_clone();
    let hand = &state.hands[0];
    let has_venusaur = hand
        .iter()
        .any(|c| matches!(c, Card::Pokemon(p) if p.id == "A1 003"));
    assert!(
        has_venusaur,
        "Venusaur (Stage 2) should be in hand after playing Juliana"
    );
    assert!(
        state.decks[0].cards.is_empty(),
        "Deck should be empty after transferring the only card"
    );
}

#[test]
fn test_juliana_no_stage2_in_deck_does_nothing() {
    // If no Stage 2 in deck, Juliana has no effect (deck has only a Basic).
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let bulbasaur_card = get_card_by_enum(CardId::A1001Bulbasaur);
    state.decks[0].cards = vec![bulbasaur_card];

    let juliana = make_trainer_card(CardId::B3a071Juliana);
    state.hands[0] = vec![Card::Trainer(juliana.clone())];
    game.set_state(state);

    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: juliana,
        },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let state = game.get_state_clone();
    // Hand should contain no Pokemon (Bulbasaur stayed in deck, not moved)
    let has_bulbasaur_in_hand = state.hands[0]
        .iter()
        .any(|c| matches!(c, Card::Pokemon(p) if p.id == "A1 001"));
    assert!(
        !has_bulbasaur_in_hand,
        "Bulbasaur (Basic) should NOT be moved to hand by Juliana"
    );
}
