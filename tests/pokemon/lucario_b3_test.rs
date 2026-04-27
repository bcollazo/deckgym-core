use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

fn played_card_with_base_hp(card_id: CardId, base_hp: u32) -> PlayedCard {
    let card = deckgym::database::get_card_by_enum(card_id);
    PlayedCard::new(card, 0, base_hp, vec![], false, vec![])
}

/// Test Lucario B3 080's Close Combat deals 90 base damage
#[test]
fn test_lucario_b3_close_combat_base_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::B3080Lucario)
            .with_energy(vec![EnergyType::Fighting, EnergyType::Fighting])],
        vec![played_card_with_base_hp(CardId::A1001Bulbasaur, 150)],
    );
    state.current_player = 0;
    game.set_state(state);

    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();
    let opponent_hp = final_state.get_active(1).get_remaining_hp();

    assert_eq!(
        opponent_hp, 60,
        "Close Combat should deal 90 base damage (150 - 90 = 60)"
    );
}

/// Test Close Combat makes Lucario take +20 more damage from attacks during opponent's next turn
#[test]
fn test_lucario_b3_close_combat_vulnerability() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Player 0: Lucario B3 080 (100 HP) with 2F energy
    // Player 1: Bulbasaur with 150 HP + Grass+Colorless energy (for Vine Whip = 40 damage)
    state.set_board(
        vec![PlayedCard::from_id(CardId::B3080Lucario)
            .with_energy(vec![EnergyType::Fighting, EnergyType::Fighting])],
        vec![played_card_with_base_hp(CardId::A1001Bulbasaur, 150)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
    );
    state.current_player = 0;
    game.set_state(state);

    // Player 0 uses Close Combat (90 damage to Bulbasaur, leaves vulnerability on Lucario)
    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    // Player 0 ends turn
    let end_turn = Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    };
    game.apply_action(&end_turn);

    let state = game.get_state_clone();
    let lucario_hp_before = state.get_active(0).get_remaining_hp();

    // Player 1 uses Vine Whip (40 base damage)
    // Lucario's Close Combat vulnerability adds +20 → 60 total damage taken
    let vine_whip = Action {
        actor: 1,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&vine_whip);

    let final_state = game.get_state_clone();
    let lucario_hp_after = final_state.get_active(0).get_remaining_hp();
    let damage_taken = lucario_hp_before - lucario_hp_after;

    assert_eq!(
        damage_taken, 60,
        "Vine Whip (40) should deal 60 damage to Lucario due to Close Combat's +20 vulnerability"
    );
}

/// Test Mega Lucario ex Fighting Pulse deals 90 base damage (no extra energy)
#[test]
fn test_mega_lucario_ex_fighting_pulse_base_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::B3081MegaLucarioEx)
            .with_energy(vec![EnergyType::Fighting, EnergyType::Fighting])],
        vec![played_card_with_base_hp(CardId::A1001Bulbasaur, 150)],
    );
    state.current_player = 0;
    game.set_state(state);

    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();
    let opponent_hp = final_state.get_active(1).get_remaining_hp();

    assert_eq!(
        opponent_hp, 60,
        "Fighting Pulse should deal 90 base damage with no extra energy (150 - 90 = 60)"
    );
}

/// Test Mega Lucario ex Fighting Pulse deals 140 damage with 1 extra Fighting energy
#[test]
fn test_mega_lucario_ex_fighting_pulse_boosted_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B3081MegaLucarioEx).with_energy(vec![
                EnergyType::Fighting,
                EnergyType::Fighting,
                EnergyType::Fighting,
            ]),
        ],
        vec![played_card_with_base_hp(CardId::A1001Bulbasaur, 200)],
    );
    state.current_player = 0;
    game.set_state(state);

    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();
    let opponent_hp = final_state.get_active(1).get_remaining_hp();

    assert_eq!(
        opponent_hp, 60,
        "Fighting Pulse should deal 140 damage with 1 extra Fighting energy (200 - 140 = 60)"
    );
}
