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
fn test_puppy_loving_girl_puts_puppy_pile_pokemon_into_hand() {
    // Puppy-Loving Girl: "Look at the top 4 cards of your deck. Put all Pokémon you find there
    // that have the Puppy Pile attack into your hand. Shuffle the other cards back into your deck."
    // Deck's top 4 cards contain Lillipup (has Puppy Pile) and other Pokémon without it.
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let lillipup_card = get_card_by_enum(CardId::B3137Lillipup);
    let charmander_card = get_card_by_enum(CardId::A1033Charmander);
    state.decks[0].cards = vec![lillipup_card.clone(), charmander_card.clone()];

    let puppy_loving_girl = make_trainer_card(CardId::B3b067PuppyLovingGirl);
    state.hands[0] = vec![Card::Trainer(puppy_loving_girl.clone())];
    game.set_state(state);

    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: puppy_loving_girl,
        },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let state = game.get_state_clone();
    let has_lillipup = state.hands[0]
        .iter()
        .any(|c| matches!(c, Card::Pokemon(p) if p.id == "B3 137"));
    assert!(has_lillipup, "Lillipup should be moved to hand");

    let has_charmander = state.hands[0]
        .iter()
        .any(|c| matches!(c, Card::Pokemon(p) if p.id == "A1 033"));
    assert!(
        !has_charmander,
        "Charmander should NOT be moved to hand since it doesn't have Puppy Pile"
    );
    assert!(
        state.decks[0].cards.contains(&charmander_card),
        "Charmander should remain in the deck"
    );
}

#[test]
fn test_puppy_loving_girl_no_puppy_pile_pokemon_does_nothing() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let charmander_card = get_card_by_enum(CardId::A1033Charmander);
    state.decks[0].cards = vec![charmander_card.clone()];

    let puppy_loving_girl = make_trainer_card(CardId::B3b067PuppyLovingGirl);
    state.hands[0] = vec![Card::Trainer(puppy_loving_girl.clone())];
    game.set_state(state);

    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: puppy_loving_girl,
        },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let state = game.get_state_clone();
    let has_charmander = state.hands[0]
        .iter()
        .any(|c| matches!(c, Card::Pokemon(p) if p.id == "A1 033"));
    assert!(
        !has_charmander,
        "Charmander should NOT be moved to hand since it doesn't have Puppy Pile"
    );
    assert!(
        state.decks[0].cards.contains(&charmander_card),
        "Charmander should remain in the deck"
    );
}
