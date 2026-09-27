use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Ampharos's Zapping Bullet does 90 to the Defending Pokémon and 20 to a randomly chosen
/// Benched Pokémon. With a single Benched Pokémon the random pick is forced.
#[test]
fn test_zapping_bullet_hits_the_only_benched_pokemon() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B1084Ampharos).with_energy(vec![
            EnergyType::Lightning,
            EnergyType::Lightning,
            EnergyType::Colorless,
        ])],
        vec![
            PlayedCard::from_id(CardId::A1211Snorlax),
            PlayedCard::from_id(CardId::A1129MewtwoEx),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B1084Ampharos, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 150 - 90);
    assert_eq!(
        state.in_play_pokemon[1][1]
            .as_ref()
            .expect("Benched Mewtwo ex should still be in play")
            .get_remaining_hp(),
        150 - 20
    );
}

/// With an empty Bench there is nothing to spread to, so only the Active takes damage.
#[test]
fn test_zapping_bullet_without_bench_only_hits_active() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B1084Ampharos).with_energy(vec![
            EnergyType::Lightning,
            EnergyType::Lightning,
            EnergyType::Colorless,
        ])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B1084Ampharos, 0),
        is_stack: false,
    });

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        150 - 90
    );
}
