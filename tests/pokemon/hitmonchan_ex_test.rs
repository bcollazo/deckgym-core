use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_hitmonchan_ex_quick_straight_ignores_weakness() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.set_board(
        vec![PlayedCard::from_id(CardId::B1124HitmonchanEx)
            .with_energy(vec![EnergyType::Fighting])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    state.current_player = 0;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B1124HitmonchanEx, 0),
        is_stack: false,
    });

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        100,
        "Quick Straight should deal 50 damage without applying Fighting weakness"
    );
}
