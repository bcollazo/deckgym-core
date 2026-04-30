use crate::deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

fn played_card_with_base_hp(card_id: CardId, base_hp: u32) -> PlayedCard {
    let card = crate::deckgym::database::get_card_by_enum(card_id);
    PlayedCard::new(card, 0, base_hp, vec![], false, vec![])
}

/// Test Rampardos's Head Smash deals 130 damage without recoil when opponent survives
#[test]
fn test_rampardos_head_smash_no_ko_no_recoil() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up Rampardos vs high-HP Bulbasaur
    state.set_board(
        vec![PlayedCard::from_id(CardId::A2089Rampardos).with_energy(vec![EnergyType::Fighting])],
        vec![played_card_with_base_hp(CardId::A1001Bulbasaur, 200)],
    );
    state.current_player = 0;

    game.set_state(state);

    // Apply Head Smash attack (attack index 0)
    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // Opponent should have 200 - 130 = 70 HP
    let opponent_hp = final_state.get_active(1).get_remaining_hp();
    assert_eq!(
        opponent_hp, 70,
        "Rampardos's Head Smash should deal 130 damage (200 - 130 = 70)"
    );

    // Rampardos should have full HP (no recoil since no KO)
    let rampardos_hp = final_state.get_active(0).get_remaining_hp();
    assert_eq!(
        rampardos_hp, 150,
        "Rampardos should take no recoil damage when opponent survives"
    );
}

/// Test Rampardos's Head Smash deals 50 recoil damage when opponent is KO'd
#[test]
fn test_rampardos_head_smash_ko_with_recoil() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up Rampardos vs low-HP Bulbasaur + bench
    state.set_board(
        vec![PlayedCard::from_id(CardId::A2089Rampardos).with_energy(vec![EnergyType::Fighting])],
        vec![
            played_card_with_base_hp(CardId::A1001Bulbasaur, 100),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );
    state.current_player = 0;
    state.points = [0, 0];

    game.set_state(state);

    // Apply Head Smash attack
    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // Player should have earned 1 point for the KO
    assert_eq!(
        final_state.points[0], 1,
        "Player should earn 1 point for KO'ing opponent's Pokemon"
    );

    // Rampardos should have taken 50 recoil damage (150 - 50 = 100)
    let rampardos_hp = final_state.get_active(0).get_remaining_hp();
    assert_eq!(
        rampardos_hp, 100,
        "Rampardos should take 50 recoil damage after KO'ing opponent (150 - 50 = 100)"
    );
}

/// Test Rampardos can KO itself with recoil damage if HP is low enough
#[test]
fn test_rampardos_head_smash_self_ko_from_recoil() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up low-HP Rampardos + bench vs low-HP Bulbasaur + bench
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A2089Rampardos)
                .with_remaining_hp(30)
                .with_energy(vec![EnergyType::Fighting]),
            PlayedCard::from_id(CardId::A2089Rampardos),
        ],
        vec![
            played_card_with_base_hp(CardId::A1001Bulbasaur, 100),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );
    state.current_player = 0;
    state.points = [0, 0];

    game.set_state(state);

    // Apply Head Smash attack
    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // Test player should earn 1 point for KO'ing opponent
    assert_eq!(
        final_state.points[0], 1,
        "Player should earn 1 point for KO'ing opponent's Pokemon"
    );

    // Opponent should earn 1 point for Rampardos self-KO from recoil
    assert_eq!(
        final_state.points[1], 1,
        "Opponent should earn 1 point when Rampardos KO's itself from recoil"
    );
}

/// Rampardos's Head Smash should resolve Rocky Helmet and recoil before promotions.
#[test]
fn test_rampardos_head_smash_rocky_helmet_promotion_order() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A2089Rampardos)
                .with_remaining_hp(20)
                .with_energy(vec![EnergyType::Fighting]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![
            played_card_with_base_hp(CardId::A1001Bulbasaur, 100)
                .with_tool(get_card_by_enum(CardId::A2148RockyHelmet)),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
    );
    state.current_player = 0;
    state.points = [0, 0];

    game.set_state(state);

    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let state = game.get_state_clone();

    assert_eq!(state.points, [1, 1], "Both players should earn 1 point");
    assert!(
        state.in_play_pokemon[0][0].is_none(),
        "Rampardos should be KO'd"
    );
    assert!(
        state.in_play_pokemon[1][0].is_none(),
        "Defending active should be KO'd"
    );

    let (actor, choices) = state.generate_possible_actions();
    assert!(actor == 0 || actor == 1);
    assert!(choices.iter().all(|choice| {
        matches!(
            choice.action,
            SimpleAction::Activate {
                player: _,
                in_play_idx: _
            }
        )
    }));

    let first_player = actor;
    let first_promotion = choices[0].clone();
    game.apply_action(&first_promotion);

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, (first_player + 1) % 2);
    assert!(choices.iter().all(|choice| {
        matches!(
            choice.action,
            SimpleAction::Activate {
                player: _,
                in_play_idx: _
            }
        )
    }));
}
