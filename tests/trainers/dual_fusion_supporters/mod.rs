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
