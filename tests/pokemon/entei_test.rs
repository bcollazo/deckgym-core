use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_entei_fire_spin_tails_discards_two_random_energy() {
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![PlayedCard::from_id(CardId::A4033Entei).with_energy(vec![
                EnergyType::Fire,
                EnergyType::Colorless,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ])],
            vec![PlayedCard::from_id(CardId::B4197WailordEx)],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A4033Entei, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        // 110 damage -> Wailord 140
        assert_eq!(state.get_active(1).get_remaining_hp(), 140, "seed {seed}");
        // heads: keeps 4 energies; tails: discards 2 -> keeps 2
        let remaining = state.get_active(0).attached_energy.len();
        assert!(
            [4, 2].contains(&remaining),
            "seed {seed}: heads keeps 4 / tails discards 2, got {remaining}"
        );
        // Conservation: discarded energies landed in discard_energies
        assert_eq!(
            state.discard_energies[0].len(),
            4 - remaining,
            "seed {seed}"
        );
    }
}
