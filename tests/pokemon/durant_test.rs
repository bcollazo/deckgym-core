use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

#[test]
fn test_bite_together_extra_damage_with_durant_on_bench() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B1168Durant)
                .with_energy(vec![EnergyType::Metal, EnergyType::Colorless]),
            PlayedCard::from_id(CardId::B1168Durant), // Durant on Bench
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });
    game.play_until_stable();

    // Bite Together: 40 base + 40 extra (Durant on Bench) = 80
    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        150 - 80
    );
}

#[test]
fn test_bite_together_base_damage_without_durant_on_bench() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B1168Durant)
                .with_energy(vec![EnergyType::Metal, EnergyType::Colorless]),
            PlayedCard::from_id(CardId::A1001Bulbasaur), // Not Durant
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });
    game.play_until_stable();

    // Bite Together: 40 base only (no Durant on Bench)
    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        150 - 40
    );
}

#[test]
fn test_mountain_munch_discards_top_card_of_opponent_deck() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![PlayedCard::from_id(CardId::A4a007Durant)
            .with_energy(vec![EnergyType::Grass, EnergyType::Grass])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    let top_card = get_card_by_enum(CardId::A1001Bulbasaur);
    state.decks[1].cards.insert(0, top_card.clone());
    let deck_size_before = state.decks[1].cards.len();

    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();

    // Mountain Munch: 40 damage dealt
    assert_eq!(state.get_active(1).get_remaining_hp(), 150 - 40);

    // Top card of opponent's deck was discarded
    assert_eq!(state.decks[1].cards.len(), deck_size_before - 1);
    assert!(state.discard_piles[1].contains(&top_card));
}
