use crate::deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

fn played_card_with_base_hp(card_id: CardId, base_hp: u32) -> PlayedCard {
    let card = get_card_by_enum(card_id);
    PlayedCard::new(card, 0, base_hp, vec![], false, vec![])
}

/// Test Xerneas B1a 037 - Geoburst
/// Damage should be reduced by the amount of damage Xerneas has
#[test]
fn test_xerneas_geoburst_full_hp() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![PlayedCard::from_id(CardId::B1a037Xerneas).with_energy(vec![
            EnergyType::Psychic,
            EnergyType::Psychic,
            EnergyType::Colorless,
        ])],
        vec![played_card_with_base_hp(CardId::A1001Bulbasaur, 150)],
    );

    game.set_state(state);

    let action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };

    game.apply_action(&action);
    let state = game.get_state_clone();

    // At full HP (120), Xerneas has 0 damage, so should deal full 120 damage
    let opponent_active = state.get_active(1);
    assert_eq!(
        opponent_active.get_remaining_hp(),
        30,
        "Opponent should have 30 HP remaining (150 - 120)"
    );
}

#[test]
fn test_xerneas_geoburst_damaged() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![PlayedCard::from_id(CardId::B1a037Xerneas)
            .with_damage(50)
            .with_energy(vec![
                EnergyType::Psychic,
                EnergyType::Psychic,
                EnergyType::Colorless,
            ])],
        vec![played_card_with_base_hp(CardId::A1001Bulbasaur, 100)],
    );

    game.set_state(state);

    let action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };

    game.apply_action(&action);
    let state = game.get_state_clone();

    // Xerneas has 50 damage, so attack should do 120 - 50 = 70 damage
    let opponent_active = state.get_active(1);
    assert_eq!(
        opponent_active.get_remaining_hp(),
        30,
        "Opponent should have 30 HP remaining (100 - 70)"
    );
}
