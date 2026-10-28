use super::*;

#[test]
fn lorelei_boosts_named_pokemon_against_active_and_bench_only_this_turn() {
    for (attacker, boost) in [
        ("A1 079", 30),
        ("A1 065", 30),
        ("A1 067", 30),
        ("A1 001", 0),
    ] {
        for supporter in [CardId::C1125Lorelei, CardId::C1165Lorelei] {
            let mut g = board(vec![pokemon(attacker)]);
            play(&mut g, supporter);
            g.apply_action(&Action {
                actor: 0,
                action: SimpleAction::ApplyDamage {
                    attacking_ref: (0, 0),
                    targets: vec![(20, 1, 0), (10, 1, 1)],
                    is_from_active_attack: true,
                },
                is_stack: false,
            });
            let s = g.get_state_clone();
            assert_eq!(s.get_active(1).public_details().damage, 20 + boost);
            assert_eq!(
                s.in_play_pokemon[1][1]
                    .as_ref()
                    .unwrap()
                    .public_details()
                    .damage,
                10 + boost
            );
            g.apply_action(&Action {
                actor: 0,
                action: SimpleAction::EndTurn,
                is_stack: false,
            });
            g.play_until_stable();
            g.apply_action(&Action {
                actor: 1,
                action: SimpleAction::EndTurn,
                is_stack: false,
            });
            g.play_until_stable();
            g.apply_action(&Action {
                actor: 0,
                action: SimpleAction::ApplyDamage {
                    attacking_ref: (0, 0),
                    targets: vec![(10, 1, 0)],
                    is_from_active_attack: true,
                },
                is_stack: false,
            });
            assert_eq!(
                g.get_state_clone().get_active(1).public_details().damage,
                30 + boost
            );
        }
    }
}
#[test]
fn lorelei_does_not_boost_damage_to_own_bench_or_non_attack_damage() {
    let mut g = board(vec![pokemon("A1 079"), pokemon("A1 001")]);
    play(&mut g, CardId::C1125Lorelei);
    g.apply_action(&Action {
        actor: 0,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (0, 0),
            targets: vec![(10, 0, 1)],
            is_from_active_attack: true,
        },
        is_stack: false,
    });
    g.apply_action(&Action {
        actor: 0,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (0, 0),
            targets: vec![(10, 1, 0)],
            is_from_active_attack: false,
        },
        is_stack: false,
    });
    let s = g.get_state_clone();
    assert_eq!(
        s.in_play_pokemon[0][1]
            .as_ref()
            .unwrap()
            .public_details()
            .damage,
        10
    );
    assert_eq!(s.get_active(1).public_details().damage, 10);
}
