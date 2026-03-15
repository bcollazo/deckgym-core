use common::init_random_players;
use deckgym::{
    players::{AttachAttackPlayer, EndTurnPlayer, MctsPlayer, Player, RandomPlayer},
    state::GameOutcome,
    test_helpers::load_test_decks,
};

mod common;

#[test]
fn test_game_api() {
    let players = init_random_players();
    let mut game = deckgym::Game::new(players, 0);
    game.play();
}

#[test]
fn test_mcts_player() {
    let (deck_a, deck_b) = load_test_decks();
    let player_a = Box::new(RandomPlayer { deck: deck_a });
    let player_b = Box::new(MctsPlayer::new(deck_b, 5));
    let players: Vec<Box<dyn Player>> = vec![player_a, player_b];
    let mut game = deckgym::Game::new(players, 6);

    // TODO: We segment the ticks like this so that this test can also be helpful
    // to print out the tree to .dot file and inspect it.
    // while game.get_state_clone().turn_count < 40 {
    //     game.play_tick();
    // }
    game.play();
}

#[test]
fn test_retreat_should_cure_poison() {
    let players = init_random_players();
    let mut game = deckgym::Game::new(players, 1406385978241804004);
    game.play();
}

#[test]
fn test_first_ko() {
    let (deck_a, deck_b) = load_test_decks();
    let player_a = Box::new(AttachAttackPlayer { deck: deck_a });
    let player_b = Box::new(EndTurnPlayer { deck: deck_b });
    let players: Vec<Box<dyn Player>> = vec![player_a, player_b];
    let mut game = deckgym::Game::new(players, 3);

    // On seed=3, AttachAttack goes first. So turn 3 should be the first attach. Bulbasaur
    // needs 2 energy, so on turn 5 is first attack, and turn 7 knocks out the opponent Koffing.
    while game.get_state_clone().turn_count < 7 {
        game.play_tick();
    }
    // Now play the rest. AA should win b.c. ET has no bench pokemon
    let winner = game.play();
    assert_eq!(game.get_state_clone().turn_count, 7);
    assert_eq!(winner, Some(GameOutcome::Win(0)));
}
