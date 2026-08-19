use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_psyduck_b4_confusion_wave_confuses_opponent_or_self() {
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![PlayedCard::from_id(CardId::B4030Psyduck).with_energy(vec![EnergyType::Colorless])],
            vec![PlayedCard::from_id(CardId::A1211Snorlax)],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::B4030Psyduck, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        // 10 damage always
        assert_eq!(state.get_active(1).get_remaining_hp(), 140, "seed {seed}");
        // heads -> opponent Confused, tails -> self Confused (exactly one)
        assert!(
            state.get_active(1).is_confused() || state.get_active(0).is_confused(),
            "seed {seed}: either the opponent (heads) or Psyduck itself (tails) is Confused"
        );
    }
}
