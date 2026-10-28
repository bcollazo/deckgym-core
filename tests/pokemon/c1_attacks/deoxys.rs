use super::*;

#[test]
fn psycho_boost_does_fifty_less_on_the_next_turn_on_every_printing() {
    for id in [CardId::C1062Deoxys, CardId::PC003Deoxys] {
        let mut game = board(vec![ready(id, 0)], vec![durable()]);
        attack(&mut game, id, 0);
        assert_eq!(defender_damage(&game), 100, "{id:?}");
        end_turn(&mut game, 0);
        end_turn(&mut game, 1);
        attack(&mut game, id, 0);
        assert_eq!(defender_damage(&game), 100 + 50, "{id:?}");
    }
}

#[test]
fn psycho_boost_is_back_to_full_damage_after_a_turn_without_using_it() {
    let id = CardId::C1062Deoxys;
    let mut game = board(vec![ready(id, 0)], vec![durable()]);
    attack(&mut game, id, 0);
    for _ in 0..2 {
        end_turn(&mut game, 0);
        end_turn(&mut game, 1);
    }
    attack(&mut game, id, 0);
    assert_eq!(defender_damage(&game), 100 + 100);
}
