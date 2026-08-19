use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_farigiraf_double_hit_flip_two_coins_sixty_per_heads() {
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![PlayedCard::from_id(CardId::B3a058Farigiraf)
                .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless])],
            vec![PlayedCard::from_id(CardId::A1211Snorlax)],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::B3a058Farigiraf, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        let remaining = state.get_active(1).get_remaining_hp();
        assert!(
            [150, 90, 30].contains(&remaining),
            "seed {seed}: Flip 2 coins, 60 per heads -> 0/60/120 damage, got {remaining}"
        );
    }
}

#[test]
fn test_dipplin_double_hit_uses_same_effect_text() {
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![PlayedCard::from_id(CardId::B4126Dipplin)
                .with_energy(vec![EnergyType::Grass, EnergyType::Fire])],
            vec![PlayedCard::from_id(CardId::A1211Snorlax)],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::B4126Dipplin, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        let remaining = state.get_active(1).get_remaining_hp();
        assert!(
            [150, 90, 30].contains(&remaining),
            "seed {seed}: Dipplin shares Double Hit text, got {remaining}"
        );
    }
}
