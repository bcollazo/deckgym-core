use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Silcoon's & Cascoon's Cocoon Collector: "Put 3 random cards from among Silcoon and
/// Cascoon from your deck onto your Bench."
#[test]
fn test_cocoon_collector_puts_silcoon_and_cascoon_from_deck_onto_bench() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4002Silcoon).with_energy(vec![EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );

    let mut state = game.get_state_clone();
    // 3 eligible cards (Silcoon/Cascoon) plus a non-eligible one, so the outcome is deterministic.
    state.decks[0].cards = vec![
        get_card_by_enum(CardId::B4004Cascoon),
        get_card_by_enum(CardId::A1033Charmander),
        get_card_by_enum(CardId::B1004Silcoon),
        get_card_by_enum(CardId::B1006Cascoon),
    ];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4002Silcoon, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    for bench_idx in 1..4 {
        let benched = state.in_play_pokemon[0][bench_idx]
            .as_ref()
            .expect("Cocoon Collector should fill all 3 Bench slots");
        assert!(
            matches!(benched.get_name().as_str(), "Silcoon" | "Cascoon"),
            "Only Silcoon and Cascoon should be put onto the Bench, got {}",
            benched.get_name()
        );
    }
    assert_eq!(
        state.decks[0].cards.len(),
        1,
        "Only the non-eligible Charmander should remain in the deck"
    );
}

#[test]
fn test_cocoon_collector_is_limited_by_available_bench_space() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B4004Cascoon).with_energy(vec![EnergyType::Colorless]),
            PlayedCard::from_id(CardId::A1033Charmander),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );

    let mut state = game.get_state_clone();
    state.decks[0].cards = vec![
        get_card_by_enum(CardId::B4002Silcoon),
        get_card_by_enum(CardId::B1006Cascoon),
        get_card_by_enum(CardId::A1033Charmander),
    ];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4004Cascoon, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let benched = state.in_play_pokemon[0][3]
        .as_ref()
        .expect("The single open Bench slot should be filled");
    assert!(matches!(benched.get_name().as_str(), "Silcoon" | "Cascoon"));
    assert_eq!(
        state.decks[0].cards.len(),
        2,
        "Only 1 card should leave the deck when there is a single open Bench slot"
    );
}
