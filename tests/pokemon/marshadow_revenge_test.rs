use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    effects::CardEffect,
    models::{EnergyType, PlayedCard},
};

#[path = "../common/mod.rs"]
mod common;

fn played_card_with_base_hp(card_id: CardId, base_hp: u32) -> PlayedCard {
    let card = deckgym::database::get_card_by_enum(card_id);
    PlayedCard::new(card, 0, base_hp, vec![], false, vec![])
}

/// Test Marshadow's Revenge attack base damage (40) when no KO happened last turn
#[test]
fn test_marshadow_revenge_base_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up Marshadow vs Bulbasaur
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1a047Marshadow)
            .with_energy(vec![EnergyType::Fighting, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;

    // Ensure no KO happened last turn
    state.set_knocked_out_by_opponent_attack_last_turn(false);

    game.set_state(state);

    // Apply Revenge attack (attack index 0)
    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // Base damage is 40, so opponent should have 70 - 40 = 30 HP
    let opponent_hp = final_state.get_active(1).get_remaining_hp();

    assert_eq!(
        opponent_hp, 30,
        "Marshadow's Revenge should deal 40 damage without KO bonus (70 - 40 = 30)"
    );
}


/// Test Marshadow's Revenge attack boosted damage (40 + 60 = 100) when KO happened last turn
#[test]
fn test_marshadow_revenge_boosted_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up Marshadow vs high-HP Bulbasaur
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1a047Marshadow)
            .with_energy(vec![EnergyType::Fighting, EnergyType::Colorless])],
        vec![played_card_with_base_hp(CardId::A1001Bulbasaur, 150)],
    );
    state.current_player = 0;

    // Simulate that a Pokemon was KO'd by opponent's attack last turn
    state.set_knocked_out_by_opponent_attack_last_turn(true);

    game.set_state(state);

    // Apply Revenge attack
    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let final_state = game.get_state_clone();

    // Boosted damage is 40 + 60 = 100, so opponent should have 150 - 100 = 50 HP
    let opponent_hp = final_state.get_active(1).get_remaining_hp();

    assert_eq!(
        opponent_hp, 50,
        "Marshadow's Revenge should deal 100 damage with KO bonus (150 - 100 = 50)"
    );
}
