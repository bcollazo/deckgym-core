use super::*;

#[test]
fn shredding_claw_keeps_ninety_base_damage_and_adds_fifty_per_heads() {
    let id = CardId::C1091MegaSkarmoryEx;
    let mut seen = std::collections::BTreeSet::new();
    for seed in 0..40 {
        let mut game = board(seed, vec![ready(id, 0)], vec![durable()]);
        attack(&mut game, id, 0);
        let d = game.get_state_clone().get_active(1).public_details().damage;
        assert!(d >= 90 && (d - 90).is_multiple_of(50));
        seen.insert(d);
    }
    assert!(seen.contains(&90));
    assert!(seen.iter().any(|d| *d > 90));
}
