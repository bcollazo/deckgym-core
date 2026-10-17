use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Whale Pump does 60 base + 10 more damage for each [W] Energy attached to Wailord.
#[test]
fn test_whale_pump_extra_damage_per_water_energy() {
    // Wailord with 2 [W] energies: 60 + 2*10 = 80 damage
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B1057Wailord)
            .with_energy(vec![EnergyType::Water, EnergyType::Water])],
        vec![PlayedCard::new(
            get_card_by_enum(CardId::A1001Bulbasaur),
            0,
            200,
            vec![],
            false,
            vec![],
        )],
    );
    let mut state = game.get_state_clone();
    state.current_player = 0;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B1057Wailord, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    // 60 + (2 * 10) = 80 damage, opponent had 200 HP -> 120 remaining
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        120,
        "Whale Pump should deal 80 damage with 2 Water energies (60 + 2*10)"
    );
}

/// With no [W] energy attached, Whale Pump should only deal its fixed 60 damage.
#[test]
fn test_whale_pump_no_water_energy() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B1057Wailord).with_energy(vec![
            EnergyType::Colorless,
            EnergyType::Colorless,
            EnergyType::Colorless,
            EnergyType::Colorless,
        ])],
        vec![PlayedCard::new(
            get_card_by_enum(CardId::A1001Bulbasaur),
            0,
            200,
            vec![],
            false,
            vec![],
        )],
    );
    let mut state = game.get_state_clone();
    state.current_player = 0;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B1057Wailord, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        140,
        "Whale Pump should deal only 60 fixed damage with no Water energy"
    );
}
