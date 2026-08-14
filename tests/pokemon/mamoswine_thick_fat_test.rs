use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Mamoswine's Thick Fat: "This Pokémon takes -30 damage from attacks from [R] or [W] Pokémon."
#[test]
fn test_mamoswine_thick_fat_reduces_damage_from_fire_attacker() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1033Charmander).with_energy(vec![EnergyType::Fire])],
        vec![PlayedCard::from_id(CardId::A2160Mamoswine)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1033Charmander, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    // Ember does 30 damage; Thick Fat reduces damage from [R]/[W] attackers by 30.
    assert_eq!(state.get_active(1).get_remaining_hp(), 160);
}

#[test]
fn test_mamoswine_thick_fat_no_reduction_from_non_fire_water_attacker() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A2160Mamoswine)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1001Bulbasaur, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    // Vine Whip does 40 damage; Thick Fat doesn't apply to [G] attackers.
    assert_eq!(state.get_active(1).get_remaining_hp(), 120);
}
