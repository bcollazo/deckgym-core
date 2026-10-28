use deckgym::{
    actions::SimpleAction,
    card_ids::CardId,
    models::{Card, PlayedCard},
    test_support::{
        get_initialized_game_with_board, get_test_game_with_board, play_trainer, trainer_from_id,
    },
};

/// Sweet Honey: "Choose 1 of your Pokémon, and then flip a coin until you get tails. For each
/// heads, heal 20 damage from that Pokémon."
#[test]
fn test_sweet_honey_heals_20_per_heads_until_tails() {
    let mut healed_amounts = std::collections::BTreeSet::new();
    for seed in 0..60u64 {
        let mut game = get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur).with_remaining_hp(10)],
            vec![PlayedCard::from_id(CardId::A1033Charmander)],
        );
        let honey = trainer_from_id(CardId::C1119SweetHoney);
        let mut state = game.get_state_clone();
        state.hands[0] = vec![Card::Trainer(honey.clone())];
        game.set_state(state);

        play_trainer(&mut game, 0, honey);

        let state = game.get_state_clone();
        let (_, actions) = state.generate_possible_actions();
        if let Some(heal) = actions
            .iter()
            .find(|a| matches!(a.action, SimpleAction::Heal { in_play_idx: 0, .. }))
        {
            game.apply_action(heal);
        }
        let state = game.get_state_clone();
        let healed = state.get_active(0).get_remaining_hp() - 10;
        assert_eq!(healed % 20, 0);
        assert!(state.hands[0].is_empty(), "Sweet Honey leaves the hand");
        healed_amounts.insert(healed);
    }
    assert!(
        healed_amounts.contains(&0),
        "tails first should heal nothing"
    );
    assert!(healed_amounts.contains(&20));
    assert!(healed_amounts.contains(&40));
}

/// Like Potion, Sweet Honey needs a damaged Pokémon to target.
#[test]
fn test_sweet_honey_needs_a_damaged_pokemon() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    let mut state = game.get_state_clone();
    state.hands[0] = vec![Card::Trainer(trainer_from_id(CardId::C1119SweetHoney))];
    game.set_state(state);
    let state = game.get_state_clone();
    let (_, actions) = state.generate_possible_actions();
    assert!(!actions
        .iter()
        .any(|a| matches!(a.action, SimpleAction::Play { .. })));

    // A damaged Benched Pokémon is enough to play it.
    let mut state = game.get_state_clone();
    state.in_play_pokemon[0][1] =
        Some(PlayedCard::from_id(CardId::A1033Charmander).with_remaining_hp(10));
    game.set_state(state);
    let state = game.get_state_clone();
    let (_, actions) = state.generate_possible_actions();
    assert!(actions
        .iter()
        .any(|a| matches!(a.action, SimpleAction::Play { .. })));
}
