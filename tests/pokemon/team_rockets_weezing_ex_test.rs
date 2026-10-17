use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game_with_board},
    Game,
};

/// Team Rocket's Weezing ex's Boiler Smog: "Once during your turn, when you play this Pokémon
/// from your hand to evolve 1 of your Pokémon, you may make your opponent's Active Pokémon
/// Poisoned and Burned."
fn evolve_koffing_into_weezing_ex(seed: u64) -> Game<'static> {
    let mut game = get_initialized_game_with_board(
        seed,
        0,
        3,
        vec![PlayedCard::from_id(CardId::B4a042TeamRocketsKoffing)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let mut state = game.get_state_clone();
    state.hands[0].clear();
    state.hands[0].push(get_card_by_enum(CardId::B4a043TeamRocketsWeezingEx));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Evolve {
            evolution: get_card_by_enum(CardId::B4a043TeamRocketsWeezingEx),
            in_play_idx: 0,
            from_deck: false,
        },
        is_stack: false,
    });

    game
}

#[test]
fn test_boiler_smog_is_optional_on_evolve_and_applies_both_statuses() {
    let mut game = evolve_koffing_into_weezing_ex(0);

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert_eq!(choices.len(), 2, "Boiler Smog should be optional");
    assert!(choices
        .iter()
        .any(|c| matches!(c.action, SimpleAction::ApplyStatusesToOpponentActive { .. })));
    assert!(choices
        .iter()
        .any(|c| matches!(c.action, SimpleAction::Noop)));

    let apply_choice = choices
        .into_iter()
        .find(|c| matches!(c.action, SimpleAction::ApplyStatusesToOpponentActive { .. }))
        .expect("Boiler Smog choice should be present");
    game.apply_action(&apply_choice);

    let state = game.get_state_clone();
    assert!(state.get_active(1).is_poisoned());
    assert!(state.get_active(1).is_burned());
}

#[test]
fn test_boiler_smog_declined_leaves_opponent_unaffected() {
    let mut game = evolve_koffing_into_weezing_ex(0);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Noop,
        is_stack: true,
    });

    let state = game.get_state_clone();
    assert!(!state.get_active(1).is_poisoned());
    assert!(!state.get_active(1).is_burned());
}

/// Team Rocket's Weezing ex's Confusion Gas: 60 damage, and the Defending Pokémon is now
/// Confused.
#[test]
fn test_weezing_ex_confusion_gas_confuses_defender() {
    let mut game = get_initialized_game_with_board(
        0,
        0,
        3,
        vec![PlayedCard::from_id(CardId::B4a043TeamRocketsWeezingEx)
            .with_energy(vec![EnergyType::Darkness, EnergyType::Darkness])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4a043TeamRocketsWeezingEx, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert!(state.get_active(1).is_confused());
}
