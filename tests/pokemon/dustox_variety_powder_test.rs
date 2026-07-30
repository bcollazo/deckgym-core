use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{PlayedCard, StatusCondition},
    test_support::get_initialized_game_with_board,
    Game,
};

/// Dustox (B4 005) – Variety Powder: "Once during your turn, you may use this Ability. 1 Special
/// Condition from among Burned, Confused, and Poisoned is chosen at random, and your opponent's
/// Active Pokémon is now affected by that Special Condition. Any Special Conditions already
/// affecting that Pokémon will not be chosen."
fn game_with_dustox(seed: u64, opponent_conditions: &[StatusCondition]) -> Game<'static> {
    let mut game = get_initialized_game_with_board(
        seed,
        0,
        3,
        vec![PlayedCard::from_id(CardId::B4005Dustox)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let mut state = game.get_state_clone();
    for condition in opponent_conditions {
        state.apply_status_condition(1, 0, *condition);
    }
    game.set_state(state);

    game
}

fn use_variety_powder(seed: u64, opponent_conditions: &[StatusCondition]) -> [bool; 3] {
    let mut game = game_with_dustox(seed, opponent_conditions);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    });

    let state = game.get_state_clone();
    let opponent_active = state.get_active(1);
    [
        opponent_active.is_burned(),
        opponent_active.is_confused(),
        opponent_active.is_poisoned(),
    ]
}

#[test]
fn test_variety_powder_inflicts_each_of_the_three_conditions_at_random() {
    let mut seen = [false, false, false];

    for seed in 0..40u64 {
        let inflicted = use_variety_powder(seed, &[]);
        assert_eq!(
            inflicted.iter().filter(|x| **x).count(),
            1,
            "seed {seed}: Variety Powder should inflict exactly 1 Special Condition"
        );
        for (seen, inflicted) in seen.iter_mut().zip(inflicted) {
            *seen |= inflicted;
        }
    }

    assert_eq!(
        seen,
        [true, true, true],
        "expected Burned, Confused and Poisoned to each be chosen on at least one seed"
    );
}

#[test]
fn test_variety_powder_does_not_choose_already_applied_conditions() {
    for seed in 0..20u64 {
        let inflicted =
            use_variety_powder(seed, &[StatusCondition::Burned, StatusCondition::Poisoned]);

        assert!(
            inflicted[1],
            "seed {seed}: Confused is the only remaining option, so it must be chosen"
        );
    }
}

#[test]
fn test_variety_powder_unavailable_when_all_conditions_already_applied() {
    let game = game_with_dustox(
        0,
        &[
            StatusCondition::Burned,
            StatusCondition::Confused,
            StatusCondition::Poisoned,
        ],
    );

    let (_, choices) = game.get_state_clone().generate_possible_actions();
    assert!(
        !choices
            .iter()
            .any(|c| matches!(c.action, SimpleAction::UseAbility { .. })),
        "Variety Powder should not be usable when no Special Condition can be chosen"
    );
}

#[test]
fn test_variety_powder_is_once_per_turn() {
    let mut game = game_with_dustox(0, &[]);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    });

    let (_, choices) = game.get_state_clone().generate_possible_actions();
    assert!(
        !choices
            .iter()
            .any(|c| matches!(c.action, SimpleAction::UseAbility { .. })),
        "Variety Powder should only be usable once per turn"
    );
}
