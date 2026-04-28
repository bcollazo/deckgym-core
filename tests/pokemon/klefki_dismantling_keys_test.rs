use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::PlayedCard,
    test_support::get_initialized_game,
};

fn has_use_ability(actions: &[Action], in_play_idx: usize) -> bool {
    actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::UseAbility { in_play_idx: idx } if idx == in_play_idx))
}

#[test]
fn test_klefki_dismantling_keys_discards_opponent_active_tool_and_self() {
    let rocky_helmet = get_card_by_enum(CardId::A2148RockyHelmet);
    let klefki = get_card_by_enum(CardId::B1120Klefki);

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::B1120Klefki),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander).with_tool(rocky_helmet.clone())],
    );
    game.set_state(state);

    let (actor, actions) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(has_use_ability(&actions, 1));

    let ability_action = actions
        .iter()
        .find(|action| matches!(action.action, SimpleAction::UseAbility { in_play_idx: 1 }))
        .expect("Dismantling Keys should be available from the Bench")
        .clone();
    game.apply_action(&ability_action);

    let state = game.get_state_clone();
    assert!(state.get_active(1).attached_tool.is_none());
    assert!(state.discard_piles[1].contains(&rocky_helmet));
    assert!(state.in_play_pokemon[0][1].is_none());
    assert!(state.discard_piles[0].contains(&klefki));
    assert_eq!(state.points, [0, 0]);
}

#[test]
fn test_klefki_dismantling_keys_requires_bench_and_opponent_active_tool() {
    let rocky_helmet = get_card_by_enum(CardId::A2148RockyHelmet);

    let mut active_klefki_game = get_initialized_game(0);
    let mut active_klefki_state = active_klefki_game.get_state_clone();
    active_klefki_state.current_player = 0;
    active_klefki_state.set_board(
        vec![PlayedCard::from_id(CardId::B1120Klefki)],
        vec![PlayedCard::from_id(CardId::A1033Charmander).with_tool(rocky_helmet)],
    );
    active_klefki_game.set_state(active_klefki_state);

    let (_actor, actions) = active_klefki_game
        .get_state_clone()
        .generate_possible_actions();
    assert!(!has_use_ability(&actions, 0));

    let mut no_tool_game = get_initialized_game(0);
    let mut no_tool_state = no_tool_game.get_state_clone();
    no_tool_state.current_player = 0;
    no_tool_state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::B1120Klefki),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    no_tool_game.set_state(no_tool_state);

    let (_actor, actions) = no_tool_game.get_state_clone().generate_possible_actions();
    assert!(!has_use_ability(&actions, 1));
}

#[test]
fn test_klefki_dismantling_keys_resolves_ko_from_lost_giant_cape() {
    let giant_cape = get_card_by_enum(CardId::A2147GiantCape);

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;
    state.points = [0, 0];
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::B1120Klefki),
        ],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_tool(giant_cape.clone())
                .with_damage(80),
            PlayedCard::from_id(CardId::A1053Squirtle),
        ],
    );
    game.set_state(state);

    let ability_action = game
        .get_state_clone()
        .generate_possible_actions()
        .1
        .into_iter()
        .find(|action| matches!(action.action, SimpleAction::UseAbility { in_play_idx: 1 }))
        .expect("Dismantling Keys should be available from the Bench");
    game.apply_action(&ability_action);

    let state = game.get_state_clone();
    assert!(state.in_play_pokemon[1][0].is_none());
    assert!(state.discard_piles[1].contains(&giant_cape));
    assert!(state.in_play_pokemon[0][1].is_none());
    assert_eq!(state.points[0], 1);

    let (actor, actions) = state.generate_possible_actions();
    assert_eq!(actor, 1);
    assert!(actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::Activate { player: 1, .. })));
}
