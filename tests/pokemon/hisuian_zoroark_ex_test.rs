use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_hisuian_zoroark_ex_spiteful_illusion_scales_with_own_discard_pile() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B3b060HisuianZoroarkEx).with_energy(vec![
                EnergyType::Colorless,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::A1004VenusaurEx)],
    );
    state.current_player = 0;
    state.discard_piles[0] = vec![
        get_card_by_enum(CardId::A1033Charmander),
        get_card_by_enum(CardId::A1053Squirtle),
    ];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3b060HisuianZoroarkEx, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        70,
        "Spiteful Illusion should do 80 + 20*2 = 120 damage for 2 Pokemon in own discard pile"
    );
}

#[test]
fn test_hisuian_zoroark_ex_spiteful_illusion_base_damage_with_empty_discard_pile() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B3b060HisuianZoroarkEx).with_energy(vec![
                EnergyType::Colorless,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::A1004VenusaurEx)],
    );
    state.current_player = 0;
    state.discard_piles[0] = vec![];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3b060HisuianZoroarkEx, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        110,
        "Spiteful Illusion should do 80 damage with an empty discard pile"
    );
}
