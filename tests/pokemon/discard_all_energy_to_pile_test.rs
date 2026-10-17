use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Test Hydreigon B1 157 - Hyper Ray: "Discard all Energy from this Pokémon." The discarded
/// Energy goes to the discard pile.
#[test]
fn test_hydreigon_hyper_ray_puts_discarded_energy_in_discard_pile() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B1157Hydreigon).with_energy(vec![
                EnergyType::Darkness,
                EnergyType::Darkness,
                EnergyType::Darkness,
                EnergyType::Colorless,
            ]),
        ],
        vec![
            PlayedCard::from_id(CardId::B4197WailordEx),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B1157Hydreigon, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert!(state.get_active(0).attached_energy.is_empty());
    let mut discarded = state.discard_energies[0].clone();
    discarded.sort_by_key(|e| *e as u8);
    let mut expected = vec![
        EnergyType::Darkness,
        EnergyType::Darkness,
        EnergyType::Darkness,
        EnergyType::Colorless,
    ];
    expected.sort_by_key(|e| *e as u8);
    assert_eq!(discarded, expected);
}

/// Test Raging Bolt B3a 055 - Baneful Boom: "Discard all Energy from this Pokémon. Knock Out
/// your opponent's Active Pokémon." The discarded Energy goes to the discard pile.
#[test]
fn test_raging_bolt_baneful_boom_puts_discarded_energy_in_discard_pile() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B3a055RagingBolt).with_energy(vec![
                EnergyType::Lightning,
                EnergyType::Lightning,
                EnergyType::Water,
            ]),
        ],
        vec![
            PlayedCard::from_id(CardId::B4197WailordEx),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3a055RagingBolt, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert!(state.get_active(0).attached_energy.is_empty());
    assert_eq!(state.discard_energies[0].len(), 3);
}
