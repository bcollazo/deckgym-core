use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

fn make_trainer_card(card_id: CardId) -> deckgym::models::TrainerCard {
    get_card_by_enum(card_id).as_trainer()
}

/// Picks Wallace's target: the queued `EvolveRandomFromDeck` choice for `in_play_idx`.
fn choose_wallace_target(game: &mut deckgym::Game, in_play_idx: usize) {
    let (_, choices) = game.get_state_clone().generate_possible_actions();
    let choice = choices
        .into_iter()
        .find(|a| {
            a.action
                == SimpleAction::EvolveRandomFromDeck {
                    in_play_idx,
                    energy_type: EnergyType::Water,
                }
        })
        .expect("Wallace target should be selectable");
    game.apply_action(&choice);
}

#[test]
fn test_wallace_evolves_low_hp_water_pokemon_from_deck() {
    // Wallace: "Choose 1 of your [W] Pokémon in play with a maximum HP of 50 or less. Put a
    // random [W] Pokémon from your deck that evolves from that Pokémon onto that Pokémon to
    // evolve it."
    // Staryu (50 HP, Water) is on the board; Starmie (evolves from Staryu) is the only card
    // in the deck, so the evolution outcome is deterministic.
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1074Staryu)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let starmie_card = get_card_by_enum(CardId::A1075Starmie);
    state.decks[0].cards = vec![starmie_card.clone()];

    let wallace = make_trainer_card(CardId::B3b068Wallace);
    state.hands[0] = vec![Card::Trainer(wallace.clone())];
    game.set_state(state);

    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: wallace,
        },
        is_stack: false,
    };
    game.apply_action(&play_action);
    choose_wallace_target(&mut game, 0);

    let state = game.get_state_clone();
    let active = state.in_play_pokemon[0][0]
        .as_ref()
        .expect("Active Pokemon should still be present");
    assert_eq!(active.get_name(), "Starmie");
    assert!(
        state.decks[0].cards.is_empty(),
        "Starmie should have been removed from the deck"
    );
}

#[test]
fn test_wallace_can_evolve_pokemon_played_this_turn() {
    // Unlike a normal evolution, Wallace has no "not played this turn" restriction: you can
    // put a Magikarp down and evolve it into Gyarados with Wallace on the same turn.
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    let mut magikarp = PlayedCard::from_id(CardId::A1077Magikarp);
    magikarp.played_this_turn = true;

    state.set_board(
        vec![magikarp],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let gyarados_card = get_card_by_enum(CardId::A1078Gyarados);
    state.decks[0].cards = vec![gyarados_card.clone()];

    let wallace = make_trainer_card(CardId::B3b068Wallace);
    state.hands[0] = vec![Card::Trainer(wallace.clone())];
    game.set_state(state);

    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: wallace,
        },
        is_stack: false,
    };
    game.apply_action(&play_action);
    choose_wallace_target(&mut game, 0);

    let state = game.get_state_clone();
    let active = state.in_play_pokemon[0][0]
        .as_ref()
        .expect("Active Pokemon should still be present");
    assert_eq!(active.get_name(), "Gyarados");
    assert!(
        state.decks[0].cards.is_empty(),
        "Gyarados should have been removed from the deck"
    );
}

#[test]
fn test_wallace_no_valid_target_does_nothing() {
    // Wallace requires a [W] Pokémon in play with max HP of 50 or less; Wartortle (80 HP)
    // doesn't qualify, so the card should have no effect (and thus can't be played).
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1054Wartortle)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let blastoise_card = get_card_by_enum(CardId::A1055Blastoise);
    state.decks[0].cards = vec![blastoise_card.clone()];

    let wallace = make_trainer_card(CardId::B3b068Wallace);
    state.hands[0] = vec![Card::Trainer(wallace)];
    game.set_state(state);

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(
        !choices
            .iter()
            .any(|choice| matches!(&choice.action, SimpleAction::Play { trainer_card } if trainer_card.id == "B3b 068")),
        "Wallace shouldn't be playable without a valid low-HP Water Pokémon target"
    );
}

#[test]
fn test_wallace_lets_player_choose_target() {
    // Wallace's player chooses which eligible [W] Pokémon to evolve; only the evolution card is
    // random. Staryu (50 HP) and Magikarp (30 HP) both qualify.
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1074Staryu),
            PlayedCard::from_id(CardId::A1077Magikarp),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.decks[0].cards = vec![
        get_card_by_enum(CardId::A1075Starmie),
        get_card_by_enum(CardId::A1078Gyarados),
    ];

    let wallace = make_trainer_card(CardId::B3b068Wallace);
    state.hands[0] = vec![Card::Trainer(wallace.clone())];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: wallace,
        },
        is_stack: false,
    });

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert_eq!(
        choices.into_iter().map(|a| a.action).collect::<Vec<_>>(),
        vec![
            SimpleAction::EvolveRandomFromDeck {
                in_play_idx: 0,
                energy_type: EnergyType::Water,
            },
            SimpleAction::EvolveRandomFromDeck {
                in_play_idx: 1,
                energy_type: EnergyType::Water,
            },
        ]
    );

    choose_wallace_target(&mut game, 1);

    let state = game.get_state_clone();
    assert_eq!(state.get_active(0).get_name(), "Staryu");
    assert_eq!(
        state.in_play_pokemon[0][1].as_ref().unwrap().get_name(),
        "Gyarados"
    );
    assert_eq!(
        state.decks[0].cards,
        vec![get_card_by_enum(CardId::A1075Starmie)]
    );
}
