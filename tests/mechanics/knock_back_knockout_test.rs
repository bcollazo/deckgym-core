use deckgym::actions::{Action, SimpleAction};
use deckgym::card_ids::CardId;
use deckgym::models::Card;
use deckgym::state::PlayedCard;
use deckgym::{Game, State};

#[test]
fn lethal_push_out_offers_only_one_promotion() {
    check_switch(CardId::A3091Hariyama, 10, 0, true);
}

#[test]
fn surviving_push_out_still_switches() {
    check_switch(CardId::A3091Hariyama, 130, 0, true);
}

#[test]
fn coin_flip_switch_with_lethal_damage_only_promotes() {
    // Exercise both coin outcomes with a carried attack (e.g. copied attacks).
    for seed in 0..32 {
        check_switch(CardId::PA095Chinchou, 10, seed, true);
    }
}

fn check_switch(card_id: CardId, remaining_hp: u32, seed: u64, expect_choice: bool) {
    let mut state = State::default();
    state.turn_count = 3;
    let attacker = PlayedCard::from_id(card_id);
    let Card::Pokemon(card) = &attacker.card else {
        panic!()
    };
    let mut attack = card.attacks[0].clone();
    attack.fixed_damage = 50;
    let defender = PlayedCard::from_id(CardId::A1a003CelebiEx).with_remaining_hp(remaining_hp);

    state.set_board(
        vec![attacker],
        vec![defender, PlayedCard::from_id(CardId::A1a003CelebiEx)],
    );
    let mut game = Game::from_state(state, vec![], seed);
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(attack),
        is_stack: false,
    });
    let state = game.get_state_clone();
    let promotions = state
        .move_generation_stack
        .iter()
        .filter(|(_, choices)| {
            choices
                .iter()
                .any(|a| matches!(a, SimpleAction::Activate { .. }))
        })
        .count();
    assert_eq!(
        promotions,
        usize::from(expect_choice),
        "a knocked-out defender must not also be switched after promotion"
    );
    for _ in 0..10 {
        let state = game.get_state_clone();
        if state.move_generation_stack.is_empty() {
            break;
        }
        let (_, actions) = state.generate_possible_actions();
        game.apply_action(&actions[0]);
    }
    let state = game.get_state_clone();
    assert!(state.move_generation_stack.is_empty());
    assert!(state.maybe_get_active(1).is_some());
    assert_eq!(state.points[0], if remaining_hp == 10 { 2 } else { 0 });
}
