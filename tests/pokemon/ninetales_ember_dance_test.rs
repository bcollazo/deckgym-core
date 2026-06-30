use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Ninetales B3b 009 - Ember Dance: flip 9 coins, 20 damage for each heads.
/// Verifies the attack resolves without panic and deals a multiple of 20.
#[test]
fn test_ninetales_ember_dance_deals_damage_per_heads() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B3b009Ninetales).with_energy(vec![
                EnergyType::Fire,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    let initial_hp = game.get_state_clone().get_active(1).get_remaining_hp();

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3b009Ninetales, 0),
        is_stack: false,
    });

    let remaining_hp = game.get_state_clone().get_active(1).get_remaining_hp();
    let damage_dealt = initial_hp - remaining_hp;

    // Damage must be a multiple of 20 (20 per head, 0–9 heads possible)
    assert_eq!(
        damage_dealt % 20,
        0,
        "Ember Dance damage ({damage_dealt}) must be a multiple of 20"
    );
    assert!(
        damage_dealt <= 180,
        "Ember Dance max damage is 180 (9 heads × 20), got {damage_dealt}"
    );
}

/// Ember Dance is available when Ninetales has the required energy attached.
#[test]
fn test_ninetales_ember_dance_is_available_with_energy() {
    use deckgym::actions::SimpleAction;

    let game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B3b009Ninetales).with_energy(vec![
                EnergyType::Fire,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    let state = game.get_state_clone();
    let (_actor, actions) = state.generate_possible_actions();

    let has_ember_dance = actions.iter().any(|a| {
        if let SimpleAction::Attack(atk) = &a.action {
            atk.title == "Ember Dance"
        } else {
            false
        }
    });

    assert!(
        has_ember_dance,
        "Ember Dance should be available when Ninetales has [R][C][C] energy"
    );
}
