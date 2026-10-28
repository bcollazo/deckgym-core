use super::*;

#[test]
fn clean_hit_adds_eighty_against_an_evolution_active() {
    let id = CardId::C1104Pidgeot;
    let mut game = board(vec![ready(id, 0)], vec![durable_of(CardId::A1002Ivysaur)]);
    attack(&mut game, id, 0);
    assert_eq!(defender_damage(&game), 40 + 80);
}

#[test]
fn clean_hit_adds_nothing_against_a_basic_active() {
    let id = CardId::C1104Pidgeot;
    let mut game = board(vec![ready(id, 0)], vec![durable()]);
    attack(&mut game, id, 0);
    assert_eq!(defender_damage(&game), 40);
}
