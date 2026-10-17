use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Mimikyu ex - Disguise: "When this Pokémon is first damaged by an attack after coming into
/// play, prevent that damage." An attack that does 0 damage (Swablu's Sing only puts the target
/// to sleep) damages nothing, so it must not use the Disguise up.
#[test]
fn test_zero_damage_attack_does_not_use_up_disguise() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B1196Swablu).with_energy(vec![EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::B2073MimikyuEx)],
    );
    let starting_hp = game.get_state_clone().get_active(1).get_remaining_hp();

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B1196Swablu, 0),
        is_stack: false,
    });
    let state = game.get_state_clone();
    assert!(state.get_active(1).is_asleep(), "Sing should apply Sleep");
    assert_eq!(state.get_active(1).get_remaining_hp(), starting_hp);

    // Now a real hit from Bulbasaur: the Disguise is still up, so this one is prevented...
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.in_play_pokemon[0][0] =
        Some(PlayedCard::from_id(CardId::A1001Bulbasaur).with_energy(vec![EnergyType::Grass]));
    game.set_state(state);
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1001Bulbasaur, 0),
        is_stack: false,
    });
    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        starting_hp,
        "the first damaging attack is prevented by Disguise"
    );

    // ...and the one after that goes through.
    let mut state = game.get_state_clone();
    state.current_player = 0;
    game.set_state(state);
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1001Bulbasaur, 0),
        is_stack: false,
    });
    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        starting_hp - 40
    );
}
