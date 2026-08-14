use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

#[test]
fn test_luxray_revenge_blast_scales_with_opponent_points() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4053Luxray).with_energy(vec![
            EnergyType::Lightning,
            EnergyType::Lightning,
            EnergyType::Colorless,
        ])],
        vec![PlayedCard::from_id(CardId::B2b111MegaCharizardXEx)],
    );

    let mut state = game.get_state_clone();
    state.points = [0, 2];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4053Luxray, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    // Base 80 damage + 50 more damage per opponent point (2 points) = 180
    assert_eq!(state.get_active(1).get_remaining_hp(), 220 - 180);
}

#[test]
fn test_luxray_revenge_blast_no_bonus_when_opponent_has_no_points() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4053Luxray).with_energy(vec![
            EnergyType::Lightning,
            EnergyType::Lightning,
            EnergyType::Colorless,
        ])],
        vec![PlayedCard::from_id(CardId::B2b111MegaCharizardXEx)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4053Luxray, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 220 - 80);
}
