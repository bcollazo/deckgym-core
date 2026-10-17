use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

fn high_hp_target(card: CardId) -> PlayedCard {
    let card_data = get_card_by_enum(card);
    PlayedCard::new(card_data, 0, 300, vec![], false, vec![])
}

/// Prism Impact: 80 base + 20 for each unique Energy type attached.
/// With 3 energies of the same type (1 unique type), damage should be 80 + 20 = 100.
#[test]
fn test_terapagos_prism_impact_single_type() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B3a068TerapagosEx).with_energy(vec![
                EnergyType::Colorless,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
        ],
        vec![high_hp_target(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3a068TerapagosEx, 0),
        is_stack: false,
    });

    // 1 unique type -> 80 + 20*1 = 100 damage
    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 300 - 100);
}

/// With 2 different energy types attached, damage should be 80 + 20*2 = 120.
#[test]
fn test_terapagos_prism_impact_two_types() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B3a068TerapagosEx).with_energy(vec![
                EnergyType::Fire,
                EnergyType::Water,
                EnergyType::Fire,
            ]),
        ],
        vec![high_hp_target(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3a068TerapagosEx, 0),
        is_stack: false,
    });

    // 2 unique types -> 80 + 20*2 = 120 damage
    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 300 - 120);
}
