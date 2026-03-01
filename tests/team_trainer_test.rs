use common::get_initialized_game;
use deckgym::{
    actions::SimpleAction,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
};

mod common;

#[test]
fn test_team_discards_from_opponent_ability_pokemon_only() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![
            PlayedCard::from_id(CardId::A2110DarkraiEx)
                .with_energy(vec![EnergyType::Darkness, EnergyType::Darkness]),
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_energy(vec![EnergyType::Grass]),
        ],
    );
    state.current_player = 0;
    state.turn_count = 1;
    state.hands[0] = vec![get_card_by_enum(CardId::B2a088Team)];
    game.set_state(state);

    let state = game.get_state_clone();
    let (_actor, actions) = state.generate_possible_actions();
    let play_action = actions
        .iter()
        .find(|action| {
            matches!(
                &action.action,
                SimpleAction::Play { trainer_card } if trainer_card.name == "Team"
            )
        })
        .expect("Expected Team to be playable");
    game.apply_action(play_action);

    let state = game.get_state_clone();
    let opponent_active = state.get_active(1);
    let opponent_bench = state.in_play_pokemon[1][1]
        .as_ref()
        .expect("Expected opponent bench slot 1 to be occupied");

    assert_eq!(
        opponent_active.attached_energy.len(),
        1,
        "Team should discard one Energy from Darkrai ex"
    );
    assert_eq!(
        opponent_bench.attached_energy,
        vec![EnergyType::Grass],
        "Team should not discard from non-Ability Pokemon"
    );
    assert_eq!(state.discard_energies[1], vec![EnergyType::Darkness]);
}

#[test]
fn test_team_not_playable_without_valid_opponent_target() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_energy(vec![EnergyType::Grass]),
            PlayedCard::from_id(CardId::A2110DarkraiEx),
        ],
    );
    state.current_player = 0;
    state.turn_count = 1;
    state.hands[0] = vec![get_card_by_enum(CardId::B2a105Team)];
    game.set_state(state);

    let state = game.get_state_clone();
    let (_actor, actions) = state.generate_possible_actions();
    let has_team_play = actions.iter().any(|action| {
        matches!(
            &action.action,
            SimpleAction::Play { trainer_card } if trainer_card.name == "Team"
        )
    });

    assert!(
        !has_team_play,
        "Team should be unplayable when opponent has no Ability Pokemon with Energy"
    );
}
