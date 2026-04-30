use crate::deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard, StatusCondition},
    test_support::get_initialized_game,
};

fn steel_apron_trainer() -> crate::deckgym::models::TrainerCard {
    match get_card_by_enum(CardId::A4153SteelApron) {
        Card::Trainer(card) => card,
        _ => panic!("Expected Steel Apron to be a trainer card"),
    }
}

#[test]
fn test_steel_apron_only_attaches_to_metal() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1181Meltan),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.hands[0] = vec![get_card_by_enum(CardId::A4153SteelApron)];
    game.set_state(state);

    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: steel_apron_trainer(),
        },
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
}

#[test]
fn test_steel_apron_cures_existing_status_on_attach() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1181Meltan)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.hands[0] = vec![get_card_by_enum(CardId::A4153SteelApron)];
    state.apply_status_condition(0, 0, StatusCondition::Poisoned);
    state.apply_status_condition(0, 0, StatusCondition::Burned);
    game.set_state(state);

    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: steel_apron_trainer(),
        },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let state = game.get_state_clone();
    let (_actor, choices) = state.generate_possible_actions();
    let attach_action = choices
        .into_iter()
        .find(|choice| {
            matches!(
                choice.action,
                SimpleAction::AttachTool { in_play_idx: 0, .. }
            )
        })
        .expect("Expected Steel Apron attach choice for active Metal Pokemon");
    game.apply_action(&attach_action);

    let state = game.get_state_clone();
    let active = state.get_active(0);
    assert!(!active.is_poisoned());
    assert!(!active.is_paralyzed());
    assert!(!active.is_asleep());
    assert!(!active.is_burned());
    assert!(!active.is_confused());
}

#[test]
fn test_steel_apron_prevents_new_status_conditions() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1181Meltan)
            .with_tool(get_card_by_enum(CardId::A4153SteelApron))],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    game.set_state(state);

    let mut state = game.get_state_clone();
    state.apply_status_condition(0, 0, StatusCondition::Asleep);
    state.apply_status_condition(0, 0, StatusCondition::Confused);
    game.set_state(state);

    let state = game.get_state_clone();
    let active = state.get_active(0);
    assert!(!active.is_poisoned());
    assert!(!active.is_paralyzed());
    assert!(!active.is_asleep());
    assert!(!active.is_burned());
    assert!(!active.is_confused());
}

#[test]
fn test_steel_apron_reduces_damage_from_opponents_attack() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1181Meltan)
            .with_tool(get_card_by_enum(CardId::A4153SteelApron))],
    );
    state.current_player = 0;
    game.set_state(state);

    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        30,
        "Steel Apron should reduce Vine Whip from 40 to 30 damage"
    );
}
