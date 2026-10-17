use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_xatu_foresight_heads_sets_opponent_hp_to_ten() {
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![PlayedCard::from_id(CardId::A4082Xatu)
                .with_energy(vec![EnergyType::Psychic, EnergyType::Psychic])],
            vec![PlayedCard::from_id(CardId::B4197WailordEx)],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A4082Xatu, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        // heads: HP set to 10; tails: unchanged (250)
        let remaining = state.get_active(1).get_remaining_hp();
        assert!(
            [10, 250].contains(&remaining),
            "seed {seed}: heads sets HP to 10 / tails leaves 250, got {remaining}"
        );
    }
}
