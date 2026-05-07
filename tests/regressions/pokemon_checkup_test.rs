use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    effects::CardEffect,
    models::{PlayedCard, StatusCondition},
    state::GameOutcome,
    test_support::get_initialized_game,
};

#[derive(Clone, Copy, Debug)]
enum RemovalSource {
    MismagiusCursedProse,
    MeowscaradaFlowerTrickActive,
}

fn run_active_case(
    seed: u64,
    statuses: &[StatusCondition],
    source: RemovalSource,
) -> Result<crate::deckgym::State, String> {
    let mut game = get_initialized_game(seed);
    let mut state = game.get_state_clone();
    state.in_play_pokemon = [[None, None, None, None], [None, None, None, None]];
    state.move_generation_stack.clear();
    state.winner = None;

    match source {
        RemovalSource::MismagiusCursedProse => {
            let mut doomed = PlayedCard::from_id(CardId::A1001Bulbasaur);
            doomed.add_effect(CardEffect::DelayedDamage { amount: 100 }, 1);
            state.set_board(
                vec![doomed],
                vec![PlayedCard::from_id(CardId::A4a033Mismagius)],
            );
        }
        RemovalSource::MeowscaradaFlowerTrickActive => {
            state.set_board(
                vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
                vec![PlayedCard::from_id(CardId::B2a003MeowscaradaEx)],
            );
        }
    }
    for &status in statuses {
        state.apply_status_condition(0, 0, status);
    }

    state.turn_count = 3;
    state.current_player = 0;
    game.set_state(state);

    match source {
        RemovalSource::MeowscaradaFlowerTrickActive => {
            game.apply_action(&Action {
                actor: 1,
                action: SimpleAction::ScheduleDelayedSpotDamage {
                    target_player: 0,
                    target_in_play_idx: 0,
                    amount: 100,
                },
                is_stack: false,
            });
        }
        RemovalSource::MismagiusCursedProse => {}
    }

    let result = catch_unwind(AssertUnwindSafe(|| {
        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::EndTurn,
            is_stack: false,
        });
    }));

    match result {
        Ok(()) => Ok(game.get_state_clone()),
        Err(panic) => Err(panic
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| panic.downcast_ref::<&str>().map(|s| (*s).to_string()))
            .unwrap_or_else(|| "<non-string panic>".to_string())),
    }
}

fn run_meowscarada_bench_case(seed: u64) -> Result<crate::deckgym::State, String> {
    let mut game = get_initialized_game(seed);
    let mut state = game.get_state_clone();
    state.in_play_pokemon = [[None, None, None, None], [None, None, None, None]];
    state.move_generation_stack.clear();
    state.winner = None;

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1053Squirtle),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::B2a003MeowscaradaEx)],
    );
    state.turn_count = 3;
    state.current_player = 0;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::ScheduleDelayedSpotDamage {
            target_player: 0,
            target_in_play_idx: 1,
            amount: 100,
        },
        is_stack: false,
    });

    let result = catch_unwind(AssertUnwindSafe(|| {
        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::EndTurn,
            is_stack: false,
        });
    }));

    match result {
        Ok(()) => Ok(game.get_state_clone()),
        Err(panic) => Err(panic
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| panic.downcast_ref::<&str>().map(|s| (*s).to_string()))
            .unwrap_or_else(|| "<non-string panic>".to_string())),
    }
}

#[test]
fn mismagius_cursed_prose_does_not_panic_with_real_checkup_statuses() {
    let cases: &[(&str, &[StatusCondition])] = &[
        ("asleep", &[StatusCondition::Asleep]),
        ("paralyzed", &[StatusCondition::Paralyzed]),
        ("poisoned", &[StatusCondition::Poisoned]),
        ("burned", &[StatusCondition::Burned]),
        (
            "poisoned_burned",
            &[StatusCondition::Poisoned, StatusCondition::Burned],
        ),
        (
            "poisoned_asleep",
            &[StatusCondition::Poisoned, StatusCondition::Asleep],
        ),
        (
            "poisoned_paralyzed",
            &[StatusCondition::Poisoned, StatusCondition::Paralyzed],
        ),
    ];

    for (label, statuses) in cases {
        for seed in 0..16 {
            let state = run_active_case(seed, statuses, RemovalSource::MismagiusCursedProse)
                .unwrap_or_else(|msg| panic!("case={label} seed={seed} panicked: {msg}"));
            assert_eq!(
                state.winner,
                Some(GameOutcome::Win(1)),
                "Cursed Prose should still KO the active for case={label} seed={seed}"
            );
        }
    }
}

#[test]
fn meowscarada_ex_flower_trick_does_not_panic_with_real_checkup_statuses() {
    let cases: &[(&str, &[StatusCondition])] = &[
        ("asleep", &[StatusCondition::Asleep]),
        ("paralyzed", &[StatusCondition::Paralyzed]),
        ("poisoned", &[StatusCondition::Poisoned]),
        ("burned", &[StatusCondition::Burned]),
        (
            "poisoned_burned",
            &[StatusCondition::Poisoned, StatusCondition::Burned],
        ),
        (
            "poisoned_asleep",
            &[StatusCondition::Poisoned, StatusCondition::Asleep],
        ),
        (
            "poisoned_paralyzed",
            &[StatusCondition::Poisoned, StatusCondition::Paralyzed],
        ),
    ];

    for (label, statuses) in cases {
        for seed in 0..16 {
            let state =
                run_active_case(seed, statuses, RemovalSource::MeowscaradaFlowerTrickActive)
                    .unwrap_or_else(|msg| panic!("case={label} seed={seed} panicked: {msg}"));
            assert_eq!(
                state.winner,
                Some(GameOutcome::Win(1)),
                "Flower Trick should still KO the active for case={label} seed={seed}"
            );
        }
    }
}

#[test]
fn meowscarada_ex_flower_trick_still_hits_bench_targets_in_legal_states() {
    for seed in 0..16 {
        let state = run_meowscarada_bench_case(seed)
            .unwrap_or_else(|msg| panic!("seed={seed} panicked: {msg}"));
        assert!(
            state.in_play_pokemon[0][0].is_some(),
            "active should remain"
        );
        assert!(
            state.in_play_pokemon[0][1].is_none(),
            "Flower Trick should still remove the benched target"
        );
    }
}

#[test]
fn confusion_and_no_status_do_not_enter_checkup_bug_path() {
    for source in [
        RemovalSource::MismagiusCursedProse,
        RemovalSource::MeowscaradaFlowerTrickActive,
    ] {
        for status in [None, Some(StatusCondition::Confused)] {
            for seed in 0..8 {
                let statuses: &[StatusCondition] = match status {
                    Some(StatusCondition::Confused) => &[StatusCondition::Confused],
                    None => &[],
                    _ => unreachable!(),
                };
                let result = run_active_case(seed, statuses, source);
                assert!(
                    result.is_ok(),
                    "source={source:?} status={status:?} seed={seed} unexpectedly panicked: {:?}",
                    result.err()
                );
            }
        }
    }
}
