use super::*;

#[test]
fn long_neck_hammering_hits_only_one_opposing_bench_and_keeps_active_damage() {
    let id = CardId::C1005AlolanExeggutorEx;
    let mut hit = [false; 2];
    for seed in 0..40 {
        let mut game = board(
            seed,
            vec![ready(id, 0), durable()],
            vec![durable(), durable(), durable()],
        );
        attack(&mut game, id, 0);
        let s = game.get_state_clone();
        assert_eq!(s.get_active(1).public_details().damage, 100);
        let damage: Vec<_> = s
            .enumerate_bench_pokemon(1)
            .map(|(_, p)| p.public_details().damage)
            .collect();
        assert!(damage == [50, 0] || damage == [0, 50]);
        hit[usize::from(damage[0] == 0)] = true;
        assert_eq!(s.get_active(0).public_details().damage, 0);
        assert_eq!(
            s.in_play_pokemon[0][1]
                .as_ref()
                .unwrap()
                .public_details()
                .damage,
            0
        );
    }
    assert_eq!(hit, [true, true]);
}
#[test]
fn long_neck_hammering_still_hits_active_when_bench_is_empty() {
    let id = CardId::C1005AlolanExeggutorEx;
    let mut game = board(0, vec![ready(id, 0)], vec![durable()]);
    attack(&mut game, id, 0);
    assert_eq!(
        game.get_state_clone().get_active(1).public_details().damage,
        100
    );
}
