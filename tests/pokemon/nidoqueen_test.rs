use deckgym::{
    actions::Action,
    card_ids::CardId::{self},
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_lovestrike_checks_for_benched_nidokings() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.current_player = 0;
    state.turn_count = 1;

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1168Nidoqueen).with_energy(vec![
                EnergyType::Darkness,
                EnergyType::Darkness,
                EnergyType::Darkness,
            ]),
            PlayedCard::from_id(CardId::A1171Nidoking),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1168Nidoqueen, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();

    // Lovestrike: deals 80 dmg + 50 per each benched Nidoking (1 benched)
    assert_eq!(state.get_active(1).get_remaining_hp(), 20);
}
