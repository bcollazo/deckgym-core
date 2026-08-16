use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_delibird_present_damages_or_heals_opponent() {
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![PlayedCard::from_id(CardId::B2032Delibird)
                .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless])],
            vec![PlayedCard::from_id(CardId::A1211Snorlax)],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::B2032Delibird, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        let remaining = state.get_active(1).get_remaining_hp();
        // heads: 70 damage -> 80; tails: heal 30 from full -> stays 150
        assert!(
            [80, 150].contains(&remaining),
            "seed {seed}: heads 70 dmg / tails heal, got {remaining}"
        );
    }
}
