use super::*;

#[test]
fn mind_jack_adds_thirty_per_opponent_benched_pokemon_on_both_printings() {
    for id in [CardId::C1055Alakazam, CardId::C1197Alakazam] {
        let mut game = board(
            vec![ready(id, 0)],
            vec![durable(), durable(), durable(), durable()],
        );
        attack(&mut game, id, 0);
        assert_eq!(defender_damage(&game), 50 + 30 * 3, "{id:?}");
    }
}

#[test]
fn mind_jack_does_only_base_damage_with_an_empty_opponent_bench() {
    let id = CardId::C1055Alakazam;
    let mut game = board(vec![ready(id, 0)], vec![durable()]);
    attack(&mut game, id, 0);
    assert_eq!(defender_damage(&game), 50);
}
