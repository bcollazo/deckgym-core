use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

const VENUSAUR_EX_MAX_HP: u32 = 190;

/// Mega Metagross ex's Gatling Slug does 100 damage plus 10 more for each [M] Energy
/// attached to it.
#[test]
fn test_mega_metagross_ex_gatling_slug_scales_with_metal_energy() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B4109MegaMetagrossEx).with_energy(vec![
                EnergyType::Metal,
                EnergyType::Metal,
                EnergyType::Metal,
            ]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1004VenusaurEx)],
    );

    // 3 [M] Energy attached → 100 + 10 × 3 = 130 damage.
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4109MegaMetagrossEx, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        VENUSAUR_EX_MAX_HP - 130
    );
}

/// Gatling Slug only counts [M] Energy, even though its cost is 3 [C] and can be paid
/// with any Energy.
#[test]
fn test_mega_metagross_ex_gatling_slug_ignores_non_metal_energy() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B4109MegaMetagrossEx).with_energy(vec![
                EnergyType::Metal,
                EnergyType::Water,
                EnergyType::Water,
            ]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1004VenusaurEx)],
    );

    // Only 1 [M] Energy attached → 100 + 10 × 1 = 110 damage.
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4109MegaMetagrossEx, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        VENUSAUR_EX_MAX_HP - 110
    );
}
