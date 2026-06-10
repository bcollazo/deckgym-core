use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

#[test]
fn test_smoochum_shivery_wave_damage_scales_with_opponent_energy() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Setup: Player 0 Active = Smoochum. Player 1 Active = Bulbasaur (70 HP) with 3 energies.
    state.set_board(
        vec![PlayedCard::from_id(CardId::A4075Smoochum)],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_energy(vec![
                EnergyType::Grass,
                EnergyType::Grass,
                EnergyType::Grass,
            ]),
        ],
    );
    state.current_player = 0;
    state.turn_count = 3;
    game.set_state(state);

    // Act: Apply Shivery Wave (index 0)
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    // Assert: 3 energies * 20 damage = 60 damage.
    // Bulbasaur starts with 70 HP. 70 - 60 = 10 HP remaining.
    let final_state = game.get_state_clone();
    let opponent_hp = final_state.get_active(1).get_remaining_hp();
    assert_eq!(
        opponent_hp, 10,
        "Shivery Wave should deal 60 damage for 3 energies"
    );
}

#[test]
fn test_smoochum_shivery_wave_zero_damage_with_no_opponent_energy() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Setup: Player 1 active has NO energy.
    state.set_board(
        vec![PlayedCard::from_id(CardId::A4075Smoochum)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    // Assert: 0 energies * 20 damage = 0 damage.
    let final_state = game.get_state_clone();
    let opponent_hp = final_state.get_active(1).get_remaining_hp();
    assert_eq!(
        opponent_hp, 70,
        "Shivery Wave should deal 0 damage when opponent has no energy"
    );
}
