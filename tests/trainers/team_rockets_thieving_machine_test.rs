use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, PlayedCard},
    test_support::get_initialized_game,
};

/// Sets up a board where player 0 holds Team Rocket's Thieving Machine and their opponent's
/// discard pile only contains `discard_cards`, then plays the Item.
fn play_thieving_machine(seed: u64, discard_cards: Vec<Card>) -> deckgym::State {
    let thieving_machine = get_card_by_enum(CardId::B4a067TeamRocketsThievingMachine).as_trainer();

    let mut game = get_initialized_game(seed);
    let mut state = game.get_state_clone();
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    state.hands[0] = vec![Card::Trainer(thieving_machine.clone())];
    state.discard_piles[1] = discard_cards;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: thieving_machine,
        },
        is_stack: false,
    });

    game.get_state_clone()
}

#[test]
fn test_thieving_machine_pulls_item_from_opponent_discard() {
    let potion = get_card_by_enum(CardId::PA001Potion);
    let state = play_thieving_machine(0, vec![potion.clone()]);

    assert!(
        state.hands[0].contains(&potion),
        "Thieving Machine should pull the only Item from the opponent's discard pile"
    );
    assert!(
        !state.discard_piles[1].contains(&potion),
        "The pulled Item should leave the opponent's discard pile"
    );
}

#[test]
fn test_thieving_machine_never_pulls_itself_or_non_items() {
    let another_thieving_machine = get_card_by_enum(CardId::B4a067TeamRocketsThievingMachine);
    let cynthia = get_card_by_enum(CardId::A2152Cynthia);
    let charmander = get_card_by_enum(CardId::A1033Charmander);
    let discard_cards = vec![
        another_thieving_machine.clone(),
        cynthia.clone(),
        charmander.clone(),
    ];

    for seed in 0..15 {
        let state = play_thieving_machine(seed, discard_cards.clone());
        assert_eq!(
            state.discard_piles[1].len(),
            discard_cards.len(),
            "Seed {seed}: nothing eligible should have left the opponent's discard pile"
        );
        assert!(
            state.hands[0].is_empty(),
            "Seed {seed}: hand should stay empty when no eligible Item exists"
        );
    }
}

#[test]
fn test_thieving_machine_not_playable_without_eligible_target() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    let thieving_machine = get_card_by_enum(CardId::B4a067TeamRocketsThievingMachine).as_trainer();
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    state.hands[0] = vec![Card::Trainer(thieving_machine.clone())];
    state.discard_piles[1] = vec![];
    game.set_state(state);

    let state = game.get_state_clone();
    let (_actor, actions) = state.generate_possible_actions();
    assert!(
        !actions.iter().any(|action| matches!(
            &action.action,
            SimpleAction::Play { trainer_card } if trainer_card.name == thieving_machine.name
        )),
        "Thieving Machine should not be playable when the opponent's discard has no eligible Item"
    );
}
