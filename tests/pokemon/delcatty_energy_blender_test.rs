use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Delcatty - Energy Blender: "You may move any amount of Energy from your Pokémon in play to your
/// other Pokémon in any way you like." The redistribution is played out one Energy at a time.
#[test]
fn test_delcatty_energy_blender_moves_energy_one_at_a_time() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B4135Delcatty).with_energy(vec![
                EnergyType::Colorless,
                EnergyType::Colorless,
                EnergyType::Water,
            ]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4135Delcatty, 0),
        is_stack: false,
    });
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 100);

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    // Two distinct Energy types on Delcatty, one destination, plus the option to stop.
    assert_eq!(choices.len(), 3);
    assert!(choices
        .iter()
        .any(|choice| matches!(choice.action, SimpleAction::Noop)));

    let move_water = choices
        .iter()
        .find(|choice| {
            matches!(
                choice.action,
                SimpleAction::MoveEnergyAndReoffer {
                    from_in_play_idx: 0,
                    to_in_play_idx: 1,
                    energy_type: EnergyType::Water,
                    ..
                }
            )
        })
        .expect("moving the Water Energy to the Bench should be offered")
        .clone();
    game.apply_action(&move_water);

    let state = game.get_state_clone();
    assert_eq!(
        state.in_play_pokemon[0][1]
            .as_ref()
            .expect("bench")
            .attached_energy,
        vec![EnergyType::Water]
    );
    assert_eq!(state.get_active(0).attached_energy.len(), 2);

    // The prompt comes back so more Energy can be moved; stopping ends the effect.
    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    let stop = choices
        .iter()
        .find(|choice| matches!(choice.action, SimpleAction::Noop))
        .expect("stopping should be offered")
        .clone();
    game.apply_action(&stop);

    let (_, choices) = game.get_state_clone().generate_possible_actions();
    assert!(
        !choices
            .iter()
            .any(|choice| matches!(choice.action, SimpleAction::MoveEnergyAndReoffer { .. })),
        "the effect is over once the player stops"
    );
}

/// With nothing to move, Energy Blender is just its damage — no prompt is raised.
#[test]
fn test_delcatty_energy_blender_without_a_second_pokemon_just_deals_damage() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4135Delcatty)
            .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4135Delcatty, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 100);
    let (_, choices) = state.generate_possible_actions();
    assert!(
        !choices
            .iter()
            .any(|choice| matches!(choice.action, SimpleAction::MoveEnergyAndReoffer { .. })),
        "with no other Pokémon in play there is nothing to offer"
    );
}

/// The move budget is the player's total attached Energy, so the prompt always terminates.
#[test]
fn test_delcatty_energy_blender_terminates() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B4135Delcatty)
                .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless]),
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_energy(vec![EnergyType::Grass]),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4135Delcatty, 0),
        is_stack: false,
    });

    // Always take the first non-Noop move; the budget must still run out.
    let mut steps = 0;
    loop {
        let (_, choices) = game.get_state_clone().generate_possible_actions();
        let Some(next) = choices
            .iter()
            .find(|choice| matches!(choice.action, SimpleAction::MoveEnergyAndReoffer { .. }))
            .cloned()
        else {
            break;
        };
        game.apply_action(&next);
        steps += 1;
        assert!(steps <= 3, "the move budget is the 3 Energy in play");
    }
    assert_eq!(steps, 3);
}
