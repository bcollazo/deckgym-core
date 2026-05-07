use crate::deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

#[test]
fn test_zorua_ascension_evolves_from_deck() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::B3105Zorua).with_energy(vec![EnergyType::Darkness])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.decks[0].cards.clear();
    state.decks[0]
        .cards
        .push(get_card_by_enum(CardId::B3106ZoroarkEx));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.get_active(0).get_name(), "Zoroark ex");
    assert!(
        state.decks[0].cards.is_empty(),
        "Ascension should remove the evolution card from the deck"
    );
}

#[test]
fn test_zoroark_ex_brutal_bash_counts_only_own_darkness_bench() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B3106ZoroarkEx).with_energy(vec![EnergyType::Darkness]),
            PlayedCard::from_id(CardId::A4a049Zorua),
            PlayedCard::from_id(CardId::B2b044Zoroark),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1a047Marshadow)],
    );
    state.current_player = 0;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        20,
        "Brutal Bash should do 60 damage for two Benched Darkness Pokemon"
    );
}

#[test]
fn test_zoroark_illusive_trickery_protects_after_attack_ko() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A4a050Zoroark)
            .with_energy(vec![EnergyType::Darkness, EnergyType::Darkness])],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1033Charmander).with_energy(vec![EnergyType::Fire]),
        ],
    );
    state.current_player = 0;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    let promote_action = actions
        .into_iter()
        .find(|action| matches!(action.action, SimpleAction::Activate { player: 1, .. }))
        .expect("Opponent should promote after Bulbasaur is Knocked Out");
    game.apply_action(&promote_action);

    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    let end_turn_action = actions
        .into_iter()
        .find(|action| matches!(action.action, SimpleAction::EndTurn))
        .expect("Zoroark's turn should be ready to end after the KO");
    game.apply_action(&end_turn_action);

    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(0).get_remaining_hp(),
        100,
        "Illusive Trickery should prevent damage during the opponent's next turn"
    );
}
