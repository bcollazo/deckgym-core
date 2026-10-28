use super::*;

#[test]
fn draconic_meteor_randomly_hits_one_opponent_without_touching_own_board() {
    let id = CardId::C1100MegaDragoniteEx;
    let mut hit = [false; 2];
    for seed in 0..40 {
        let mut game = board(
            seed,
            vec![ready(id, 0), durable()],
            vec![durable(), durable()],
        );
        attack(&mut game, id, 0);
        let s = game.get_state_clone();
        let a = s.get_active(1).public_details().damage;
        let b = s.in_play_pokemon[1][1]
            .as_ref()
            .unwrap()
            .public_details()
            .damage;
        assert!((a, b) == (250, 0) || (a, b) == (0, 250));
        hit[usize::from(a == 0)] = true;
        assert!(s
            .enumerate_in_play_pokemon(0)
            .all(|(_, p)| p.public_details().damage == 0));
    }
    assert_eq!(hit, [true, true]);
}
#[test]
fn draconic_meteor_hits_the_lone_active_for_250() {
    let id = CardId::C1100MegaDragoniteEx;
    let mut game = board(0, vec![ready(id, 0)], vec![durable()]);
    attack(&mut game, id, 0);
    assert_eq!(
        game.get_state_clone().get_active(1).public_details().damage,
        250
    );
}
