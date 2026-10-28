use super::*;
use deckgym::models::StatusCondition;

#[test]
fn pester_the_dizzy_adds_ninety_against_a_confused_active() {
    let id = CardId::PC005Malamar;
    let mut game = board(vec![ready(id, 0)], vec![durable()]);
    let mut state = game.get_state_clone();
    state.apply_status_condition(1, 0, StatusCondition::Confused);
    game.set_state(state);
    attack(&mut game, id, 0);
    assert_eq!(defender_damage(&game), 10 + 90);
}

#[test]
fn pester_the_dizzy_does_only_base_damage_otherwise() {
    let id = CardId::PC005Malamar;
    let mut game = board(vec![ready(id, 0)], vec![durable()]);
    attack(&mut game, id, 0);
    assert_eq!(defender_damage(&game), 10);
}
