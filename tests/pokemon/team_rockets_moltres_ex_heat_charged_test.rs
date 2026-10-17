use std::collections::HashSet;

use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::EnergyType,
    models::PlayedCard,
    test_support::{attack_action, get_initialized_game},
};

/// Team Rocket's Moltres ex's Heat Charged: flip 3 coins, and for each heads attach a [R] Energy
/// from the Energy Zone to Moltres itself. Across many seeds the resulting energy count should
/// always land within the binomial range of the coin flips (starting energy + 0..=3 heads), and
/// more than one outcome should actually occur (i.e. the mechanic isn't a no-op).
#[test]
fn test_moltres_ex_heat_charged_attaches_energy_per_heads() {
    let mut seen_energy_counts = HashSet::new();
    for seed in 0..30 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.current_player = 0;
        state.turn_count = 3;
        state.set_board(
            vec![PlayedCard::from_id(CardId::B4a007TeamRocketsMoltresEx)
                .with_energy(vec![EnergyType::Fire])],
            vec![PlayedCard::from_id(CardId::A1211Snorlax)],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::B4a007TeamRocketsMoltresEx, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        let energy_count = state.get_active(0).attached_energy.len();
        assert!(
            (1..=4).contains(&energy_count),
            "seed {seed}: expected 1 starting + 0..=3 heads of Fire energy, got {energy_count}"
        );
        assert!(
            state
                .get_active(0)
                .attached_energy
                .iter()
                .all(|e| *e == EnergyType::Fire),
            "seed {seed}: all attached energy should be Fire"
        );
        seen_energy_counts.insert(energy_count);
    }
    assert!(
        seen_energy_counts.len() > 1,
        "expected varying numbers of heads across seeds, got only {seen_energy_counts:?}"
    );
}
