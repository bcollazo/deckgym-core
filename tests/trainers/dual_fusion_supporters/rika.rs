use super::*;

use deckgym::models::EnergyType as E;
#[test]
fn rika_attaches_two_fighting_to_one_chosen_named_pokemon_without_consuming_turn_energy() {
    for supporter in [CardId::C1128Rika, CardId::C1168Rika] {
        let mut g = board(vec![
            pokemon("A2a 017"),
            pokemon("B2a 065"),
            pokemon("A1 001"),
        ]);
        let before = g.get_state_clone().energy_zone[0];
        play(&mut g, supporter);
        let (actor, a) = g.get_state_clone().generate_possible_actions();
        assert_eq!(actor, 0);
        assert_eq!(a.len(), 2);
        assert!(a.iter().all(|a|matches!(&a.action,SimpleAction::Attach{attachments,is_turn_energy:false} if attachments.len()==1 && attachments[0].0==2 && attachments[0].1==E::Fighting && attachments[0].2<2)));
        let a = a
            .into_iter()
            .find(
                |a| matches!(&a.action,SimpleAction::Attach{attachments,..} if attachments[0].2==1),
            )
            .unwrap();
        g.apply_action(&a);
        let s = g.get_state_clone();
        assert!(s.get_active(0).attached_energy.is_empty());
        assert_eq!(
            s.in_play_pokemon[0][1].as_ref().unwrap().attached_energy,
            vec![E::Fighting; 2]
        );
        assert_eq!(s.energy_zone[0], before);
    }
}
#[test]
fn rika_is_unplayable_without_exact_named_targets() {
    for name in ["A1 001", "A3 094"] {
        let mut g = board(vec![pokemon(name)]);
        assert!(!offered(&mut g, CardId::C1128Rika));
    }
}
