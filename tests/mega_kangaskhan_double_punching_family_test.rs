use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard},
    state::GameOutcome,
};

mod common;

fn played_card_with_base_hp(card_id: CardId, base_hp: u32) -> PlayedCard {
    let card = get_card_by_enum(card_id);
    PlayedCard::new(card, 0, base_hp, vec![], false, vec![])
}

fn trainer_from_id(card_id: CardId) -> deckgym::models::TrainerCard {
    match get_card_by_enum(card_id) {
        Card::Trainer(trainer_card) => trainer_card,
        _ => panic!("Expected trainer card"),
    }
}

#[test]
fn test_mega_kangaskhan_double_punching_family_ko_then_promotion_then_ko() {
    let mut game = get_initialized_game(0);
    game.play_until_stable();

    let mut state = game.get_state_clone();
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B2127MegaKangaskhanEx).with_energy(vec![
                EnergyType::Water,
                EnergyType::Water,
                EnergyType::Water,
            ]),
        ],
        vec![
            played_card_with_base_hp(CardId::A1001Bulbasaur, 80),
            PlayedCard::from_id(CardId::A1033Charmander).with_remaining_hp(30),
        ],
    );
    state.current_player = 0;
    game.set_state(state);

    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);
    game.play_until_stable();

    let state = game.get_state_clone();
    assert!(state.in_play_pokemon[1][0].is_none());
    assert!(state.in_play_pokemon[1][1].is_none());
    assert_eq!(state.winner, Some(GameOutcome::Win(0)));
}

#[test]
fn test_mega_kangaskhan_double_punching_family_red_vs_ex_with_rocky_helmet() {
    let mut game = get_initialized_game(0);
    game.play_until_stable();

    let mut state = game.get_state_clone();
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B2127MegaKangaskhanEx).with_energy(vec![
                EnergyType::Grass,
                EnergyType::Grass,
                EnergyType::Grass,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::B2127MegaKangaskhanEx)
            .with_tool(get_card_by_enum(CardId::A2148RockyHelmet))],
    );
    state.current_player = 0;
    state.hands[0] = vec![get_card_by_enum(CardId::A2b071Red)];
    game.set_state(state);

    let trainer_card = trainer_from_id(CardId::A2b071Red);
    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let attack_action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&attack_action);
    game.play_until_stable();

    let state = game.get_state_clone();
    let opponent_active = state.get_active(1);
    assert_eq!(
        opponent_active.get_remaining_hp(),
        20,
        "Expected 160 damage to EX (down from 180hp)"
    );
    let attacker_active = state.get_active(0);
    assert_eq!(
        attacker_active.get_remaining_hp(),
        140,
        "Expected Rocky Helmet twice"
    );
}
