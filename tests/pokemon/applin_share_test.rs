use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

/// Test that Applin's "Share" attack lets the player heal 30 damage from one
/// of their Benched Pokémon, but does not offer the active Applin itself.
#[test]
fn test_applin_share_heals_benched_pokemon() {
    let mut game = get_initialized_game(0);

    let mut state = game.get_state_clone();
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B3019Applin)
                .with_energy(vec![EnergyType::Colorless])
                .with_remaining_hp(10),
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_remaining_hp(20),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    state.current_player = 0;
    game.set_state(state);

    let attack_action = Action {
        actor: 0,
        action: attack_action(CardId::B3019Applin, 0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    // Player must choose which Pokémon to heal; only the Benched Bulbasaur
    // should be offered, not the damaged active Applin.
    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert_eq!(choices.len(), 1);
    assert_eq!(
        choices[0].action,
        SimpleAction::Heal {
            in_play_idx: 1,
            amount: 30,
            cure_status: false,
        }
    );

    game.apply_action(&choices[0].clone());

    let final_state = game.get_state_clone();
    assert_eq!(
        final_state.in_play_pokemon[0][1]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        50
    );
    // Active Applin should remain damaged.
    assert_eq!(
        final_state.in_play_pokemon[0][0]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        10
    );
}

/// Test that Applin's "Share" attack does nothing if there is no damaged
/// Benched Pokémon to heal.
#[test]
fn test_applin_share_no_choices_when_bench_undamaged() {
    let mut game = get_initialized_game(0);

    let mut state = game.get_state_clone();
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B3019Applin).with_energy(vec![EnergyType::Colorless]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    state.current_player = 0;
    game.set_state(state);

    let attack_action = Action {
        actor: 0,
        action: attack_action(CardId::B3019Applin, 0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    // No Pokémon to heal, so the only option is to end the turn.
    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert_eq!(
        choices,
        vec![Action {
            actor: 0,
            action: SimpleAction::EndTurn,
            is_stack: false,
        }]
    );
}
