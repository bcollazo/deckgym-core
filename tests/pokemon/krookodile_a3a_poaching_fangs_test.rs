use deckgym::{
    actions::Action,
    card_ids::CardId::{self},
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game, play_trainer, trainer_from_id},
};

#[test]
fn test_poaching_fangs_flip_coin_interact_opponent_hand_and_deck() {
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();

        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![
                PlayedCard::from_id(CardId::A3a041Krookodile).with_energy(vec![
                    EnergyType::Darkness,
                    EnergyType::Darkness,
                    EnergyType::Darkness,
                ]),
            ],
            vec![PlayedCard::from_id(CardId::A1211Snorlax)],
        );
        state.hands[1] = vec![
            get_card_by_enum(CardId::B3028Emboar).clone(),
            get_card_by_enum(CardId::B3028Emboar).clone(),
            get_card_by_enum(CardId::B3028Emboar).clone(),
        ];
        state.decks[1].cards = vec![get_card_by_enum(CardId::A3175Magearna).clone()];
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A3a041Krookodile, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();

        // Poaching Fangs: 90 damage dealt
        assert_eq!(state.get_active(1).get_remaining_hp(), 60, "seed {seed}");

        // Assert that cards from opponent hand goes to their deck
        assert_eq!(
            state.hands[1].len() + state.decks[1].cards.len(),
            4,
            "seed {seed}"
        );
        assert!(
            !state.decks[1].cards.is_empty() && state.hands[1].len() <= 3,
            "seed {seed}"
        );
        let emboar = get_card_by_enum(CardId::B3028Emboar);
        assert!(
            state.hands[1].contains(&emboar) || state.decks[1].cards.contains(&emboar),
            "seed {seed}: Emboar shoulnd't move outside oppoenet hand <-> deck"
        );
    }
}

#[test]
fn test_poaching_fangs_interaction_trainer_will() {
    // Trainer Will will force the first next coin flip to be heads
    let will = trainer_from_id(CardId::A4156Will);
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.current_player = 0;
    state.turn_count = 1;

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A3a041Krookodile).with_energy(vec![
                EnergyType::Darkness,
                EnergyType::Darkness,
                EnergyType::Darkness,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    state.hands[0] = vec![get_card_by_enum(CardId::A4156Will).clone()];
    state.hands[1] = vec![
        get_card_by_enum(CardId::B3028Emboar).clone(),
        get_card_by_enum(CardId::B3028Emboar).clone(),
        get_card_by_enum(CardId::B3028Emboar).clone(),
    ];
    state.decks[1].cards = vec![get_card_by_enum(CardId::A3175Magearna).clone()];
    game.set_state(state);

    play_trainer(&mut game, 0, will.clone());
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A3a041Krookodile, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();

    // Poaching Fangs: 90 damage dealt
    assert_eq!(state.get_active(1).get_remaining_hp(), 60);

    // Will forces heads, thus forces Poaching Fangs to trigger
    assert_eq!(state.hands[1].len() + state.decks[1].cards.len(), 4);
    assert!(state.decks[1].cards.len() >= 2 && state.hands[1].len() <= 2);
    let emboar = get_card_by_enum(CardId::B3028Emboar);
    assert!(
        state.hands[1].contains(&emboar) || state.decks[1].cards.contains(&emboar),
        "Emboar shoulnd't move outside oppoenet hand <-> deck"
    );
}
