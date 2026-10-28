use super::*;
use deckgym::models::EnergyType as E;

#[test]
fn psychic_blaster_flips_only_for_psychic_energy_and_has_no_fixed_base_damage() {
    let id = CardId::PC002MegaMewtwoYEx;
    let mut damage = std::collections::BTreeSet::new();
    for seed in 0..80 {
        let attacker =
            PlayedCard::from_id(id).with_energy(vec![E::Psychic, E::Psychic, E::Water, E::Fire]);
        let mut game = board(seed, vec![attacker], vec![durable()]);
        attack(&mut game, id, 0);
        let d = game.get_state_clone().get_active(1).public_details().damage;
        assert!([0, 60, 120].contains(&d));
        damage.insert(d);
    }
    assert_eq!(damage, std::collections::BTreeSet::from([0, 60, 120]));
}
