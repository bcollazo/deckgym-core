use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_oricorio_inspiring_dance_increases_damage_on_next_turn() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B2022Oricorio).with_energy(vec![EnergyType::Fire])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax).with_energy(vec![EnergyType::Grass])],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2022Oricorio, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let mut next_turn_state = game.get_state_clone();
    next_turn_state.turn_count = 2;
    next_turn_state.current_player = 0;
    game.set_state(next_turn_state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2022Oricorio, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 110);
}

#[test]
fn test_happiny_chubby_cheer_b4a063_increases_damage_on_next_turn() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B4a063Happiny)],
        vec![PlayedCard::from_id(CardId::A1211Snorlax).with_energy(vec![EnergyType::Grass])],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4a063Happiny, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let mut next_turn_state = game.get_state_clone();
    next_turn_state.turn_count = 2;
    next_turn_state.current_player = 0;
    game.set_state(next_turn_state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4a063Happiny, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 110);
}

#[test]
fn test_happiny_chubby_cheer_b4a077_increases_damage_on_next_turn() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B4a077Happiny)],
        vec![PlayedCard::from_id(CardId::A1211Snorlax).with_energy(vec![EnergyType::Grass])],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4a077Happiny, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let mut next_turn_state = game.get_state_clone();
    next_turn_state.turn_count = 2;
    next_turn_state.current_player = 0;
    game.set_state(next_turn_state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4a077Happiny, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 110);
}
