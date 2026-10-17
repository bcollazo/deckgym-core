use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard},
    test_support::get_test_game_with_board,
};

#[test]
fn test_kiawe_resolves_attach_choice_before_ending_turn() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A3037Turtonator),
            PlayedCard::from_id(CardId::A3027AlolanMarowak),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let Card::Trainer(kiawe) = get_card_by_enum(CardId::A3150Kiawe) else {
        panic!("Kiawe should be a trainer card");
    };
    let mut state = game.get_state_clone();
    state.hands[0] = vec![Card::Trainer(kiawe.clone())];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: kiawe,
        },
        is_stack: false,
    });

    // The attach choice is offered first (Turtonator or Alolan Marowak), not EndTurn.
    let (actor, actions) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert_eq!(actions.len(), 2);
    assert!(actions
        .iter()
        .all(|action| matches!(action.action, SimpleAction::Attach { .. })));
    let attach_to_marowak = actions
        .into_iter()
        .find(|action| {
            matches!(
                &action.action,
                SimpleAction::Attach { attachments, .. } if attachments[0].2 == 2
            )
        })
        .expect("Alolan Marowak should be a target");
    game.apply_action(&attach_to_marowak);

    let state = game.get_state_clone();
    assert_eq!(
        state.in_play_pokemon[0][2]
            .as_ref()
            .unwrap()
            .attached_energy,
        vec![EnergyType::Fire, EnergyType::Fire]
    );

    // Once the effect settles, the only option is to end the turn.
    let (_, actions) = state.generate_possible_actions();
    assert_eq!(actions.len(), 1);
    assert!(matches!(actions[0].action, SimpleAction::EndTurn));
    game.apply_action(&actions[0]);
    assert_eq!(game.get_state_clone().current_player, 1);
}
