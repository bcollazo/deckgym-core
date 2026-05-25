use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

#[test]
fn test_crawdaunt_unruly_claw_offers_discard_and_noop_on_evolve() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.set_board(
        vec![PlayedCard::from_id(CardId::A4060Corphish)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur).with_energy(vec![EnergyType::Grass])],
    );
    state.current_player = 0;
    state.turn_count = 3;
    state.hands[0].clear();
    state.hands[0].push(get_card_by_enum(CardId::A4061Crawdaunt));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Evolve {
            evolution: get_card_by_enum(CardId::A4061Crawdaunt),
            in_play_idx: 0,
            from_deck: false,
        },
        is_stack: false,
    });

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert_eq!(
        choices.len(),
        2,
        "Unruly Claw should be optional (2 choices)"
    );
    assert!(
        choices
            .iter()
            .any(|c| matches!(c.action, SimpleAction::Noop)),
        "Noop should be an option"
    );
    assert!(
        choices
            .iter()
            .any(|c| matches!(c.action, SimpleAction::DiscardRandomOpponentActiveEnergy)),
        "Discard choice should be offered when opponent has energy"
    );
}

#[test]
fn test_crawdaunt_unruly_claw_discards_opponent_energy() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.set_board(
        vec![PlayedCard::from_id(CardId::A4060Corphish)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur).with_energy(vec![EnergyType::Grass])],
    );
    state.current_player = 0;
    state.turn_count = 3;
    state.hands[0].clear();
    state.hands[0].push(get_card_by_enum(CardId::A4061Crawdaunt));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Evolve {
            evolution: get_card_by_enum(CardId::A4061Crawdaunt),
            in_play_idx: 0,
            from_deck: false,
        },
        is_stack: false,
    });

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::DiscardRandomOpponentActiveEnergy,
        is_stack: true,
    });

    assert!(
        game.get_state_clone()
            .get_active(1)
            .attached_energy
            .is_empty(),
        "Opponent's active should have no energy after Unruly Claw"
    );
}

#[test]
fn test_crawdaunt_unruly_claw_does_not_offer_choice_without_opponent_energy() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.set_board(
        vec![PlayedCard::from_id(CardId::A4060Corphish)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    state.hands[0].clear();
    state.hands[0].push(get_card_by_enum(CardId::A4061Crawdaunt));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Evolve {
            evolution: get_card_by_enum(CardId::A4061Crawdaunt),
            in_play_idx: 0,
            from_deck: false,
        },
        is_stack: false,
    });

    let (_, choices) = game.get_state_clone().generate_possible_actions();
    assert!(
        !choices
            .iter()
            .any(|c| matches!(c.action, SimpleAction::DiscardRandomOpponentActiveEnergy)),
        "No discard choice when opponent's active has no energy"
    );
}
