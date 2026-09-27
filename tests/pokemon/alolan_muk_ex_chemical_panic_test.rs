use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard, StatusCondition},
    test_support::{attack_action, get_test_game_with_board},
};

fn chemical_panic_game(preexisting: &[StatusCondition]) -> deckgym::Game<'static> {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A3111AlolanMukEx).with_energy(vec![
                EnergyType::Darkness,
                EnergyType::Darkness,
                EnergyType::Colorless,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    let mut state = game.get_state_clone();
    for condition in preexisting {
        state.apply_status_condition(1, 0, *condition);
    }
    game.set_state(state);
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A3111AlolanMukEx, 0),
        is_stack: false,
    });
    game
}

/// Chemical Panic does 80 damage and inflicts one of the five Special Conditions at random.
#[test]
fn test_chemical_panic_inflicts_some_special_condition() {
    let game = chemical_panic_game(&[]);

    let state = game.get_state_clone();
    let defender = state.get_active(1);
    assert!(
        defender.is_asleep()
            || defender.is_burned()
            || defender.is_confused()
            || defender.is_paralyzed()
            || defender.is_poisoned(),
        "Chemical Panic should apply one Special Condition"
    );
    assert_eq!(defender.get_remaining_hp(), 150 - 80);
}

/// Conditions already affecting the Defending Pokémon are never chosen, so with four of the five
/// already applied the fifth is forced.
#[test]
fn test_chemical_panic_skips_conditions_already_applied() {
    let game = chemical_panic_game(&[
        StatusCondition::Asleep,
        StatusCondition::Burned,
        StatusCondition::Confused,
        StatusCondition::Paralyzed,
    ]);

    let state = game.get_state_clone();
    assert!(
        state.get_active(1).is_poisoned(),
        "Poisoned is the only condition left to choose"
    );
}
