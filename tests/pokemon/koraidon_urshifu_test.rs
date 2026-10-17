use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_koraidon_collision_course_discards_two_fighting_energy() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B2a063Koraidon).with_energy(vec![
                EnergyType::Fighting,
                EnergyType::Fighting,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
        ],
        // Wailord ex (Water) is not weak to Fighting
        vec![PlayedCard::from_id(CardId::B4197WailordEx)],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2a063Koraidon, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    // 120 damage -> Wailord 130
    assert_eq!(state.get_active(1).get_remaining_hp(), 130);
    // The 2 [F] were discarded, leaving only the 2 [C]
    assert_eq!(state.get_active(0).attached_energy.len(), 2);
    assert_eq!(state.discard_energies[0].len(), 2);
    assert!(
        state.discard_energies[0]
            .iter()
            .all(|&e| e == EnergyType::Fighting),
        "discarded energies must be Fighting"
    );
}

#[test]
fn test_single_strike_urshifu_giga_impact_discards_one_darkness() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B3113SingleStrikeUrshifu).with_energy(vec![
                EnergyType::Darkness,
                EnergyType::Darkness,
                EnergyType::Colorless,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::B4197WailordEx)],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3113SingleStrikeUrshifu, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    // 110 damage -> Wailord 250 - 110 = 140
    assert_eq!(state.get_active(1).get_remaining_hp(), 140);
    // One [D] discarded: 3 - 1 = 2 remaining
    assert_eq!(state.get_active(0).attached_energy.len(), 2);
    assert_eq!(state.discard_energies[0].len(), 1);
    assert_eq!(state.discard_energies[0][0], EnergyType::Darkness);
}
