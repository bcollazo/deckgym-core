use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    generate_possible_actions,
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

    let player = state.current_player;
    let bulbasaur = get_card_by_enum(CardId::A1001Bulbasaur);
    let base_total_hp = 70;
    let base_remaining_hp = 70;

    state.in_play_pokemon[player][0] = Some(PlayedCard::new(
        bulbasaur,
        base_total_hp,
        base_remaining_hp,
        vec![],
        false,
        vec![],
    ));

    state.hands[player] = vec![get_card_by_enum(CardId::A2147GiantCape)];
    state.move_generation_stack.clear();
    game.set_state(state);

    let trainer_card = trainer_from_id(CardId::A2147GiantCape);
    let play_action = Action {
        actor: player,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let state = game.get_state_clone();
    let (_actor, choices) = generate_possible_actions(&state);
    let attach_action = Action {
        actor: player,
        action: attach_choice_for_idx(&choices, 0),
        is_stack: false,
    };
    game.apply_action(&attach_action);

    let state = game.get_state_clone();
    let active = state.in_play_pokemon[player][0].as_ref().unwrap();
    assert!(active.attached_tool.is_some());
    assert_eq!(active.total_hp, base_total_hp + 20);
    assert_eq!(active.remaining_hp, base_remaining_hp + 20);
}

#[test]
fn test_leaf_cape_only_attaches_to_grass() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    let player = state.current_player;
    let bulbasaur = get_card_by_enum(CardId::A1001Bulbasaur);
    let charmander = get_card_by_enum(CardId::A1033Charmander);
    let base_total_hp = 70;
    let base_remaining_hp = 70;

    state.in_play_pokemon[player][0] = Some(PlayedCard::new(
        bulbasaur,
        base_total_hp,
        base_remaining_hp,
        vec![],
        false,
        vec![],
    ));
    state.in_play_pokemon[player][1] = Some(PlayedCard::new(
        charmander,
        base_total_hp,
        base_remaining_hp,
        vec![],
        false,
        vec![],
    ));

    state.hands[player] = vec![get_card_by_enum(CardId::A3147LeafCape)];
    state.move_generation_stack.clear();
    game.set_state(state);

    let trainer_card = trainer_from_id(CardId::A3147LeafCape);
    let play_action = Action {
        actor: player,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let state = game.get_state_clone();
    let (_actor, choices) = generate_possible_actions(&state);

    let attachable_indices: Vec<usize> = choices
        .iter()
        .filter_map(|choice| match choice.action {
            SimpleAction::AttachTool { in_play_idx, .. } => Some(in_play_idx),
            _ => None,
        })
        .collect();

    assert_eq!(attachable_indices, vec![0]);

    let attach_action = Action {
        actor: player,
        action: attach_choice_for_idx(&choices, 0),
        is_stack: false,
    };
    game.apply_action(&attach_action);

    let state = game.get_state_clone();
    let active = state.in_play_pokemon[player][0].as_ref().unwrap();
    assert!(active.attached_tool.is_some());
    assert_eq!(active.total_hp, base_total_hp + 30);
    assert_eq!(active.remaining_hp, base_remaining_hp + 30);
}
