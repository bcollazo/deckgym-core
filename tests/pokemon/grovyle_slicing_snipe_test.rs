use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

#[test]
fn test_grovyle_slicing_snipe_targets_only_opponents_bench() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![PlayedCard::from_id(CardId::B3006Grovyle)
            .with_energy(vec![EnergyType::Grass, EnergyType::Grass])],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1053Squirtle),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
    );

    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert_eq!(
        choices.len(),
        2,
        "Expected one bench damage choice per benched Pokemon"
    );
    assert!(choices
        .iter()
        .all(|choice| matches!(choice.action, SimpleAction::ApplyDamage { .. })));

    let target_charmander = choices
        .iter()
        .find(|choice| {
            matches!(
                choice.action,
                SimpleAction::ApplyDamage {
                    ref targets,
                    is_from_active_attack: true,
                    ..
                } if *targets == vec![(50, 1, 2)]
            )
        })
        .cloned()
        .expect("Expected a choice targeting the second benched Pokemon");

    game.apply_action(&target_charmander);
    let state = game.get_state_clone();

    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        70,
        "Slicing Snipe should not damage the opponent's Active Pokemon"
    );
    assert_eq!(
        state.in_play_pokemon[1][1]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        60,
        "Unchosen benched Pokemon should remain undamaged"
    );
    assert_eq!(
        state.in_play_pokemon[1][2]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        10,
        "Chosen benched Pokemon should take 50 damage"
    );
}

#[test]
fn test_grovyle_slicing_snipe_does_nothing_without_opponents_bench() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![PlayedCard::from_id(CardId::B3006Grovyle)
            .with_energy(vec![EnergyType::Grass, EnergyType::Grass])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let (_actor, choices) = state.generate_possible_actions();

    assert!(
        !choices
            .iter()
            .any(|choice| matches!(choice.action, SimpleAction::ApplyDamage { .. })),
        "Slicing Snipe should not queue damage choices when the opponent has no Bench"
    );
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        70,
        "Slicing Snipe should not damage the opponent's Active Pokemon"
    );
}
