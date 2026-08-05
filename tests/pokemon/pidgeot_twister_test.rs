use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game_with_board},
};

/// Runs Twister once with the given seed and returns
/// `(damage dealt, energies discarded from the opponent's Active Pokémon)`.
fn run_twister(seed: u64) -> (u32, usize) {
    let mut game = get_initialized_game_with_board(
        seed,
        0,
        3,
        vec![PlayedCard::from_id(CardId::B1182Pidgeot)
            .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless])],
        // Dusknoir has 150 HP and is weak to Darkness, so the 80 damage neither knocks
        // it out nor gets a weakness bonus from a [C] attacker.
        vec![PlayedCard::from_id(CardId::B1105Dusknoir).with_energy(vec![
            EnergyType::Psychic,
            EnergyType::Psychic,
            EnergyType::Psychic,
        ])],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B1182Pidgeot, 0),
        is_stack: false,
    });

    let after = game.get_state_clone();
    let defender = after.get_active(1);
    (
        150 - defender.get_remaining_hp(),
        3 - defender.attached_energy.len(),
    )
}

/// Twister discards 1 Energy per heads of 2 coins, and does nothing at all
/// (no damage, no discard) when both coins are tails.
#[test]
fn test_twister_discards_one_energy_per_heads_and_whiffs_on_all_tails() {
    let mut saw_whiff = false;
    let mut saw_hit = false;

    for seed in 0..20 {
        let (damage, discarded) = run_twister(seed);
        if discarded == 0 {
            assert_eq!(
                damage, 0,
                "with both coins tails Twister should do nothing at all"
            );
            saw_whiff = true;
        } else {
            assert!(discarded <= 2, "Twister only flips 2 coins");
            assert_eq!(damage, 80, "Twister should deal 80 damage on any heads");
            saw_hit = true;
        }
    }

    assert!(saw_whiff, "both coins should sometimes come up tails");
    assert!(saw_hit, "at least one heads should sometimes come up");
}
