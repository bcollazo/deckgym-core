use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_lanturn_ex_flash_cannon_paralyzed_or_confused() {
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![
                PlayedCard::from_id(CardId::A4065LanturnEx).with_energy(vec![
                    EnergyType::Lightning,
                    EnergyType::Colorless,
                    EnergyType::Colorless,
                ]),
            ],
            vec![PlayedCard::from_id(CardId::A1211Snorlax)],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A4065LanturnEx, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        assert_eq!(
            state.get_active(1).get_remaining_hp(),
            70,
            "seed {seed}: 80 damage"
        );
        // heads -> Paralyzed, tails -> Confused (always exactly one applies)
        assert!(
            state.get_active(1).is_paralyzed() || state.get_active(1).is_confused(),
            "seed {seed}: opponent must be Paralyzed (heads) or Confused (tails)"
        );
    }
}
