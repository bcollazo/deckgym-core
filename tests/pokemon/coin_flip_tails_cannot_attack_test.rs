use deckgym::{
    actions::Action,
    card_ids::CardId,
    effects::CardEffect,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

/// Hippowdon - Crashing Fangs: "Flip a coin. If tails, during your next turn, this Pokémon
/// can't attack." Damage is dealt either way; only tails leaves the self-debuff.
#[test]
fn test_crashing_fangs_only_blocks_next_turn_on_tails() {
    let mut saw_heads = false;
    let mut saw_tails = false;

    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.current_player = 0;
        state.turn_count = 3;
        state.set_board(
            vec![
                PlayedCard::from_id(CardId::B1129Hippowdon).with_energy(vec![
                    EnergyType::Fighting,
                    EnergyType::Fighting,
                    EnergyType::Colorless,
                ]),
            ],
            vec![PlayedCard::from_id(CardId::B4197WailordEx)],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::B1129Hippowdon, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        assert_eq!(
            state.get_active(1).get_remaining_hp(),
            150,
            "seed {seed}: Crashing Fangs always deals its 100 damage"
        );
        if state.get_active(0).has_effect(&CardEffect::CannotAttack) {
            saw_tails = true;
        } else {
            saw_heads = true;
        }
    }

    assert!(
        saw_heads && saw_tails,
        "both coin results should occur across 50 seeds"
    );
}
