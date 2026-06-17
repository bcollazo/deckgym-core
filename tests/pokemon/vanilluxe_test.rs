use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_vanilluxe_sweets_relay_no_bonus_without_prior_use() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B2039Vanilluxe)
            .with_energy(vec![EnergyType::Water, EnergyType::Water])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2039Vanilluxe, 0),
        is_stack: false,
    });

    let final_state = game.get_state_clone();
    assert_eq!(
        final_state.get_active(1).get_remaining_hp(),
        10,
        "Sweets Relay should only deal its base 60 damage with no prior Sweets Relay use"
    );
}

#[test]
fn test_vanilluxe_sweets_relay_gets_bonus_after_own_last_turn_use() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B2039Vanilluxe)
            .with_energy(vec![EnergyType::Water, EnergyType::Water])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    state.set_attack_name_used_last_turn(0, Some("Sweets Relay".to_string()));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2039Vanilluxe, 0),
        is_stack: false,
    });

    let final_state = game.get_state_clone();
    assert_eq!(
        final_state.get_active(1).get_remaining_hp(),
        30,
        "Sweets Relay should deal 60 base + 60 bonus damage after a prior turn's Sweets Relay use"
    );
}
