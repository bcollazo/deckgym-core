use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

fn phoenix_turbo(actor: usize) -> Action {
    Action {
        actor,
        action: attack_action(CardId::A4034HoOhEx, 0),
        is_stack: false,
    }
}

/// Phoenix Turbo: 80 damage, then attach one [R], [W], and [L] to your Benched Basic Pokémon in
/// any way you like (each Energy independently; all on one Pokémon is allowed).
#[test]
fn test_ho_oh_ex_phoenix_turbo_distributes_energy_across_benched_basics() {
    // Active Ho-Oh ex (Fire) attacks a Chansey (not weak to Fire, so 80 is exact). Two benched
    // Basics at idx 1 (Bulbasaur) and idx 2 (Abra).
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A4034HoOhEx).with_energy(vec![
                EnergyType::Fire,
                EnergyType::Fire,
                EnergyType::Fire,
            ]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1115Abra),
        ],
        vec![PlayedCard::from_id(CardId::A1202Chansey)], // 120 HP
    );

    game.apply_action(&phoenix_turbo(0));

    let state = game.get_state_clone();
    // 80 damage was dealt (regardless of the energy distribution that follows).
    assert_eq!(
        state.in_play_pokemon[1][0]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        40
    );

    let (_actor, actions) = state.generate_possible_actions();
    // 2 benched Basics ^ 3 Energies = 8 possible distributions.
    let distributions = actions
        .iter()
        .filter(|a| matches!(a.action, SimpleAction::Attach { .. }))
        .count();
    assert_eq!(distributions, 8);

    // "All on one" is allowed: all three Energies onto Bulbasaur (idx 1).
    let all_on_bulbasaur = SimpleAction::Attach {
        attachments: vec![
            (1, EnergyType::Fire, 1),
            (1, EnergyType::Water, 1),
            (1, EnergyType::Lightning, 1),
        ],
        is_turn_energy: false,
    };
    // A genuine split is also offered ([R]+[W] on Bulbasaur, [L] on Abra).
    let split = SimpleAction::Attach {
        attachments: vec![
            (1, EnergyType::Fire, 1),
            (1, EnergyType::Water, 1),
            (1, EnergyType::Lightning, 2),
        ],
        is_turn_energy: false,
    };
    assert!(actions.iter().any(|a| a.action == split));

    let choice = actions
        .into_iter()
        .find(|a| a.action == all_on_bulbasaur)
        .expect("should be able to pile all three Energies onto one Benched Basic");
    game.apply_action(&choice);

    let state = game.get_state_clone();
    let bulbasaur = state.in_play_pokemon[0][1].as_ref().unwrap();
    assert_eq!(bulbasaur.attached_energy.len(), 3);
    for energy in [EnergyType::Fire, EnergyType::Water, EnergyType::Lightning] {
        assert!(bulbasaur.attached_energy.contains(&energy));
    }
    // The other Basic received nothing.
    assert!(state.in_play_pokemon[0][2]
        .as_ref()
        .unwrap()
        .attached_energy
        .is_empty());
}

/// A played Fossil counts as a Benched Basic Pokémon and is a valid target.
#[test]
fn test_ho_oh_ex_phoenix_turbo_can_target_benched_fossil() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A4034HoOhEx).with_energy(vec![
                EnergyType::Fire,
                EnergyType::Fire,
                EnergyType::Fire,
            ]),
            PlayedCard::from_id(CardId::A1216HelixFossil), // Fossil, counts as Basic
        ],
        vec![PlayedCard::from_id(CardId::A1202Chansey)],
    );

    game.apply_action(&phoenix_turbo(0));

    // The only Basic is the Fossil at idx 1, so all three Energies must go to it.
    let all_on_fossil = SimpleAction::Attach {
        attachments: vec![
            (1, EnergyType::Fire, 1),
            (1, EnergyType::Water, 1),
            (1, EnergyType::Lightning, 1),
        ],
        is_turn_energy: false,
    };
    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    let choice = actions
        .into_iter()
        .find(|a| a.action == all_on_fossil)
        .expect("a played Fossil should be a valid Benched Basic target");
    game.apply_action(&choice);

    let fossil = game.get_state_clone().in_play_pokemon[0][1]
        .as_ref()
        .unwrap()
        .clone();
    assert_eq!(fossil.attached_energy.len(), 3);
}

/// With no Benched Basic Pokémon, Phoenix Turbo still deals 80 and the Energy fizzles.
#[test]
fn test_ho_oh_ex_phoenix_turbo_deals_damage_with_no_benched_basic() {
    // The only benched Pokémon is an evolved (Stage 1) Ivysaur, which is not a valid target.
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A4034HoOhEx).with_energy(vec![
                EnergyType::Fire,
                EnergyType::Fire,
                EnergyType::Fire,
            ]),
            PlayedCard::from_id(CardId::A1002Ivysaur),
        ],
        vec![PlayedCard::from_id(CardId::A1202Chansey)],
    );

    game.apply_action(&phoenix_turbo(0));

    let state = game.get_state_clone();
    // Damage is still dealt...
    assert_eq!(
        state.in_play_pokemon[1][0]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        40
    );
    // ...no distribution choice was offered, and the evolved bench Pokémon got no Energy.
    let (_actor, actions) = state.generate_possible_actions();
    assert!(!actions
        .iter()
        .any(|a| matches!(a.action, SimpleAction::Attach { .. })));
    assert!(state.in_play_pokemon[0][1]
        .as_ref()
        .unwrap()
        .attached_energy
        .is_empty());
}
