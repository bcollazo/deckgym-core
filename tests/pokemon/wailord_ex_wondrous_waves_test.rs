use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard, StatusCondition},
    test_support::{attack_action, get_test_game_with_board},
};

const VENUSAUR_EX_MAX_HP: u32 = 190;

fn wailord_ex_board() -> deckgym::Game<'static> {
    get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B4037WailordEx).with_energy(vec![
                EnergyType::Water,
                EnergyType::Water,
                EnergyType::Water,
                EnergyType::Water,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::A1004VenusaurEx)],
    )
}

/// Wailord ex's Wondrous Waves does 100 damage and clears every Special Condition
/// affecting Wailord ex itself.
#[test]
fn test_wailord_ex_wondrous_waves_cures_own_special_conditions() {
    let mut game = wailord_ex_board();

    let mut state = game.get_state_clone();
    state.apply_status_condition(0, 0, StatusCondition::Poisoned);
    state.apply_status_condition(0, 0, StatusCondition::Burned);
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4037WailordEx, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let wailord = state.get_active(0);
    assert!(!wailord.is_poisoned());
    assert!(!wailord.is_burned());
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        VENUSAUR_EX_MAX_HP - 100
    );
}

/// Wondrous Waves only heals Wailord ex; the Defending Pokémon keeps its Special Conditions.
#[test]
fn test_wailord_ex_wondrous_waves_leaves_opponent_conditions_alone() {
    let mut game = wailord_ex_board();

    let mut state = game.get_state_clone();
    state.apply_status_condition(0, 0, StatusCondition::Poisoned);
    state.apply_status_condition(1, 0, StatusCondition::Poisoned);
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4037WailordEx, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert!(!state.get_active(0).is_poisoned());
    assert!(state.get_active(1).is_poisoned());
}
