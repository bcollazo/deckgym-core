use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Illumise's Ire-Fly does 30 damage, or 30 + 60 = 90 when Volbeat (a Pokemon card, not a
/// Trainer) is in the attacker's discard pile.
#[test]
fn test_ire_fly_extra_damage_when_volbeat_in_discard() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4a002Illumise)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    let mut state = game.get_state_clone();
    state.discard_piles[0].push(get_card_by_enum(CardId::B4a001Volbeat));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4a002Illumise, 0),
        is_stack: false,
    });

    // Snorlax 150 HP - 90 = 60.
    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        60,
        "Ire-Fly should do 90 damage with Volbeat in the discard pile"
    );
}

#[test]
fn test_ire_fly_base_damage_without_volbeat_in_discard() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4a002Illumise)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    let mut state = game.get_state_clone();
    state.discard_piles[0].clear();
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4a002Illumise, 0),
        is_stack: false,
    });

    // Snorlax 150 HP - 30 = 120.
    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        120,
        "Ire-Fly should do only 30 damage without Volbeat in the discard pile"
    );
}
