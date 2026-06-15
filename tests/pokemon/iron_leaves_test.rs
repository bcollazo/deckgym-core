use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

fn played_card_with_base_hp(card_id: CardId, base_hp: u32) -> PlayedCard {
    let card = deckgym::database::get_card_by_enum(card_id);
    PlayedCard::new(card, 0, base_hp, vec![], false, vec![])
}

/// Test Iron Leaves' Avenging Edge base damage (50) when no KO happened last turn
#[test]
fn test_iron_leaves_avenging_edge_base_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B3a028IronLeaves).with_energy(vec![
                EnergyType::Psychic,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.set_knocked_out_by_opponent_attack_last_turn(false);
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3a028IronLeaves, 0),
        is_stack: false,
    });

    let final_state = game.get_state_clone();
    let opponent_hp = final_state.get_active(1).get_remaining_hp();
    // Bulbasaur has 70 HP; 70 - 50 = 20
    assert_eq!(opponent_hp, 20, "Avenging Edge should deal 50 base damage");
}

/// Test Iron Leaves' Avenging Edge boosted damage (50 + 50 = 100) when KO happened last turn
#[test]
fn test_iron_leaves_avenging_edge_boosted_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B3a028IronLeaves).with_energy(vec![
                EnergyType::Psychic,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
        ],
        vec![played_card_with_base_hp(CardId::A1001Bulbasaur, 150)],
    );
    state.current_player = 0;
    state.set_knocked_out_by_opponent_attack_last_turn(true);
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3a028IronLeaves, 0),
        is_stack: false,
    });

    let final_state = game.get_state_clone();
    let opponent_hp = final_state.get_active(1).get_remaining_hp();
    // 150 - (50 + 50) = 50
    assert_eq!(
        opponent_hp, 50,
        "Avenging Edge should deal 100 damage with KO bonus"
    );
}
