use super::*;
use deckgym::actions::SimpleAction;
use deckgym::models::EnergyType as E;

#[test]
fn secret_guidance_offers_every_distribution_only_to_psychic_bench_including_dual_types() {
    let id = CardId::C1057MewEx;
    let mut totals = std::collections::BTreeSet::new();
    for seed in 0..80 {
        let mut game = board(
            seed,
            vec![
                ready(id, 0),
                PlayedCard::from_id(CardId::C1056MegaMewtwoXEx),
                PlayedCard::from_id(CardId::A1128Mewtwo),
                PlayedCard::from_id(CardId::A1001Bulbasaur),
            ],
            vec![durable()],
        );
        attack(&mut game, id, 0);
        let (actor, actions) = game.get_state_clone().generate_possible_actions();
        let allocations: Vec<_> = actions
            .into_iter()
            .filter(|a| {
                matches!(
                    a.action,
                    SimpleAction::Attach {
                        is_turn_energy: false,
                        ..
                    }
                )
            })
            .collect();
        if allocations.is_empty() {
            totals.insert(0);
            continue;
        }
        assert_eq!(actor, 0);
        let first = &allocations[0];
        let SimpleAction::Attach { attachments, .. } = &first.action else {
            unreachable!()
        };
        let heads: u32 = attachments.iter().map(|(n, _, _)| n).sum();
        assert!((1..=3).contains(&heads));
        assert_eq!(allocations.len(), heads as usize + 1);
        for a in &allocations {
            let SimpleAction::Attach { attachments, .. } = &a.action else {
                unreachable!()
            };
            assert!(attachments
                .iter()
                .all(|(_, energy, idx)| *energy == E::Psychic && [1, 2].contains(idx)));
            assert_eq!(attachments.iter().map(|(n, _, _)| n).sum::<u32>(), heads);
        }
        game.apply_action(first);
        let s = game.get_state_clone();
        assert_eq!(s.get_active(0).attached_energy, vec![E::Psychic]);
        assert!(s.in_play_pokemon[0][3]
            .as_ref()
            .unwrap()
            .attached_energy
            .is_empty());
        assert_eq!(
            s.enumerate_bench_pokemon(0)
                .map(|(_, p)| p.attached_energy.len())
                .sum::<usize>(),
            heads as usize
        );
        totals.insert(heads);
    }
    assert_eq!(totals, std::collections::BTreeSet::from([0, 1, 2, 3]));
}
#[test]
fn secret_guidance_with_no_eligible_bench_finishes_without_attaching_to_mew() {
    let id = CardId::C1057MewEx;
    for seed in 0..20 {
        let mut game = board(
            seed,
            vec![ready(id, 0), PlayedCard::from_id(CardId::A1001Bulbasaur)],
            vec![durable()],
        );
        attack(&mut game, id, 0);
        let s = game.get_state_clone();
        let (actor, actions) = s.generate_possible_actions();
        assert_eq!(actor, 0);
        assert!(actions
            .iter()
            .all(|a| matches!(a.action, SimpleAction::EndTurn)));
        game.apply_action(&actions[0]);
        assert_eq!(game.get_state_clone().current_player, 1);
        assert_eq!(s.get_active(0).attached_energy, vec![E::Psychic]);
        assert!(s.in_play_pokemon[0][1]
            .as_ref()
            .unwrap()
            .attached_energy
            .is_empty());
    }
}
