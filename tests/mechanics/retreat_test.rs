use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

#[test]
fn test_retreat_discards_energy_into_discard_energies() {
    let mut game = get_initialized_game(42);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1035Charizard)
                .with_energy(vec![EnergyType::Fire, EnergyType::Water]),
            PlayedCard::from_id(CardId::A1053Squirtle),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.turn_count = 3;
    state.current_player = 0;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Retreat(1),
        is_stack: false,
    });

    let state = game.get_state_clone();

    assert_eq!(state.discard_energies[0].len(), 2);
    assert!(state.discard_energies[0].contains(&EnergyType::Fire));
    assert!(state.discard_energies[0].contains(&EnergyType::Water));
}

#[test]
fn test_forced_activate_does_not_consume_current_player_retreat() {
    let mut game = get_initialized_game(42);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_energy(vec![EnergyType::Grass]),
            PlayedCard::from_id(CardId::A1033Charmander).with_energy(vec![EnergyType::Fire]),
        ],
        vec![
            PlayedCard::from_id(CardId::A1094Pikachu),
            PlayedCard::from_id(CardId::A1053Squirtle),
        ],
    );
    state.turn_count = 3;
    state.current_player = 0;
    game.set_state(state);

    let (_, before_actions) = game.get_state_clone().generate_possible_actions();
    assert!(
        before_actions
            .iter()
            .any(|a| matches!(a.action, SimpleAction::Retreat(1))),
        "Expected Retreat(1) to be available before forced switch"
    );

    // My forced switch
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Activate {
            player: 0,
            in_play_idx: 1,
        },
        is_stack: false,
    });
    // Opponent's forced switch
    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::Activate {
            player: 1,
            in_play_idx: 1,
        },
        is_stack: false,
    });

    let (_, after_actions) = game.get_state_clone().generate_possible_actions();
    assert!(
        after_actions
            .iter()
            .any(|a| matches!(a.action, SimpleAction::Retreat(1))),
        "Forced switch should not consume current player's retreat right"
    );
}
