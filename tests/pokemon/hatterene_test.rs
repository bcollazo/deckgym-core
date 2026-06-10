use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard, StatusCondition},
    test_support::get_initialized_game,
};

/// Hatterene's Mental Crush deals 70 damage normally, or 70 + 70 = 140 damage
/// if the opponent's Active Pokémon is Confused.
#[test]
fn test_mental_crush_extra_damage_when_opponent_confused() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Player 0: Hatterene (Psychic) with 2 Psychic energy (enough for Mental Crush).
    // Player 1: Snorlax (150 HP, weak to Fighting - no weakness bonus vs Psychic).
    state.set_board(
        vec![PlayedCard::from_id(CardId::B3071Hatterene)
            .with_energy(vec![EnergyType::Psychic, EnergyType::Psychic])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    state.current_player = 0;
    state.apply_status_condition(1, 0, StatusCondition::Confused);
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    // Snorlax 150 HP - 140 (70 base + 70 confused bonus) = 10.
    let hp = game.get_state_clone().get_active(1).get_remaining_hp();
    assert_eq!(
        hp, 10,
        "Mental Crush should deal 140 damage to a Confused opponent"
    );
}

/// When the opponent's Active Pokémon is not Confused, Mental Crush deals only
/// its base 70 damage.
#[test]
fn test_mental_crush_base_damage_when_opponent_not_confused() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::B3071Hatterene)
            .with_energy(vec![EnergyType::Psychic, EnergyType::Psychic])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    state.current_player = 0;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    // Snorlax 150 HP - 70 (base damage only) = 80.
    let hp = game.get_state_clone().get_active(1).get_remaining_hp();
    assert_eq!(
        hp, 80,
        "Mental Crush should deal only base 70 damage when opponent is not Confused"
    );
}
