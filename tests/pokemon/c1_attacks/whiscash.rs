use super::*;
use deckgym::models::EnergyType;

fn fighting(extra: usize) -> Vec<EnergyType> {
    vec![EnergyType::Fighting; 1 + extra]
}

#[test]
fn power_press_adds_sixty_with_three_extra_fighting_energy() {
    let id = CardId::C1078Whiscash;
    let mut game = board(
        vec![PlayedCard::from_id(id).with_energy(fighting(3))],
        vec![durable_of(CardId::A1033Charmander)],
    );
    attack(&mut game, id, 0);
    assert_eq!(defender_damage(&game), 40 + 60);
}

#[test]
fn power_press_needs_all_three_extra_energy() {
    let id = CardId::C1078Whiscash;
    let mut game = board(
        vec![PlayedCard::from_id(id).with_energy(fighting(2))],
        vec![durable_of(CardId::A1033Charmander)],
    );
    attack(&mut game, id, 0);
    assert_eq!(defender_damage(&game), 40);
}
