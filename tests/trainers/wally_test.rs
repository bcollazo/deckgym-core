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

/// Wally: "Take a [C] Energy from your Energy Zone and attach it to 1 of your Stage 2 Pokémon."
#[test]
fn test_wally_attaches_colorless_energy_to_stage_2() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    // Blastoise is a Stage 2; the benched Charmander is not a legal Wally target.
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1055Blastoise),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let wally = make_trainer_card(CardId::B4153Wally);
    state.hands[0] = vec![Card::Trainer(wally.clone())];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: wally,
        },
        is_stack: false,
    });

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert_eq!(
        choices.len(),
        1,
        "Only the Stage 2 Blastoise should be a Wally target"
    );
    assert!(matches!(
        &choices[0].action,
        SimpleAction::Attach { attachments, is_turn_energy: false }
            if attachments == &vec![(1, EnergyType::Colorless, 0)]
    ));
    game.apply_action(&choices[0].clone());

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(0).attached_energy,
        vec![EnergyType::Colorless]
    );
}

#[test]
fn test_wally_can_target_a_benched_stage_2() {
    // Wally is not restricted to the Active Spot: every Stage 2 in play is a valid target.
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1055Blastoise),
            PlayedCard::from_id(CardId::A1003Venusaur),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let wally = make_trainer_card(CardId::B4153Wally);
    state.hands[0] = vec![Card::Trainer(wally.clone())];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: wally,
        },
        is_stack: false,
    });

    let (_, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(
        choices.len(),
        2,
        "Both Stage 2 Pokemon should be targetable"
    );
    let bench_choice = choices
        .iter()
        .find(|choice| {
            matches!(
                &choice.action,
                SimpleAction::Attach { attachments, .. }
                    if attachments == &vec![(1, EnergyType::Colorless, 1)]
            )
        })
        .expect("Benched Venusaur should be a Wally target")
        .clone();
    game.apply_action(&bench_choice);

    let state = game.get_state_clone();
    assert_eq!(
        state.in_play_pokemon[0][1]
            .as_ref()
            .expect("Venusaur should still be benched")
            .attached_energy,
        vec![EnergyType::Colorless]
    );
    assert!(state.get_active(0).attached_energy.is_empty());
}

#[test]
fn test_wally_cannot_be_played_without_stage_2() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1054Wartortle)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let wally = make_trainer_card(CardId::B4153Wally);
    state.hands[0] = vec![Card::Trainer(wally)];
    game.set_state(state);

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(
        !choices.iter().any(
            |choice| matches!(&choice.action, SimpleAction::Play { trainer_card } if trainer_card.id == "B4 153")
        ),
        "Wally shouldn't be playable without a Stage 2 Pokemon in play"
    );
}
