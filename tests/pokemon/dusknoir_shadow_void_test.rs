use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::PlayedCard,
    test_support::get_initialized_game,
};

/// Test Dusknoir's Shadow Void ability moving damage correctly
#[test]
fn test_dusknoir_shadow_void_move_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up Bulbasaur active with damage + Dusknoir on bench
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_damage(40),
            PlayedCard::from_id(CardId::A2072Dusknoir),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;

    game.set_state(state);

    // Use Dusknoir's Shadow Void ability
    let ability_action = Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 1 },
        is_stack: false,
    };
    game.apply_action(&ability_action);

    // The ability should queue a move generation for selecting which Pokemon's damage to move
    let state = game.get_state_clone();
    let (_actor, actions) = state.generate_possible_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a.action, SimpleAction::MoveAllDamage { .. })),
        "Shadow Void should queue a move generation for selecting damage source"
    );

    // Select to move damage from Bulbasaur (index 0) to Dusknoir (index 1)
    let move_damage_action = Action {
        actor: 0,
        action: SimpleAction::MoveAllDamage { from: 0, to: 1 },
        is_stack: false,
    };
    game.apply_action(&move_damage_action);

    let final_state = game.get_state_clone();

    // Bulbasaur should now have full HP (70)
    let bulbasaur_hp = final_state.get_active(0).get_remaining_hp();
    assert_eq!(
        bulbasaur_hp, 70,
        "Bulbasaur should be fully healed after Shadow Void (70 HP)"
    );

    // Dusknoir should have taken the 40 damage (130 - 40 = 90 HP)
    let dusknoir_hp = final_state
        .enumerate_bench_pokemon(0)
        .next()
        .unwrap()
        .1
        .get_remaining_hp();
    assert_eq!(
        dusknoir_hp, 90,
        "Dusknoir should have 90 HP after receiving 40 damage (130 - 40)"
    );
}

/// Test Dusknoir's Shadow Void causing KO and awarding points to opponent
#[test]
fn test_dusknoir_shadow_void_ko() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up Bulbasaur active with damage + low-HP Dusknoir on bench
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_damage(50),
            PlayedCard::from_id(CardId::A2072Dusknoir).with_remaining_hp(30),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.points = [0, 0];

    game.set_state(state);

    // Use Dusknoir's Shadow Void ability
    let ability_action = Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 1 },
        is_stack: false,
    };
    game.apply_action(&ability_action);

    // Select to move damage from Bulbasaur to Dusknoir
    let move_damage_action = Action {
        actor: 0,
        action: SimpleAction::MoveAllDamage { from: 0, to: 1 },
        is_stack: false,
    };
    game.apply_action(&move_damage_action);

    let final_state = game.get_state_clone();

    // Dusknoir should be KO'd (removed from play)
    assert!(
        final_state.enumerate_bench_pokemon(0).next().is_none(),
        "Dusknoir should be KO'd after receiving lethal damage"
    );

    // Opponent should receive 1 point for KO'ing a non-ex Pokemon
    assert_eq!(
        final_state.points[1], 1,
        "Opponent should receive 1 point for KO'ing Dusknoir"
    );
}

/// Test Dusknoir's Shadow Void can be used multiple times per turn
#[test]
fn test_dusknoir_shadow_void_multiple_uses() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up Bulbasaur active with damage, Dusknoir on bench, Squirtle with damage
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_damage(20),
            PlayedCard::from_id(CardId::A2072Dusknoir),
            PlayedCard::from_id(CardId::A1053Squirtle).with_damage(20),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;

    game.set_state(state);

    // First use: Move damage from Bulbasaur
    let ability_action = Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 1 },
        is_stack: false,
    };
    game.apply_action(&ability_action);

    let move_damage_action = Action {
        actor: 0,
        action: SimpleAction::MoveAllDamage { from: 0, to: 1 },
        is_stack: false,
    };
    game.apply_action(&move_damage_action);

    // Second use: Move damage from Squirtle
    let ability_action2 = Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 1 },
        is_stack: false,
    };
    game.apply_action(&ability_action2);

    let move_damage_action2 = Action {
        actor: 0,
        action: SimpleAction::MoveAllDamage { from: 2, to: 1 },
        is_stack: false,
    };
    game.apply_action(&move_damage_action2);

    let final_state = game.get_state_clone();

    // Bulbasaur should be fully healed
    let bulbasaur_hp = final_state.get_active(0).get_remaining_hp();
    assert_eq!(bulbasaur_hp, 70, "Bulbasaur should be fully healed");

    // Squirtle should be fully healed
    let squirtle_hp = final_state
        .enumerate_in_play_pokemon(0)
        .find(|(i, _)| *i == 2)
        .unwrap()
        .1
        .get_remaining_hp();
    assert_eq!(squirtle_hp, 60, "Squirtle should be fully healed");

    // Dusknoir should have taken both damages (130 - 20 - 20 = 90 HP)
    let dusknoir_hp = final_state
        .enumerate_bench_pokemon(0)
        .find(|(_, p)| p.get_name() == "Dusknoir")
        .unwrap()
        .1
        .get_remaining_hp();
    assert_eq!(
        dusknoir_hp, 90,
        "Dusknoir should have 90 HP after receiving 40 total damage"
    );
}
