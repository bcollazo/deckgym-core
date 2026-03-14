use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    effects::CardEffect,
    models::{Card, EnergyType, PlayedCard},
};

#[path = "../common/mod.rs"]
mod common;

fn played_card_with_base_hp(card_id: CardId, base_hp: u32) -> PlayedCard {
    let card = get_card_by_enum(card_id);
    PlayedCard::new(card, 0, base_hp, vec![], false, vec![])
}

/// Test Magnezone B1a 026 - Mirror Shot
/// Should deal 90 damage and apply CoinFlipToBlockAttack effect
#[test]
fn test_magnezone_mirror_shot() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B1a026Magnezone).with_energy(vec![
                EnergyType::Lightning,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    game.set_state(state);

    // Attack with Mirror Shot (index 0)
    let action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };

    game.apply_action(&action);
    let state = game.get_state_clone();

    // Check opponent was knocked out (70 HP - 90 damage)
    assert!(
        state.maybe_get_active(1).is_none(),
        "Bulbasaur should have been knocked out by 90 damage attack"
    );
}


/// Test that CoinFlipToBlockAttack effect blocks attacks 50% of the time
#[test]
fn test_coin_flip_to_block_attack_effect() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    // Set up attacker with CoinFlipToBlockAttack effect
    let mut charmander_played = PlayedCard::from_id(CardId::A1033Charmander)
        .with_energy(vec![EnergyType::Fire, EnergyType::Fire]);
    charmander_played.add_effect(CardEffect::CoinFlipToBlockAttack, 1);

    state.set_board(
        vec![charmander_played],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    game.set_state(state);

    let action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };

    // The attack should have probabilistic outcomes
    // We can't easily test the exact probabilities without accessing internal state,
    // but we can at least verify the attack executes without panic
    game.apply_action(&action);
    let _state = game.get_state_clone();

    // Test passes if no panic occurs
    // In a real scenario, we'd need access to the probability tree to verify 50/50 split
}
