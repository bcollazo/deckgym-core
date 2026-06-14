use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    effects::CardEffect,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

/// Cascoon's Harden: "During your opponent's next turn, prevent all damage done to
/// this Pokémon by attacks if that damage is 40 or less."
#[test]
fn test_cascoon_harden_prevents_low_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B1006Cascoon).with_energy(vec![EnergyType::Grass])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Grass])],
    );
    game.set_state(state);

    // Cascoon uses Harden, applying the prevention effect to itself.
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    // Opponent's turn: Bulbasaur attacks with Vine Whip (40 damage).
    let mut state = game.get_state_clone();
    state.current_player = 1;
    game.set_state(state);
    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    assert_eq!(
        game.get_state_clone().get_active(0).get_remaining_hp(),
        80,
        "Harden should prevent the 40-damage Vine Whip entirely"
    );
}

#[test]
fn test_cascoon_harden_does_not_prevent_high_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 1;

    // Cascoon already hardened; opponent has a >40 damage attacker (Weezing's Smokescreen, 50).
    let mut cascoon =
        PlayedCard::from_id(CardId::B1006Cascoon).with_energy(vec![EnergyType::Grass]);
    cascoon.add_effect(CardEffect::PreventDamageIfLessOrEqual { threshold: 40 }, 1);
    state.set_board(
        vec![cascoon],
        vec![PlayedCard::from_id(CardId::A1a050Weezing)
            .with_energy(vec![EnergyType::Darkness, EnergyType::Darkness])],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    assert_eq!(
        game.get_state_clone().get_active(0).get_remaining_hp(),
        30,
        "Harden should not prevent damage above 40 (Weezing's 50-damage Tackle)"
    );
}
