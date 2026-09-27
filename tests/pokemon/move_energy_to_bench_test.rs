use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Regice's Reflect Energy does 70 damage and then moves 2 Energy from Regice onto a Benched
/// Pokémon the attacker picks.
#[test]
fn test_reflect_energy_moves_two_energy_to_chosen_bench() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B3045Regice).with_energy(vec![
                EnergyType::Water,
                EnergyType::Water,
                EnergyType::Water,
            ]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3045Regice, 0),
        is_stack: false,
    });

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(choices
        .iter()
        .all(|choice| matches!(choice.action, SimpleAction::MoveActiveEnergyToBench { .. })));
    game.apply_action(&choices[0].clone());

    let state = game.get_state_clone();
    assert_eq!(state.get_active(0).attached_energy.len(), 1);
    assert_eq!(
        state.in_play_pokemon[0][1]
            .as_ref()
            .expect("Bulbasaur should still be benched")
            .attached_energy,
        vec![EnergyType::Water, EnergyType::Water]
    );
    assert_eq!(state.get_active(1).get_remaining_hp(), 150 - 70);
}

/// Swanna's Feathery Cyclone moves *all* of its Energy to a Benched Pokémon.
#[test]
fn test_feathery_cyclone_moves_all_energy_to_chosen_bench() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A4063Swanna).with_energy(vec![
                EnergyType::Water,
                EnergyType::Water,
                EnergyType::Fire,
            ]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A4063Swanna, 0),
        is_stack: false,
    });

    let (_, choices) = game.get_state_clone().generate_possible_actions();
    game.apply_action(&choices[0].clone());

    let state = game.get_state_clone();
    assert!(state.get_active(0).attached_energy.is_empty());
    assert_eq!(
        state.in_play_pokemon[0][1]
            .as_ref()
            .expect("Bulbasaur should still be benched")
            .attached_energy
            .len(),
        3
    );
    assert_eq!(state.get_active(1).get_remaining_hp(), 150 - 60);
}

/// With no Benched Pokémon there is nowhere to move the Energy, so the attack is damage only.
#[test]
fn test_feathery_cyclone_without_bench_keeps_energy() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A4063Swanna)
            .with_energy(vec![EnergyType::Water, EnergyType::Water])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A4063Swanna, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.get_active(0).attached_energy.len(), 2);
    assert_eq!(state.get_active(1).get_remaining_hp(), 150 - 60);
}
