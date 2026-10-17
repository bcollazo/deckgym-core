use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_whiscash_earthquake_extra_damage_or_self_damage() {
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![
                PlayedCard::from_id(CardId::A2a017Whiscash).with_energy(vec![
                    EnergyType::Water,
                    EnergyType::Water,
                    EnergyType::Colorless,
                    EnergyType::Colorless,
                ]),
            ],
            vec![PlayedCard::from_id(CardId::A1211Snorlax)],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A2a017Whiscash, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        let opponent_remaining = state.get_active(1).get_remaining_hp();
        // heads: 80 + 60 = 140 -> 10; tails: 80 -> 70
        assert!(
            [10, 70].contains(&opponent_remaining),
            "seed {seed}: heads 140 / tails 80, got {opponent_remaining}"
        );
        // heads: Whiscash unharmed (120); tails: takes 20 -> 100
        let self_remaining = state.get_active(0).get_remaining_hp();
        assert!(
            [120, 100].contains(&self_remaining),
            "seed {seed}: self 120 / 100, got {self_remaining}"
        );
        // Heads (140 dmg) must not coincide with self-damage, and vice-versa
        assert_eq!(
            opponent_remaining == 10,
            self_remaining == 120,
            "seed {seed}: heads heals nothing / tails damages self"
        );
    }
}
