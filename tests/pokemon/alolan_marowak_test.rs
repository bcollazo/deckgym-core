use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_burning_bonemerang_coin_flips_damage_burn_effect_per_heads() {
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();

        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![
                PlayedCard::from_id(CardId::A3027AlolanMarowak).with_energy(vec![
                    EnergyType::Fire,
                    EnergyType::Fire,
                    EnergyType::Colorless,
                ]),
            ],
            vec![PlayedCard::from_id(CardId::A1211Snorlax)],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A3027AlolanMarowak, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        // Flip 2 coins, Burning Bonemerang deals 70 damage for each heads, and if flipped at least 1 heads inflicts "burning" status to opponent's active Pokémon
        let opponent_snorlax_remaining_hp = state.get_active(1).get_remaining_hp();
        assert!(
            [150, 80, 10].contains(&opponent_snorlax_remaining_hp),
            "seed {seed }"
        );
        // Check for status "burned" on opponent's active Pokémon
        assert_eq!(
            opponent_snorlax_remaining_hp != 150,
            state.get_active(1).is_burned(),
            "seed {seed}"
        );
    }
}
