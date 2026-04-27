use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

fn played_card_with_base_hp(card_id: CardId, base_hp: u32) -> PlayedCard {
    PlayedCard::new(get_card_by_enum(card_id), 0, base_hp, vec![], false, vec![])
}

/// Gallade ex's Energized Blade does 70 base + 20 per energy on the opponent's Active Pokemon.
#[test]
fn test_gallade_ex_energized_blade_scales_with_opponent_energy() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A2095GalladeEx)
            .with_energy(vec![EnergyType::Fighting, EnergyType::Fighting])],
        vec![
            played_card_with_base_hp(CardId::A1001Bulbasaur, 200).with_energy(vec![
                EnergyType::Grass,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
        ],
    );
    state.current_player = 0;
    state.turn_count = 1;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });
    game.play_until_stable();

    // 70 base + 20 * 3 energy = 130 damage => 200 - 130 = 70
    let opponent_hp = game.get_state_clone().get_active(1).get_remaining_hp();
    assert_eq!(
        opponent_hp, 70,
        "Energized Blade should deal 130 damage (70 + 20 * 3 energy), leaving 70 HP"
    );
}

/// Gallade ex's Energized Blade deals only base 70 when the opponent has no energy.
#[test]
fn test_gallade_ex_energized_blade_no_bonus_without_opponent_energy() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A2095GalladeEx)
            .with_energy(vec![EnergyType::Fighting, EnergyType::Fighting])],
        vec![played_card_with_base_hp(CardId::A1001Bulbasaur, 150)],
    );
    state.current_player = 0;
    state.turn_count = 1;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });
    game.play_until_stable();

    // 70 base + 20 * 0 = 70 damage => 150 - 70 = 80
    let opponent_hp = game.get_state_clone().get_active(1).get_remaining_hp();
    assert_eq!(
        opponent_hp, 80,
        "Energized Blade should deal only 70 base damage when opponent has no energy"
    );
}

/// Gallade's Earthen Sword does 70 + 70 bonus damage when a Stadium is in play.
#[test]
fn test_gallade_earthen_sword_bonus_damage_with_stadium() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::B3084Gallade).with_energy(vec![
            EnergyType::Fighting,
            EnergyType::Colorless,
            EnergyType::Colorless,
        ])],
        vec![played_card_with_base_hp(CardId::A1001Bulbasaur, 200)],
    );
    state.current_player = 0;
    state.turn_count = 1;
    state.active_stadium = Some(get_card_by_enum(CardId::B2155PeculiarPlaza));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });
    game.play_until_stable();

    // 70 base + 70 bonus = 140 damage => 200 - 140 = 60
    let opponent_hp = game.get_state_clone().get_active(1).get_remaining_hp();
    assert_eq!(
        opponent_hp, 60,
        "Earthen Sword should deal 140 damage (70 + 70 bonus) when a Stadium is in play"
    );
}

/// Gallade's Earthen Sword deals only base 70 when no Stadium is in play.
#[test]
fn test_gallade_earthen_sword_base_damage_without_stadium() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::B3084Gallade).with_energy(vec![
            EnergyType::Fighting,
            EnergyType::Colorless,
            EnergyType::Colorless,
        ])],
        vec![played_card_with_base_hp(CardId::A1001Bulbasaur, 150)],
    );
    state.current_player = 0;
    state.turn_count = 1;
    state.active_stadium = None;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });
    game.play_until_stable();

    // 70 base + 0 = 70 damage => 150 - 70 = 80
    let opponent_hp = game.get_state_clone().get_active(1).get_remaining_hp();
    assert_eq!(
        opponent_hp, 80,
        "Earthen Sword should deal only 70 base damage when no Stadium is in play"
    );
}
