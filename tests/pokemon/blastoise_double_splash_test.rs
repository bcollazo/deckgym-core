use crate::deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

fn played_card_with_base_hp(card_id: CardId, base_hp: u32) -> PlayedCard {
    let card = get_card_by_enum(card_id);
    PlayedCard::new(card, 0, base_hp, vec![], false, vec![])
}

/// Test Blastoise B1a 019 - Double Splash with extra energy
/// Should deal 90 to active and 50 to 1 benched Pokemon when 2+ extra Water energies attached
#[test]
fn test_blastoise_double_splash_with_extra_energy() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B1a019Blastoise).with_energy(vec![
                EnergyType::Water,
                EnergyType::Water,
                EnergyType::Water,
                EnergyType::Water,
                EnergyType::Water,
            ]),
        ],
        vec![
            played_card_with_base_hp(CardId::A1001Bulbasaur, 150),
            PlayedCard::from_id(CardId::A1053Squirtle),
        ],
    );

    game.set_state(state);

    // Attack with Double Splash
    let action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };

    game.apply_action(&action);
    let state = game.get_state_clone();

    // Check that a stack action is queued with bench target choices
    let (actor, choices) = state.generate_possible_actions();
    assert_eq!(actor, 0, "Actor should be player 0");
    assert!(
        choices
            .iter()
            .any(|a| matches!(a.action, SimpleAction::ApplyDamage { .. })),
        "Expected bench target ApplyDamage choices"
    );

    // Apply the first choice (damage to bench position 1)
    game.apply_action(&choices[0]);
    let state = game.get_state_clone();

    // Verify active took 90 damage (150 - 90 = 60)
    let opponent_active = state.get_active(1);
    assert_eq!(
        opponent_active.get_remaining_hp(),
        60,
        "Opponent active should have 60 HP remaining (150 - 90)"
    );

    // Verify bench took 50 damage (60 - 50 = 10 HP remaining)
    let opponent_bench = state.enumerate_bench_pokemon(1).next();
    assert!(
        opponent_bench.is_some(),
        "Opponent bench Pokemon should still be alive (60 - 50 = 10)"
    );
    assert_eq!(
        opponent_bench.unwrap().1.get_remaining_hp(),
        10,
        "Opponent bench Pokemon should have 10 HP remaining (60 - 50)"
    );
}

/// Test Blastoise B1a 019 - Double Splash without extra energy
/// Should deal only 90 to active when no extra energies
#[test]
fn test_blastoise_double_splash_without_extra_energy() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B1a019Blastoise).with_energy(vec![
                EnergyType::Water,
                EnergyType::Water,
                EnergyType::Fire,
            ]),
        ],
        vec![
            played_card_with_base_hp(CardId::A1001Bulbasaur, 150),
            PlayedCard::from_id(CardId::A1053Squirtle),
        ],
    );

    game.set_state(state);

    // Attack with Double Splash
    let action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };

    game.apply_action(&action);
    let state = game.get_state_clone();

    // Check that there are NO stacked ApplyDamage actions (no bench damage)
    let (_actor, actions) = state.generate_possible_actions();
    let has_apply_damage = actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::ApplyDamage { .. }));
    assert!(
        !has_apply_damage,
        "Move generation stack should have no ApplyDamage actions (no extra energy for bench damage)"
    );

    // Verify active took 90 damage (150 - 90 = 60)
    let opponent_active = state.get_active(1);
    assert_eq!(
        opponent_active.get_remaining_hp(),
        60,
        "Opponent active should have 60 HP remaining (150 - 90)"
    );

    // Verify bench took NO damage (still at 60 HP)
    let opponent_bench = state.enumerate_bench_pokemon(1).next();
    assert!(
        opponent_bench.is_some(),
        "Opponent bench Pokemon should still be alive"
    );
    assert_eq!(
        opponent_bench.unwrap().1.get_remaining_hp(),
        60,
        "Opponent bench should still have 60 HP (no bench damage without extra energy)"
    );
}

/// Test Blastoise B1a 019 - Double Splash with extra energy but no bench
/// Should deal 90 to active only (no bench to hit)
#[test]
fn test_blastoise_double_splash_with_extra_energy_no_bench() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B1a019Blastoise).with_energy(vec![
                EnergyType::Water,
                EnergyType::Water,
                EnergyType::Water,
                EnergyType::Water,
                EnergyType::Water,
            ]),
        ],
        vec![played_card_with_base_hp(CardId::A1001Bulbasaur, 150)],
    );

    game.set_state(state);

    // Attack with Double Splash
    let action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };

    game.apply_action(&action);
    let state = game.get_state_clone();

    // Check that there are NO stacked ApplyDamage actions (no bench to hit)
    let (_actor, actions) = state.generate_possible_actions();
    let has_apply_damage = actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::ApplyDamage { .. }));
    assert!(
        !has_apply_damage,
        "Move generation stack should have no ApplyDamage actions (no bench Pokemon to hit)"
    );

    // Verify active took 90 damage (150 - 90 = 60)
    let opponent_active = state.get_active(1);
    assert_eq!(
        opponent_active.get_remaining_hp(),
        60,
        "Opponent active should have 60 HP remaining (150 - 90), even with extra energy but no bench"
    );
}
