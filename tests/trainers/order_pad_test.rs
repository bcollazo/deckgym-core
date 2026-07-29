use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, PlayedCard, TrainerCard},
    test_support::get_initialized_game,
};

fn trainer_from_id(card_id: CardId) -> TrainerCard {
    match get_card_by_enum(card_id) {
        Card::Trainer(trainer_card) => trainer_card,
        _ => panic!("Expected trainer card"),
    }
}

/// Sets up a board where player 0 holds Order Pad and their deck only contains
/// `deck_cards`, then plays Order Pad.
fn play_order_pad(seed: u64, deck_cards: Vec<Card>) -> deckgym::State {
    let order_pad = trainer_from_id(CardId::B4145OrderPad);

    let mut game = get_initialized_game(seed);
    let mut state = game.get_state_clone();
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    state.hands[0] = vec![Card::Trainer(order_pad.clone())];
    state.decks[0].cards = deck_cards;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: order_pad,
        },
        is_stack: false,
    });

    game.get_state_clone()
}

#[test]
fn test_order_pad_is_playable_and_puts_item_in_hand_on_heads() {
    let potion = get_card_by_enum(CardId::PA001Potion);

    let mut heads_seen = 0;
    let mut tails_seen = 0;
    for seed in 0..30 {
        let state = play_order_pad(seed, vec![potion.clone()]);

        if state.hands[0].contains(&potion) {
            heads_seen += 1;
            assert!(
                state.decks[0].cards.is_empty(),
                "Seed {seed}: the Item should have left the deck on heads"
            );
        } else {
            tails_seen += 1;
            assert_eq!(
                state.decks[0].cards,
                vec![potion.clone()],
                "Seed {seed}: the Item should stay in the deck on tails"
            );
        }
    }

    assert!(heads_seen > 0, "Order Pad should hit heads for some seed");
    assert!(tails_seen > 0, "Order Pad should hit tails for some seed");
}

#[test]
fn test_order_pad_never_pulls_a_non_item_card() {
    // Deck holds a Supporter, a Tool and a Pokemon: none of them are Item cards, so
    // Order Pad can never move any of them to hand, on heads or tails.
    let cynthia = get_card_by_enum(CardId::A2152Cynthia);
    let giant_cape = get_card_by_enum(CardId::A2147GiantCape);
    let charmander = get_card_by_enum(CardId::A1033Charmander);
    let deck_cards = vec![cynthia, giant_cape, charmander];

    for seed in 0..15 {
        let state = play_order_pad(seed, deck_cards.clone());

        assert_eq!(
            state.decks[0].cards.len(),
            deck_cards.len(),
            "Seed {seed}: Order Pad should not pull non-Item cards out of the deck"
        );
        assert!(
            state.hands[0].is_empty(),
            "Seed {seed}: hand should stay empty when the deck has no Item cards"
        );
    }
}
