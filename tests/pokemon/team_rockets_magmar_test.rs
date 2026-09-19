use deckgym::{
    card_ids::CardId,
    actions::Action,
    models::{EnergyType, PlayedCard, StatusCondition},
    test_support::{attack_action, get_initialized_game},
};

/// Team Rocket's Magmar's Derisive Roasting: 10 damage + 50 more
/// for each Special Condition affecting the opponent's Active Pokémon.
#[test]
fn test_magmar_derisive_roasting_base_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B4a006TeamRocketsMagmar)
            .with_energy(vec![EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4a006TeamRocketsMagmar, 0),
        is_stack: false,
    });
    game.play_until_stable();

    // No special conditions: should deal base 10 damage, Snorlax has 150hp
    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 140);
}

#[test]
fn test_magmar_derisive_roasting_one_condition() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B4a006TeamRocketsMagmar)
            .with_energy(vec![EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    state.apply_status_condition(1, 0, StatusCondition::Poisoned);
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4a006TeamRocketsMagmar, 0),
        is_stack: false,
    });
    game.play_until_stable();

    // One special condition: 10 + 50 = 60 damage, Snorlax has 150hp
    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 90);
}