use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::PlayedCard,
    test_support::get_initialized_game,
};

#[test]
fn test_drizzile_swift_shot_damages_opponent_active_on_evolve() {
    assert_swift_shot_damage(
        CardId::B3048Sobble,
        CardId::B3049Drizzile,
        20,
        50,
        "Drizzile's Swift Shot should offer 20 damage to the opponent's Active Pokemon",
    );
}

#[test]
fn test_inteleon_swift_shot_damages_opponent_active_on_evolve() {
    assert_swift_shot_damage(
        CardId::B3049Drizzile,
        CardId::B3050Inteleon,
        30,
        40,
        "Inteleon's Swift Shot should offer 30 damage to the opponent's Active Pokemon",
    );
}

fn assert_swift_shot_damage(
    base: CardId,
    evolution: CardId,
    damage: u32,
    expected_remaining_hp: u32,
    message: &str,
) {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.set_board(
        vec![PlayedCard::from_id(base)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    state.hands[0].clear();
    state.hands[0].push(get_card_by_enum(evolution));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Evolve {
            evolution: get_card_by_enum(evolution),
            in_play_idx: 0,
            from_deck: false,
        },
        is_stack: false,
    });

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert_eq!(choices.len(), 2, "Swift Shot should be optional");
    assert!(choices
        .iter()
        .any(|choice| matches!(choice.action, SimpleAction::Noop)));

    let swift_shot = choices
        .iter()
        .find(|choice| {
            matches!(
                choice.action,
                SimpleAction::ApplyDamage {
                    attacking_ref: (0, 0),
                    ref targets,
                    is_from_active_attack: false,
                } if targets == &vec![(damage, 1, 0)]
            )
        })
        .cloned()
        .expect(message);

    game.apply_action(&swift_shot);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        expected_remaining_hp,
        "{message}"
    );
}
