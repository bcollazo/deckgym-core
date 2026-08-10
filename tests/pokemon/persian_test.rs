use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game, play_trainer, trainer_from_id},
};

#[test]
fn test_persian_shadow_claw_flips_coin_possible_opponent_discard() {
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();

        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![PlayedCard::from_id(CardId::A1197Persian)
                .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless])],
            vec![PlayedCard::from_id(CardId::A1211Snorlax)],
        );
        state.hands[1] = vec![get_card_by_enum(CardId::B3028Emboar).clone()];
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A1197Persian, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        let snorlax_initial_health_points = 150;
        let expected_damage_dealt = 40;
        let hand_len_before = 1;

        // Shadow Claw: 40 damage dealt
        assert_eq!(
            state.get_active(1).get_remaining_hp(),
            snorlax_initial_health_points - expected_damage_dealt
        );

        // Assert whether opponents hand got smaller
        assert!(
            state.hands[1].len() == hand_len_before || state.hands[1].len() == hand_len_before - 1
        );
    }
}

#[test]
fn test_shadow_claw_interaction_trainer_will() {
    // Trainer Will will force the first next coin flip to be heads
    let will = trainer_from_id(CardId::A4156Will);
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1197Persian)
            .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    state.hands[0] = vec![get_card_by_enum(CardId::A4156Will).clone()];
    state.hands[1] = vec![get_card_by_enum(CardId::B3028Emboar).clone()];
    game.set_state(state);

    play_trainer(&mut game, 0, will.clone());
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1197Persian, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    let snorlax_initial_health_points = 150;
    let expected_damage_dealt = 40;
    let hand_len_before = 1;

    // Shadow Claw: 40 damage dealt
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        snorlax_initial_health_points - expected_damage_dealt
    );

    // Opponent hand got smaller
    assert!(state.hands[1].len() == hand_len_before - 1);
}
