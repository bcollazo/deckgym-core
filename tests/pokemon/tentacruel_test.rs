use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_tentacruel_poisoned_and_paralyzed_together_on_heads() {
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![
                PlayedCard::from_id(CardId::A4a016Tentacruel).with_energy(vec![
                    EnergyType::Water,
                    EnergyType::Water,
                    EnergyType::Colorless,
                ]),
            ],
            vec![PlayedCard::from_id(CardId::A1211Snorlax)],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A4a016Tentacruel, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        // 50 damage always
        assert_eq!(state.get_active(1).get_remaining_hp(), 100, "seed {seed}");
        // Poisoned and Paralyzed always come together on heads
        assert_eq!(
            state.get_active(1).is_poisoned(),
            state.get_active(1).is_paralyzed(),
            "seed {seed}: Poisoned and Paralyzed must be applied together"
        );
    }
}
