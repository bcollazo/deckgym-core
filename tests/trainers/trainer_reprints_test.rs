//! A reprint of a Trainer plays exactly like its first printing, with no code of its own.
//!
//! Trainer logic is keyed by effect text (`deckgym::trainer_identity::canonical_trainer_id`), so
//! the Deluxe Pack: Mega (B4b) reprints below were never named anywhere in the engine.

use deckgym::{
    actions::SimpleAction,
    card_ids::CardId,
    card_validation::{get_implementation_status, ImplementationStatus},
    database::get_card_by_enum,
    models::{Card, PlayedCard},
    test_support::{get_test_game_with_board, play_trainer, trainer_from_id},
    Game,
};
use strum::IntoEnumIterator;

fn board() -> Game<'static> {
    get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    )
}

/// Hand = the one Trainer, opponent hand = four Bulbasaur; the playable moves on offer.
fn offered_moves(game: &mut Game<'static>, card_id: CardId) -> Vec<SimpleAction> {
    let trainer = trainer_from_id(card_id);
    let mut state = game.get_state_clone();
    state.hands[0] = vec![Card::Trainer(trainer)];
    state.hands[1] = vec![Card::Pokemon(pokemon(CardId::A1001Bulbasaur)); 4];
    game.set_state(state);
    let (_, actions) = game.get_state_clone().generate_possible_actions();
    actions.into_iter().map(|a| a.action).collect()
}

fn pokemon(card_id: CardId) -> deckgym::models::PokemonCard {
    match get_card_by_enum(card_id) {
        Card::Pokemon(p) => p,
        _ => panic!("Expected a Pokémon"),
    }
}

/// Copycat: "Shuffle your hand into your deck. Draw a card for each card in your opponent's
/// hand." B4b 424 (the ☆☆ reprint) must be offered and must do the same as B1 225.
#[test]
fn copycat_reprint_is_offered_and_draws_like_the_first_printing() {
    for card_id in [
        CardId::B1225Copycat,
        CardId::B4b225Copycat,
        CardId::B4b424Copycat,
    ] {
        let mut game = board();
        let moves = offered_moves(&mut game, card_id);
        assert!(
            moves.iter().any(|m| matches!(m, SimpleAction::Play { trainer_card } if trainer_card.id == trainer_from_id(card_id).id)),
            "{card_id:?} should be offered as a play"
        );
        play_trainer(&mut game, 0, trainer_from_id(card_id));
        let state = game.get_state_clone();
        assert_eq!(
            state.hands[0].len(),
            4,
            "{card_id:?} should draw one card per opponent card"
        );
    }
}

/// Every Trainer printing in the database has an implementation, reprints included. A set that
/// reprints an old Supporter must not reintroduce "Trainer logic not implemented".
#[test]
fn every_trainer_printing_is_implemented() {
    let missing: Vec<CardId> = CardId::iter()
        .filter(|id| matches!(get_card_by_enum(*id), Card::Trainer(_)))
        .filter(|id| get_implementation_status(*id) != ImplementationStatus::Complete)
        .collect();
    assert!(missing.is_empty(), "Trainers without logic: {missing:?}");
}
