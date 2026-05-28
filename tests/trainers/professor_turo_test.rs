use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, PlayedCard, TrainerCard},
    state::GameOutcome,
    test_support::get_initialized_game,
};

fn make_turo_trainer_card() -> TrainerCard {
    match get_card_by_enum(CardId::B3a073ProfessorTuro) {
        Card::Trainer(tc) => tc,
        _ => panic!("Expected trainer card"),
    }
}

fn make_iron_hands() -> PlayedCard {
    PlayedCard::from_id(CardId::B3a017IronHands)
}

/// When Professor Turo is played and the player chooses their Active Future Pokémon,
/// they must then promote a Pokémon from the bench.
#[test]
fn test_professor_turo_active_triggers_promotion() {
    let mut game = get_initialized_game(1);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    // Player 0: Iron Hands (Future) active + Bulbasaur on bench
    state.set_board(
        vec![
            make_iron_hands(),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    state.hands[0].clear();
    let trainer_card = make_turo_trainer_card();
    state.hands[0].push(Card::Trainer(trainer_card.clone()));
    game.set_state(state);

    // Play Professor Turo
    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    };
    game.apply_action(&play_action);

    // Now the game should ask which Future Pokémon to shuffle — choose the active (index 0)
    let state = game.get_state_clone();
    let (actor, choices) = state.generate_possible_actions();
    assert_eq!(actor, 0);
    let shuffle_active = choices.iter().find(|a| {
        matches!(
            &a.action,
            SimpleAction::ShuffleInPlayPokemonIntoDeck { in_play_idx: 0 }
        )
    });
    assert!(
        shuffle_active.is_some(),
        "Should be able to choose Active pokemon to shuffle, choices: {:?}",
        choices
    );

    game.apply_action(shuffle_active.unwrap());

    // After shuffling active, Iron Hands should be in the deck
    let state = game.get_state_clone();
    let iron_hands_card = get_card_by_enum(CardId::B3a017IronHands);
    assert!(
        state.decks[0].cards.contains(&iron_hands_card),
        "Shuffled Pokemon should be in deck"
    );

    // Player 0 should now be prompted to promote a bench pokemon
    let (promo_actor, promo_choices) = state.generate_possible_actions();
    assert_eq!(promo_actor, 0);
    assert!(
        promo_choices
            .iter()
            .any(|a| matches!(a.action, SimpleAction::Activate { in_play_idx: 1, .. })),
        "Should be prompted to promote bench pokemon, choices: {:?}",
        promo_choices
    );
}

/// When Professor Turo is played with the Active Future Pokémon and no bench Pokémon,
/// the player loses immediately.
#[test]
fn test_professor_turo_active_no_bench_loses() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    // Player 0: Iron Hands (Future) active only, no bench
    state.set_board(
        vec![make_iron_hands()],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    state.hands[0].clear();
    let trainer_card = make_turo_trainer_card();
    state.hands[0].push(Card::Trainer(trainer_card.clone()));
    game.set_state(state);

    // Play Professor Turo
    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    };
    game.apply_action(&play_action);

    // Should only offer shuffling the active (no bench Future Pokemon either)
    let state = game.get_state_clone();
    let (actor, choices) = state.generate_possible_actions();
    assert_eq!(actor, 0);
    let shuffle_active = choices.iter().find(|a| {
        matches!(
            &a.action,
            SimpleAction::ShuffleInPlayPokemonIntoDeck { in_play_idx: 0 }
        )
    });
    assert!(
        shuffle_active.is_some(),
        "Should be able to choose Active pokemon, choices: {:?}",
        choices
    );

    game.apply_action(shuffle_active.unwrap());

    // With no bench pokemon, player 0 should lose
    let state = game.get_state_clone();
    assert_eq!(
        state.winner,
        Some(GameOutcome::Win(1)),
        "Player with no bench should lose when active is shuffled away"
    );
}
