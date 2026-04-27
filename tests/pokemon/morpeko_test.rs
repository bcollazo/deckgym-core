use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

#[test]
fn test_morpeko_energizer_wheel_moves_two_darkness_energy_to_bench() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B2b045Morpeko).with_energy(vec![
                EnergyType::Darkness,
                EnergyType::Darkness,
                EnergyType::Darkness,
            ]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert_eq!(choices.len(), 1);
    assert!(matches!(
        choices[0].action,
        SimpleAction::MoveEnergy {
            from_in_play_idx: 0,
            to_in_play_idx: 1,
            energy_type: EnergyType::Darkness,
            amount: 2,
        }
    ));

    game.apply_action(&choices[0]);
    let state = game.get_state_clone();

    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        10,
        "Energizer Wheel should deal 50 damage"
    );
    assert_eq!(
        state.get_active(0).attached_energy,
        vec![EnergyType::Darkness],
        "Morpeko should keep any Darkness Energy beyond the 2 moved by the effect"
    );
    assert_eq!(
        state.in_play_pokemon[0][1]
            .as_ref()
            .unwrap()
            .attached_energy,
        vec![EnergyType::Darkness, EnergyType::Darkness]
    );
}

#[test]
fn test_morpeko_full_belly_bolt_bonus_only_when_undamaged() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![PlayedCard::from_id(CardId::B3062Morpeko).with_energy(vec![EnergyType::Lightning])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        30,
        "Undamaged Morpeko should deal 40 damage"
    );

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B3062Morpeko)
            .with_energy(vec![EnergyType::Lightning])
            .with_remaining_hp(60)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        60,
        "Damaged Morpeko should deal only its base 10 damage"
    );
}
