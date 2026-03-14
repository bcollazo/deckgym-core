use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    effects::CardEffect,
    models::{Card, EnergyType, PlayedCard},
};

#[path = "../common/mod.rs"]
mod common;

fn played_card_with_base_hp(card_id: CardId, base_hp: u32) -> PlayedCard {
    let card = get_card_by_enum(card_id);
    PlayedCard::new(card, 0, base_hp, vec![], false, vec![])
}

/// Test Porygon-Z B1a 058 - Cyberjack
/// Should deal 20 + (20 * number of Trainer cards in opponent's deck)
#[test]
fn test_porygonz_cyberjack() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B1a058PorygonZ).with_energy(vec![
                EnergyType::Colorless,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
        ],
        vec![played_card_with_base_hp(CardId::A1001Bulbasaur, 150)],
    );

    // Put 4 Trainer cards in opponent's deck
    state.decks[1].cards = vec![
        get_card_by_enum(CardId::A2b111PokeBall),
        get_card_by_enum(CardId::A4b373ProfessorsResearch),
        get_card_by_enum(CardId::A1223Giovanni),
        get_card_by_enum(CardId::PA001Potion),
    ];

    game.set_state(state);

    let action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };

    game.apply_action(&action);
    let state = game.get_state_clone();

    // Should deal 20 + (4 * 20) = 20 + 80 = 100 damage
    let opponent_active = state.get_active(1);
    assert_eq!(
        opponent_active.get_remaining_hp(),
        50,
        "Opponent should have 50 HP remaining (150 - 100)"
    );
}
