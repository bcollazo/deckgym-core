use deckgym::{
    card_ids::CardId,
    models::{EnergyType, PlayedCard, StatusCondition},
    test_support::get_initialized_game,
};

/// Test that Flower Shield prevents a [P]-energy Pokémon from being Poisoned
/// while Comfey is in play.
#[test]
fn test_flower_shield_prevents_poison_on_psychic_pokemon() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Player 0: Mewtwo active with [P] energy, Comfey on the Bench. Flower Shield is a board
    // ability, so the holder need not be Active -- but the *protected* Pokemon must be, since
    // Special Conditions only ever apply to the Active Spot (in-app Tips).
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1129MewtwoEx).with_energy(vec![EnergyType::Psychic]),
            PlayedCard::from_id(CardId::A3080Comfey),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    game.set_state(state);

    // Try to apply Poison to the Active Mewtwo, who has [P] energy
    let mut state = game.get_state_clone();
    state.apply_status_condition(0, 0, StatusCondition::Poisoned);
    game.set_state(state);

    let final_state = game.get_state_clone();
    assert!(
        !final_state.in_play_pokemon[0][0]
            .as_ref()
            .unwrap()
            .is_poisoned(),
        "Mewtwo with [P] Energy should be immune to Poison while Comfey (Flower Shield) is in play"
    );
}

/// Test that Flower Shield does NOT protect Pokémon without [P] energy.
#[test]
fn test_flower_shield_does_not_protect_non_psychic_pokemon() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Player 0: Bulbasaur active with only [G] energy (no Psychic), Comfey on the Bench
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_energy(vec![EnergyType::Grass]),
            PlayedCard::from_id(CardId::A3080Comfey),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    game.set_state(state);

    // Apply Poison to the Active Bulbasaur, who has only [G] energy
    let mut state = game.get_state_clone();
    state.apply_status_condition(0, 0, StatusCondition::Poisoned);
    game.set_state(state);

    let final_state = game.get_state_clone();
    assert!(
        final_state.in_play_pokemon[0][0]
            .as_ref()
            .unwrap()
            .is_poisoned(),
        "Bulbasaur without [P] Energy should still be Poisoned even with Comfey in play"
    );
}

/// Test that Flower Shield does not protect Pokémon on the opponent's side.
#[test]
fn test_flower_shield_does_not_protect_opponent() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Player 0 has Comfey; opponent (player 1) has Mewtwo with [P] energy
    state.set_board(
        vec![PlayedCard::from_id(CardId::A3080Comfey)],
        vec![PlayedCard::from_id(CardId::A1129MewtwoEx).with_energy(vec![EnergyType::Psychic])],
    );
    game.set_state(state);

    // Apply Poison to opponent's Mewtwo
    let mut state = game.get_state_clone();
    state.apply_status_condition(1, 0, StatusCondition::Poisoned);
    game.set_state(state);

    let final_state = game.get_state_clone();
    assert!(
        final_state.in_play_pokemon[1][0]
            .as_ref()
            .unwrap()
            .is_poisoned(),
        "Opponent's Mewtwo should be Poisoned; Flower Shield only protects the Comfey owner's Pokémon"
    );
}
