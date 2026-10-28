use super::*;

#[test]
fn moonlit_raid_adds_fifty_against_a_basic_active_on_both_printings() {
    for id in [CardId::C1084Umbreon, CardId::C1186Umbreon] {
        let mut game = board(vec![ready(id, 0)], vec![durable()]);
        attack(&mut game, id, 0);
        assert_eq!(defender_damage(&game), 30 + 50, "{id:?}");
    }
}

#[test]
fn moonlit_raid_adds_nothing_against_an_evolved_active() {
    let id = CardId::C1084Umbreon;
    let mut game = board(vec![ready(id, 0)], vec![durable_of(CardId::A1002Ivysaur)]);
    attack(&mut game, id, 0);
    assert_eq!(defender_damage(&game), 30);
}
