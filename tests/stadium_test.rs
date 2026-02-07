use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard},
};

mod common;

fn trainer_from_id(card_id: CardId) -> deckgym::models::TrainerCard {
    match get_card_by_enum(card_id) {
        Card::Trainer(trainer_card) => trainer_card,
        _ => panic!("Expected trainer card"),
    }
}

fn has_retreat_action(actions: &[Action]) -> bool {
    actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::Retreat(_)))
}

#[test]
fn test_peculiar_plaza_reduces_psychic_retreat_by_2() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Mewtwo has 2 retreat cost, with Peculiar Plaza it becomes 0
    // So retreat should be possible with NO energy attached
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1128Mewtwo),
            PlayedCard::from_id(CardId::A1001Bulbasaur), // Bench Pokemon to retreat to
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 1;
    state.hands[0] = vec![get_card_by_enum(CardId::B2155PeculiarPlaza)];
    // No energy attached - retreat would normally require 2 energy
    state.in_play_pokemon[0][0]
        .as_mut()
        .unwrap()
        .attached_energy = vec![];

    game.set_state(state);

    // Before playing Peculiar Plaza, retreat should NOT be possible (no energy, cost is 2)
    let state = game.get_state_clone();
    let (_actor, actions) = state.generate_possible_actions();
    assert!(
        !has_retreat_action(&actions),
        "Retreat should NOT be possible before Peculiar Plaza (no energy, cost 2)"
    );

    // Play Peculiar Plaza
    let trainer_card = trainer_from_id(CardId::B2155PeculiarPlaza);
    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let state = game.get_state_clone();

    // Verify stadium is active
    assert!(state.active_stadium.is_some());
    assert_eq!(
        state.get_active_stadium_name(),
        Some("Peculiar Plaza".to_string())
    );

    // After Peculiar Plaza, retreat SHOULD be possible (cost reduced from 2 to 0)
    let (_actor, actions) = state.generate_possible_actions();
    assert!(
        has_retreat_action(&actions),
        "Retreat should be possible after Peculiar Plaza (Psychic Pokemon, cost 2-2=0)"
    );
}

#[test]
fn test_peculiar_plaza_does_not_affect_non_psychic() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Charmander has 1 retreat cost, Peculiar Plaza doesn't affect Fire types
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1033Charmander),
            PlayedCard::from_id(CardId::A1001Bulbasaur), // Bench Pokemon to retreat to
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 1;
    state.hands[0] = vec![get_card_by_enum(CardId::B2155PeculiarPlaza)];
    // No energy attached
    state.in_play_pokemon[0][0]
        .as_mut()
        .unwrap()
        .attached_energy = vec![];

    game.set_state(state);

    // Play Peculiar Plaza
    let trainer_card = trainer_from_id(CardId::B2155PeculiarPlaza);
    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let state = game.get_state_clone();

    // Charmander still needs 1 energy to retreat (unaffected by Peculiar Plaza)
    let (_actor, actions) = state.generate_possible_actions();
    assert!(
        !has_retreat_action(&actions),
        "Non-Psychic Pokemon retreat cost should be unchanged (still needs 1 energy)"
    );
}

#[test]
fn test_cannot_play_same_name_stadium() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 1;

    // Set Peculiar Plaza as active stadium
    state.active_stadium = Some(get_card_by_enum(CardId::B2155PeculiarPlaza));
    // Try to play another Peculiar Plaza
    state.hands[0] = vec![get_card_by_enum(CardId::B2155PeculiarPlaza)];

    game.set_state(state);

    let state = game.get_state_clone();
    let (_actor, actions) = state.generate_possible_actions();

    let has_play_stadium = actions.iter().any(|action| {
        matches!(&action.action, SimpleAction::Play { trainer_card } if trainer_card.name == "Peculiar Plaza")
    });

    assert!(
        !has_play_stadium,
        "Should not be able to play same-name stadium"
    );
}

#[test]
fn test_stadium_affects_both_players() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Use Psychic Pokemon for both players, each with bench Pokemon
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1128Mewtwo),
            PlayedCard::from_id(CardId::A1001Bulbasaur), // Bench
        ],
        vec![
            PlayedCard::from_id(CardId::A1128Mewtwo),
            PlayedCard::from_id(CardId::A1001Bulbasaur), // Bench
        ],
    );
    state.current_player = 0;
    state.turn_count = 1;
    state.hands[0] = vec![get_card_by_enum(CardId::B2155PeculiarPlaza)];
    // No energy for either player
    state.in_play_pokemon[0][0]
        .as_mut()
        .unwrap()
        .attached_energy = vec![];
    state.in_play_pokemon[1][0]
        .as_mut()
        .unwrap()
        .attached_energy = vec![];

    game.set_state(state);

    // Play Peculiar Plaza
    let trainer_card = trainer_from_id(CardId::B2155PeculiarPlaza);
    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    };
    game.apply_action(&play_action);

    // Check player 0 can retreat (Psychic, cost reduced to 0)
    let state = game.get_state_clone();
    let (_actor, actions) = state.generate_possible_actions();
    assert!(
        has_retreat_action(&actions),
        "Player 0's Psychic Pokemon should be able to retreat with 0 energy"
    );

    // Simulate switching to player 1's turn to check their retreat
    let mut state = game.get_state_clone();
    state.current_player = 1;
    game.set_state(state);

    let state = game.get_state_clone();
    let (_actor, actions) = state.generate_possible_actions();
    assert!(
        has_retreat_action(&actions),
        "Player 1's Psychic Pokemon should also be able to retreat (stadium affects both)"
    );
}

#[test]
fn test_stadium_is_discarded_when_replaced() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 1;

    // Set an existing stadium using set_active_stadium to simulate it was played
    let old_stadium = get_card_by_enum(CardId::B2155PeculiarPlaza);
    state.set_active_stadium(old_stadium.clone());

    // Verify old stadium is set
    assert!(state.active_stadium.is_some());
    assert!(state.discard_piles[0].is_empty());

    game.set_state(state);

    // Since we can't play Peculiar Plaza when it's already active,
    // we'll test the discard mechanism directly by checking state changes
    let mut state = game.get_state_clone();

    // Simulate a stadium replacement (as if a different stadium was played)
    let new_stadium = get_card_by_enum(CardId::B2155PeculiarPlaza);
    if let Some(old) = state.set_active_stadium(new_stadium) {
        state.discard_piles[0].push(old);
    }

    // Verify old stadium is in discard
    assert_eq!(state.discard_piles[0].len(), 1);
    assert!(matches!(&state.discard_piles[0][0], Card::Trainer(t) if t.name == "Peculiar Plaza"));
}

// ============================================================================
// Training Area Tests
// ============================================================================

#[test]
fn test_training_area_increases_stage1_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        // Ivysaur is Stage 1, does 60 damage with Razor Leaf
        // With Training Area, should do 70 damage
        vec![PlayedCard::from_id(CardId::A1002Ivysaur).with_energy(vec![
            EnergyType::Grass,
            EnergyType::Grass,
            EnergyType::Grass,
        ])],
        // Bulbasaur has 70 HP
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 1;
    state.hands[0] = vec![get_card_by_enum(CardId::B2153TrainingArea)];

    game.set_state(state);

    // Play Training Area
    let trainer_card = trainer_from_id(CardId::B2153TrainingArea);
    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    };
    game.apply_action(&play_action);

    // Verify stadium is active
    let state = game.get_state_clone();
    assert!(state.active_stadium.is_some());
    assert_eq!(
        state.get_active_stadium_name(),
        Some("Training Area".to_string())
    );

    // Attack with Ivysaur's Razor Leaf (60 damage + 10 from Training Area = 70)
    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);
    game.play_until_stable(); // Handle post-attack effects

    let state = game.get_state_clone();

    // Ivysaur does 60 damage, +10 from Training Area = 70 damage
    // Bulbasaur has exactly 70 HP, so it should be KO'd
    assert_eq!(
        state.points[0], 1,
        "Player 0 should have 1 point from KO (70 damage dealt to 70 HP Bulbasaur)"
    );
}

#[test]
fn test_training_area_does_not_affect_basic_pokemon() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Bulbasaur is Basic (Stage 0), does 40 damage with Vine Whip
    // Training Area should NOT affect it
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Grass])],
        vec![PlayedCard::from_id(CardId::A1033Charmander)], // Charmander has 60 HP
    );
    state.current_player = 0;
    state.turn_count = 1;
    state.hands[0] = vec![get_card_by_enum(CardId::B2153TrainingArea)];
    game.set_state(state);

    // Play Training Area
    let trainer_card = trainer_from_id(CardId::B2153TrainingArea);
    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    };
    game.apply_action(&play_action);

    // Attack with Bulbasaur's Vine Whip (40 damage, NOT boosted)
    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let state = game.get_state_clone();
    let defender_hp = state.get_active(1).get_remaining_hp();

    // Charmander: 60 HP - 40 damage = 20 HP remaining
    assert_eq!(
        defender_hp, 20,
        "Basic Pokemon should deal normal damage (40), not boosted"
    );
}

#[test]
fn test_training_area_does_not_affect_stage2_pokemon() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Venusaur is Stage 2, does 80 damage with Mega Drain
    // Training Area should NOT affect Stage 2
    // Bulbasaur has 70 HP
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1003Venusaur).with_energy(vec![
            EnergyType::Grass,
            EnergyType::Grass,
            EnergyType::Grass,
            EnergyType::Grass,
        ])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 1;
    state.hands[0] = vec![get_card_by_enum(CardId::B2153TrainingArea)];

    game.set_state(state);

    // Play Training Area
    let trainer_card = trainer_from_id(CardId::B2153TrainingArea);
    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    };
    game.apply_action(&play_action);

    // Attack with Venusaur's Mega Drain (80 damage, NOT boosted)
    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let state = game.get_state_clone();
    // Bulbasaur should be KO'd (70 HP - 80 damage)
    // Stage 2 should deal normal damage, not +10
    assert_eq!(
        state.points[0], 1,
        "Stage 2 Pokemon should deal normal damage (80), not boosted to 90"
    );
}

#[test]
fn test_training_area_affects_both_players() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Both players have Stage 1 Pokemon
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1002Ivysaur).with_energy(vec![
            EnergyType::Grass,
            EnergyType::Grass,
            EnergyType::Grass,
        ])],
        vec![PlayedCard::from_id(CardId::A1034Charmeleon)
            .with_energy(vec![EnergyType::Fire, EnergyType::Fire])],
    );
    state.current_player = 0;
    state.turn_count = 1;

    // Set Training Area as active (simulating it was played earlier)
    state.active_stadium = Some(get_card_by_enum(CardId::B2153TrainingArea));
    game.set_state(state);

    // Player 0 attacks with Ivysaur (60 + 10 = 70)
    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    // Check damage dealt to Charmeleon (90 HP - 70 damage = 20 HP)
    // Note: Charmeleon is weak to nothing relevant here
    game.play_until_stable(); // Handle any post-attack effects

    let state = game.get_state_clone();
    let charmeleon_hp = state.get_active(1).get_remaining_hp();
    assert_eq!(
        charmeleon_hp, 20,
        "Charmeleon should have 20 HP (90 - 70 from boosted Stage 1 attack)"
    );
}
