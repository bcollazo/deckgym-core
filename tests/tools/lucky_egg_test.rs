use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

fn play_lucky_egg_on_active(game: &mut deckgym::Game<'static>) {
    let lucky_egg = get_card_by_enum(CardId::B3148LuckyEgg);
    let trainer_card = match lucky_egg {
        Card::Trainer(tc) => tc,
        _ => panic!("Expected trainer card"),
    };

    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let state = game.get_state_clone();
    let (_actor, choices) = state.generate_possible_actions();
    let attach_action = choices
        .iter()
        .find(|action| {
            matches!(
                action.action,
                SimpleAction::AttachTool { in_play_idx: 0, .. }
            )
        })
        .cloned()
        .expect("Expected attach tool choice for active slot 0");
    game.apply_action(&attach_action);
}

#[test]
fn test_lucky_egg_draws_to_five_on_opponent_attack_ko() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1033Charmander), // bench so game doesn't immediately end
        ],
        vec![PlayedCard::from_id(CardId::A1143Machop)],
    );
    state.current_player = 0;
    state.turn_count = 2;

    // Give player 0 only 2 cards in hand (below 5)
    state.hands[0] = vec![
        get_card_by_enum(CardId::B3148LuckyEgg),
        get_card_by_enum(CardId::A1001Bulbasaur),
    ];
    // Fill deck so there are cards to draw
    state.decks[0].cards = vec![
        get_card_by_enum(CardId::PA001Potion),
        get_card_by_enum(CardId::PA001Potion),
        get_card_by_enum(CardId::PA001Potion),
        get_card_by_enum(CardId::PA001Potion),
        get_card_by_enum(CardId::PA001Potion),
    ];
    state.in_play_pokemon[0][0]
        .as_mut()
        .unwrap()
        .attached_energy = vec![EnergyType::Grass];

    game.set_state(state);

    // Attach Lucky Egg to player 0's active
    play_lucky_egg_on_active(&mut game);

    // Player 1 delivers a lethal attack to player 0's active Bulbasaur
    let ko_action = Action {
        actor: 1,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (1, 0),
            targets: vec![(100, 0, 0)],
            is_from_active_attack: true,
        },
        is_stack: false,
    };
    game.apply_action(&ko_action);

    let state = game.get_state_clone();
    // Player 0's Bulbasaur is KO'd - Lucky Egg should have drawn to 5
    assert!(
        state.in_play_pokemon[0][0].is_none(),
        "Player 0's active should be KO'd"
    );
    assert_eq!(
        state.hands[0].len(),
        5,
        "Lucky Egg should draw hand to 5 cards, got {}",
        state.hands[0].len()
    );
}

#[test]
fn test_lucky_egg_does_not_draw_without_opponent_attack() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
        vec![PlayedCard::from_id(CardId::A1143Machop)],
    );
    state.current_player = 0;
    state.turn_count = 2;

    state.hands[0] = vec![
        get_card_by_enum(CardId::B3148LuckyEgg),
        get_card_by_enum(CardId::A1001Bulbasaur),
    ];
    state.decks[0].cards = vec![
        get_card_by_enum(CardId::PA001Potion),
        get_card_by_enum(CardId::PA001Potion),
        get_card_by_enum(CardId::PA001Potion),
    ];
    state.in_play_pokemon[0][0]
        .as_mut()
        .unwrap()
        .attached_energy = vec![EnergyType::Grass];

    game.set_state(state);

    play_lucky_egg_on_active(&mut game);

    // Use is_from_active_attack = false (not a direct opponent attack)
    let ko_action = Action {
        actor: 1,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (1, 0),
            targets: vec![(100, 0, 0)],
            is_from_active_attack: false,
        },
        is_stack: false,
    };
    game.apply_action(&ko_action);

    let state = game.get_state_clone();
    assert!(
        state.in_play_pokemon[0][0].is_none(),
        "Player 0's active should be KO'd"
    );
    // Hand should still be 2 (1 card was the Lucky Egg but it was attached, so 1 remaining + the Bulbasaur)
    // The Lucky Egg card goes to discard with the Pokemon, so hand stays at 1 (the Bulbasaur)
    assert!(
        state.hands[0].len() < 5,
        "Lucky Egg should NOT draw when KO is not from opponent's active attack, hand={}",
        state.hands[0].len()
    );
}
