use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

fn dudunsparce_game() -> deckgym::Game<'static> {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B3a059Dunsparce)
            .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless])],
        vec![
            PlayedCard::from_id(CardId::B4197WailordEx).with_energy(vec![
                EnergyType::Water,
                EnergyType::Water,
                EnergyType::Water,
            ]),
        ],
    );
    let mut state = game.get_state_clone();
    state.hands[0].clear();
    state.hands[0].push(get_card_by_enum(CardId::B3a060Dudunsparce));
    game.set_state(state);
    game
}

/// Dudunsparce - Sudden Drilling: "If this Pokémon evolved from Dunsparce during this turn,
/// discard 2 random Energy from your opponent's Active Pokémon."
#[test]
fn test_sudden_drilling_discards_two_energy_when_it_evolved_this_turn() {
    let mut game = dudunsparce_game();

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Evolve {
            evolution: get_card_by_enum(CardId::B3a060Dudunsparce),
            in_play_idx: 0,
            from_deck: false,
        },
        is_stack: false,
    });
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3a060Dudunsparce, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 190);
    assert_eq!(
        state.get_active(1).attached_energy.len(),
        1,
        "2 of the 3 attached Energy should have been discarded"
    );
}

#[test]
fn test_sudden_drilling_keeps_energy_when_it_did_not_evolve_this_turn() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B3a060Dudunsparce)
            .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless])],
        vec![
            PlayedCard::from_id(CardId::B4197WailordEx).with_energy(vec![
                EnergyType::Water,
                EnergyType::Water,
                EnergyType::Water,
            ]),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3a060Dudunsparce, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 190);
    assert_eq!(state.get_active(1).attached_energy.len(), 3);
}
