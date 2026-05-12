use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::PlayedCard,
    test_support::get_initialized_game,
};

/// Test Brambleghast's Accept Pain ability moves 30 damage from Active to itself
#[test]
fn test_brambleghast_accept_pain_moves_30_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Active has 40 damage; Brambleghast on bench should absorb 30 of it
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_damage(40),
            PlayedCard::from_id(CardId::B3073Brambleghast),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    game.set_state(state);

    let ability_action = Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 1 },
        is_stack: false,
    };
    game.apply_action(&ability_action);

    let final_state = game.get_state_clone();

    // Bulbasaur (70 HP) had 40 damage; after moving 30 it has 10 damage left => 60 HP remaining
    let active_hp = final_state.get_active(0).get_remaining_hp();
    assert_eq!(
        active_hp, 60,
        "Active Bulbasaur should have 60 HP remaining after Accept Pain"
    );

    // Brambleghast (90 HP) should have taken 30 damage => 60 HP remaining
    let brambleghast_hp = final_state
        .enumerate_bench_pokemon(0)
        .next()
        .unwrap()
        .1
        .get_remaining_hp();
    assert_eq!(
        brambleghast_hp, 60,
        "Brambleghast should have 60 HP after absorbing 30 damage"
    );
}

/// Test Accept Pain cannot be used when the Active Pokémon has less than 30 damage
#[test]
fn test_brambleghast_accept_pain_not_available_without_enough_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Active has only 20 damage — not enough to move 30
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_damage(20),
            PlayedCard::from_id(CardId::B3073Brambleghast),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    game.set_state(state);

    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    let can_use_ability = actions
        .iter()
        .any(|a| matches!(a.action, SimpleAction::UseAbility { in_play_idx: 1 }));
    assert!(
        !can_use_ability,
        "Accept Pain should not be available when Active has fewer than 30 damage"
    );
}
