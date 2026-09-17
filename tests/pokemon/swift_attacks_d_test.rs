use deckgym::{
    actions::Action,
    card_ids::CardId,
    effects::CardEffect,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Starmie - Swift: "This attack's damage isn't affected by Weakness or by any effects on your
/// opponent's Active Pokémon." Turtonator is [R] and Weak to [W], but takes the printed 60 rather
/// than a weakness-boosted 80.
#[test]
fn test_starmie_swift_damage_is_not_boosted_by_weakness() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4033Starmie)
            .with_energy(vec![EnergyType::Water, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A3037Turtonator)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4033Starmie, 0),
        is_stack: false,
    });
    game.play_until_stable();

    // 120 HP - 60 = 60 (a weakness-boosted 80 would leave 40).
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 60);
}

/// Ledian - Swift: same clause, against an Onix that is Weak to [G].
#[test]
fn test_ledian_swift_damage_is_not_boosted_by_weakness() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2002Ledian).with_energy(vec![EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1150Onix)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2002Ledian, 0),
        is_stack: false,
    });
    game.play_until_stable();

    // 110 HP - 40 = 70 (a weakness-boosted 60 would leave 50).
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 70);
}

/// Swift also ignores damage-reducing effects sitting on the opponent's Active Pokémon.
#[test]
fn test_starmie_swift_ignores_effects_on_the_defender() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4033Starmie)
            .with_energy(vec![EnergyType::Water, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    let mut state = game.get_state_clone();
    state.in_play_pokemon[1][0]
        .as_mut()
        .expect("defender")
        .add_effect(CardEffect::ReducedDamage { amount: 30 }, 1);
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4033Starmie, 0),
        is_stack: false,
    });
    game.play_until_stable();

    // The full 60 lands despite the -30 reduction on the defender.
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 90);
}

/// Mew - Psy Report / Noctowl - Silent Wing: revealing the opponent's hand is informational only,
/// so the attack resolves as its plain damage and leaves the hand intact.
#[test]
fn test_psy_report_deals_its_damage_and_leaves_the_hand_alone() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1283Mew).with_energy(vec![EnergyType::Psychic])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    let hand_before = game.get_state_clone().hands[1].clone();

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1283Mew, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 130);
    assert_eq!(state.hands[1], hand_before);
}
