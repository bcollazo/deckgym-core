use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_sandslash_fury_swipes_heads_or_nothing() {
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();

        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![
                PlayedCard::from_id(CardId::B2078Sandslash).with_energy(vec![EnergyType::Fighting])
            ],
            vec![PlayedCard::from_id(CardId::A1211Snorlax)],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::B2078Sandslash, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        // For every heads for 3 coin flips, Fury Sipes deals 30* damage each
        let opponent_snorlax_remaining_hp = state.get_active(1).get_remaining_hp();
        assert!(
            opponent_snorlax_remaining_hp >= 60 || opponent_snorlax_remaining_hp <= 150,
            "seed {seed}"
        );
    }
}
