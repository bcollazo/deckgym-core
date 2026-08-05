use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game_with_board},
};

/// Runs Mountain Crush once with the given seed and returns
/// `(damage dealt, cards milled from the opponent's deck)`.
fn run_mountain_crush(seed: u64) -> (u32, usize) {
    let mut game = get_initialized_game_with_board(
        seed,
        0,
        3,
        vec![
            PlayedCard::from_id(CardId::B3094Coalossal).with_energy(vec![
                EnergyType::Fighting,
                EnergyType::Fighting,
                EnergyType::Colorless,
            ]),
        ],
        // Dusknoir has 150 HP and is weak to Darkness, so the 90 damage neither
        // knocks it out nor gets a weakness bonus from a [F] attacker.
        vec![PlayedCard::from_id(CardId::B1105Dusknoir)],
    );

    let before = game.get_state_clone();
    let deck_before = before.decks[1].cards.len();
    let discard_before = before.discard_piles[1].len();

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3094Coalossal, 0),
        is_stack: false,
    });

    let after = game.get_state_clone();
    let milled = deck_before - after.decks[1].cards.len();
    assert_eq!(
        after.discard_piles[1].len() - discard_before,
        milled,
        "every card milled off the top of the deck should land in the discard pile"
    );

    (150 - after.get_active(1).get_remaining_hp(), milled)
}

/// Mountain Crush always deals its 90 damage, and mills the opponent's deck for
/// each heads of a flip-until-tails sequence (so sometimes 0 cards).
#[test]
fn test_mountain_crush_mills_opponent_deck_for_each_heads() {
    let mut saw_no_mill = false;
    let mut saw_mill = false;

    for seed in 0..20 {
        let (damage, milled) = run_mountain_crush(seed);
        assert_eq!(damage, 90, "Mountain Crush should always deal 90 damage");
        assert!(milled <= 8, "flip-until-tails is truncated at 8 heads");
        if milled == 0 {
            saw_no_mill = true;
        } else {
            saw_mill = true;
        }
    }

    assert!(
        saw_no_mill,
        "an immediate tails should sometimes mill nothing"
    );
    assert!(saw_mill, "heads should sometimes mill at least one card");
}
