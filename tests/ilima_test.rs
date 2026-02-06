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
    let current_player = state.current_player;
    let opponent = (current_player + 1) % 2;

    state.in_play_pokemon[current_player] = [None, None, None, None];
    state.in_play_pokemon[current_player][0] = Some(make_damaged_colorless_active());
    state.hands[current_player].clear();

    let trainer_card = make_ilima_trainer_card();
    state.hands[current_player].push(Card::Trainer(trainer_card.clone()));
    game.set_state(state);

    let play_action = Action {
        actor: current_player,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let state = game.get_state_clone();
    let (actor, choices) = state
        .move_generation_stack
        .last()
        .expect("Ilima should prompt for a target");
    assert_eq!(*actor, current_player);
    assert!(!choices.is_empty());

    let pick_action = Action {
        actor: current_player,
        action: choices[0].clone(),
        is_stack: true,
    };
    game.apply_action(&pick_action);

    let state = game.get_state_clone();
    assert_eq!(
        state.winner,
        Some(GameOutcome::Win(opponent)),
        "Player should lose if they Ilima their last Pokemon in play"
    );
}

#[test]
fn test_ilima_returns_active_and_triggers_promotion() {
    let mut game = get_initialized_game(1);
    let mut state = game.get_state_clone();
    state.move_generation_stack.clear();
    let current_player = state.current_player;

    state.in_play_pokemon[current_player] = [None, None, None, None];
    state.in_play_pokemon[current_player][0] = Some(make_damaged_colorless_active());

    state.in_play_pokemon[current_player][1] = Some(PlayedCard::from_id(CardId::A1001Bulbasaur));

    state.hands[current_player].clear();
    let trainer_card = make_ilima_trainer_card();
    state.hands[current_player].push(Card::Trainer(trainer_card.clone()));
    game.set_state(state);

    let play_action = Action {
        actor: current_player,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let state = game.get_state_clone();
    let (_actor, choices) = state
        .move_generation_stack
        .last()
        .expect("Ilima should prompt for a target");

    let pick_action = Action {
        actor: current_player,
        action: choices[0].clone(),
        is_stack: true,
    };
    game.apply_action(&pick_action);

    let state = game.get_state_clone();
    let pidgey_card = get_card_by_enum(CardId::A1186Pidgey);
    assert!(
        state.hands[current_player].contains(&pidgey_card),
        "Returned Pokemon should be in hand"
    );

    let (promo_actor, promo_choices) = state
        .move_generation_stack
        .last()
        .expect("Promotion choices should be queued after returning active");
    assert_eq!(*promo_actor, current_player);
    assert!(
        promo_choices
            .iter()
            .any(|action| matches!(action, SimpleAction::Activate { in_play_idx: 1, .. })),
        "Promotion choices should include Activate for bench index 1, move_generation_stack: {:?}",
        state.move_generation_stack
    );
}
