use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Pokémon Tools are attachable to any Pokémon, but stage/type-specific effects only apply to a
/// matching holder. These tests lock in the effect-gating for the tools whose restriction used
/// to live in `can_attach_tool_to`.

#[test]
fn test_leaf_cape_gives_no_hp_to_non_grass_holder() {
    // Charmander (Fire, 60 HP) holding Leaf Cape ([G] +30 HP) should stay at 60.
    let game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1033Charmander)
            .with_tool(get_card_by_enum(CardId::A3147LeafCape))],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    assert_eq!(game.get_state_clone().get_active(0).get_remaining_hp(), 60);
}

#[test]
fn test_elegant_cape_gives_no_hp_to_non_stage_1_holder() {
    // Bulbasaur (Basic, 70 HP) holding Elegant Cape (Stage 1 +30 HP) should stay at 70.
    let game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_tool(get_card_by_enum(CardId::B3b065ElegantCape))],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    assert_eq!(game.get_state_clone().get_active(0).get_remaining_hp(), 70);
}

#[test]
fn test_metal_core_barrier_gives_no_reduction_to_non_metal_holder() {
    // Bulbasaur (Grass) holding Metal Core Barrier ([M] -50 damage) takes full damage.
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_tool(get_card_by_enum(CardId::B2148MetalCoreBarrier))],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1001Bulbasaur, 0), // Vine Whip, 40
        is_stack: false,
    });

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        70 - 40,
        "Metal Core Barrier should not reduce damage on a non-Metal holder"
    );
}
