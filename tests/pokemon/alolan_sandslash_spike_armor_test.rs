use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

/// Alolan Sandslash's Spike Armor: "During your opponent's next turn, if this
/// Pokémon is damaged by an attack, do 40 damage to the Attacking Pokémon."
/// This counterattack is applied at the same place RockyHelmet does its recoil.
#[test]
fn test_alolan_sandslash_spike_armor_counters_attacker() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Alolan Sandslash (100 HP, Spike Armor = 20 dmg) vs Bulbasaur (70 HP, Vine Whip = 40 dmg)
    state.set_board(
        vec![PlayedCard::from_id(CardId::A3039AlolanSandslash)
            .with_energy(vec![EnergyType::Water])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
    );
    state.current_player = 0;
    game.set_state(state);

    // Sandslash uses Spike Armor: 20 damage to Bulbasaur + arm counterattack effect.
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A3039AlolanSandslash, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        50,
        "Bulbasaur should take 20 damage from Spike Armor (70 - 20 = 50)"
    );

    // End Sandslash's turn.
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });

    // Bulbasaur attacks Sandslash with Vine Whip (40 damage).
    game.apply_action(&Action {
        actor: 1,
        action: attack_action(CardId::A1001Bulbasaur, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    // Sandslash: 100 - 40 = 60.
    assert_eq!(
        state.get_active(0).get_remaining_hp(),
        60,
        "Alolan Sandslash should take 40 damage from Vine Whip"
    );
    // Spike Armor counters for 40: Bulbasaur 50 - 40 = 10.
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        10,
        "Bulbasaur should take 40 counterattack damage from Spike Armor (50 - 40 = 10)"
    );
}

/// Spike Armor only lasts during the opponent's next turn: once that turn passes,
/// a later attack should not trigger the counterattack.
#[test]
fn test_alolan_sandslash_spike_armor_expires_after_one_turn() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A3039AlolanSandslash)
            .with_energy(vec![EnergyType::Water])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
    );
    state.current_player = 0;
    game.set_state(state);

    // Sandslash uses Spike Armor.
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A3039AlolanSandslash, 0),
        is_stack: false,
    });
    // End Sandslash's turn (opponent's "next turn" begins).
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    // Opponent's next turn passes without attacking.
    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    // Sandslash's turn passes.
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });

    let bulbasaur_hp_before = game.get_state_clone().get_active(1).get_remaining_hp();

    // Now Bulbasaur attacks; the Spike Armor effect should already have expired.
    game.apply_action(&Action {
        actor: 1,
        action: attack_action(CardId::A1001Bulbasaur, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        bulbasaur_hp_before,
        "Spike Armor should have expired; Bulbasaur should take no counterattack damage"
    );
}
