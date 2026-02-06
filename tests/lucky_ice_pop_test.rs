use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, PlayedCard, TrainerCard},
};

mod common;

fn make_lucky_ice_pop_trainer_card() -> TrainerCard {
    match get_card_by_enum(CardId::B2145LuckyIcePop) {
        Card::Trainer(tc) => tc,
        _ => panic!("Expected trainer card"),
    }
}

fn setup_damaged_active(seed: u64) -> (deckgym::Game<'static>, TrainerCard) {
    let mut game = get_initialized_game(seed);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    // Setup: Put a damaged Bulbasaur as active (70 max HP, 50 remaining = 20 damage taken)
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur).with_damage(20)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    // Add Lucky Ice Pop to hand
    let trainer_card = make_lucky_ice_pop_trainer_card();
    let card = Card::Trainer(trainer_card.clone());
    state.hands[0].push(card);
    game.set_state(state);

    (game, trainer_card)
}

#[test]
fn test_lucky_ice_pop_heals_active_pokemon() {
    // Run across many seeds; every outcome should heal 20 damage
    for seed in 0..50 {
        let (mut game, trainer_card) = setup_damaged_active(seed);

        let play_action = Action {
            actor: 0,
            action: SimpleAction::Play { trainer_card },
            is_stack: false,
        };
        game.apply_action(&play_action);

        let state = game.get_state_clone();
        let active = state.get_active(0);
        assert_eq!(
            active.remaining_hp, 70,
            "Seed {seed}: Active should be healed by 20 (50 -> 70)"
        );
    }
}

#[test]
fn test_lucky_ice_pop_coin_flip_returns_to_hand_or_stays_in_discard() {
    let mut heads_count = 0;
    let mut tails_count = 0;

    for seed in 0..200 {
        let (mut game, trainer_card) = setup_damaged_active(seed);

        let card = Card::Trainer(trainer_card.clone());
        let play_action = Action {
            actor: 0,
            action: SimpleAction::Play { trainer_card },
            is_stack: false,
        };
        game.apply_action(&play_action);

        let state = game.get_state_clone();
        let in_hand = state.hands[0].contains(&card);
        let in_discard = state.discard_piles[0].contains(&card);

        // Card must be in exactly one of: hand or discard
        assert!(
            in_hand || in_discard,
            "Seed {seed}: Card must be in hand or discard"
        );
        assert!(
            !(in_hand && in_discard),
            "Seed {seed}: Card cannot be in both hand and discard"
        );

        if in_hand {
            heads_count += 1;
        } else {
            tails_count += 1;
        }
    }

    // With 200 trials of a 50/50 coin flip, both outcomes should appear
    assert!(
        heads_count > 0,
        "Expected at least one heads (card returned to hand) across 200 seeds"
    );
    assert!(
        tails_count > 0,
        "Expected at least one tails (card stayed in discard) across 200 seeds"
    );
}
