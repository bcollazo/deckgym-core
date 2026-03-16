use std::panic::{catch_unwind, AssertUnwindSafe};

use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    effects::CardEffect,
    models::{PlayedCard, StatusCondition},
    state::GameOutcome,
};

mod common;

#[derive(Clone, Copy, Debug)]
enum RemovalSource {
    ActiveDelayedDamage,
    ActiveDelayedSpotDamage,
    BenchDelayedSpotDamage,
}

fn run_case(
    seed: u64,
    status: Option<StatusCondition>,
    source: RemovalSource,
) -> Result<deckgym::State, String> {
    let mut game = get_initialized_game(seed);
    let mut state = game.get_state_clone();
    state.in_play_pokemon = [[None, None, None, None], [None, None, None, None]];
    state.move_generation_stack.clear();
    state.winner = None;

    match source {
        RemovalSource::ActiveDelayedDamage => {
            let mut doomed = PlayedCard::from_id(CardId::A1001Bulbasaur);
            if let Some(status) = status {
                doomed = doomed.with_status(status);
            }
            doomed.add_effect(CardEffect::DelayedDamage { amount: 100 }, 1);

            state.set_board(
                vec![doomed],
                vec![PlayedCard::from_id(CardId::A1053Squirtle)],
            );
        }
        RemovalSource::ActiveDelayedSpotDamage => {
            let mut doomed = PlayedCard::from_id(CardId::A1001Bulbasaur);
            if let Some(status) = status {
                doomed = doomed.with_status(status);
            }

            state.set_board(
                vec![doomed],
                vec![PlayedCard::from_id(CardId::A1053Squirtle)],
            );
        }
        RemovalSource::BenchDelayedSpotDamage => {
            let mut doomed = PlayedCard::from_id(CardId::A1001Bulbasaur);
            if let Some(status) = status {
                doomed = doomed.with_status(status);
            }

            state.set_board(
                vec![PlayedCard::from_id(CardId::A1053Squirtle), doomed],
                vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
            );
        }
    }

    state.turn_count = 3;
    state.current_player = 0;
    game.set_state(state);

    match source {
        RemovalSource::ActiveDelayedSpotDamage => {
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
        RemovalSource::BenchDelayedSpotDamage => {
            game.apply_action(&Action {
                actor: 1,
                action: SimpleAction::ScheduleDelayedSpotDamage {
                    target_player: 0,
                    target_in_play_idx: 1,
                    amount: 100,
                },
                is_stack: false,
            });
        }
        RemovalSource::ActiveDelayedDamage => {}
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

#[test]
fn checkup_handles_removed_active_targets_after_delayed_damage() {
    for status in [
        StatusCondition::Asleep,
        StatusCondition::Paralyzed,
        StatusCondition::Poisoned,
        StatusCondition::Burned,
    ] {
        for seed in 0..16 {
            let state = run_case(seed, Some(status), RemovalSource::ActiveDelayedDamage)
                .unwrap_or_else(|msg| panic!("status={status:?} seed={seed} panicked: {msg}"));
            assert_eq!(
                state.winner,
                Some(GameOutcome::Win(1)),
                "delayed damage should still KO the active for status={status:?} seed={seed}"
            );
        }
    }
}

#[test]
fn checkup_handles_removed_active_targets_after_delayed_spot_damage() {
    for status in [
        StatusCondition::Asleep,
        StatusCondition::Paralyzed,
        StatusCondition::Poisoned,
        StatusCondition::Burned,
    ] {
        for seed in 0..16 {
            let state = run_case(seed, Some(status), RemovalSource::ActiveDelayedSpotDamage)
                .unwrap_or_else(|msg| panic!("status={status:?} seed={seed} panicked: {msg}"));
            assert_eq!(
                state.winner,
                Some(GameOutcome::Win(1)),
                "delayed spot damage should still KO the active for status={status:?} seed={seed}"
            );
        }
    }
}

#[test]
fn checkup_handles_removed_bench_targets_after_delayed_spot_damage() {
    for status in [
        StatusCondition::Asleep,
        StatusCondition::Paralyzed,
        StatusCondition::Poisoned,
        StatusCondition::Burned,
    ] {
        for seed in 0..16 {
            let state = run_case(seed, Some(status), RemovalSource::BenchDelayedSpotDamage)
                .unwrap_or_else(|msg| panic!("status={status:?} seed={seed} panicked: {msg}"));
            assert!(
                state.in_play_pokemon[0][0].is_some(),
                "active should remain after bench-delayed-spot-damage for status={status:?} seed={seed}"
            );
            assert!(
                state.in_play_pokemon[0][1].is_none(),
                "bench target should be removed for status={status:?} seed={seed}"
            );
        }
    }
}

#[test]
fn confusion_and_no_status_do_not_enter_checkup_bug_path() {
    for source in [
        RemovalSource::ActiveDelayedDamage,
        RemovalSource::ActiveDelayedSpotDamage,
        RemovalSource::BenchDelayedSpotDamage,
    ] {
        for status in [None, Some(StatusCondition::Confused)] {
            for seed in 0..8 {
                let result = run_case(seed, status, source);
                assert!(
                    result.is_ok(),
                    "source={source:?} status={status:?} seed={seed} unexpectedly panicked: {:?}",
                    result.err()
                );
            }
        }
    }
}
