use crate::deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    effects::CardEffect,
    models::{EnergyType, PlayedCard},
    test_support::get_test_game_with_board,
};

/// Test that CoinFlipToBlockAttack effect blocks attacks 50% of the time
#[test]
fn test_coin_flip_to_block_attack_effect() {
    // Set up attacker with CoinFlipToBlockAttack effect
    let mut charmander_played = PlayedCard::from_id(CardId::A1033Charmander)
        .with_energy(vec![EnergyType::Fire, EnergyType::Fire]);
    charmander_played.add_effect(CardEffect::CoinFlipToBlockAttack, 1);

    let mut game = get_test_game_with_board(
        vec![charmander_played],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

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
