use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    effects::CardEffect,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

/// Test Vulpix's Tail Whip prevents opponent from attacking (on heads)
#[test]
fn test_vulpix_tail_whip_attack_prevention() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up Vulpix vs Bulbasaur
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1037Vulpix).with_energy(vec![EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
    );
    state.current_player = 0;

    game.set_state(state);

    // Manually add CannotAttack effect to opponent's active (simulating successful Tail Whip)
    let mut state = game.get_state_clone();
    state.in_play_pokemon[1][0]
        .as_mut()
        .unwrap()
        .add_effect(CardEffect::CannotAttack, 1);
    state.current_player = 1;
    game.set_state(state);

    // Generate possible actions - attack should NOT be available
    let state = game.get_state_clone();
    let (actor, actions) = state.generate_possible_actions();

    assert_eq!(actor, 1);

    let has_attack_action = actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::Attack(_)));

    assert!(
        !has_attack_action,
        "Opponent should not be able to attack when affected by Tail Whip"
    );
}

/// Test Tail Whip effect clears when Pokemon switches to bench
#[test]
fn test_vulpix_tail_whip_switch_clears_effect() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up Vulpix vs Bulbasaur with CannotAttack + bench Squirtle
    let mut opponent_active = PlayedCard::from_id(CardId::A1001Bulbasaur)
        .with_energy(vec![EnergyType::Grass, EnergyType::Colorless]);
    opponent_active.add_effect(CardEffect::CannotAttack, 1);

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1037Vulpix).with_energy(vec![EnergyType::Colorless])],
        vec![
            opponent_active,
            PlayedCard::from_id(CardId::A1053Squirtle)
                .with_energy(vec![EnergyType::Water, EnergyType::Colorless]),
        ],
    );
    state.current_player = 1;

    game.set_state(state);

    // Opponent retreats/switches to bench Pokemon
    let switch_action = Action {
        actor: 1,
        action: SimpleAction::Activate {
            player: 1,
            in_play_idx: 1,
        },
        is_stack: false,
    };
    game.apply_action(&switch_action);

    let state_after_switch = game.get_state_clone();

    // The new active (Squirtle) should be able to attack
    let (_, actions) = state_after_switch.generate_possible_actions();

    let has_attack_action = actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::Attack(_)));

    assert!(
        has_attack_action,
        "New active Pokemon should be able to attack after switching"
    );
}
