use super::*;
use deckgym::{
    models::StatusCondition,
    test_support::{play_trainer, trainer_from_id},
};

fn lullaby_game(id: CardId, seed: u64) -> Game<'static> {
    let mut game = get_initialized_game_with_board(seed, 0, 3, vec![ready(id, 0)], vec![durable()]);
    attack(&mut game, id, 0);
    game
}

// The Sleep itself is not a coin flip: the defender is always Asleep afterwards, with 2 coins to
// flip at each Checkup.
#[test]
fn soothing_lullaby_always_puts_the_defender_to_sleep() {
    for id in [CardId::C1051Jigglypuff, CardId::C1184Jigglypuff] {
        for seed in 0..20 {
            let game = lullaby_game(id, seed);
            assert_eq!(defender_damage(&game), 30, "{id:?}");
            let state = game.get_state_clone();
            assert!(state.get_active(1).is_asleep(), "{id:?}");
            assert_eq!(state.get_active(1).public_details().sleep_coins, 2);
        }
    }
}

// Only both coins heads wake it, so after the Checkup 1 in 4 are awake (the exact 1/4 is checked
// on the Checkup itself).
#[test]
fn soothing_lullaby_defender_wakes_at_checkup_about_one_time_in_four() {
    let id = CardId::C1051Jigglypuff;
    let (mut asleep, mut awake) = (0, 0);
    for seed in 0..200 {
        let mut game = lullaby_game(id, seed);
        end_turn(&mut game, 0);
        if game.get_state_clone().get_active(1).is_asleep() {
            asleep += 1;
        } else {
            awake += 1;
        }
    }
    assert!(
        awake > 0 && asleep > awake * 2,
        "{asleep} asleep vs {awake} awake"
    );
}

// A Pokémon that cannot be Asleep (Hoothoot's Insomnia) does not carry the 2 coins either.
#[test]
fn soothing_lullaby_does_nothing_to_a_defender_that_cannot_sleep() {
    let id = CardId::C1051Jigglypuff;
    let mut game = get_initialized_game_with_board(
        0,
        0,
        3,
        vec![ready(id, 0)],
        vec![durable_of(CardId::A4140Hoothoot)],
    );
    attack(&mut game, id, 0);
    let state = game.get_state_clone();
    assert!(!state.get_active(1).is_asleep());
    assert_eq!(state.get_active(1).public_details().sleep_coins, 0);
}

// Full Heal ends the Sleep, and with it the 2 coins: a later ordinary Sleep flips 1.
#[test]
fn soothing_lullaby_sleep_ends_with_full_heal() {
    let mut game = lullaby_game(CardId::C1051Jigglypuff, 0);
    let mut state = game.get_state_clone();
    state.hands[1] = vec![Card::Trainer(trainer_from_id(CardId::C1122FullHeal))];
    state.current_player = 1;
    game.set_state(state);
    play_trainer(&mut game, 1, trainer_from_id(CardId::C1122FullHeal));

    let mut state = game.get_state_clone();
    assert!(!state.get_active(1).is_asleep());
    state.apply_status_condition(1, 0, StatusCondition::Asleep);
    assert_eq!(state.get_active(1).public_details().sleep_coins, 1);
}
