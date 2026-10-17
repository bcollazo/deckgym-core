use deckgym::{
    actions::Action,
    card_ids::CardId,
    effects::CardEffect,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

/// Carracosta's Blocking Shell: "Prevent all damage done to this Pokémon by attacks
/// from Basic Pokémon during your opponent's next turn." (100 damage)
#[test]
fn test_carracosta_blocking_shell_prevents_basic_attack_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B1067Carracosta).with_energy(vec![
                EnergyType::Water,
                EnergyType::Water,
                EnergyType::Colorless,
            ]),
        ],
        // Mewtwo ex is a Basic Pokémon with enough HP to survive the 100 damage.
        vec![PlayedCard::from_id(CardId::A1129MewtwoEx)
            .with_energy(vec![EnergyType::Psychic, EnergyType::Colorless])],
    );
    game.set_state(state);

    // Carracosta uses Blocking Shell, dealing 100 and shielding itself from Basics.
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B1067Carracosta, 0),
        is_stack: false,
    });

    // Opponent's turn: the Basic Mewtwo ex attacks with Psychic Sphere (50 damage).
    let mut state = game.get_state_clone();
    state.current_player = 1;
    game.set_state(state);
    game.apply_action(&Action {
        actor: 1,
        action: attack_action(CardId::A1129MewtwoEx, 0),
        is_stack: false,
    });

    assert_eq!(
        game.get_state_clone().get_active(0).get_remaining_hp(),
        150,
        "Blocking Shell should prevent all damage from a Basic Pokémon's attack"
    );
}

/// Blocking Shell only blocks Basic Pokémon, so attacks from evolved Pokémon still land.
#[test]
fn test_carracosta_blocking_shell_does_not_prevent_evolved_attack_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 1;

    // Carracosta has already used Blocking Shell (effect active for the opponent's turn).
    let mut carracosta = PlayedCard::from_id(CardId::B1067Carracosta);
    carracosta.add_effect(CardEffect::PreventDamageFromBasic, 1);
    state.set_board(
        // Weezing is a Stage 1 (evolved) Pokémon, so its 50-damage Tackle is not blocked.
        vec![carracosta],
        vec![PlayedCard::from_id(CardId::A1a050Weezing)
            .with_energy(vec![EnergyType::Darkness, EnergyType::Darkness])],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 1,
        action: attack_action(CardId::A1a050Weezing, 0),
        is_stack: false,
    });

    assert_eq!(
        game.get_state_clone().get_active(0).get_remaining_hp(),
        100,
        "Blocking Shell should not prevent damage from an evolved Pokémon's attack"
    );
}
