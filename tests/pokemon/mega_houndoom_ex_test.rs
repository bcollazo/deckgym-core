use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_mega_houndoom_ex_grimhound_flare_flip_three_coins_eighty_per_heads() {
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![
                PlayedCard::from_id(CardId::PB080MegaHoundoomEx).with_energy(vec![
                    EnergyType::Fire,
                    EnergyType::Fire,
                    EnergyType::Colorless,
                ]),
            ],
            // Wailord ex (250 HP) survives the max 240 (3 heads) without a knockout
            vec![PlayedCard::from_id(CardId::B4197WailordEx)],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::PB080MegaHoundoomEx, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        let remaining = state.get_active(1).get_remaining_hp();
        assert!(
            [250, 170, 90, 10].contains(&remaining),
            "seed {seed}: Flip 3 coins, 80 per heads -> 0/80/160/240 damage, got {remaining}"
        );
    }
}
