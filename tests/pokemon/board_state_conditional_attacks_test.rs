use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

fn filler_cards(count: usize) -> Vec<Card> {
    (0..count)
        .map(|_| get_card_by_enum(CardId::A1001Bulbasaur))
        .collect()
}

fn attack(game: &mut deckgym::Game, card_id: CardId) {
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(card_id, 0),
        is_stack: false,
    });
}

/// Ludicolo - Rhythmic Steps: 60 damage, +60 with exactly 1, 3 or 5 cards in hand.
#[test]
fn test_rhythmic_steps_boosted_with_odd_hand_size() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B1055Ludicolo)
            .with_energy(vec![EnergyType::Water, EnergyType::Water])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    let mut state = game.get_state_clone();
    state.hands[0] = filler_cards(3);
    game.set_state(state);

    attack(&mut game, CardId::B1055Ludicolo);
    // 150 - (60 + 60) = 30.
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 30);
}

#[test]
fn test_rhythmic_steps_base_with_even_hand_size() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B1055Ludicolo)
            .with_energy(vec![EnergyType::Water, EnergyType::Water])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    let mut state = game.get_state_clone();
    state.hands[0] = filler_cards(4);
    game.set_state(state);

    attack(&mut game, CardId::B1055Ludicolo);
    // 150 - 60 = 90.
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 90);
}

/// Chimecho - Extrasensory: 40 damage, +40 when both players hold the same number of cards.
#[test]
fn test_extrasensory_boosted_with_equal_hand_sizes() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4073Chimecho)
            .with_energy(vec![EnergyType::Psychic, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    let mut state = game.get_state_clone();
    state.hands[0] = filler_cards(3);
    state.hands[1] = filler_cards(3);
    game.set_state(state);

    attack(&mut game, CardId::B4073Chimecho);
    // 150 - (40 + 40) = 70.
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 70);
}

#[test]
fn test_extrasensory_base_with_different_hand_sizes() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4073Chimecho)
            .with_energy(vec![EnergyType::Psychic, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    let mut state = game.get_state_clone();
    state.hands[0] = filler_cards(3);
    state.hands[1] = filler_cards(5);
    game.set_state(state);

    attack(&mut game, CardId::B4073Chimecho);
    // 150 - 40 = 110.
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 110);
}

/// Tyrantrum - Tyrannical Fang: 100 damage, +80 with fewer Pokemon in play than the opponent.
#[test]
fn test_tyrannical_fang_boosted_when_outnumbered() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B2090Tyrantrum).with_energy(vec![
                EnergyType::Fighting,
                EnergyType::Fighting,
                EnergyType::Fighting,
            ]),
        ],
        vec![
            PlayedCard::from_id(CardId::A1211Snorlax),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );
    attack(&mut game, CardId::B2090Tyrantrum);
    // Snorlax is Weak to Fighting: (100 + 80) + 20 = 200 >= 150 HP, so it is knocked out.
    assert!(game.get_state_clone().maybe_get_active(1).is_none());
}

#[test]
fn test_tyrannical_fang_base_when_not_outnumbered() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B2090Tyrantrum).with_energy(vec![
                EnergyType::Fighting,
                EnergyType::Fighting,
                EnergyType::Fighting,
            ]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    attack(&mut game, CardId::B2090Tyrantrum);
    // Snorlax is Weak to Fighting: 150 - (100 + 20) = 30.
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 30);
}

/// Pheromosa - Prelude: 30 damage, +60 while the attacker has no points.
#[test]
fn test_prelude_boosted_with_no_points() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4016Pheromosa)
            .with_energy(vec![EnergyType::Grass, EnergyType::Grass])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    let mut state = game.get_state_clone();
    state.points = [0, 0];
    game.set_state(state);

    attack(&mut game, CardId::B4016Pheromosa);
    // 150 - (30 + 60) = 60.
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 60);
}

#[test]
fn test_prelude_base_once_a_point_was_taken() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4016Pheromosa)
            .with_energy(vec![EnergyType::Grass, EnergyType::Grass])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    let mut state = game.get_state_clone();
    state.points = [1, 0];
    game.set_state(state);

    attack(&mut game, CardId::B4016Pheromosa);
    // 150 - 30 = 120.
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 120);
}

/// Ting-Lu - Arrogant Impact: 130 damage, but nothing at all when Ting-Lu is at 60 HP or less.
#[test]
fn test_arrogant_impact_does_nothing_at_low_hp() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2a062TingLu)
            .with_energy(vec![EnergyType::Fighting; 3])
            .with_remaining_hp(60)],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    attack(&mut game, CardId::B2a062TingLu);
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 150);
}

#[test]
fn test_arrogant_impact_full_damage_above_threshold() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2a062TingLu)
            .with_energy(vec![EnergyType::Fighting; 3])
            .with_remaining_hp(70)],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    attack(&mut game, CardId::B2a062TingLu);
    // Snorlax is Weak to Fighting: 130 + 20 = 150, exactly a knockout.
    assert!(game.get_state_clone().maybe_get_active(1).is_none());
}

/// Flutter Mane - Hexing Flight: 90 damage, but nothing unless it moved from the Bench to the
/// Active Spot this turn.
#[test]
fn test_hexing_flight_does_nothing_without_moving_from_bench() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B3b035FlutterMane)
            .with_energy(vec![EnergyType::Psychic, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    attack(&mut game, CardId::B3b035FlutterMane);
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 150);
}

#[test]
fn test_hexing_flight_full_damage_after_moving_from_bench() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::B3b035FlutterMane)
                .with_energy(vec![EnergyType::Psychic, EnergyType::Colorless]),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: deckgym::actions::SimpleAction::Activate {
            player: 0,
            in_play_idx: 1,
        },
        is_stack: false,
    });
    attack(&mut game, CardId::B3b035FlutterMane);

    // 150 - 90 = 60.
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 60);
}
