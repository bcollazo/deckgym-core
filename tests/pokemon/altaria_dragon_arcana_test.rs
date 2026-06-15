use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn altaria_dragon_arcana_uses_bonus_damage_with_two_energy_types() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    let altaria = PlayedCard::from_id(CardId::A4a055Altaria).with_energy(vec![
        EnergyType::Colorless,
        EnergyType::Colorless,
        EnergyType::Dragon,
    ]);
    let opponent = PlayedCard::from_id(CardId::A2119DialgaEx).with_remaining_hp(200);

    state.set_board(vec![altaria], vec![opponent]);
    state.current_player = 0;
    state.turn_count = 1;
    game.set_state(state.clone());

    let initial_hp = state.get_active(1).get_remaining_hp();

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A4a055Altaria, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let opponent_active = state.get_active(1);
    assert_eq!(opponent_active.get_remaining_hp(), initial_hp - 100);
}

#[test]
fn altaria_dragon_arcana_uses_base_damage_with_one_energy_type() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    let altaria = PlayedCard::from_id(CardId::A4a055Altaria)
        .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless]);
    let opponent = PlayedCard::from_id(CardId::A2119DialgaEx).with_remaining_hp(200);

    state.set_board(vec![altaria], vec![opponent]);
    state.current_player = 0;
    state.turn_count = 1;
    game.set_state(state.clone());

    let initial_hp = state.get_active(1).get_remaining_hp();

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A4a055Altaria, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let opponent_active = state.get_active(1);
    assert_eq!(opponent_active.get_remaining_hp(), initial_hp - 40);
}
