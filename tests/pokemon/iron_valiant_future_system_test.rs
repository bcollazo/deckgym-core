use deckgym::{
    actions::SimpleAction,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

/// Future System reduces attack cost for Future Pokémon by 1 Colorless.
/// Iron Boulder needs [P][P][C][C] but with Iron Valiant on bench only [P][P][C] suffices.
#[test]
fn test_future_system_reduces_future_pokemon_attack_cost() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    // Iron Boulder active with [P][P][C] — one Colorless short without Future System
    let iron_boulder = PlayedCard::from_id(CardId::B3a029IronBoulder).with_energy(vec![
        EnergyType::Psychic,
        EnergyType::Psychic,
        EnergyType::Colorless,
    ]);
    // Iron Valiant on bench, providing Future System
    let iron_valiant = PlayedCard::from_id(CardId::B3a027IronValiant);
    state.set_board(
        vec![iron_boulder, iron_valiant],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    game.set_state(state);

    let (actor, actions) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(
        actions
            .iter()
            .any(|a| matches!(a.action, SimpleAction::Attack(0))),
        "Iron Boulder should be able to attack with [P][P][C] thanks to Future System"
    );
}

/// Future System does NOT reduce attack cost for non-Future Pokémon.
/// Bulbasaur needs [G][C]; with only [G] attached and Iron Valiant on bench, it cannot attack.
#[test]
fn test_future_system_does_not_reduce_non_future_pokemon_attack_cost() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    // Bulbasaur active with only [G] — missing [C] for Vine Whip
    let bulbasaur =
        PlayedCard::from_id(CardId::A1001Bulbasaur).with_energy(vec![EnergyType::Grass]);
    // Iron Valiant on bench
    let iron_valiant = PlayedCard::from_id(CardId::B3a027IronValiant);
    state.set_board(
        vec![bulbasaur, iron_valiant],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    game.set_state(state);

    let (actor, actions) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(
        !actions
            .iter()
            .any(|a| matches!(a.action, SimpleAction::Attack(_))),
        "Bulbasaur should NOT be able to attack with only [G] — Future System only helps Future Pokémon"
    );
}
