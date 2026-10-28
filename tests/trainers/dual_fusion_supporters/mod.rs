use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, PlayedCard},
    test_support::{get_test_game_with_board, play_trainer},
    Game,
};
fn pokemon(id: &str) -> PlayedCard {
    PlayedCard::from_id(CardId::from_card_id(id).unwrap())
}
fn board(ours: Vec<PlayedCard>) -> Game<'static> {
    get_test_game_with_board(
        ours,
        vec![pokemon("A1 211"), pokemon("A1 001"), pokemon("A1 033")],
    )
}
fn give(game: &mut Game<'static>, id: CardId) {
    let mut s = game.get_state_clone();
    s.hands[0] = vec![get_card_by_enum(id)];
    game.set_state(s);
}
fn offered(game: &mut Game<'static>, id: CardId) -> bool {
    give(game, id);
    game.get_state_clone().generate_possible_actions().1.iter().any(|a|matches!(&a.action,SimpleAction::Play{trainer_card} if trainer_card.id==get_card_by_enum(id).get_id()))
}
fn play(game: &mut Game<'static>, id: CardId) {
    assert!(offered(game, id), "{id:?} must be playable");
    let Card::Trainer(t) = get_card_by_enum(id) else {
        unreachable!()
    };
    play_trainer(game, 0, t);
}

mod lorelei;

mod blue;

mod serena;

mod rika;

mod mart;

mod rising;

#[test]
fn every_supporter_printing_is_implemented_including_all_twelve_new_printings() {
    use deckgym::card_validation::{get_implementation_status, ImplementationStatus};
    use deckgym::models::TrainerType;
    use strum::IntoEnumIterator;
    let mut new_printings = 0;
    for id in CardId::iter() {
        if let Card::Trainer(t) = get_card_by_enum(id) {
            if t.trainer_card_type == TrainerType::Supporter {
                assert_eq!(
                    get_implementation_status(id),
                    ImplementationStatus::Complete,
                    "{id:?}"
                );
                if t.id.starts_with("C1 ") {
                    new_printings += 1;
                }
            }
        }
    }
    assert_eq!(new_printings, 12);
}
