use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{Card, EnergyType, PlayedCard, TrainerType},
    test_support::{attack_action, get_initialized_game},
};

/// Team Rocket's Slowpoke's Scavenge: puts a random Item card from
/// the discard pile into the player's hand.
#[test]
fn test_slowpoke_scavenge_retrieves_item_from_discard() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B4a025TeamRocketsSlowpoke)
            .with_energy(vec![EnergyType::Psychic])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    // Put a Potion (item card) in the discard pile
    let potion = deckgym::database::get_card_by_enum(CardId::PA001Potion);
    state.discard_piles[0].push(potion);
    game.set_state(state);

    let hand_before = game.get_state_clone().hands[0].len();

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4a025TeamRocketsSlowpoke, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    // Hand should have grown by 1
    assert_eq!(state.hands[0].len(), hand_before + 1);
    // Discard pile should now be empty
    assert!(state.discard_piles[0].is_empty());
    // The retrieved card should be an Item
    assert!(state.hands[0].iter().any(|c| matches!(
        c,
        Card::Trainer(t) if t.trainer_card_type == TrainerType::Item
    )));
}

#[test]
fn test_slowpoke_scavenge_does_nothing_with_empty_discard() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B4a025TeamRocketsSlowpoke)
            .with_energy(vec![EnergyType::Psychic])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    // No items in discard pile
    state.discard_piles[0].clear();
    game.set_state(state);

    let hand_before = game.get_state_clone().hands[0].len();

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4a025TeamRocketsSlowpoke, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    // Hand should be unchanged
    assert_eq!(state.hands[0].len(), hand_before);
}