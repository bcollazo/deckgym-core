use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

fn lapras_game(knocked_out_last_turn: bool) -> deckgym::Game<'static> {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2b017Lapras).with_energy(vec![
            EnergyType::Water,
            EnergyType::Water,
            EnergyType::Colorless,
        ])],
        vec![PlayedCard::from_id(CardId::B4197WailordEx)],
    );
    let mut state = game.get_state_clone();
    state.set_knocked_out_by_opponent_attack_last_turn(knocked_out_last_turn);
    game.set_state(state);
    game
}

/// Lapras - Raging Freeze paralyzes the Defending Pokemon (with no damage bonus) when one of
/// the attacker's Pokemon was Knocked Out during the opponent's last turn.
#[test]
fn test_raging_freeze_paralyzes_after_a_knockout_last_turn() {
    let mut game = lapras_game(true);
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2b017Lapras, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        190,
        "Raging Freeze always deals exactly its 60 damage"
    );
    assert!(state.get_active(1).is_paralyzed());
}

#[test]
fn test_raging_freeze_does_not_paralyze_without_a_knockout_last_turn() {
    let mut game = lapras_game(false);
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2b017Lapras, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 190);
    assert!(!state.get_active(1).is_paralyzed());
}

/// Toxtricity - Vengeful Shock adds both 60 damage and Paralysis under the same condition.
#[test]
fn test_vengeful_shock_adds_damage_and_paralysis_after_a_knockout_last_turn() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B3061Toxtricity)
            .with_energy(vec![EnergyType::Lightning, EnergyType::Lightning])],
        vec![PlayedCard::from_id(CardId::B4197WailordEx)],
    );
    let mut state = game.get_state_clone();
    state.set_knocked_out_by_opponent_attack_last_turn(true);
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3061Toxtricity, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    // Wailord ex is Weak to Lightning: 250 HP - (40 + 60 + 20) = 130.
    assert_eq!(state.get_active(1).get_remaining_hp(), 130);
    assert!(state.get_active(1).is_paralyzed());
}
