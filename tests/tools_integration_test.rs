use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, PlayedCard},
};

mod common;

fn trainer_from_id(card_id: CardId) -> deckgym::models::TrainerCard {
    match get_card_by_enum(card_id) {
        Card::Trainer(trainer_card) => trainer_card,
        _ => panic!("Expected trainer card"),
    }
}

fn attach_choice_for_idx(actions: &[Action], in_play_idx: usize) -> SimpleAction {
    actions
        .iter()
        .find(|action| match action.action {
            SimpleAction::AttachTool {
                in_play_idx: idx, ..
            } => idx == in_play_idx,
            _ => false,
        })
        .map(|action| action.action.clone())
        .expect("Expected attach tool choice for target")
}

#[test]
fn test_giant_cape_attach_increases_hp() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    let base_remaining_hp = 70;

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;

    state.hands[0] = vec![get_card_by_enum(CardId::A2147GiantCape)];
    game.set_state(state);

    let trainer_card = trainer_from_id(CardId::A2147GiantCape);
    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let state = game.get_state_clone();
    let (_actor, choices) = state.generate_possible_actions();
    let attach_action = Action {
        actor: 0,
        action: attach_choice_for_idx(&choices, 0),
        is_stack: false,
    };
    game.apply_action(&attach_action);

    let state = game.get_state_clone();
    let active = state.get_active(0);
    assert!(active.attached_tool.is_some());
    assert_eq!(active.get_remaining_hp(), base_remaining_hp + 20);
}

#[test]
fn test_leaf_cape_only_attaches_to_grass() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    let base_remaining_hp = 70;

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;

    state.hands[0] = vec![get_card_by_enum(CardId::A3147LeafCape)];
    game.set_state(state);

    let trainer_card = trainer_from_id(CardId::A3147LeafCape);
    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let state = game.get_state_clone();
    let (_actor, choices) = state.generate_possible_actions();

    let attachable_indices: Vec<usize> = choices
        .iter()
        .filter_map(|choice| match choice.action {
            SimpleAction::AttachTool { in_play_idx, .. } => Some(in_play_idx),
            _ => None,
        })
        .collect();

    assert_eq!(attachable_indices, vec![0]);

    let attach_action = Action {
        actor: 0,
        action: attach_choice_for_idx(&choices, 0),
        is_stack: false,
    };
    game.apply_action(&attach_action);

    let state = game.get_state_clone();
    let active = state.get_active(0);
    assert!(active.attached_tool.is_some());
    assert_eq!(active.get_remaining_hp(), base_remaining_hp + 30);
}

#[test]
fn test_guzma_kos_pokemon_surviving_on_giant_cape() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_tool(get_card_by_enum(CardId::A2147GiantCape))
                .with_damage(80),
            PlayedCard::from_id(CardId::A1053Squirtle),
        ],
    );
    state.current_player = 0;
    state.turn_count = 3;
    state.points = [0, 0];
    state.hands[0] = vec![get_card_by_enum(CardId::A3151Guzma)];
    game.set_state(state);

    let trainer_card = trainer_from_id(CardId::A3151Guzma);
    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let state = game.get_state_clone();
    assert!(
        state.in_play_pokemon[1][0].is_none(),
        "Opponent active should be KO'd after Giant Cape is discarded"
    );
    assert_eq!(state.points[0], 1, "Player 0 should gain 1 point");

    let (actor, choices) = state.generate_possible_actions();
    assert_eq!(actor, 1, "Opponent should be prompted to promote");
    let activate_action = choices
        .iter()
        .find(|action| matches!(action.action, SimpleAction::Activate { .. }))
        .expect("Expected Activate action for promotion");
    game.apply_action(activate_action);

    let state = game.get_state_clone();
    assert!(
        state.in_play_pokemon[1][0].is_some(),
        "Opponent should have a promoted active Pokemon"
    );
}

#[test]
fn test_ko_discards_attached_tool() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_tool(get_card_by_enum(CardId::A2147GiantCape))],
    );
    state.current_player = 0;
    state.turn_count = 3;
    state.points = [0, 0];
    game.set_state(state);

    let damage_action = Action {
        actor: 0,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (0, 0),
            targets: vec![(100, 1, 0)],
            is_from_active_attack: true,
        },
        is_stack: false,
    };
    game.apply_action(&damage_action);

    let state = game.get_state_clone();
    assert!(state.in_play_pokemon[1][0].is_none());
    assert!(
        state.discard_piles[1]
            .iter()
            .any(|card| *card == get_card_by_enum(CardId::A2147GiantCape)),
        "Expected attached tool to be discarded on KO"
    );
}
