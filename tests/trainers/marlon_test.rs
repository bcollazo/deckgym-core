use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, PlayedCard},
    test_support::get_initialized_game,
};

/// Marlon: "Heal 70 damage from 1 of your Carracosta or Jellicent."
#[test]
fn test_marlon_heals_carracosta() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    // A damaged Carracosta should be a valid Marlon target.
    state.set_board(
        vec![PlayedCard::from_id(CardId::B1067Carracosta).with_remaining_hp(40)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let marlon = match get_card_by_enum(CardId::B1221Marlon) {
        Card::Trainer(tc) => tc,
        _ => panic!("Expected trainer card"),
    };
    state.hands[0].push(Card::Trainer(marlon.clone()));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: marlon,
        },
        is_stack: false,
    });

    // Choose to heal Carracosta (index 0).
    let state = game.get_state_clone();
    let (_actor, actions) = state.generate_possible_actions();
    let heal_action = actions
        .iter()
        .find(|a| matches!(a.action, SimpleAction::Heal { in_play_idx: 0, .. }))
        .expect("Should have a heal action for Carracosta");
    game.apply_action(heal_action);

    assert_eq!(
        game.get_state_clone().get_active(0).get_remaining_hp(),
        110,
        "Marlon should heal Carracosta by 70 (40 + 70)"
    );
}

/// Marlon can only target Carracosta or Jellicent, not other Pokémon.
#[test]
fn test_marlon_only_targets_carracosta_or_jellicent() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    // Active is a damaged Bulbasaur (not a valid target); bench has a damaged Jellicent.
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_remaining_hp(10),
            PlayedCard::from_id(CardId::B1069Jellicent).with_remaining_hp(40),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let marlon = match get_card_by_enum(CardId::B1221Marlon) {
        Card::Trainer(tc) => tc,
        _ => panic!("Expected trainer card"),
    };
    state.hands[0].push(Card::Trainer(marlon.clone()));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: marlon,
        },
        is_stack: false,
    });

    // Only Jellicent (index 1) should be a valid heal target.
    let state = game.get_state_clone();
    let (_actor, actions) = state.generate_possible_actions();
    let heal_targets: Vec<usize> = actions
        .iter()
        .filter_map(|a| match a.action {
            SimpleAction::Heal { in_play_idx, .. } => Some(in_play_idx),
            _ => None,
        })
        .collect();
    assert_eq!(
        heal_targets,
        vec![1],
        "Only the benched Jellicent should be a valid Marlon target"
    );

    let heal_action = actions
        .iter()
        .find(|a| matches!(a.action, SimpleAction::Heal { in_play_idx: 1, .. }))
        .expect("Should have a heal action for Jellicent");
    game.apply_action(heal_action);

    let state = game.get_state_clone();
    assert_eq!(
        state.in_play_pokemon[0][1]
            .as_ref()
            .expect("Jellicent should still be in play")
            .get_remaining_hp(),
        110,
        "Marlon should heal Jellicent by 70 (40 + 70)"
    );
}
