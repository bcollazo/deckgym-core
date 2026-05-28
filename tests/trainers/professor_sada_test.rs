use deckgym::{
    actions::SimpleAction,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

/// Professor Sada attaches 3 different energy types from the discard pile to Ancient Pokémon.
#[test]
fn test_professor_sada_attaches_three_energies_to_ancient_pokemon() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B3a036KoraidonEx),
            PlayedCard::from_id(CardId::B3a047RoaringMoon),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 1;
    // 3 different energy types in discard
    state.discard_energies[0] = vec![EnergyType::Fire, EnergyType::Fighting, EnergyType::Darkness];
    state.hands[0] = vec![get_card_by_enum(CardId::B3a072ProfessorSada)];
    game.set_state(state);

    let state = game.get_state_clone();
    let (_actor, actions) = state.generate_possible_actions();
    let play_action = actions
        .iter()
        .find(|a| {
            matches!(
                &a.action,
                SimpleAction::Play { trainer_card } if trainer_card.name == "Professor Sada"
            )
        })
        .expect("Professor Sada should be playable");
    game.apply_action(play_action);

    // Should now prompt for attachment distribution choices
    let state = game.get_state_clone();
    let (_actor, attach_choices) = state.generate_possible_actions();
    assert!(
        !attach_choices.is_empty(),
        "There should be attachment choices after playing Professor Sada"
    );
    assert!(
        attach_choices
            .iter()
            .all(|a| matches!(a.action, SimpleAction::ProfessorSadaAttach { .. })),
        "Choices should all be ProfessorSadaAttach actions"
    );

    // Apply first choice and verify 3 energies attached total to Ancient Pokémon
    game.apply_action(&attach_choices[0]);

    let final_state = game.get_state_clone();
    let total_energy_on_ancient: usize = final_state.in_play_pokemon[0]
        .iter()
        .flatten()
        .filter(|p| matches!(p.get_name().as_str(), "Koraidon ex" | "Roaring Moon"))
        .map(|p| p.attached_energy.len())
        .sum();
    assert_eq!(
        total_energy_on_ancient, 3,
        "All 3 energies should be on Ancient Pokémon"
    );
    assert!(
        final_state.discard_energies[0].is_empty(),
        "Discard pile should be empty after attaching all 3 energies"
    );
}

/// Professor Sada cannot be played if there are fewer than 3 distinct energy types in discard.
#[test]
fn test_professor_sada_not_playable_with_fewer_than_three_types() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::B3a036KoraidonEx)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 1;
    // Only 2 different types in discard
    state.discard_energies[0] = vec![EnergyType::Fire, EnergyType::Fire, EnergyType::Fighting];
    state.hands[0] = vec![get_card_by_enum(CardId::B3a072ProfessorSada)];
    game.set_state(state);

    let state = game.get_state_clone();
    let (_actor, actions) = state.generate_possible_actions();
    let play_action = actions.iter().find(|a| {
        matches!(
            &a.action,
            SimpleAction::Play { trainer_card } if trainer_card.name == "Professor Sada"
        )
    });
    assert!(
        play_action.is_none(),
        "Professor Sada should not be playable with fewer than 3 distinct energy types"
    );
}
