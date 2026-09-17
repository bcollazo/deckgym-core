use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Palafin - Jet Punch: 50 to the Defending Pokémon, and also 50 to a chosen Benched Pokémon.
#[test]
fn test_palafin_jet_punch_hits_active_and_a_chosen_bench_pokemon() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2a028Palafin).with_energy(vec![
            EnergyType::Water,
            EnergyType::Water,
            EnergyType::Water,
        ])],
        vec![
            PlayedCard::from_id(CardId::A1211Snorlax),
            PlayedCard::from_id(CardId::A1211Snorlax),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2a028Palafin, 0),
        is_stack: false,
    });

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(choices
        .iter()
        .all(|choice| matches!(choice.action, SimpleAction::ApplyDamage { .. })));
    game.apply_action(&choices[0].clone());

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 100);
    assert_eq!(
        state.in_play_pokemon[1][1]
            .as_ref()
            .expect("bench Snorlax")
            .get_remaining_hp(),
        100
    );
}

/// Palafin with no opposing Bench still deals its 50 damage to the Defending Pokémon.
#[test]
fn test_palafin_jet_punch_still_damages_active_without_a_bench() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2a028Palafin).with_energy(vec![
            EnergyType::Water,
            EnergyType::Water,
            EnergyType::Water,
        ])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2a028Palafin, 0),
        is_stack: false,
    });
    game.play_until_stable();

    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 100);
}

/// Indeedee - Zen Shard: 70 damage to a chosen Benched Pokémon only (never the Active).
#[test]
fn test_indeedee_zen_shard_only_targets_the_bench() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2076Indeedee).with_energy(vec![
            EnergyType::Psychic,
            EnergyType::Psychic,
            EnergyType::Psychic,
        ])],
        vec![
            PlayedCard::from_id(CardId::A1211Snorlax),
            PlayedCard::from_id(CardId::A1211Snorlax),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2076Indeedee, 0),
        is_stack: false,
    });

    let (_, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(
        choices.len(),
        1,
        "only the single Benched Pokémon is a target"
    );
    game.apply_action(&choices[0].clone());

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 150);
    assert_eq!(
        state.in_play_pokemon[1][1]
            .as_ref()
            .expect("bench Snorlax")
            .get_remaining_hp(),
        80
    );
}

/// Mandibuzz - Blindside: 60 damage to a chosen opposing Pokémon that already has damage on it.
#[test]
fn test_mandibuzz_blindside_only_targets_damaged_pokemon() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B3110Mandibuzz).with_energy(vec![EnergyType::Darkness])],
        vec![
            PlayedCard::from_id(CardId::A1211Snorlax),
            PlayedCard::from_id(CardId::A1211Snorlax).with_remaining_hp(100),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3110Mandibuzz, 0),
        is_stack: false,
    });

    let (_, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(
        choices.len(),
        1,
        "only the damaged Benched Pokémon qualifies"
    );
    game.apply_action(&choices[0].clone());

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 150);
    assert_eq!(
        state.in_play_pokemon[1][1]
            .as_ref()
            .expect("bench Snorlax")
            .get_remaining_hp(),
        40
    );
}

/// Galarian Obstagoon - Bass Control: 80 damage to any 1 opposing Pokémon (Active included).
#[test]
fn test_galarian_obstagoon_bass_control_can_target_the_active() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B4097GalarianObstagoon).with_energy(vec![
                EnergyType::Darkness,
                EnergyType::Darkness,
                EnergyType::Darkness,
            ]),
        ],
        vec![
            PlayedCard::from_id(CardId::A1211Snorlax),
            PlayedCard::from_id(CardId::A1211Snorlax),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4097GalarianObstagoon, 0),
        is_stack: false,
    });

    let (_, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(
        choices.len(),
        2,
        "the Active and the Bench are both targets"
    );
    game.apply_action(&choices[0].clone());

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 70);
}
