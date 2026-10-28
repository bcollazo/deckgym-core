use super::*;

#[test]
fn overdrive_smash_adds_fifty_on_the_next_turn_only() {
    let id = CardId::C1110PorygonZ;
    let mut game = board(vec![ready(id, 0)], vec![durable()]);
    attack(&mut game, id, 0);
    assert_eq!(defender_damage(&game), 50);
    end_turn(&mut game, 0);
    end_turn(&mut game, 1);
    attack(&mut game, id, 0);
    assert_eq!(defender_damage(&game), 50 + 100);
    end_turn(&mut game, 0);
    end_turn(&mut game, 1);
    attack(&mut game, id, 0);
    // The bonus is re-armed by each use, so it applies again the turn after.
    assert_eq!(defender_damage(&game), 50 + 100 + 100);
}
