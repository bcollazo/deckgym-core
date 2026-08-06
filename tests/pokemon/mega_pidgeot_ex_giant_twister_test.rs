use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game_with_board},
};

/// Runs Giant Twister once with the given seed and returns
/// `(damage dealt, energies discarded from the opponent's Active Pokémon)`.
fn run_giant_twister(seed: u64) -> (u32, usize) {
    let mut game = get_initialized_game_with_board(
        seed,
        0,
        3,
        vec![
            PlayedCard::from_id(CardId::PB006MegaPidgeotEx).with_energy(vec![
                EnergyType::Colorless,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
        ],
        // Wailord ex has 250 HP and is weak to Lightning, so the 100 damage neither
        // knocks it out nor gets a weakness bonus from a [C] attacker.
        vec![
            PlayedCard::from_id(CardId::B4037WailordEx).with_energy(vec![
                EnergyType::Water,
                EnergyType::Water,
                EnergyType::Water,
                EnergyType::Water,
            ]),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::PB006MegaPidgeotEx, 0),
        is_stack: false,
    });

    let after = game.get_state_clone();
    let defender = after.get_active(1);
    (
        250 - defender.get_remaining_hp(),
        4 - defender.attached_energy.len(),
    )
}

/// Giant Twister discards 1 Energy per heads of 3 coins, and does nothing at all
/// (no damage, no discard) when all three coins are tails.
#[test]
fn test_giant_twister_discards_one_energy_per_heads_and_whiffs_on_all_tails() {
    let mut saw_whiff = false;
    let mut saw_hit = false;

    for seed in 0..20 {
        let (damage, discarded) = run_giant_twister(seed);
        if discarded == 0 {
            assert_eq!(
                damage, 0,
                "with all three coins tails Giant Twister should do nothing at all"
            );
            saw_whiff = true;
        } else {
            assert!(discarded <= 3, "Giant Twister only flips 3 coins");
            assert_eq!(
                damage, 100,
                "Giant Twister should deal 100 damage on any heads"
            );
            saw_hit = true;
        }
    }

    assert!(saw_whiff, "all three coins should sometimes come up tails");
    assert!(saw_hit, "at least one heads should sometimes come up");
}
