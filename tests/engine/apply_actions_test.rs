use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType},
    test_support::{get_initialized_game, init_random_players},
};

#[test]
fn test_initial_build_phase() {
    let game = get_initialized_game(rand::random());
    let state = game.get_state_clone();

    // Both players should have an active pokemon and are basic
    assert!(state.in_play_pokemon[0][0].is_some());
    assert!(&state.in_play_pokemon[0][0]
        .as_ref()
        .unwrap()
        .card
        .is_basic());
    assert!(state.in_play_pokemon[1][0].is_some());
    assert!(&state.in_play_pokemon[1][0]
        .as_ref()
        .unwrap()
        .card
        .is_basic());

    // No Supporter cards should have been played and thus in discard pile
    assert!(state.discard_piles[0].is_empty());
    assert!(state.discard_piles[1].is_empty());

    // Decks should have 14 and 15 cards
    assert_eq!(state.decks[state.current_player].cards.len(), 14);
    assert_eq!(state.decks[(state.current_player + 1) % 2].cards.len(), 15);
}

#[test]
fn test_no_supporter_or_evolutions_in_first_turns() {
    let players = init_random_players();
    let mut game = deckgym::Game::new(players, rand::random());
    let mut state = game.get_state_clone();
    while state.turn_count == 0 {
        let action = game.play_tick();
        state = game.get_state_clone();
        if let SimpleAction::Evolve { .. } = action.action {
            panic!("Evolution played in first 2 turns");
        }
        if let SimpleAction::Play { trainer_card } = action.action {
            if trainer_card.trainer_card_type == deckgym::models::TrainerType::Supporter {
                panic!("Supporter card played in first 2 turns");
            }
        }
    }
    while state.turn_count == 1 {
        let action = game.play_tick();
        state = game.get_state_clone();
        if let SimpleAction::Evolve { .. } = action.action {
            panic!("Evolution played in first 2 turns");
        }
    }
    while state.turn_count == 2 {
        let action = game.play_tick();
        state = game.get_state_clone();
        if let SimpleAction::Evolve { .. } = action.action {
            panic!("Evolution played in first 2 turns");
        }
    }
}

#[test]
fn test_cloned_cards_are_equal() {
    assert_eq!(
        get_card_by_enum(CardId::A1177Weezing),
        get_card_by_enum(CardId::A1177Weezing)
    );
    assert_eq!(
        get_card_by_enum(CardId::A1177Weezing).clone(),
        get_card_by_enum(CardId::A1177Weezing)
    );
    assert!(get_card_by_enum(CardId::A1177Weezing) == get_card_by_enum(CardId::A1177Weezing));
    assert!(
        get_card_by_enum(CardId::A1177Weezing).clone() == get_card_by_enum(CardId::A1177Weezing)
    );
}

#[test]
fn test_end_turn() {
    let mut game = get_initialized_game(rand::random());
    let state = game.get_state_clone();
    let current_player = state.current_player;
    let action = Action {
        actor: current_player,
        action: SimpleAction::EndTurn,
        is_stack: false,
    };
    game.apply_action(&action);
    let state = game.get_state_clone();
    assert_ne!(current_player, state.current_player);
}

#[test]
fn test_draw_action() {
    let mut game = get_initialized_game(rand::random());
    let state = game.get_state_clone();
    let deck_size = state.decks[state.current_player].cards.len();
    let action = Action {
        actor: state.current_player,
        action: SimpleAction::DrawCard { amount: 1 },
        is_stack: false,
    };
    game.apply_action(&action);
    let state = game.get_state_clone();
    assert_eq!(deck_size - 1, state.decks[state.current_player].cards.len());
}

#[test]
fn test_play_pokeball_action() {
    let mut game = get_initialized_game(0);
    let state = game.get_state_clone();
    let current_player = state.current_player;
    let hand = state.hands[current_player].clone();
    let pokeball = &hand[2];
    if let Card::Trainer(trainer_card) = pokeball {
        let action = SimpleAction::Play {
            trainer_card: trainer_card.clone(),
        };

        let deck_size = state.decks[state.current_player].cards.len();
        let action = Action {
            actor: state.current_player,
            action,
            is_stack: false,
        };
        game.apply_action(&action);

        let state = game.get_state_clone();
        assert_eq!(hand.len(), state.hands[current_player].len()); // filled with basic
        assert_ne!(&state.hands[current_player][2], pokeball); // removed from hand
        assert_eq!(1, state.discard_piles[current_player].len()); // discarded
        assert_eq!(deck_size - 1, state.decks[state.current_player].cards.len());
        // deck size
    } else {
        panic!("Expected a trainer card");
    }
}

#[test]
fn test_place_action() {
    let mut game = get_initialized_game(3);
    let state = game.get_state_clone();
    let current_player = state.current_player;
    let hand = state.hands[current_player].clone();
    let bulbasaur = &hand[0];
    let action = SimpleAction::Place(bulbasaur.clone(), 2);
    assert_eq!(state.enumerate_bench_pokemon(current_player).count(), 0); // no bench
    game.set_state(state);

    let action = Action {
        actor: current_player,
        action,
        is_stack: false,
    };
    game.apply_action(&action);

    let state = game.get_state_clone();
    assert_eq!(hand.len() - 1, state.hands[current_player].len()); // removed from hand
    assert!(state.enumerate_bench_pokemon(current_player).count() > 0); // placed on bench
}

#[test]
fn test_attach_action() {
    let mut game = get_initialized_game(3);
    let state = game.get_state_clone();

    let current_player = state.current_player;
    let action = SimpleAction::Attach {
        attachments: vec![(1, EnergyType::Grass, 0)],
        is_turn_energy: true,
    };
    assert_eq!(
        &state.in_play_pokemon[current_player][0]
            .clone()
            .unwrap()
            .attached_energy,
        &vec![]
    ); // no energy

    // Assert no Attach actions are available
    let (actor, actions) = state.generate_possible_actions();
    assert!(!actions
        .iter()
        .any(|x| matches!(x.action, SimpleAction::Attach { .. })));

    let action = Action {
        actor,
        action,
        is_stack: false,
    };
    game.apply_action(&action);

    let state = game.get_state_clone();
    assert_eq!(
        &state.in_play_pokemon[current_player][0]
            .clone()
            .unwrap()
            .attached_energy,
        &vec![EnergyType::Grass]
    ); // 1 grass energy
}

/// Right after `get_initialized_game` finishes (setup phase ended, turn 1 has started),
/// the player going first must have no `current` energy (turn-1-no-attach rule) but both
/// players must already have a `next` energy visible — the queue preview.
#[test]
fn test_energy_zone_state_at_start_of_turn_1() {
    let game = get_initialized_game(0);
    let state = game.get_state_clone();
    assert_eq!(state.turn_count, 1);

    let first_player = state.current_player;
    let second_player = (first_player + 1) % 2;

    assert!(
        state.energy_zone[first_player].current.is_none(),
        "Player going first must NOT have an attachable energy on turn 1"
    );
    assert!(
        state.energy_zone[first_player].next.is_some(),
        "Player going first should already see the energy they'll receive on turn 3"
    );
    assert!(
        state.energy_zone[second_player].next.is_some(),
        "Player going second should already see the energy they'll receive on turn 2"
    );
}

/// After the player going first ends turn 1, the player going second must have a
/// `current` energy (their first attaching turn) and a fresh `next` rolled.
#[test]
fn test_energy_zone_rotates_on_turn_2() {
    let mut game = get_initialized_game(0);
    let state = game.get_state_clone();
    let first_player = state.current_player;
    let second_player = (first_player + 1) % 2;

    let second_player_next_at_start = state.energy_zone[second_player].next;

    // End turn 1 by directly applying EndTurn (the turn-1 transition path).
    game.apply_action(&Action {
        actor: first_player,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.turn_count, 2);
    assert_eq!(state.current_player, second_player);
    assert_eq!(
        state.energy_zone[second_player].current, second_player_next_at_start,
        "next slot must rotate into current at turn 2 start"
    );
    assert!(
        state.energy_zone[second_player].next.is_some(),
        "a fresh next must be rolled for the new active player"
    );
}
