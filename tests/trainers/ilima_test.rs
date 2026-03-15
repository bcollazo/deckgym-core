use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, PlayedCard, TrainerCard},
    state::GameOutcome,
};

mod common;

fn make_ilima_trainer_card() -> TrainerCard {
    match get_card_by_enum(CardId::A3149Ilima) {
        Card::Trainer(tc) => tc,
        _ => panic!("Expected trainer card"),
    }
}

fn make_damaged_colorless_active() -> PlayedCard {
    PlayedCard::from_id(CardId::A1186Pidgey).with_damage(30)
}

#[test]
fn test_ilima_last_pokemon_losses_game() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    // Player 0 has only one Pokemon (active), Player 1 has a normal setup
    state.set_board(
        vec![make_damaged_colorless_active()],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.hands[0].clear();

    let trainer_card = make_ilima_trainer_card();
    state.hands[0].push(Card::Trainer(trainer_card.clone()));
    game.set_state(state);

    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let state = game.get_state_clone();
    let (actor, choices) = state.generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(!choices.is_empty());

    game.apply_action(&choices[0]);

    let state = game.get_state_clone();
    assert_eq!(
        state.winner,
        Some(GameOutcome::Win(1)),
        "Player should lose if they Ilima their last Pokemon in play"
    );
}

#[test]
fn test_ilima_returns_active_and_triggers_promotion() {
    let mut game = get_initialized_game(1);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    // Player 0 has active + bench, Player 1 has a normal setup
    state.set_board(
        vec![
            make_damaged_colorless_active(),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    state.hands[0].clear();
    let trainer_card = make_ilima_trainer_card();
    state.hands[0].push(Card::Trainer(trainer_card.clone()));
    game.set_state(state);

    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let state = game.get_state_clone();
    let (_actor, choices) = state.generate_possible_actions();

    game.apply_action(&choices[0]);

    let state = game.get_state_clone();
    let pidgey_card = get_card_by_enum(CardId::A1186Pidgey);
    assert!(
        state.hands[0].contains(&pidgey_card),
        "Returned Pokemon should be in hand"
    );

    let (promo_actor, promo_choices) = state.generate_possible_actions();
    assert_eq!(promo_actor, 0);
    assert!(
        promo_choices
            .iter()
            .any(|action| matches!(action.action, SimpleAction::Activate { in_play_idx: 1, .. })),
        "Promotion choices should include Activate for bench index 1, choices: {:?}",
        promo_choices
    );
}
