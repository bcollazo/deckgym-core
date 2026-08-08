use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_mountain_swing_discard_three_cards_own_deck() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A2082Rhyperior).with_energy(vec![
                EnergyType::Fighting,
                EnergyType::Fighting,
                EnergyType::Fighting,
                EnergyType::Colorless,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::A1003Venusaur)],
    );

    let top_card = get_card_by_enum(CardId::A1283Mew);
    for _ in 0..3 {
        state.decks[0].cards.insert(0, top_card.clone());
    }
    let own_deck_size_before = state.decks[0].cards.len();
    let expected_discard_size = 3;
    let venusaur_initial_health_points = 160;
    let expected_damage_dealt = 150;

    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A2082Rhyperior, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();

    // Mountain Swing: 150 damage delt
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        venusaur_initial_health_points - expected_damage_dealt
    );

    // Own deck lenght became 0
    assert_eq!(
        state.decks[0].cards.len(),
        own_deck_size_before - expected_discard_size
    );
    assert!(state.discard_piles[0].contains(&top_card));
    assert!(state.discard_piles[0].len() == expected_discard_size);
}
