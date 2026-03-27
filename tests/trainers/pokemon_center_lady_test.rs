use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{Card, PlayedCard, StatusCondition, TrainerCard},
    test_support::get_initialized_game,
};

#[test]
fn test_pokemon_center_lady_heals_30_damage() {
    // Arrange: Create a game with damaged pokemon
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    // Setup: Put a damaged Bulbasaur in active spot
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur).with_remaining_hp(20)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    // Add Pokemon Center Lady to hand
    let pokemon_center_lady = Card::Trainer(TrainerCard {
        id: "A2b 070".to_string(),
        trainer_card_type: deckgym::models::TrainerType::Supporter,
        name: "Pokémon Center Lady".to_string(),
        effect:
            "Heal 30 damage from 1 of your Pokémon, and it recovers from all Special Conditions."
                .to_string(),
        rarity: "◊◊".to_string(),
        booster_pack: "Shining Revelry (A2b)".to_string(),
    });
    state.hands[0].push(pokemon_center_lady.clone());
    game.set_state(state);

    // Verify initial state
    let state = game.get_state_clone();
    let bulbasaur_before = state.get_active(0);
    assert_eq!(
        bulbasaur_before.get_remaining_hp(),
        20,
        "Bulbasaur should have 20 HP (70 - 50 damage)"
    );

    // Act: Play Pokemon Center Lady
    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: match pokemon_center_lady {
                Card::Trainer(tc) => tc,
                _ => panic!("Expected trainer card"),
            },
        },
        is_stack: false,
    };
    game.apply_action(&play_action);

    // Choose to heal Bulbasaur (index 0)
    let state = game.get_state_clone();
    let (_actor, actions) = state.generate_possible_actions();
    let heal_action = actions
        .iter()
        .find(|a| matches!(a.action, SimpleAction::Heal { in_play_idx: 0, .. }))
        .expect("Should have heal action for Bulbasaur");
    game.apply_action(heal_action);

    // Assert: Bulbasaur should be healed by 30
    let state = game.get_state_clone();
    let bulbasaur_after = state.get_active(0);
    assert_eq!(
        bulbasaur_after.get_remaining_hp(),
        50,
        "Bulbasaur should be healed to 50 HP (20 + 30)"
    );
}

#[test]
fn test_pokemon_center_lady_cures_poisoned() {
    // Arrange: Create a game with poisoned pokemon
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    // Setup: Put a poisoned Bulbasaur in active spot
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.apply_status_condition(0, 0, StatusCondition::Poisoned);

    // Add Pokemon Center Lady to hand
    let pokemon_center_lady = Card::Trainer(TrainerCard {
        id: "A2b 070".to_string(),
        trainer_card_type: deckgym::models::TrainerType::Supporter,
        name: "Pokémon Center Lady".to_string(),
        effect:
            "Heal 30 damage from 1 of your Pokémon, and it recovers from all Special Conditions."
                .to_string(),
        rarity: "◊◊".to_string(),
        booster_pack: "Shining Revelry (A2b)".to_string(),
    });
    state.hands[0].push(pokemon_center_lady.clone());
    game.set_state(state);

    // Verify initial state
    let state = game.get_state_clone();
    let bulbasaur_before = state.get_active(0);
    assert!(
        bulbasaur_before.is_poisoned(),
        "Bulbasaur should be poisoned initially"
    );

    // Act: Play Pokemon Center Lady and choose Bulbasaur
    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: match pokemon_center_lady {
                Card::Trainer(tc) => tc,
                _ => panic!("Expected trainer card"),
            },
        },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let state = game.get_state_clone();
    let (_actor, actions) = state.generate_possible_actions();
    let heal_action = actions
        .iter()
        .find(|a| matches!(a.action, SimpleAction::Heal { in_play_idx: 0, .. }))
        .expect("Should have heal action");
    game.apply_action(heal_action);

    // Assert: Bulbasaur should no longer be poisoned
    let state = game.get_state_clone();
    let bulbasaur_after = state.get_active(0);
    assert!(
        !bulbasaur_after.is_poisoned(),
        "Bulbasaur should no longer be poisoned"
    );
}

#[test]
fn test_pokemon_center_lady_heals_and_cures_together() {
    // Arrange: Create a game with damaged + poisoned pokemon
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    // Setup: Put a damaged and poisoned Bulbasaur in active spot
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur).with_remaining_hp(30)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.apply_status_condition(0, 0, StatusCondition::Poisoned);
    state.apply_status_condition(0, 0, StatusCondition::Paralyzed);
    state.apply_status_condition(0, 0, StatusCondition::Asleep);

    // Add Pokemon Center Lady to hand
    let pokemon_center_lady = Card::Trainer(TrainerCard {
        id: "A2b 089".to_string(),
        trainer_card_type: deckgym::models::TrainerType::Supporter,
        name: "Pokémon Center Lady".to_string(),
        effect:
            "Heal 30 damage from 1 of your Pokémon, and it recovers from all Special Conditions."
                .to_string(),
        rarity: "☆☆".to_string(),
        booster_pack: "Shining Revelry (A2b)".to_string(),
    });
    state.hands[0].push(pokemon_center_lady.clone());
    game.set_state(state);

    // Verify initial state
    let state = game.get_state_clone();
    let bulbasaur_before = state.get_active(0);
    assert_eq!(
        bulbasaur_before.get_remaining_hp(),
        30,
        "Bulbasaur should have 30 HP"
    );
    assert!(bulbasaur_before.is_poisoned(), "Should be poisoned");
    assert!(bulbasaur_before.is_paralyzed(), "Should be paralyzed");
    assert!(bulbasaur_before.is_asleep(), "Should be asleep");

    // Act: Play Pokemon Center Lady and choose Bulbasaur
    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: match pokemon_center_lady {
                Card::Trainer(tc) => tc,
                _ => panic!("Expected trainer card"),
            },
        },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let state = game.get_state_clone();
    let (_actor, actions) = state.generate_possible_actions();
    let heal_action = actions
        .iter()
        .find(|a| matches!(a.action, SimpleAction::Heal { in_play_idx: 0, .. }))
        .expect("Should have heal action");
    game.apply_action(heal_action);

    // Assert: Bulbasaur should be healed and cured
    let state = game.get_state_clone();
    let bulbasaur_after = state.get_active(0);
    assert_eq!(
        bulbasaur_after.get_remaining_hp(),
        60,
        "Bulbasaur should be healed to 60 HP (30 + 30)"
    );
    assert!(
        !bulbasaur_after.is_poisoned(),
        "Bulbasaur should no longer be poisoned"
    );
    assert!(
        !bulbasaur_after.is_paralyzed(),
        "Bulbasaur should no longer be paralyzed"
    );
    assert!(
        !bulbasaur_after.is_asleep(),
        "Bulbasaur should no longer be asleep"
    );
}
