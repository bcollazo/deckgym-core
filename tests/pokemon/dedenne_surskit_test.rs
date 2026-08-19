use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_dedenne_thunder_wave_discards_lightning_from_opponent() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B1094Dedenne).with_energy(vec![EnergyType::Colorless])],
        // Opponent with 2 [L] attached
        vec![PlayedCard::from_id(CardId::A1211Snorlax)
            .with_energy(vec![EnergyType::Lightning, EnergyType::Lightning])],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B1094Dedenne, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    // 20 damage -> Snorlax 130
    assert_eq!(state.get_active(1).get_remaining_hp(), 130);
    // One [L] discarded from opponent's active
    assert_eq!(state.get_active(1).attached_energy.len(), 1);
    assert_eq!(state.discard_energies[1].len(), 1);
    assert_eq!(state.discard_energies[1][0], EnergyType::Lightning);
}

#[test]
fn test_surskit_bubble_discards_fire_from_opponent() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B3009Surskit).with_energy(vec![EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)
            .with_energy(vec![EnergyType::Fire, EnergyType::Fire])],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3009Surskit, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    // 10 damage -> Snorlax 140
    assert_eq!(state.get_active(1).get_remaining_hp(), 140);
    assert_eq!(state.get_active(1).attached_energy.len(), 1);
    assert_eq!(state.discard_energies[1].len(), 1);
    assert_eq!(state.discard_energies[1][0], EnergyType::Fire);
}
