use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_lugia_aeroblast_added_damage_per_heads() {
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();

        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![PlayedCard::from_id(CardId::B2131Lugia).with_energy(vec![
                EnergyType::Colorless,
                EnergyType::Colorless,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ])],
            vec![PlayedCard::from_id(CardId::A1004VenusaurEx)],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::B2131Lugia, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        // For every heads for 2 coin flips, Aeroblast deals 40* damage each plus 80 base damage
        let opponent_venusaur_ex_remaining_hp = state.get_active(1).get_remaining_hp();
        assert!(
            opponent_venusaur_ex_remaining_hp >= 30 || opponent_venusaur_ex_remaining_hp <= 110,
            "seed {seed}"
        );
    }
}
