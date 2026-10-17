use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

const VENUSAUR_EX_MAX_HP: u32 = 190;

fn rotom_ex_board() -> deckgym::Game<'static> {
    get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B4055RotomEx)
                .with_energy(vec![EnergyType::Lightning, EnergyType::Lightning]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1004VenusaurEx)],
    )
}

/// Rotom ex's Junk Spark does 30 damage plus 10 more for each Item card in your
/// discard pile.
#[test]
fn test_rotom_ex_junk_spark_scales_with_items_in_discard() {
    let mut game = rotom_ex_board();

    let mut state = game.get_state_clone();
    state.discard_piles[0] = vec![
        get_card_by_enum(CardId::PA005PokeBall),
        get_card_by_enum(CardId::PA005PokeBall),
        get_card_by_enum(CardId::A2146PokemonCommunication),
    ];
    game.set_state(state);

    // 3 Item cards in the discard pile → 30 + 10 × 3 = 60 damage.
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4055RotomEx, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        VENUSAUR_EX_MAX_HP - 60
    );
}

/// Only Item cards count: Supporters, Tools and Pokémon in the discard pile are ignored,
/// and the opponent's discard pile is irrelevant.
#[test]
fn test_rotom_ex_junk_spark_ignores_non_item_cards() {
    let mut game = rotom_ex_board();

    let mut state = game.get_state_clone();
    state.discard_piles[0] = vec![
        get_card_by_enum(CardId::PA005PokeBall),
        get_card_by_enum(CardId::A1219Erika),
        get_card_by_enum(CardId::A2148RockyHelmet),
        get_card_by_enum(CardId::A1001Bulbasaur),
    ];
    state.discard_piles[1] = vec![
        get_card_by_enum(CardId::PA005PokeBall),
        get_card_by_enum(CardId::PA005PokeBall),
    ];
    game.set_state(state);

    // Only the single Poké Ball counts → 30 + 10 × 1 = 40 damage.
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4055RotomEx, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        VENUSAUR_EX_MAX_HP - 40
    );
}
