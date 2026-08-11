use deckgym::{
    actions::Action,
    card_ids::CardId::{self},
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_leech_life_heals_equals_to_damage_dealt() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.current_player = 0;
    state.turn_count = 1;

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1159Kabutops).with_energy(vec![EnergyType::Fighting])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
    );
    game.set_state(state);

    // Opponent attacks first: Bulbasaur damages Kabutos for 30 dmg
    let mut state = game.get_state_clone();
    state.current_player = 1;
    game.set_state(state);
    game.apply_action(&Action {
        actor: 1,
        action: attack_action(CardId::A1001Bulbasaur, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let hp_after_opponent_attacks = state.get_active(0).get_remaining_hp();
    assert!(hp_after_opponent_attacks < 140);

    // Own Kabutops attacks and heals by "Leech lilfe"
    let mut state = game.get_state_clone();
    state.current_player = 0;
    game.set_state(state);
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1159Kabutops, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();

    // Bulbasaur took 50 damage
    assert_eq!(state.get_active(1).get_remaining_hp(), 20);
    // Leech Life: heals equals 50 dmg
    assert_eq!(
        state.get_active(0).get_remaining_hp(),
        hp_after_opponent_attacks + 50
    );
}
