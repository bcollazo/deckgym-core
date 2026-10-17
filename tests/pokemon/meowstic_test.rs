use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::PlayedCard,
    test_support::get_test_game_with_board,
};

#[test]
fn test_meowstic_perplexing_ears_confuses_opponent_active() {
    // Meowstic's Perplexing Ears: Once during your turn, if this Pokémon is in the Active Spot,
    // you may make your opponent's Active Pokémon Confused.
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B3066Meowstic)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    });

    assert!(
        game.get_state_clone().get_active(1).is_confused(),
        "Meowstic's Perplexing Ears should confuse the opponent's Active Pokémon"
    );
}

#[test]
fn test_meowstic_perplexing_ears_not_available_from_bench() {
    // Perplexing Ears requires Meowstic to be in the Active Spot.
    let game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::B3066Meowstic),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );

    let (actor, actions) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    let has_bench_ability = actions
        .iter()
        .any(|a| matches!(a.action, SimpleAction::UseAbility { in_play_idx: 1 }));
    assert!(
        !has_bench_ability,
        "Perplexing Ears should not be available when Meowstic is on the Bench"
    );
}
