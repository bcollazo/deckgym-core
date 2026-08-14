use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

#[test]
fn test_eelektross_energy_crush_scales_with_all_opponent_energy() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4058Eelektross)
            .with_energy(vec![EnergyType::Lightning, EnergyType::Colorless])],
        vec![
            PlayedCard::from_id(CardId::B2b111MegaCharizardXEx)
                .with_energy(vec![EnergyType::Fire, EnergyType::Fire]),
            PlayedCard::from_id(CardId::A1033Charmander).with_energy(vec![EnergyType::Fire]),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4058Eelektross, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    // Base 50 damage + 20 more damage per Energy across all opponent's Pokemon (3 total) = 110
    assert_eq!(state.get_active(1).get_remaining_hp(), 220 - 110);
}

#[test]
fn test_eelektross_energy_crush_base_damage_when_opponent_has_no_energy() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4058Eelektross)
            .with_energy(vec![EnergyType::Lightning, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::B2b111MegaCharizardXEx)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4058Eelektross, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 220 - 50);
}
