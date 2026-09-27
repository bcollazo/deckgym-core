use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

fn super_fang_against(remaining_hp: u32) -> u32 {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A2135Bidoof)
            .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax).with_remaining_hp(remaining_hp)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A2135Bidoof, 0),
        is_stack: false,
    });
    game.play_until_stable();

    game.get_state_clone()
        .maybe_get_active(1)
        .map(|active| active.get_remaining_hp())
        .unwrap_or(0)
}

/// Bidoof - Super Fang: "Halve your opponent's Active Pokémon's remaining HP, rounded down."
/// Pocket tracks HP in multiples of 10, so 70 becomes 30 rather than 35.
#[test]
fn test_super_fang_halves_remaining_hp_rounded_down_to_the_nearest_ten() {
    assert_eq!(super_fang_against(150), 70);
    assert_eq!(super_fang_against(80), 40);
    assert_eq!(super_fang_against(70), 30);
    assert_eq!(super_fang_against(30), 10);
}

/// Bidoof is Colorless and Snorlax is Weak to Fighting, but Super Fang sets HP directly, so
/// Weakness never enters into it — and halving 10 HP knocks the defender out.
#[test]
fn test_super_fang_knocks_out_a_ten_hp_defender() {
    assert_eq!(super_fang_against(10), 0);
}
