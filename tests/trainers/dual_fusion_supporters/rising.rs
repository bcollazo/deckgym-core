use super::*;

use deckgym::models::EnergyType as E;
#[test]
fn rising_star_counts_dual_pokemon_types_and_switches_without_paying_retreat() {
    for supporter in [CardId::C1130RisingStar, CardId::C1170RisingStar] {
        let mut g = board(vec![
            pokemon("C1 042").with_energy(vec![E::Fire]),
            pokemon("A1 001"),
        ]);
        play(&mut g, supporter);
        let (actor, a) = g.get_state_clone().generate_possible_actions();
        assert_eq!(actor, 0);
        assert_eq!(a.len(), 1);
        assert!(matches!(
            a[0].action,
            SimpleAction::Activate {
                player: 0,
                in_play_idx: 1
            }
        ));
        g.apply_action(&a[0]);
        let s = g.get_state_clone();
        assert_eq!(s.get_active(0).get_name(), "Bulbasaur");
        assert_eq!(
            s.in_play_pokemon[0][1].as_ref().unwrap().attached_energy,
            vec![E::Fire]
        );
    }
}
#[test]
fn rising_star_counts_distinct_types_not_attached_energy_or_duplicate_pokemon() {
    let mut g = board(vec![
        pokemon("A1 001").with_energy(vec![E::Fire, E::Water, E::Lightning]),
        pokemon("A1 002"),
    ]);
    assert!(!offered(&mut g, CardId::C1130RisingStar));
    let mut g = board(vec![pokemon("C1 042")]);
    assert!(!offered(&mut g, CardId::C1130RisingStar));
    let mut g = board(vec![
        pokemon("A1 001"),
        pokemon("A1 033"),
        pokemon("A1 053"),
    ]);
    assert!(offered(&mut g, CardId::C1130RisingStar));
}
