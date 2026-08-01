use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::PlayedCard,
    test_support::get_test_game_with_board,
};

/// Primarina's "Melodious Healing": "Once during your turn, you may heal 30 damage from each of
/// your [W] Pokémon." Only the [W] Pokemon heal, and only once per turn.
#[test]
fn test_melodious_healing_heals_only_your_water_pokemon() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A3048Primarina).with_damage(50),
            // Squirtle is [W]; Charmander is not.
            PlayedCard::from_id(CardId::A1053Squirtle).with_damage(40),
            PlayedCard::from_id(CardId::A1033Charmander).with_damage(40),
        ],
        vec![PlayedCard::from_id(CardId::A1055Blastoise).with_damage(60)],
    );

    let use_ability = Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    };
    game.apply_action(&use_ability);

    let state = game.get_state_clone();
    assert_eq!(state.get_remaining_hp(0, 0), 140 - 50 + 30);
    assert_eq!(state.get_remaining_hp(0, 1), 60 - 40 + 30);
    // Charmander is [R], so it is untouched.
    assert_eq!(state.get_remaining_hp(0, 2), 60 - 40);
    // The opponent's [W] Pokemon is not healed either.
    assert_eq!(state.get_remaining_hp(1, 0), 150 - 60);

    // The ability is once per turn, so it is no longer offered.
    let (_, actions) = state.generate_possible_actions();
    assert!(!actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::UseAbility { in_play_idx: 0 })));
}
