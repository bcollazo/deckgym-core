use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, PlayedCard},
    test_support::get_initialized_game,
};

fn make_trainer_card(card_id: CardId) -> deckgym::models::TrainerCard {
    get_card_by_enum(card_id).as_trainer()
}

#[test]
fn test_elesa_returns_all_tools_to_owners_hand() {
    // Elesa: "Return all Pokémon Tools attached to each Pokémon (both yours and your
    // opponent's) to their owner's hand."
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_tool(get_card_by_enum(CardId::A2148RockyHelmet)),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)
            .with_tool(get_card_by_enum(CardId::A2148RockyHelmet))],
    );

    let elesa = make_trainer_card(CardId::B3b066Elesa);
    state.hands[0] = vec![Card::Trainer(elesa.clone())];
    game.set_state(state);

    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: elesa,
        },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let state = game.get_state_clone();

    let player_active = state.in_play_pokemon[0][0]
        .as_ref()
        .expect("Player active should remain");
    assert!(
        player_active.attached_tool.is_none(),
        "Rocky Helmet should have been detached from player's Bulbasaur"
    );
    let opponent_active = state.in_play_pokemon[1][0]
        .as_ref()
        .expect("Opponent active should remain");
    assert!(
        opponent_active.attached_tool.is_none(),
        "Rocky Helmet should have been detached from opponent's Charmander"
    );

    let player_hand_tool_count = state.hands[0]
        .iter()
        .filter(|c| matches!(c, Card::Trainer(tc) if tc.id == "A2 148"))
        .count();
    assert_eq!(
        player_hand_tool_count, 1,
        "Player should get their Rocky Helmet back in hand"
    );

    let opponent_hand_tool_count = state.hands[1]
        .iter()
        .filter(|c| matches!(c, Card::Trainer(tc) if tc.id == "A2 148"))
        .count();
    assert_eq!(
        opponent_hand_tool_count, 1,
        "Opponent should get their Rocky Helmet back in hand"
    );
}

#[test]
fn test_elesa_knocks_out_opponent_active_hanging_on_via_giant_cape() {
    // Opponent's Bulbasaur (70 base HP) has a Giant Cape (+20 HP) and 70 damage on it, so it's
    // sitting at 20 remaining HP only because of the Giant Cape. Once Elesa returns the Giant
    // Cape to hand, its effective HP drops back to 70 and the existing 70 damage counters
    // knock it out, awarding a point and requiring the opponent to promote from the bench.
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_tool(get_card_by_enum(CardId::A2147GiantCape))
                .with_damage(70),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
    );

    let elesa = make_trainer_card(CardId::B3b066Elesa);
    state.hands[0] = vec![Card::Trainer(elesa.clone())];
    game.set_state(state);

    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: elesa,
        },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let state = game.get_state_clone();
    assert!(
        state.in_play_pokemon[1][0].is_none(),
        "Opponent's Bulbasaur should have been knocked out and removed from the active spot"
    );
    assert_eq!(
        state.points[0], 1,
        "Player should have won 1 point for knocking out opponent's Bulbasaur"
    );

    // Opponent must now promote from the bench.
    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 1);
    assert!(choices.iter().all(|choice| {
        matches!(
            choice.action,
            SimpleAction::Activate {
                player: 1,
                in_play_idx: _
            }
        )
    }));
}

#[test]
fn test_elesa_knocks_out_opponent_active_hanging_on_via_leaf_cape() {
    // Leaf Cape only attaches to [G] Pokémon, so use Oddish (60 base HP, Grass). It has a Leaf
    // Cape (+30 HP) and 60 damage on it, so it's sitting at 30 remaining HP only because of the
    // Leaf Cape. Once Elesa returns the Leaf Cape to hand, its effective HP drops back to 60 and
    // the existing 60 damage counters knock it out, awarding a point and requiring the opponent
    // to promote from the bench.
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
        vec![
            PlayedCard::from_id(CardId::A1011Oddish)
                .with_tool(get_card_by_enum(CardId::A3147LeafCape))
                .with_damage(60),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );

    let elesa = make_trainer_card(CardId::B3b066Elesa);
    state.hands[0] = vec![Card::Trainer(elesa.clone())];
    game.set_state(state);

    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: elesa,
        },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let state = game.get_state_clone();
    assert!(
        state.in_play_pokemon[1][0].is_none(),
        "Opponent's Oddish should have been knocked out and removed from the active spot"
    );
    assert_eq!(
        state.points[0], 1,
        "Player should have won 1 point for knocking out opponent's Oddish"
    );

    // Opponent must now promote from the bench.
    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 1);
    assert!(choices.iter().all(|choice| {
        matches!(
            choice.action,
            SimpleAction::Activate {
                player: 1,
                in_play_idx: _
            }
        )
    }));
}

#[test]
fn test_elesa_no_tools_attached_does_nothing() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );

    let elesa = make_trainer_card(CardId::B3b066Elesa);
    state.hands[0] = vec![Card::Trainer(elesa.clone())];
    game.set_state(state);

    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: elesa,
        },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let state = game.get_state_clone();
    assert!(state.hands[0].is_empty(), "No tools should return to hand");
    assert!(
        !state.hands[1].iter().any(|c| matches!(c, Card::Trainer(tc) if tc.trainer_card_type == deckgym::models::TrainerType::Tool)),
        "No tools should return to hand"
    );
}
