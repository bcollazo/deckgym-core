use common::get_initialized_game;
use deckgym::{
    actions::SimpleAction,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
};

mod common;

#[test]
fn test_piers_discards_single_energy_without_panic() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B2100GalarianObstagoon),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 1;
    state.hands[0] = vec![get_card_by_enum(CardId::B2152Piers)];

    state.in_play_pokemon[1][0]
        .as_mut()
        .unwrap()
        .attached_energy = vec![EnergyType::Darkness];

    game.set_state(state);

    let state = game.get_state_clone();
    let (_actor, actions) = state.generate_possible_actions();
    let play_action = actions
        .iter()
        .find(|action| {
            matches!(
                &action.action,
                SimpleAction::Play { trainer_card } if trainer_card.name == "Piers"
            )
        })
        .expect("Expected Piers to be playable");
    game.apply_action(play_action);

    let state = game.get_state_clone();
    let opponent_active = state.get_active(1);
    assert!(
        opponent_active.attached_energy.is_empty(),
        "Piers should discard the opponent's only energy"
    );
    assert_eq!(state.discard_energies[1], vec![EnergyType::Darkness]);
}
