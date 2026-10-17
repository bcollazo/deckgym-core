use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

const VENUSAUR_EX_MAX_HP: u32 = 190;

/// Regidrago's Draconic Slam: 140 damage normally, but only 40 damage ("-100 damage") if
/// Regidrago itself already has damage on it.
#[test]
fn test_regidrago_draconic_slam_full_damage_when_undamaged() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B4a057Regidrago).with_energy(vec![
                EnergyType::Grass,
                EnergyType::Fire,
                EnergyType::Colorless,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::A1004VenusaurEx)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4a057Regidrago, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        VENUSAUR_EX_MAX_HP - 140
    );
}

#[test]
fn test_regidrago_draconic_slam_reduced_damage_when_self_damaged() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4a057Regidrago)
            .with_energy(vec![
                EnergyType::Grass,
                EnergyType::Fire,
                EnergyType::Colorless,
            ])
            .with_damage(10)],
        vec![PlayedCard::from_id(CardId::A1004VenusaurEx)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4a057Regidrago, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        VENUSAUR_EX_MAX_HP - 40
    );
}
