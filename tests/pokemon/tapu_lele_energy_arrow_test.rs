use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

/// Energy Arrow does 20 damage per energy attached to the chosen target.
#[test]
fn test_energy_arrow_damage_scales_with_target_energy() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    // Tapu Lele with 1 Psychic energy; opponent active has 2 energy, benched has 1 energy.
    state.set_board(
        vec![PlayedCard::from_id(CardId::A3084TapuLele).with_energy(vec![EnergyType::Psychic])],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_energy(vec![EnergyType::Grass, EnergyType::Grass]),
            PlayedCard::from_id(CardId::A1033Charmander).with_energy(vec![EnergyType::Fire]),
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
    // Should have 2 choices: active (2 energy → 40 dmg) and benched (1 energy → 20 dmg)
    assert_eq!(choices.len(), 2);
    assert!(choices
        .iter()
        .all(|c| matches!(c.action, SimpleAction::ApplyDamage { .. })));

    // The choice targeting the active (in_play_idx 0, 2 energy) should deal 40 damage
    let target_active = choices
        .iter()
        .find(|c| {
            matches!(
                &c.action,
                SimpleAction::ApplyDamage { targets, .. } if targets == &vec![(40, 1, 0)]
            )
        })
        .cloned()
        .expect("Expected a choice dealing 40 damage to opponent's active");

    game.apply_action(&target_active);
    let state = game.get_state_clone();

    // Bulbasaur HP 70, took 40 damage → 30 remaining
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        30,
        "Bulbasaur should take 40 damage (2 energy × 20)"
    );
    // Charmander (bench) should be untouched
    assert_eq!(
        state.in_play_pokemon[1][1]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        60,
        "Charmander should be undamaged"
    );
}

/// Energy Arrow deals 0 damage when the target has no energy.
#[test]
fn test_energy_arrow_deals_zero_when_target_has_no_energy() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    // Opponent active has no energy attached.
    state.set_board(
        vec![PlayedCard::from_id(CardId::A3084TapuLele).with_energy(vec![EnergyType::Psychic])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    // One choice targeting the active with 0 energy → 0 damage
    assert_eq!(choices.len(), 1);

    let target = choices[0].clone();
    assert!(matches!(
        &target.action,
        SimpleAction::ApplyDamage { targets, .. } if targets == &vec![(0, 1, 0)]
    ));

    game.apply_action(&target);
    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        70,
        "Bulbasaur should take 0 damage (no energy)"
    );
}
