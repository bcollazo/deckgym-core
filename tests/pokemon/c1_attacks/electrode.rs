use super::*;

#[test]
fn self_destruct_hits_the_defender_and_electrode_for_ninety() {
    let id = CardId::C1045Electrode;
    let mut game = board(vec![ready(id, 0)], vec![durable()]);
    attack(&mut game, id, 0);
    assert_eq!(defender_damage(&game), 90);
    let state = game.get_state_clone();
    // Electrode has 90 HP, so it knocks itself out and the opponent scores the point.
    assert_eq!(state.points[1], 1);
}
