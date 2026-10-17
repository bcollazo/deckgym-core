use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Ice Shard deals 10 damage, plus 30 more when the opponent's Active Pokemon
/// is a Fighting Pokemon.
#[test]
fn test_snover_ice_shard_extra_damage_vs_fighting() {
    // Machop (A1 143) is a Fighting Pokemon with 70 HP. Its weakness is Psychic,
    // so Snover's Water-type attack does not trigger a weakness bonus.
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A2044Snover).with_energy(vec![EnergyType::Water])],
        vec![PlayedCard::from_id(CardId::A1143Machop)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A2044Snover, 0),
        is_stack: false,
    });

    // 10 base + 30 (opponent is Fighting) = 40 damage. 70 - 40 = 30 HP remaining.
    let opponent_hp = game.get_state_clone().get_active(1).get_remaining_hp();
    assert_eq!(
        opponent_hp, 30,
        "Ice Shard should deal 40 damage to a Fighting Pokemon (70 - 40 = 30)"
    );
}

/// Ice Shard deals only its base 10 damage when the opponent's Active Pokemon
/// is not a Fighting Pokemon.
#[test]
fn test_snover_ice_shard_base_damage_vs_non_fighting() {
    // Bulbasaur (A1 001) is a Grass Pokemon with 70 HP; weakness is Fire, so no
    // weakness bonus from Snover's Water-type attack.
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A2044Snover).with_energy(vec![EnergyType::Water])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A2044Snover, 0),
        is_stack: false,
    });

    // 10 base damage only. 70 - 10 = 60 HP remaining.
    let opponent_hp = game.get_state_clone().get_active(1).get_remaining_hp();
    assert_eq!(
        opponent_hp, 60,
        "Ice Shard should deal only 10 base damage to a non-Fighting Pokemon (70 - 10 = 60)"
    );
}
