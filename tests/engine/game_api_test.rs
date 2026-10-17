use deckgym::{
    card_ids::CardId,
    models::{PlayedCard, StatusCondition},
    players::{AttachAttackPlayer, EndTurnPlayer, MctsPlayer, Player, RandomPlayer},
    state::GameOutcome,
    test_support::{get_initialized_game, init_random_players, load_test_decks},
};
use rand::{rngs::StdRng, SeedableRng};

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

/// A Knock Out that gives the opponent their 3rd point ends the game before any promotion is
/// queued, so the Active Spot stays empty. MCTS expands such terminal children and must not
/// generate moves for them: free-play move generation assumes an Active Pokémon (it used to reach
/// ability checks like Baxcalibur's Ice Maker that read it).
#[test]
fn test_mcts_handles_game_ending_checkup_knockout() {
    let mut game = get_initialized_game(0);
    game.play_until_stable();

    // Player 1 is deciding on the last turn. If they end it, Checkup Poison Knocks Out player 0's
    // Raikou ex (10 HP left), giving player 1 their 2nd and 3rd points. Past the turn limit no
    // turn-start draw is queued, so the finished state has player 0 to act with an empty Active
    // Spot, Baxcalibur on the Bench, and nothing on the move generation stack.
    let mut state = game.get_state_clone();
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A4a025RaikouEx).with_remaining_hp(10),
            PlayedCard::from_id(CardId::B2a036Baxcalibur),
        ],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
    );
    state.apply_status_condition(0, 0, StatusCondition::Poisoned);
    state.current_player = 1;
    state.turn_count = 30;
    state.points = [0, 1];

    let (actor, actions) = state.generate_possible_actions();
    assert_eq!(actor, 1);
    assert!(
        actions.len() > 1,
        "MCTS is only consulted with a real choice"
    );
    let (_, deck_b) = load_test_decks();
    let mut mcts = MctsPlayer::new(deck_b, 100);
    let mut rng = StdRng::seed_from_u64(0);
    let chosen = mcts.decision_fn(&mut rng, &state, &actions);
    assert!(actions.contains(&chosen));
}
