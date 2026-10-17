use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard, StatusCondition},
    test_support::{attack_action, get_initialized_game},
};

/// Breloom's Pre-Dawn Strike deals 30 damage normally, or 30 + 60 = 90 damage
/// if the opponent's Active Pokémon is Asleep.
#[test]
fn test_pre_dawn_strike_extra_damage_when_opponent_asleep() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Player 0: Breloom (Grass) with 1 Grass energy (enough for Pre-Dawn Strike).
    // Player 1: Snorlax (150 HP, weak to Fighting - no weakness bonus vs Grass).
    state.set_board(
        vec![PlayedCard::from_id(CardId::B3012Breloom).with_energy(vec![EnergyType::Grass])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    state.current_player = 0;
    state.apply_status_condition(1, 0, StatusCondition::Asleep);
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3012Breloom, 0),
        is_stack: false,
    });

    // Snorlax 150 HP - 90 (30 base + 60 asleep bonus) = 60.
    let hp = game.get_state_clone().get_active(1).get_remaining_hp();
    assert_eq!(
        hp, 60,
        "Pre-Dawn Strike should deal 90 damage to an Asleep opponent"
    );
}

/// When the opponent's Active Pokémon is not Asleep, Pre-Dawn Strike deals only
/// its base 30 damage.
#[test]
fn test_pre_dawn_strike_base_damage_when_opponent_not_asleep() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::B3012Breloom).with_energy(vec![EnergyType::Grass])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    state.current_player = 0;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3012Breloom, 0),
        is_stack: false,
    });

    // Snorlax 150 HP - 30 (base damage only) = 120.
    let hp = game.get_state_clone().get_active(1).get_remaining_hp();
    assert_eq!(
        hp, 120,
        "Pre-Dawn Strike should deal only base 30 damage when opponent is not Asleep"
    );
}
