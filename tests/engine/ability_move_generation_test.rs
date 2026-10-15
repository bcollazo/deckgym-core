use deckgym::{
    actions::SimpleAction, card_ids::CardId, models::PlayedCard,
    test_support::get_test_game_with_board,
};

#[test]
fn test_discard_fossil_generated_once_per_fossil() {
    let game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1216HelixFossil),
        ],
        vec![PlayedCard::from_id(CardId::A1202Chansey)],
    );

    let (_, actions) = game.get_state_clone().generate_possible_actions();
    let discard_fossil_actions: Vec<_> = actions
        .iter()
        .filter(|a| matches!(a.action, SimpleAction::DiscardFossil { .. }))
        .collect();

    assert_eq!(discard_fossil_actions.len(), 1);
    assert_eq!(
        discard_fossil_actions[0].action,
        SimpleAction::DiscardFossil { in_play_idx: 1 }
    );
}

/// After a Knock Out, before promoting, the current player can have no Active Pokémon. Abilities
/// that look at the Active Pokémon must not panic during move generation in that state.
#[test]
fn test_active_dependent_abilities_with_no_active_do_not_panic() {
    let benched_ability_users = [
        CardId::B2a036Baxcalibur, // Ice Maker
        CardId::A3a062Celesteela, // Ultra Thrusters
        CardId::B1073GreninjaEx,  // Shifting Stream
        CardId::A1a019Vaporeon,   // Wash Out
    ];
    for card_id in benched_ability_users {
        let game = get_test_game_with_board(
            vec![
                PlayedCard::from_id(CardId::A1001Bulbasaur),
                PlayedCard::from_id(card_id),
            ],
            vec![PlayedCard::from_id(CardId::A1005Caterpie)],
        );
        let mut state = game.get_state_clone();
        state.in_play_pokemon[0][0] = None;

        let (_, actions) = state.generate_possible_actions();
        assert!(
            !actions
                .iter()
                .any(|a| matches!(a.action, SimpleAction::UseAbility { .. })),
            "{card_id:?} ability should not be usable without an Active Pokémon"
        );
    }
}
