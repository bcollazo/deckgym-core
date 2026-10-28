use super::*;
use deckgym::actions::SimpleAction;

#[test]
fn reflexive_retaliation_deals_thirty_only_for_active_attack_damage() {
    for (target, damage, expected) in [(0, 40, 30), (0, 0, 0), (1, 40, 0)] {
        let mut game = board(
            0,
            vec![durable()],
            vec![
                PlayedCard::from_id(CardId::C1080LycanrocEx),
                PlayedCard::from_id(CardId::C1080LycanrocEx),
            ],
        );
        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::ApplyDamage {
                attacking_ref: (0, 0),
                targets: vec![(damage, 1, target)],
                is_from_active_attack: true,
            },
            is_stack: false,
        });
        assert_eq!(
            game.get_state_clone().get_active(0).public_details().damage,
            expected
        );
    }
}
#[test]
fn reflexive_retaliation_still_triggers_when_the_hit_knocks_lycanroc_out() {
    let mut game = board(
        0,
        vec![durable()],
        vec![
            PlayedCard::from_id(CardId::C1080LycanrocEx).with_remaining_hp(10),
            durable(),
        ],
    );
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (0, 0),
            targets: vec![(40, 1, 0)],
            is_from_active_attack: true,
        },
        is_stack: false,
    });
    assert_eq!(
        game.get_state_clone().get_active(0).public_details().damage,
        30
    );
    assert_eq!(game.get_state_clone().points[0], 2);
}
