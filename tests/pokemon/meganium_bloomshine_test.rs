use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Meganium's "Bloomshine": 80 damage, then "Heal 20 damage from each of your Pokémon."
#[test]
fn test_bloomshine_damages_opponent_and_heals_all_your_pokemon() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A4010Meganium)
                .with_energy(vec![
                    EnergyType::Grass,
                    EnergyType::Grass,
                    EnergyType::Colorless,
                ])
                .with_damage(50),
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_damage(30),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
        vec![PlayedCard::from_id(CardId::A1004VenusaurEx)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A4010Meganium, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();

    // Venusaur ex (190 HP) takes the 80 damage.
    assert_eq!(state.get_remaining_hp(1, 0), 110);

    // Every one of the attacker's Pokemon heals 20, and healing never exceeds full HP.
    assert_eq!(state.get_remaining_hp(0, 0), 150 - 50 + 20);
    assert_eq!(state.get_remaining_hp(0, 1), 70 - 30 + 20);
    assert_eq!(state.get_remaining_hp(0, 2), 60);
}
