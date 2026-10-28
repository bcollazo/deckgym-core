use super::*;
use deckgym::models::EnergyType;

#[test]
fn amass_attaches_two_fighting_energy_to_itself_on_both_printings() {
    for id in [CardId::C1076Pupitar, CardId::C1146Pupitar] {
        let mut game = board(vec![ready(id, 0)], vec![durable()]);
        attack(&mut game, id, 0);
        let state = game.get_state_clone();
        let energy = &state.get_active(0).attached_energy;
        assert_eq!(
            energy.iter().filter(|e| **e == EnergyType::Fighting).count(),
            2,
            "{id:?}"
        );
        assert_eq!(defender_damage(&game), 0);
    }
}
