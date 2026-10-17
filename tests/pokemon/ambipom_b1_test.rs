use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_excited_tail_without_lucky_mittens() {
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();

        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![PlayedCard::from_id(CardId::B1186Ambipom)
                .with_energy(vec![EnergyType::Colorless])],
            vec![PlayedCard::from_id(CardId::A1211Snorlax)],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::B1186Ambipom, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        // Flip 2 coins, each heads deals 30 dmg.
        let opponent_snorlax_remaining_hp = state.get_active(1).get_remaining_hp();
        assert!(
            [150, 120, 90].contains(&opponent_snorlax_remaining_hp),
            "seed {seed }"
        );
    }
}

#[test]
fn test_excited_tail_with_lucky_mittens() {
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();

        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![PlayedCard::from_id(CardId::B1186Ambipom)
                .with_energy(vec![EnergyType::Colorless])
                .with_tool(get_card_by_enum(CardId::B1220LuckyMittens))],
            vec![PlayedCard::from_id(CardId::A1211Snorlax)],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::B1186Ambipom, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        // Lucky Mittens interacts with Excited Tail causing to flip 4 coins, each heads deals 30 dmg.
        let opponent_snorlax_remaining_hp = state.get_active(1).get_remaining_hp();
        assert!(
            [150, 120, 90, 60, 30].contains(&opponent_snorlax_remaining_hp),
            "seed {seed }"
        );
    }
}
