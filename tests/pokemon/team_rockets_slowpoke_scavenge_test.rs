use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

fn scavenge_game(discard: Vec<deckgym::models::Card>) -> deckgym::Game<'static> {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4a025TeamRocketsSlowpoke)
            .with_energy(vec![EnergyType::Psychic])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    let mut state = game.get_state_clone();
    state.discard_piles[0] = discard;
    state.hands[0].clear();
    game.set_state(state);
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4a025TeamRocketsSlowpoke, 0),
        is_stack: false,
    });
    game
}

/// Scavenge puts a random Item card from your discard pile into your hand, leaving other card
/// types behind.
#[test]
fn test_scavenge_recovers_an_item_from_the_discard_pile() {
    let game = scavenge_game(vec![
        get_card_by_enum(CardId::A1211Snorlax),
        get_card_by_enum(CardId::A2b111PokeBall),
    ]);

    let state = game.get_state_clone();
    assert_eq!(state.hands[0].len(), 1);
    assert_eq!(state.hands[0][0].get_name(), "Poké Ball");
    assert_eq!(state.discard_piles[0].len(), 1);
    assert_eq!(state.discard_piles[0][0].get_name(), "Snorlax");
}

/// With no Item cards in the discard pile, Scavenge does nothing.
#[test]
fn test_scavenge_without_items_does_nothing() {
    let game = scavenge_game(vec![get_card_by_enum(CardId::A1211Snorlax)]);

    let state = game.get_state_clone();
    assert!(state.hands[0].is_empty());
    assert_eq!(state.discard_piles[0].len(), 1);
}
