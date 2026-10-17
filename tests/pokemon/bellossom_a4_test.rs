use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_petal_dance_coin_self_confused_and_flips_damage_per_heads() {
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();

        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![PlayedCard::from_id(CardId::A4163Bellossom)
                .with_energy(vec![EnergyType::Grass, EnergyType::Grass])],
            vec![PlayedCard::from_id(CardId::A1004VenusaurEx)],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A4163Bellossom, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        // Flip 3 coins, each heads deals 60 dmg, Bellossom is now confused.
        let opponent_venusaur_ex_remaining_hp = state.get_active(1).get_remaining_hp();
        assert!(
            [190, 130, 70, 10].contains(&opponent_venusaur_ex_remaining_hp),
            "seed {seed }"
        );
        // Check for Bellossom status "Confused"
        assert!(
            state.get_active(0).is_confused(),
            "seed {seed}: Petal Dance should always confuse whos attacking"
        );
    }
}
