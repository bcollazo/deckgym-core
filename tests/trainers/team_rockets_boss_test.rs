use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, PlayedCard, TrainerCard},
    test_support::get_initialized_game,
};

fn make_boss_trainer_card() -> TrainerCard {
    get_card_by_enum(CardId::B4a071TeamRocketsBoss).as_trainer()
}

fn play_boss(game: &mut deckgym::Game) {
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: make_boss_trainer_card(),
        },
        is_stack: false,
    });
}

/// Team Rocket's Boss: "Look at your opponent's hand and put any number of Basic Pokémon you
/// find there onto your opponent's Bench."
#[test]
fn test_boss_offers_every_subset_of_opponents_basics_up_to_bench_space() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    state.current_player = 0;
    state.hands[0] = vec![Card::Trainer(make_boss_trainer_card())];
    state.hands[1] = vec![
        get_card_by_enum(CardId::A1001Bulbasaur),
        get_card_by_enum(CardId::A1033Charmander),
    ];
    game.set_state(state);

    play_boss(&mut game);

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    // With 2 Basics in hand and 2 free Bench slots: choose 0, either 1 of 2, or both -> 4 choices.
    assert_eq!(
        choices.len(),
        4,
        "Should offer every subset (including none) of the opponent's 2 Basics; got {choices:?}"
    );

    let put_both = choices
        .iter()
        .find(|a| matches!(&a.action, SimpleAction::BenchOpponentPokemonFromHand { cards } if cards.len() == 2))
        .expect("Should have a choice putting both Basics onto the Bench")
        .clone();
    game.apply_action(&put_both);

    let state = game.get_state_clone();
    assert!(
        state.hands[1].is_empty(),
        "Both Basics should have left the opponent's hand"
    );
    assert!(
        state.in_play_pokemon[1][1].is_some() && state.in_play_pokemon[1][2].is_some(),
        "Both Basics should now be on the opponent's Bench"
    );
}

#[test]
fn test_boss_can_choose_to_bench_none() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    state.current_player = 0;
    state.hands[0] = vec![Card::Trainer(make_boss_trainer_card())];
    state.hands[1] = vec![get_card_by_enum(CardId::A1001Bulbasaur)];
    game.set_state(state);

    play_boss(&mut game);

    let (_actor, choices) = game.get_state_clone().generate_possible_actions();
    let put_none = choices
        .iter()
        .find(|a| matches!(&a.action, SimpleAction::BenchOpponentPokemonFromHand { cards } if cards.is_empty()))
        .expect("Should have a choice putting nothing onto the Bench")
        .clone();
    game.apply_action(&put_none);

    let state = game.get_state_clone();
    assert_eq!(
        state.hands[1].len(),
        1,
        "Declining should leave the opponent's hand untouched"
    );
    assert!(
        state.in_play_pokemon[1][1].is_none(),
        "Declining should leave the opponent's Bench untouched"
    );
}

#[test]
fn test_boss_is_playable_even_without_eligible_basics() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    state.current_player = 0;
    state.hands[0] = vec![Card::Trainer(make_boss_trainer_card())];
    state.hands[1] = vec![];
    game.set_state(state);

    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(&a.action, SimpleAction::Play { .. })),
        "Team Rocket's Boss should always be playable, even with nothing to bench"
    );
}
