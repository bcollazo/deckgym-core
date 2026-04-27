use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

/// Test Bonsly's Teary Attack deals 10 damage to the opponent
#[test]
fn test_bonsly_teary_attack_deals_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::B3078Bonsly)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
    );
    state.current_player = 0;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let bulbasaur_hp = state.get_active(1).get_remaining_hp();
    assert_eq!(
        bulbasaur_hp, 60,
        "Bulbasaur should take 10 damage from Teary Attack (70 - 10 = 60)"
    );
}

/// Test Bonsly's Teary Attack reduces opponent's next attack by 30
#[test]
fn test_bonsly_teary_attack_reduces_opponent_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Bonsly (30 HP) vs Bulbasaur (70 HP, Vine Whip = 40 damage)
    state.set_board(
        vec![PlayedCard::from_id(CardId::B3078Bonsly)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
    );
    state.current_player = 0;
    game.set_state(state);

    // Bonsly uses Teary Attack (no energy required): 10 damage + -30 effect on opponent
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    // End Bonsly's turn
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });

    // Bulbasaur attacks with Vine Whip (normally 40 damage, should be 40-30=10 with effect)
    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let bonsly_hp = state.get_active(0).get_remaining_hp();
    // Bonsly has 30 HP, should take only 10 damage (40 - 30) = 20 HP remaining
    assert_eq!(
        bonsly_hp, 20,
        "Bonsly should take only 10 damage (40 - 30 reduction) and have 20 HP remaining"
    );
}
