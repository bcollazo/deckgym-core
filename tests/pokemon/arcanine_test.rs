use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_arcanine_flame_mane_always_deals_damage_and_heads_burns() {
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![PlayedCard::from_id(CardId::B1029Arcanine)
                .with_energy(vec![EnergyType::Fire, EnergyType::Fire])],
            vec![PlayedCard::from_id(CardId::A1211Snorlax)],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::B1029Arcanine, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        // 50 damage always
        assert_eq!(state.get_active(1).get_remaining_hp(), 100, "seed {seed}");
    }
}
