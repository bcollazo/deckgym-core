use super::*;
use deckgym::models::EnergyType;

#[test]
fn brainwave_adds_twenty_per_psychic_energy_attached() {
    let id = CardId::C1060Espeon;
    let mut game = board(
        vec![ready(id, 0).with_energy(vec![
            EnergyType::Psychic,
            EnergyType::Psychic,
            EnergyType::Water,
        ])],
        vec![durable()],
    );
    attack(&mut game, id, 0);
    assert_eq!(defender_damage(&game), 10 + 20 * 2);
}
