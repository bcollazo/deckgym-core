use deckgym::{
    actions::Action,
    card_ids::CardId::{self},
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game, play_trainer, trainer_from_id},
};

#[test]
fn test_whiny_voice_flip_coin_interact_opponent_hand_and_decd() {
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();

        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![PlayedCard::from_id(CardId::B2104Purrloin).with_energy(vec![EnergyType::Darkness])],
            vec![PlayedCard::from_id(CardId::A1211Snorlax)],
        );
        state.hands[1] = vec![get_card_by_enum(CardId::B3028Emboar).clone()];
        state.decks[1].cards = vec![get_card_by_enum(CardId::A3175Magearna).clone()];
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::B2104Purrloin, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();

        // Whiny Voice: 0 damage dealt
        assert_eq!(state.get_active(1).get_remaining_hp(), 150, "seed {seed}");

        // Assert whether opponents card in hand went to shuffeld deck
        assert_eq!(
            state.hands[1].len() + state.decks[1].cards.len(),
            2,
            "seed {seed}"
        );
        assert_eq!(
            state.hands[1].len() + state.decks[1].cards.len(),
            2,
            "seed {seed}: Emboar should be on decks when heads"
        );
        assert!(
            state.decks[1].cards == vec![get_card_by_enum(CardId::A3175Magearna).clone()]
                || state.decks[1].cards
                    == vec![
                        get_card_by_enum(CardId::A3175Magearna).clone(),
                        get_card_by_enum(CardId::B3028Emboar).clone()
                    ]
                || state.decks[1].cards
                    == vec![
                        get_card_by_enum(CardId::B3028Emboar).clone(),
                        get_card_by_enum(CardId::A3175Magearna).clone()
                    ]
        )
    }
}

#[test]
fn test_whiny_voice_interaction_trainer_will() {
    // Trainer Will will force the first next coin flip to be heads
    let will = trainer_from_id(CardId::A4156Will);
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.current_player = 0;
    state.turn_count = 1;

    state.set_board(
        vec![PlayedCard::from_id(CardId::B2104Purrloin).with_energy(vec![EnergyType::Darkness])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    state.hands[0] = vec![get_card_by_enum(CardId::A4156Will).clone()];
    state.hands[1] = vec![get_card_by_enum(CardId::B3028Emboar).clone()];
    state.decks[1].cards = vec![get_card_by_enum(CardId::A3175Magearna).clone()];
    game.set_state(state);

    play_trainer(&mut game, 0, will.clone());
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2104Purrloin, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();

    // Whiny Voice: 0 damage dealt
    assert_eq!(state.get_active(1).get_remaining_hp(), 150);

    // Will forces heads, thus forces Whiny Voice to trigger
    assert_eq!(state.hands[1].len() + state.decks[1].cards.len(), 2);
    assert!(
        state.decks[1].cards
            == vec![
                get_card_by_enum(CardId::A3175Magearna).clone(),
                get_card_by_enum(CardId::B3028Emboar).clone()
            ]
            || state.decks[1].cards
                == vec![
                    get_card_by_enum(CardId::B3028Emboar).clone(),
                    get_card_by_enum(CardId::A3175Magearna).clone()
                ]
    )
}
