use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

fn played_card_with_base_hp(card_id: CardId, base_hp: u32) -> PlayedCard {
    let card = deckgym::database::get_card_by_enum(card_id);
    PlayedCard::new(card, 0, base_hp, vec![], false, vec![])
}

/// Test Lucario's Fighting Coach ability gives +20 damage to Fighting attacks
#[test]
fn test_lucario_fighting_coach_single() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up Riolu active + Lucario on bench vs high-HP opponent
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A2091Riolu).with_energy(vec![EnergyType::Fighting]),
            PlayedCard::from_id(CardId::A2092Lucario),
        ],
        vec![played_card_with_base_hp(CardId::A1001Bulbasaur, 100)],
    );
    state.current_player = 0;

    game.set_state(state);

    // Apply Riolu's Jab attack (20 base damage + 20 from Fighting Coach = 40)
    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // With 1 Fighting Coach: 20 + 20 = 40 damage, so 100 - 40 = 60 HP
    let opponent_hp = final_state.get_active(1).get_remaining_hp();

    assert_eq!(
        opponent_hp, 60,
        "Riolu's attack should deal 40 damage with 1 Fighting Coach boost (20 + 20)"
    );
}

/// Test two Lucarios stack Fighting Coach (+40 total damage)
#[test]
fn test_lucario_fighting_coach_stacked() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up active Lucario + TWO bench Lucarios vs high-HP opponent
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A2092Lucario)
                .with_energy(vec![EnergyType::Fighting, EnergyType::Fighting]),
            PlayedCard::from_id(CardId::A2092Lucario),
            PlayedCard::from_id(CardId::A2092Lucario),
        ],
        vec![played_card_with_base_hp(CardId::A1001Bulbasaur, 150)],
    );
    state.current_player = 0;

    game.set_state(state);

    // Apply attack: 40 base + 20 (active Lucario) + 20 (bench1) + 20 (bench2) = 100
    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // With 3 Lucarios: 40 + (20 * 3) = 100 damage, so 150 - 100 = 50 HP
    let opponent_hp = final_state.get_active(1).get_remaining_hp();

    assert_eq!(
        opponent_hp, 50,
        "Lucario's attack should deal 100 damage with 3 Fighting Coaches (40 + 60)"
    );
}

/// Test Fighting Coach doesn't boost non-Fighting type attacks
#[test]
fn test_lucario_fighting_coach_no_boost_non_fighting() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up Bulbasaur (Grass type) active + Lucario bench vs high-HP opponent
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_energy(vec![EnergyType::Grass, EnergyType::Colorless]),
            PlayedCard::from_id(CardId::A2092Lucario),
        ],
        vec![played_card_with_base_hp(CardId::A1053Squirtle, 100)],
    );
    state.current_player = 0;

    game.set_state(state);

    // Apply Vine Whip attack (40 damage, should NOT get Fighting Coach boost)
    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // No boost: 40 damage, so 100 - 40 = 60 HP
    let opponent_hp = final_state.get_active(1).get_remaining_hp();

    assert_eq!(
        opponent_hp, 60,
        "Grass-type attack should NOT get Fighting Coach boost (40 damage only)"
    );
}
