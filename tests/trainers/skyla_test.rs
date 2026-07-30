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

/// Skyla: "Switch your Active Stage 1 Pokémon with 1 of your Benched Pokémon."
#[test]
fn test_skyla_switches_active_stage_1_with_bench() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    // Wartortle is a Stage 1, so Skyla can switch it out for the benched Charmander.
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1054Wartortle),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let skyla = make_trainer_card(CardId::B4152Skyla);
    state.hands[0] = vec![Card::Trainer(skyla.clone())];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: skyla,
        },
        is_stack: false,
    });

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(choices.iter().all(|choice| matches!(
        choice.action,
        SimpleAction::Activate {
            player: 0,
            in_play_idx: 1
        }
    )));
    game.apply_action(&choices[0].clone());

    let state = game.get_state_clone();
    assert_eq!(state.get_active(0).get_name(), "Charmander");
    assert_eq!(
        state.in_play_pokemon[0][1]
            .as_ref()
            .expect("Wartortle should be benched")
            .get_name(),
        "Wartortle"
    );
}

#[test]
fn test_skyla_does_not_consume_the_turn_retreat() {
    // Skyla's switch is free: the player can still pay to retreat afterwards, putting the
    // Pokemon Skyla promoted right back on the Bench.
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1054Wartortle),
            // Charmander needs 1 Energy to pay its own retreat cost once promoted.
            PlayedCard::from_id(CardId::A1033Charmander).with_energy(vec![EnergyType::Fire]),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let skyla = make_trainer_card(CardId::B4152Skyla);
    state.hands[0] = vec![Card::Trainer(skyla.clone())];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: skyla,
        },
        is_stack: false,
    });
    let (_, choices) = game.get_state_clone().generate_possible_actions();
    game.apply_action(&choices[0].clone());
    assert_eq!(
        game.get_state_clone().get_active(0).get_name(),
        "Charmander"
    );

    // The regular retreat for the turn is still available.
    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    let retreat_action = choices
        .iter()
        .find(|choice| matches!(choice.action, SimpleAction::Retreat(1)))
        .expect("Retreating should still be possible after Skyla")
        .clone();
    game.apply_action(&retreat_action);

    let state = game.get_state_clone();
    assert_eq!(state.get_active(0).get_name(), "Wartortle");
    assert_eq!(
        state.in_play_pokemon[0][1]
            .as_ref()
            .expect("Charmander should be back on the Bench")
            .get_name(),
        "Charmander"
    );
    // The retreat was paid for, unlike Skyla's free switch.
    assert!(state.get_active(0).attached_energy.is_empty());
    assert_eq!(state.discard_energies[0], vec![EnergyType::Fire]);
}

#[test]
fn test_skyla_cannot_be_played_with_basic_active() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    // Squirtle is Basic, so Skyla has no legal target even though the bench is occupied.
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1053Squirtle),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let skyla = make_trainer_card(CardId::B4152Skyla);
    state.hands[0] = vec![Card::Trainer(skyla)];
    game.set_state(state);

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(
        !choices.iter().any(
            |choice| matches!(&choice.action, SimpleAction::Play { trainer_card } if trainer_card.id == "B4 152")
        ),
        "Skyla shouldn't be playable when the Active Pokemon isn't a Stage 1"
    );
}

#[test]
fn test_skyla_cannot_be_played_with_empty_bench() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1054Wartortle)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let skyla = make_trainer_card(CardId::B4152Skyla);
    state.hands[0] = vec![Card::Trainer(skyla)];
    game.set_state(state);

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(
        !choices.iter().any(
            |choice| matches!(&choice.action, SimpleAction::Play { trainer_card } if trainer_card.id == "B4 152")
        ),
        "Skyla shouldn't be playable without a Benched Pokemon to switch to"
    );
}
