use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::PlayedCard,
    test_support::get_initialized_game_with_board,
    Game,
};

/// Raichu (B4 050) – Evoshock: "Once during your turn, when you play this Pokémon from your hand
/// to evolve 1 of your Pokémon, you may flip a coin. If heads, your opponent's Active Pokémon is
/// now Paralyzed."
fn evolve_pikachu_into_raichu(seed: u64) -> Game<'static> {
    let mut game = get_initialized_game_with_board(
        seed,
        0,
        3,
        vec![PlayedCard::from_id(CardId::B4049Pikachu)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let mut state = game.get_state_clone();
    state.hands[0].clear();
    state.hands[0].push(get_card_by_enum(CardId::B4050Raichu));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Evolve {
            evolution: get_card_by_enum(CardId::B4050Raichu),
            in_play_idx: 0,
            from_deck: false,
        },
        is_stack: false,
    });

    game
}

#[test]
fn test_raichu_evoshock_is_optional_on_evolve() {
    let game = evolve_pikachu_into_raichu(0);

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert_eq!(choices.len(), 2, "Evoshock should be optional (2 choices)");
    assert!(
        choices
            .iter()
            .any(|c| matches!(c.action, SimpleAction::UseAbility { in_play_idx: 0 })),
        "Evoshock should be offered on the Pokémon that just evolved"
    );
    assert!(
        choices
            .iter()
            .any(|c| matches!(c.action, SimpleAction::Noop)),
        "Declining Evoshock should be an option"
    );
}

#[test]
fn test_raichu_evoshock_paralyzes_opponent_active_on_heads() {
    let mut saw_paralyzed = false;
    let mut saw_not_paralyzed = false;

    for seed in 0..20u64 {
        let mut game = evolve_pikachu_into_raichu(seed);

        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::UseAbility { in_play_idx: 0 },
            is_stack: true,
        });

        let state = game.get_state_clone();
        assert_eq!(state.get_active(0).get_name(), "Raichu");
        if state.get_active(1).is_paralyzed() {
            saw_paralyzed = true;
        } else {
            saw_not_paralyzed = true;
        }
    }

    assert!(
        saw_paralyzed,
        "expected at least one seed where Evoshock flipped heads and paralyzed the opponent"
    );
    assert!(
        saw_not_paralyzed,
        "expected at least one seed where Evoshock flipped tails and did nothing"
    );
}

#[test]
fn test_raichu_evoshock_declined_leaves_opponent_unaffected() {
    let mut game = evolve_pikachu_into_raichu(0);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Noop,
        is_stack: true,
    });

    assert!(
        !game.get_state_clone().get_active(1).is_paralyzed(),
        "Declining Evoshock should leave the opponent's Active Pokémon unaffected"
    );
}
