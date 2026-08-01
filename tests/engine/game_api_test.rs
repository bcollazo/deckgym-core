use deckgym::{
    players::{AttachAttackPlayer, EndTurnPlayer, MctsPlayer, Player, RandomPlayer},
    state::GameOutcome,
    test_support::{init_random_players, load_test_decks},
};

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

/// A deck whose energy types resolve to [C] (a Colorless deck, or any deck file without an
/// explicit `Energy:` line whose Pokémon are Colorless) must be playable: `play_tick` colors
/// its log lines by the deck's first energy type, and every energy type needs a color.
#[test]
fn test_game_plays_deck_with_colorless_energy() {
    let colorless_deck = deckgym::Deck::from_string(
        "2 Furfrou B4 142\n\
         2 Professor's Research P-A 007\n\
         2 Poké Ball P-A 005\n\
         2 Potion P-A 001\n\
         2 X Speed P-A 002\n\
         2 Red Card P-A 006\n\
         2 Rare Candy A3 144\n\
         2 Copycat B1 225\n\
         2 Sabrina A1 225\n\
         2 Cyrus A2 150",
    )
    .expect("Valid Deck Format");
    let (_, deck_b) = load_test_decks();
    let players: Vec<Box<dyn Player>> = vec![
        Box::new(RandomPlayer {
            deck: colorless_deck,
        }),
        Box::new(RandomPlayer { deck: deck_b }),
    ];
    let mut game = deckgym::Game::new(players, 0);
    game.play();
}

/// Same for a [N] (Dragon) deck: Dragon Pokémon have no Energy of their own, so a deck file
/// without an explicit `Energy:` line resolves to [N] and must still be playable.
#[test]
fn test_game_plays_deck_with_dragon_energy() {
    let dragon_deck = deckgym::Deck::from_string(
        "Energy: Dragon\n\
         2 Furfrou B4 142\n\
         2 Professor's Research P-A 007\n\
         2 Poké Ball P-A 005\n\
         2 Potion P-A 001\n\
         2 X Speed P-A 002\n\
         2 Red Card P-A 006\n\
         2 Rare Candy A3 144\n\
         2 Copycat B1 225\n\
         2 Sabrina A1 225\n\
         2 Cyrus A2 150",
    )
    .expect("Valid Deck Format");
    let (_, deck_b) = load_test_decks();
    let players: Vec<Box<dyn Player>> = vec![
        Box::new(RandomPlayer { deck: dragon_deck }),
        Box::new(RandomPlayer { deck: deck_b }),
    ];
    let mut game = deckgym::Game::new(players, 0);
    game.play();
}
