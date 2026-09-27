use deckgym::{
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard},
    test_support::{get_initialized_game_with_board, play_trainer, trainer_from_id},
};

/// Penny: "Look at a random Supporter card that's not Penny from your opponent's deck and shuffle
/// it back into their deck. Use the effect of that card as the effect of this card."
///
/// With Giovanni as the only eligible Supporter in the opponent's deck, Penny must copy it — so
/// the Penny player's attack gets Giovanni's +10.
#[test]
fn test_penny_copies_the_only_supporter_in_the_opponents_deck() {
    let damage_with = |play_penny: bool| {
        let mut game = get_initialized_game_with_board(
            0,
            0,
            3,
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
            vec![PlayedCard::from_id(CardId::A1202Chansey)],
        );
        let mut state = game.get_state_clone();
        // Only Giovanni is an eligible copy target; the rest of the deck holds no Supporters.
        state.decks[1].cards = vec![
            get_card_by_enum(CardId::A1223Giovanni),
            get_card_by_enum(CardId::A1001Bulbasaur),
            get_card_by_enum(CardId::PA005PokeBall),
        ];
        state.hands[0].clear();
        game.set_state(state);

        if play_penny {
            let penny = trainer_from_id(CardId::A3b069Penny);
            let mut state = game.get_state_clone();
            state.hands[0] = vec![Card::Trainer(penny.clone())];
            game.set_state(state);
            play_trainer(&mut game, 0, penny);
        }

        game.apply_action(&deckgym::actions::Action {
            actor: 0,
            action: deckgym::test_support::attack_action(CardId::A1001Bulbasaur, 0),
            is_stack: false,
        });
        120 - game.get_state_clone().get_remaining_hp(1, 0)
    };

    assert_eq!(damage_with(false), 40, "Vine Whip alone does 40");
    assert_eq!(
        damage_with(true),
        50,
        "Penny should copy Giovanni's +10 damage"
    );
}

/// The copied Supporter stays in the opponent's deck — it is only looked at and shuffled back.
#[test]
fn test_penny_leaves_the_opponents_deck_composition_unchanged() {
    let mut game = get_initialized_game_with_board(
        0,
        0,
        3,
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1202Chansey)],
    );
    let penny = trainer_from_id(CardId::A3b069Penny);
    let mut state = game.get_state_clone();
    state.decks[1].cards = vec![
        get_card_by_enum(CardId::A1223Giovanni),
        get_card_by_enum(CardId::A1001Bulbasaur),
    ];
    state.hands[0] = vec![Card::Trainer(penny.clone())];
    // The opening hand is dealt from the real deck list, which can already contain a Giovanni.
    state.hands[1].clear();
    game.set_state(state);

    play_trainer(&mut game, 0, penny);

    let state = game.get_state_clone();
    assert_eq!(state.decks[1].cards.len(), 2);
    assert!(
        state.decks[1]
            .cards
            .iter()
            .any(|c| c.get_name() == "Giovanni"),
        "Giovanni should have been shuffled back into the opponent's deck"
    );
    assert!(
        !state.hands[1].iter().any(|c| c.get_name() == "Giovanni"),
        "The copied Supporter should go back to the deck, not to the opponent's hand"
    );
    assert!(
        !state.discard_piles[1]
            .iter()
            .any(|c| c.get_name() == "Giovanni"),
        "The copied Supporter should not be discarded from the opponent's deck"
    );
}

/// With no eligible Supporter in the opponent's deck, Penny is still playable and simply does
/// nothing beyond going to the discard pile.
#[test]
fn test_penny_with_no_supporter_in_the_opponents_deck_is_a_no_op() {
    let mut game = get_initialized_game_with_board(
        0,
        0,
        3,
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1202Chansey)],
    );
    let penny = trainer_from_id(CardId::A3b069Penny);
    let mut state = game.get_state_clone();
    state.decks[1].cards = vec![get_card_by_enum(CardId::A1001Bulbasaur)];
    state.hands[0] = vec![Card::Trainer(penny.clone())];
    game.set_state(state);

    play_trainer(&mut game, 0, penny);

    let state = game.get_state_clone();
    assert!(state.hands[0].is_empty());
    assert!(state.discard_piles[0]
        .iter()
        .any(|c| c.get_name() == "Penny"));
    assert_eq!(state.decks[1].cards.len(), 1);
}

/// Penny never copies another Penny, so a deck full of Pennies cannot recurse.
#[test]
fn test_penny_does_not_copy_another_penny() {
    let mut game = get_initialized_game_with_board(
        0,
        0,
        3,
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1202Chansey)],
    );
    let penny = trainer_from_id(CardId::B2a092Penny);
    let mut state = game.get_state_clone();
    state.decks[1].cards = vec![
        get_card_by_enum(CardId::A3b069Penny),
        get_card_by_enum(CardId::B2a092Penny),
    ];
    state.hands[0] = vec![Card::Trainer(penny.clone())];
    game.set_state(state);

    play_trainer(&mut game, 0, penny);

    let state = game.get_state_clone();
    assert_eq!(state.decks[1].cards.len(), 2);
    assert!(state.discard_piles[0]
        .iter()
        .any(|c| c.get_name() == "Penny"));
}
