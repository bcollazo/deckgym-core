use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Wishiwashi ex - School Storm: 30 damage, +40 for each Benched Wishiwashi *or* Wishiwashi ex.
#[test]
fn test_wishiwashi_ex_school_storm_counts_both_wishiwashi_names() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A3051WishiwashiEx).with_energy(vec![
                EnergyType::Water,
                EnergyType::Water,
                EnergyType::Water,
            ]),
            PlayedCard::from_id(CardId::A3051WishiwashiEx),
            PlayedCard::from_id(CardId::A1211Snorlax),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A3051WishiwashiEx, 0),
        is_stack: false,
    });
    game.play_until_stable();

    // 30 + 1 * 40 = 70 (the Active Wishiwashi ex does not count itself; Snorlax does not count)
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 80);
}

/// Team Rocket's Magmar - Derisive Roasting: 10 damage, +50 per Special Condition on the defender.
#[test]
fn test_team_rockets_magmar_derisive_roasting_scales_with_special_conditions() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4a006TeamRocketsMagmar)
            .with_energy(vec![EnergyType::Fire])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    // No conditions: plain 10 damage.
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4a006TeamRocketsMagmar, 0),
        is_stack: false,
    });
    game.play_until_stable();
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 140);

    // Two conditions: 10 + 2 * 50 = 110.
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.apply_status_condition(1, 0, deckgym::models::StatusCondition::Poisoned);
    state.apply_status_condition(1, 0, deckgym::models::StatusCondition::Burned);
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4a006TeamRocketsMagmar, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 30);
}

/// Teal Mask Ogerpon - Ogre's Whip: damage equal to its own remaining HP.
#[test]
fn test_teal_mask_ogerpon_ogres_whip_damage_equals_its_remaining_hp() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4019TealMaskOgerpon)
            .with_energy(vec![
                EnergyType::Grass,
                EnergyType::Grass,
                EnergyType::Colorless,
            ])
            .with_remaining_hp(60)],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4019TealMaskOgerpon, 0),
        is_stack: false,
    });
    game.play_until_stable();

    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 90);
}

/// Roserade - Poison Ring: 50 damage, Poisoned, and the defender cannot retreat next turn.
#[test]
fn test_roserade_poison_ring_poisons_and_blocks_retreat() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2005Roserade)
            .with_energy(vec![EnergyType::Grass, EnergyType::Grass])],
        vec![
            PlayedCard::from_id(CardId::A1211Snorlax).with_energy(vec![
                EnergyType::Colorless,
                EnergyType::Colorless,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2005Roserade, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 100);
    assert!(state.get_active(1).is_poisoned());

    // The defender's own turn: retreating must not be offered despite paying-capable Energy.
    game.play_until_stable();
    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    if actor == 1 {
        assert!(
            !choices
                .iter()
                .any(|choice| matches!(choice.action, deckgym::actions::SimpleAction::Retreat(_))),
            "Poison Ring should block the defender's retreat"
        );
    }
}
