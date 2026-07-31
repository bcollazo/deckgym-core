use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::get_test_game_with_board,
};

/// Dragon's Blessing: "Once during your turn, if this Pokémon is on your Bench, you may attach
/// an Energy from your discard pile to your Active Pokémon."
#[test]
fn test_dragonair_dragons_blessing_attaches_chosen_energy_to_active() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::B4175Dragonair),
        ],
        vec![PlayedCard::from_id(CardId::A1002Ivysaur)],
    );

    let mut state = game.get_state_clone();
    state.discard_energies[0] = vec![EnergyType::Fire, EnergyType::Water];
    game.set_state(state);

    let ability_action = Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 1 },
        is_stack: false,
    };
    game.apply_action(&ability_action);

    // Player should be offered a choice of which discarded Energy type to attach.
    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert_eq!(
        choices.len(),
        2,
        "Expected a choice for each distinct discarded Energy type; got: {choices:?}"
    );
    assert!(choices.iter().all(|choice| matches!(
        choice.action,
        SimpleAction::AttachTypedFromDiscard {
            in_play_idx: 0,
            count: 1,
            ..
        }
    )));

    let water_choice = choices
        .iter()
        .find(|choice| {
            matches!(
                choice.action,
                SimpleAction::AttachTypedFromDiscard {
                    energy_type: EnergyType::Water,
                    ..
                }
            )
        })
        .expect("Should have a choice to attach Water energy")
        .clone();
    game.apply_action(&water_choice);

    let state = game.get_state_clone();
    let active = state.get_active(0);
    assert_eq!(active.attached_energy, vec![EnergyType::Water]);
    assert_eq!(state.discard_energies[0], vec![EnergyType::Fire]);

    // Ability is "once during your turn", so it should no longer be offered.
    let (_actor, actions) = state.generate_possible_actions();
    assert!(!actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::UseAbility { in_play_idx: 1 })));
}

/// Dragon's Blessing can only be used while Dragonair is on the Bench, not while Active.
#[test]
fn test_dragonair_dragons_blessing_not_available_while_active() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4175Dragonair)],
        vec![PlayedCard::from_id(CardId::A1002Ivysaur)],
    );

    let mut state = game.get_state_clone();
    state.discard_energies[0] = vec![EnergyType::Fire];
    game.set_state(state);

    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    assert!(!actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::UseAbility { in_play_idx: 0 })));
}
