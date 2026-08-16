use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_giratina_shadow_force_discards_two_random_energy() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A2a061Giratina).with_energy(vec![
                EnergyType::Grass,
                EnergyType::Psychic,
                EnergyType::Colorless,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::B4197WailordEx)],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A2a061Giratina, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    // 120 damage -> Wailord 130
    assert_eq!(state.get_active(1).get_remaining_hp(), 130);
    // 2 of the 3 energies discarded
    assert_eq!(state.get_active(0).attached_energy.len(), 1);
    assert_eq!(state.discard_energies[0].len(), 2);
}

#[test]
fn test_rayquaza_dragon_breath_discards_two_random_energy() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B4119Rayquaza).with_energy(vec![
            EnergyType::Fire,
            EnergyType::Lightning,
            EnergyType::Colorless,
            EnergyType::Colorless,
        ])],
        vec![PlayedCard::from_id(CardId::B4197WailordEx)],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4119Rayquaza, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    // 140 damage -> Wailord 110
    assert_eq!(state.get_active(1).get_remaining_hp(), 110);
    // 2 of the 4 energies discarded
    assert_eq!(state.get_active(0).attached_energy.len(), 2);
    assert_eq!(state.discard_energies[0].len(), 2);
}
