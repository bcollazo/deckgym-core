use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    generate_possible_actions,
    models::{EnergyType, PlayedCard},
};

mod common;

#[test]
fn test_raikou_rocky_helmet_promotion_order() {
    let mut game = get_initialized_game(0);
    game.play_until_stable();

    // Set Raikou against Rocky Helmet
    let mut state = game.get_state_clone();
    state.set_board(
        0,
        vec![
            PlayedCard::from_id(CardId::A4a025RaikouEx)
                .with_energy(vec![EnergyType::Lightning, EnergyType::Lightning])
                .with_hp(20),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );
    state.set_board(
        1,
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_tool(get_card_by_enum(CardId::A2148RockyHelmet)),
            PlayedCard::from_id(CardId::A1033Charmander),
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

    // Assert Player has to choose target
    let (actor, choices) = generate_possible_actions(&game.get_state_clone());
    assert_eq!(actor, 0);
    assert!(choices
        .iter()
        .all(|choice| matches!(choice.action, SimpleAction::ApplyDamage { .. })));

    let apply_damage_action = choices[0].clone();
    game.apply_action(&apply_damage_action);

    // Assert Raikou was K.O. and attacker must activate
    let (actor, choices) = generate_possible_actions(&game.get_state_clone());
    assert_eq!(actor, 0);
    assert!(choices.iter().all(|choice| {
        matches!(
            choice.action,
            SimpleAction::Activate {
                player: 0,
                in_play_idx: _
            }
        )
    }));

    let promote_action = choices[0].clone();
    game.apply_action(&promote_action);

    // TODO: Assert this way, or assert directly it should be the next player's turn.
    let (actor, choices) = generate_possible_actions(&game.get_state_clone());
    assert_eq!(actor, 0);
    assert!(choices
        .iter()
        .all(|choice| matches!(choice.action, SimpleAction::EndTurn)));
}
