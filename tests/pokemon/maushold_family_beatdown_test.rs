use std::collections::HashSet;

use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

/// Maushold - Family Beatdown: "Flip a coin for each Tandemaus and Maushold you have in play.
/// This attack does 60 damage for each heads." Only Tandemaus/Maushold are counted, so the
/// Bulbasaur on the Bench adds no coin.
#[test]
fn test_family_beatdown_flips_one_coin_per_tandemaus_or_maushold() {
    let mut observed: HashSet<u32> = HashSet::new();

    for seed in 0..80 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.current_player = 0;
        state.turn_count = 3;
        state.set_board(
            vec![
                PlayedCard::from_id(CardId::B2143Maushold).with_energy(vec![EnergyType::Colorless]),
                PlayedCard::from_id(CardId::B2142Tandemaus),
                PlayedCard::from_id(CardId::A1001Bulbasaur),
            ],
            vec![PlayedCard::from_id(CardId::B4197WailordEx)],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::B2143Maushold, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let remaining = game.get_state_clone().get_active(1).get_remaining_hp();
        observed.insert(250 - remaining);
    }

    // Two eligible Pokemon in play => 0, 1 or 2 heads => 0, 60 or 120 damage.
    let expected: HashSet<u32> = [0, 60, 120].into_iter().collect();
    assert_eq!(
        observed, expected,
        "Family Beatdown should flip exactly 2 coins worth 60 damage each"
    );
}
