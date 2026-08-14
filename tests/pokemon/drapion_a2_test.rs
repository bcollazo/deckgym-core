use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_cross_poison_coin_flips_damage_per_heads_poison_effect() {
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();

        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![PlayedCard::from_id(CardId::A2106Drapion).with_energy(vec![
                EnergyType::Darkness,
                EnergyType::Darkness,
                EnergyType::Darkness,
            ])],
            vec![PlayedCard::from_id(CardId::A1004VenusaurEx)],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A2106Drapion, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        // Flip 4 coins, each heads deals 40 dmg, if at least 2 heads happened opponent's active Pokémon becomes Poisoned
        let opponent_venusaur_ex_remaining_hp = state.get_active(1).get_remaining_hp();
        assert!(
            [190, 150, 110, 70, 30].contains(&opponent_venusaur_ex_remaining_hp),
            "seed {seed }"
        );
        // Check for status "Poisoned" on opponent's active Pokémon
        assert_eq!(
            opponent_venusaur_ex_remaining_hp <= 110,
            state.get_active(1).is_poisoned(),
            "seed {seed}"
        );
    }
}
