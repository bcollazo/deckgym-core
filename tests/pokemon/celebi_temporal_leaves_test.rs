use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// An Ivysaur that was evolved from a Bulbasaur, so it has an evolution card underneath it.
fn evolved_ivysaur() -> PlayedCard {
    PlayedCard::new(
        get_card_by_enum(CardId::A1002Ivysaur),
        0,
        90,
        vec![],
        false,
        vec![get_card_by_enum(CardId::A1001Bulbasaur)],
    )
}

/// Celebi's Temporal Leaves does 40 damage and then devolves the Defending Pokémon, returning
/// the Evolution card to the opponent's hand. Damage counters stay on the Pokémon underneath.
#[test]
fn test_temporal_leaves_devolves_opponent_active() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A4a006Celebi)
            .with_energy(vec![EnergyType::Grass, EnergyType::Grass])],
        vec![evolved_ivysaur()],
    );
    let hand_before = game.get_state_clone().hands[1].len();

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A4a006Celebi, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let active = state.get_active(1);
    assert_eq!(active.get_name(), "Bulbasaur");
    // Bulbasaur has 70 HP and keeps the 40 damage Temporal Leaves dealt to Ivysaur.
    assert_eq!(active.get_remaining_hp(), 30);
    assert!(active.cards_behind.is_empty());
    assert_eq!(state.hands[1].len(), hand_before + 1);
    assert!(state.hands[1]
        .iter()
        .any(|card| card.get_name() == "Ivysaur"));
}

/// A Basic Defending Pokémon is not devolved; the attack just does its damage.
#[test]
fn test_temporal_leaves_does_not_devolve_a_basic_pokemon() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A4a006Celebi)
            .with_energy(vec![EnergyType::Grass, EnergyType::Grass])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    let hand_before = game.get_state_clone().hands[1].len();

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A4a006Celebi, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_name(), "Snorlax");
    assert_eq!(state.get_active(1).get_remaining_hp(), 150 - 40);
    assert_eq!(state.hands[1].len(), hand_before);
}
