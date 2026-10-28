use super::*;
use deckgym::{actions::SimpleAction, models::EnergyType};

fn move_to(game: &mut Game<'static>, energy: EnergyType, to: usize) {
    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    let chosen = choices
        .into_iter()
        .find(|choice| {
            matches!(&choice.action, SimpleAction::MoveActiveEnergyToBenchAndReoffer {
                to_in_play_idx, energy_type
            } if *to_in_play_idx == to && *energy_type == energy)
        })
        .expect("a choice moving that Energy to that Pokémon");
    game.apply_action(&chosen);
}

#[test]
fn wings_of_the_moone_moves_every_energy_to_the_bench_in_any_way_on_every_printing() {
    for id in [
        CardId::C1066Lunala,
        CardId::C1198Lunala,
        CardId::PC008Lunala,
    ] {
        let mut game = board(
            vec![
                PlayedCard::from_id(id).with_energy(vec![
                    EnergyType::Psychic,
                    EnergyType::Psychic,
                    EnergyType::Water,
                ]),
                durable(),
                durable(),
            ],
            vec![durable()],
        );
        attack(&mut game, id, 0);
        assert_eq!(defender_damage(&game), 110, "{id:?}");

        // 3 Energy (P, P, W) can go to either of 2 Benched Pokémon: 2 types x 2 targets, and
        // there is no way to stop with Energy left on Lunala.
        let (_, choices) = game.get_state_clone().generate_possible_actions();
        assert_eq!(choices.len(), 4, "{id:?}");
        move_to(&mut game, EnergyType::Psychic, 1);
        move_to(&mut game, EnergyType::Psychic, 2);
        move_to(&mut game, EnergyType::Water, 1);

        let state = game.get_state_clone();
        assert!(state.get_active(0).attached_energy.is_empty(), "{id:?}");
        let attached = |idx: usize| {
            let mut e = state.in_play_pokemon[0][idx]
                .as_ref()
                .unwrap()
                .attached_energy
                .clone();
            e.sort_by_key(|t| format!("{t:?}"));
            e
        };
        assert_eq!(attached(1), vec![EnergyType::Psychic, EnergyType::Water]);
        assert_eq!(attached(2), vec![EnergyType::Psychic]);
    }
}

#[test]
fn wings_of_the_moone_keeps_its_energy_without_a_benched_pokemon() {
    let id = CardId::C1066Lunala;
    let mut game = board(vec![ready(id, 0)], vec![durable()]);
    attack(&mut game, id, 0);
    assert_eq!(defender_damage(&game), 110);
    assert_eq!(
        game.get_state_clone().get_active(0).attached_energy.len(),
        2
    );
}
