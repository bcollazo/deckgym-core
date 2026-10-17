use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::PlayedCard,
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_psyduck_confusion_wave_confuses_both_actives() {
    // Psyduck's Confusion Wave: Both Active Pokémon are now Confused.
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::B3b011Psyduck)],
        vec![PlayedCard::from_id(CardId::A1053Squirtle)],
    );
    state.current_player = 0;
    game.set_state(state);

    assert!(!game.get_state_clone().get_active(0).is_confused());
    assert!(!game.get_state_clone().get_active(1).is_confused());

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3b011Psyduck, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert!(
        state.get_active(0).is_confused(),
        "Psyduck itself should be Confused after using Confusion Wave"
    );
    assert!(
        state.get_active(1).is_confused(),
        "Opponent's Active Pokémon should be Confused after Confusion Wave"
    );
}
