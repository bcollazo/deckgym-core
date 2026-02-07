use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
};

mod common;

#[test]
fn test_serperior_jungle_totem_ability() {
    // Serperior's Jungle Totem: Each Grass Energy attached to your Grass Pokémon provides 2 Grass Energy
    // Bulbasaur's Vine Whip requires 1 Grass + 1 Colorless (2 total)
    // With Jungle Totem, 1 Grass energy should count as 2, making the attack usable

    // Initialize with basic decks
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up player 0 with Bulbasaur in active position with only 1 Grass energy
    // and Serperior on the bench
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_energy(vec![EnergyType::Grass]),
            PlayedCard::from_id(CardId::A1a006Serperior),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;

    game.set_state(state);

    // Generate possible actions
    let state = game.get_state_clone();
    let (actor, actions) = state.generate_possible_actions();

    // Check if attack action is available
    let has_attack_action = actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::Attack(_)));

    assert_eq!(actor, 0, "Current player should be player 0");
    assert!(
        has_attack_action,
        "With Serperior's Jungle Totem, Bulbasaur should be able to attack with only 1 Grass energy"
    );

    // Verify the specific attack index (should be attack 0 - Vine Whip)
    let attack_actions: Vec<_> = actions
        .iter()
        .filter(|action| matches!(action.action, SimpleAction::Attack(_)))
        .collect();

    assert_eq!(
        attack_actions.len(),
        1,
        "Should have exactly one attack action available"
    );

    if let SimpleAction::Attack(index) = attack_actions[0].action {
        assert_eq!(index, 0, "Attack index should be 0 (Vine Whip)");
    }
}

#[test]
fn test_hydreigon_roar_in_unison_jolteon_active_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::B1081JolteonEx)],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::B1157Hydreigon).with_remaining_hp(100),
        ],
    );
    state.current_player = 1;
    state.turn_count = 3;
    game.set_state(state);

    let ability_action = Action {
        actor: 1,
        action: SimpleAction::UseAbility { in_play_idx: 1 },
        is_stack: false,
    };
    game.apply_action(&ability_action);

    let state = game.get_state_clone();
    let hydreigon = state.in_play_pokemon[1][1]
        .as_ref()
        .expect("Hydreigon should still be in play");
    assert_eq!(hydreigon.get_remaining_hp(), 30);
    assert_eq!(hydreigon.attached_energy.len(), 2);

    // Attach turn energy to the same Hydreigon to confirm a third Electromagnetic Wall trigger.
    let attach_action = Action {
        actor: 1,
        action: SimpleAction::Attach {
            attachments: vec![(1, EnergyType::Darkness, 1)],
            is_turn_energy: true,
        },
        is_stack: false,
    };
    game.apply_action(&attach_action);

    let state = game.get_state_clone();
    let hydreigon = state.in_play_pokemon[1][1]
        .as_ref()
        .expect("Hydreigon should still be in play");
    assert_eq!(hydreigon.get_remaining_hp(), 10);
}

#[test]
fn test_hydreigon_roar_in_unison_jolteon_ko_no_panic() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::B1081JolteonEx)],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::B1157Hydreigon).with_remaining_hp(20),
        ],
    );
    state.current_player = 1;
    state.turn_count = 3;
    state.points = [0, 0];
    game.set_state(state);

    let ability_action = Action {
        actor: 1,
        action: SimpleAction::UseAbility { in_play_idx: 1 },
        is_stack: false,
    };
    game.apply_action(&ability_action);

    let state = game.get_state_clone();
    assert!(state.in_play_pokemon[1][1].is_none());
    assert_eq!(state.points[0], 1);
}

#[test]
fn test_giratina_ex_ability_end_turn_panics_if_ko_by_jolteon() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::B1081JolteonEx)],
        vec![
            PlayedCard::from_id(CardId::A2b035GiratinaEx).with_remaining_hp(10),
            PlayedCard::from_id(CardId::A2b035GiratinaEx),
            PlayedCard::from_id(CardId::A2110DarkraiEx),
            PlayedCard::from_id(CardId::A2110DarkraiEx),
        ],
    );
    state.current_player = 1;
    state.turn_count = 13;
    game.set_state(state);

    let ability_action = Action {
        actor: 1,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    };
    game.apply_action(&ability_action);

    let state = game.get_state_clone();
    let (_actor, actions) = state.generate_possible_actions();
    let end_turn_action = actions
        .iter()
        .find(|action| matches!(action.action, SimpleAction::EndTurn))
        .expect("Expected EndTurn action");

    // This should panic because the active slot is empty after the KO.
    game.apply_action(end_turn_action);
}
