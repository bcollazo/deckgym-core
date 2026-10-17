use std::collections::HashSet;

use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game_with_board},
};

/// Porygon-Z's "Buggy Beam": 80 damage, then "Change the type of the next Energy that will be
/// generated for your opponent to 1 of the following at random: [G], [R], [W], [L], [P], [F],
/// [D], or [M]."
///
/// The opponent's deck (weezing-arbok) only ever generates [D], so any other type in their
/// energy zone preview must have come from Buggy Beam.
#[test]
fn test_buggy_beam_randomizes_opponents_next_energy() {
    let mut observed = HashSet::new();

    for seed in 0..40u64 {
        let mut game = get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![PlayedCard::from_id(CardId::A2129PorygonZ).with_energy(vec![
                EnergyType::Grass,
                EnergyType::Grass,
                EnergyType::Grass,
            ])],
            vec![PlayedCard::from_id(CardId::A1004VenusaurEx)],
        );

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A2129PorygonZ, 0),
            is_stack: false,
        });

        let state = game.get_state_clone();
        assert_eq!(state.get_remaining_hp(1, 0), 190 - 80, "seed {seed}");

        let next = state.energy_zone[1]
            .next
            .expect("opponent should have a next energy queued");
        assert!(
            next.is_selectable(),
            "seed {seed}: Buggy Beam should only pick one of the 8 basic Energy types"
        );
        observed.insert(next);

        // The attacker's own energy zone is untouched.
        assert_eq!(state.energy_zone[0].next, Some(EnergyType::Grass));
    }

    assert!(
        observed.len() > 1,
        "expected Buggy Beam to produce more than one Energy type across seeds, got {observed:?}"
    );
    assert!(
        observed.iter().any(|e| *e != EnergyType::Darkness),
        "expected at least one Energy type the opponent's deck could not have generated"
    );
}
