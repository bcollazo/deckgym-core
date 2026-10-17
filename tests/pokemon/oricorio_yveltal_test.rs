use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_oricorio_kindle_discards_random_energy_from_both_actives() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![PlayedCard::from_id(CardId::A3034Oricorio).with_energy(vec![
            EnergyType::Fire,
            EnergyType::Fire,
            EnergyType::Grass,
        ])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)
            .with_energy(vec![EnergyType::Lightning, EnergyType::Grass])],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A3034Oricorio, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    // 40 damage -> Snorlax 110
    assert_eq!(state.get_active(1).get_remaining_hp(), 110);
    // One random energy discarded from EACH active
    assert_eq!(state.get_active(0).attached_energy.len(), 2);
    assert_eq!(state.get_active(1).attached_energy.len(), 1);
    assert_eq!(state.discard_energies[0].len(), 1);
    assert_eq!(state.discard_energies[1].len(), 1);
}

#[test]
fn test_yveltal_evil_crash_discards_random_energy_from_both_actives() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B2108Yveltal).with_energy(vec![
            EnergyType::Darkness,
            EnergyType::Darkness,
            EnergyType::Darkness,
        ])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)
            .with_energy(vec![EnergyType::Fire, EnergyType::Water])],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2108Yveltal, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    // 90 damage -> Snorlax 60
    assert_eq!(state.get_active(1).get_remaining_hp(), 60);
    assert_eq!(state.get_active(0).attached_energy.len(), 2);
    assert_eq!(state.get_active(1).attached_energy.len(), 1);
    assert_eq!(state.discard_energies[0].len(), 1);
    assert_eq!(state.discard_energies[1].len(), 1);
}
