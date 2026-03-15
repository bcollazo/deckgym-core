use common::init_decks;

mod common;

#[test]
fn test_hitmonlee_blastoise() {
    for _ in 0..10 {
        let players = init_decks("hitmonlee.txt", "blastoiseex.txt");
        let mut game = deckgym::Game::new(players, rand::random::<u64>());
        game.play();
    }
}
