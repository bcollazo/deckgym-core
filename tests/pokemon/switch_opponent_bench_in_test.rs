use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Sandy Shocks's Pull In and Pound drags one of the opponent's Benched Pokémon into the Active
/// Spot (the attacker chooses) and then does 50 damage to it.
#[test]
fn test_pull_in_and_pound_switches_and_damages_the_new_active() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B3a035SandyShocks).with_energy(vec![
                EnergyType::Fighting,
                EnergyType::Fighting,
                EnergyType::Colorless,
            ]),
        ],
        vec![
            PlayedCard::from_id(CardId::A1211Snorlax),
            PlayedCard::from_id(CardId::A1129MewtwoEx),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3a035SandyShocks, 0),
        is_stack: false,
    });

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(choices.iter().all(|choice| matches!(
        choice.action,
        SimpleAction::SwitchOpponentBenchedThenDamage { .. }
    )));
    game.apply_action(&choices[0].clone());
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_name(), "Mewtwo ex");
    assert_eq!(state.get_active(1).get_remaining_hp(), 150 - 50);
    assert_eq!(
        state.in_play_pokemon[1][1]
            .as_ref()
            .expect("Snorlax should have been benched")
            .get_remaining_hp(),
        150
    );
}

/// With an empty opponent Bench there is nobody to drag up, so nothing happens.
#[test]
fn test_entrap_without_opponent_bench_does_nothing() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4a028TeamRocketsHypno)
            .with_energy(vec![EnergyType::Psychic, EnergyType::Psychic])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4a028TeamRocketsHypno, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_name(), "Snorlax");
    assert_eq!(state.get_active(1).get_remaining_hp(), 150);
}
