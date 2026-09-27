use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

fn attack(game: &mut deckgym::Game, card_id: CardId) {
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(card_id, 0),
        is_stack: false,
    });
}

/// Mr. Mime - Synchro Dance: 40 damage, +40 when both Active Pokemon have the same amount of
/// Energy attached.
#[test]
fn test_synchro_dance_boosted_when_energy_counts_match() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4069MrMime)
            .with_energy(vec![EnergyType::Psychic, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)
            .with_energy(vec![EnergyType::Water, EnergyType::Water])],
    );
    attack(&mut game, CardId::B4069MrMime);
    // 150 - (40 + 40) = 70.
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 70);
}

#[test]
fn test_synchro_dance_base_when_energy_counts_differ() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4069MrMime)
            .with_energy(vec![EnergyType::Psychic, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax).with_energy(vec![EnergyType::Water])],
    );
    attack(&mut game, CardId::B4069MrMime);
    // 150 - 40 = 110.
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 110);
}

/// Kecleon - Samesies Slap: 20 damage, +30 when both Active Pokemon share an Energy type.
#[test]
fn test_samesies_slap_boosted_when_sharing_an_energy_type() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4136Kecleon).with_energy(vec![EnergyType::Water])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)
            .with_energy(vec![EnergyType::Water, EnergyType::Fire])],
    );
    attack(&mut game, CardId::B4136Kecleon);
    // 150 - (20 + 30) = 100.
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 100);
}

#[test]
fn test_samesies_slap_base_when_no_shared_energy_type() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4136Kecleon).with_energy(vec![EnergyType::Water])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax).with_energy(vec![EnergyType::Fire])],
    );
    attack(&mut game, CardId::B4136Kecleon);
    // 150 - 20 = 130.
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 130);
}

/// Scrafty - Crush the Weak: 50 damage, +50 when it has strictly more Energy than the defender.
#[test]
fn test_crush_the_weak_boosted_with_more_energy() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2107Scrafty)
            .with_energy(vec![EnergyType::Darkness, EnergyType::Darkness])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax).with_energy(vec![EnergyType::Water])],
    );
    attack(&mut game, CardId::B2107Scrafty);
    // 150 - (50 + 50) = 50.
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 50);
}

#[test]
fn test_crush_the_weak_base_with_equal_energy() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2107Scrafty)
            .with_energy(vec![EnergyType::Darkness, EnergyType::Darkness])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)
            .with_energy(vec![EnergyType::Water, EnergyType::Water])],
    );
    attack(&mut game, CardId::B2107Scrafty);
    // 150 - 50 = 100.
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 100);
}
