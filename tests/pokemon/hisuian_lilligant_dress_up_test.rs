use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_hisuian_lilligant_dress_up_base_damage_without_tool() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::B3b007HisuianLilligant)
            .with_energy(vec![EnergyType::Grass])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3b007HisuianLilligant, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        30,
        "Dress Up should deal 40 damage without a Tool (Bulbasaur has 70 HP)"
    );
}

#[test]
fn test_hisuian_lilligant_dress_up_extra_damage_with_tool() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::B3b007HisuianLilligant)
            .with_energy(vec![EnergyType::Grass])
            .with_tool(get_card_by_enum(CardId::A2148RockyHelmet))],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3b007HisuianLilligant, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        10,
        "Dress Up should deal 60 damage with a Tool attached (Bulbasaur has 70 HP)"
    );
}
