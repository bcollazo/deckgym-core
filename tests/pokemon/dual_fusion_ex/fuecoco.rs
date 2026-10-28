use super::*;

#[test]
fn feisty_flames_gets_its_bonus_only_when_fuecoco_is_hurt() {
    let id = CardId::C1025FuecocoEx;
    for (hurt, expected) in [(0, 50), (10, 120)] {
        let mut game = board(0, vec![ready(id, 1).with_damage(hurt)], vec![durable()]);
        attack(&mut game, id, 1);
        assert_eq!(
            game.get_state_clone().get_active(1).public_details().damage,
            expected
        );
    }
}
