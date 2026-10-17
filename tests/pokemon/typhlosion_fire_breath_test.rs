use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::PlayedCard,
    test_support::get_test_game_with_board,
};

/// Fire Breath: "Once during your turn, you may make your opponent's Active Pokémon Burned."
/// Typhlosion doesn't need to be in the Active Spot to use it.
#[test]
fn test_typhlosion_fire_breath_burns_opponent_active_from_bench() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A4029Typhlosion),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let ability_action = Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 1 },
        is_stack: false,
    };
    game.apply_action(&ability_action);

    let state = game.get_state_clone();
    assert!(
        state.get_active(1).is_burned(),
        "Opponent's Active Pokémon should be Burned after Fire Breath"
    );
}

#[test]
fn test_typhlosion_fire_breath_only_once_per_turn() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A4029Typhlosion)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let ability_action = Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    };
    game.apply_action(&ability_action);

    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    let can_use_ability_again = actions
        .iter()
        .any(|a| matches!(a.action, SimpleAction::UseAbility { in_play_idx: 0 }));
    assert!(
        !can_use_ability_again,
        "Fire Breath should only be usable once per turn"
    );
}
