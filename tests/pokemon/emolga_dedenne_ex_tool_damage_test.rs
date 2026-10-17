use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Emolga's Windup Thunder does 30 damage for each Pokémon Tool attached to all
/// of your Pokémon in play (active + bench).
#[test]
fn test_emolga_windup_thunder_damage_scales_with_tools() {
    let rocky_helmet = get_card_by_enum(CardId::A2148RockyHelmet);
    let giant_cape = get_card_by_enum(CardId::A2147GiantCape);

    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B3b023Emolga)
                .with_energy(vec![EnergyType::Lightning, EnergyType::Lightning])
                .with_tool(rocky_helmet),
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_tool(giant_cape),
        ],
        vec![
            PlayedCard::from_id(CardId::A1004VenusaurEx),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
    );

    // 2 tools attached → 30 × 2 = 60 damage
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3b023Emolga, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let opponent_active_hp = state.get_active(1).get_remaining_hp();
    let venusaur_ex_max_hp = 190;
    assert_eq!(opponent_active_hp, venusaur_ex_max_hp - 60);
}

/// With no tools attached, Windup Thunder deals 0 damage.
#[test]
fn test_emolga_windup_thunder_no_damage_without_tools() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B3b023Emolga)
                .with_energy(vec![EnergyType::Lightning, EnergyType::Lightning]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1004VenusaurEx)],
    );

    // 0 tools → 30 × 0 = 0 damage
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3b023Emolga, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let opponent_active_hp = state.get_active(1).get_remaining_hp();
    let venusaur_ex_max_hp = 190;
    assert_eq!(opponent_active_hp, venusaur_ex_max_hp);
}

/// Dedenne ex's Dede-Circuit does 40 damage for each Pokémon Tool attached to
/// all of your Pokémon in play.
#[test]
fn test_dedenne_ex_dede_circuit_damage_scales_with_tools() {
    let rocky_helmet = get_card_by_enum(CardId::A2148RockyHelmet);

    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B3b024DedenneEx)
                .with_energy(vec![EnergyType::Lightning, EnergyType::Lightning]),
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_tool(rocky_helmet),
        ],
        vec![
            PlayedCard::from_id(CardId::A1004VenusaurEx),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
    );

    // 1 tool on benched Bulbasaur → 40 × 1 = 40 damage
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3b024DedenneEx, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let opponent_active_hp = state.get_active(1).get_remaining_hp();
    let venusaur_ex_max_hp = 190;
    assert_eq!(opponent_active_hp, venusaur_ex_max_hp - 40);
}
