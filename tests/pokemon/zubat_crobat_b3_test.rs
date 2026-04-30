use crate::deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

fn played_card_with_base_hp(card_id: CardId, base_hp: u32) -> PlayedCard {
    PlayedCard::new(get_card_by_enum(card_id), 0, base_hp, vec![], false, vec![])
}

#[test]
fn test_zubat_spin_turn_damages_and_can_switch_with_bench() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B3098Zubat).with_energy(vec![EnergyType::Colorless]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1053Squirtle)],
    );

    game.set_state(state);
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(
        choices.iter().any(|choice| matches!(
            choice.action,
            SimpleAction::Activate {
                player: 0,
                in_play_idx: 1
            }
        )),
        "Spin Turn should offer the attacking player a choice to switch with their Bench"
    );

    let switch_choice = choices
        .into_iter()
        .find(|choice| {
            matches!(
                choice.action,
                SimpleAction::Activate {
                    player: 0,
                    in_play_idx: 1
                }
            )
        })
        .expect("Expected a switch choice for the benched Pokemon");

    game.apply_action(&switch_choice);
    let state = game.get_state_clone();

    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        50,
        "Spin Turn should deal 10 damage before switching"
    );
    assert_eq!(state.get_active(0).get_name(), "Bulbasaur");
    assert_eq!(
        state.in_play_pokemon[0][1]
            .as_ref()
            .expect("Zubat should move to the Bench")
            .get_name(),
        "Zubat"
    );
}

#[test]
fn test_crobat_surprise_strike_bonus_after_moving_from_bench() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::B3100Crobat)
                .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless]),
        ],
        vec![played_card_with_base_hp(CardId::A1053Squirtle, 140)],
    );

    game.set_state(state);
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Activate {
            player: 0,
            in_play_idx: 1,
        },
        is_stack: false,
    });
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        20,
        "Surprise Strike should deal 120 damage after Crobat moves from Bench to Active"
    );
}

#[test]
fn test_crobat_surprise_strike_base_damage_without_bench_move() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![PlayedCard::from_id(CardId::B3100Crobat)
            .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless])],
        vec![played_card_with_base_hp(CardId::A1053Squirtle, 140)],
    );

    game.set_state(state);
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        80,
        "Surprise Strike should deal only 60 damage without the Bench-to-Active bonus"
    );
}
