use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::get_test_game_with_board,
};

/// Porygon2's Buggy Evolution: "Whenever you attach an Energy from your Energy Zone to this
/// Pokémon, put a random card from your deck that evolves from this Pokémon onto this Pokémon
/// to evolve it."
#[test]
fn test_porygon2_buggy_evolution_evolves_on_zone_energy_attach() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A4136Porygon2).with_energy(vec![EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let mut state = game.get_state_clone();
    state.decks[0].cards = vec![get_card_by_enum(CardId::A2129PorygonZ).clone()];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attach {
            attachments: vec![(1, EnergyType::Colorless, 0)],
            is_turn_energy: false,
        },
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.get_active(0).get_name(), "Porygon-Z");
    // Energy carries over from Porygon2 plus the newly attached one.
    assert_eq!(state.get_active(0).attached_energy.len(), 2);
    assert!(state.decks[0].cards.is_empty());
}

#[test]
fn test_porygon2_buggy_evolution_stays_porygon2_without_evolution_card_in_deck() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A4136Porygon2).with_energy(vec![EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let mut state = game.get_state_clone();
    state.decks[0].cards = vec![get_card_by_enum(CardId::A1033Charmander).clone()];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attach {
            attachments: vec![(1, EnergyType::Colorless, 0)],
            is_turn_energy: false,
        },
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.get_active(0).get_name(), "Porygon2");
    assert_eq!(state.get_active(0).attached_energy.len(), 2);
}
