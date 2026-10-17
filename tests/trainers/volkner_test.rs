use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard, TrainerCard},
    test_support::get_initialized_game,
};

fn make_volkner_trainer_card() -> TrainerCard {
    match get_card_by_enum(CardId::A2153Volkner) {
        Card::Trainer(tc) => tc,
        _ => panic!("Expected trainer card"),
    }
}

fn play_volkner(game: &mut deckgym::Game) {
    let trainer_card = make_volkner_trainer_card();
    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    };
    game.apply_action(&play_action);
}

/// Volkner cannot be played without an Electivire or Luxray in play.
#[test]
fn test_cannot_play_volkner_without_target() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    state.hands[0].clear();
    let trainer_card = make_volkner_trainer_card();
    state.hands[0].push(Card::Trainer(trainer_card));
    state.discard_energies[0] = vec![EnergyType::Lightning, EnergyType::Lightning];
    game.set_state(state);

    let (_, actions) = game.get_state_clone().generate_possible_actions();
    let can_play = actions
        .iter()
        .any(|a| matches!(&a.action, SimpleAction::Play { .. }));
    assert!(
        !can_play,
        "Should not be able to play Volkner without Electivire or Luxray, actions: {actions:?}"
    );
}

/// Playing Volkner lets the player choose Electivire or Luxray, then attaches
/// 2 Lightning Energy from the discard pile to the chosen Pokemon.
#[test]
fn test_volkner_attaches_two_lightning_energy_to_chosen_target() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A2057Electivire),
            PlayedCard::from_id(CardId::A2060Luxray),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    state.hands[0].clear();
    let trainer_card = make_volkner_trainer_card();
    state.hands[0].push(Card::Trainer(trainer_card));
    state.discard_energies[0] = vec![
        EnergyType::Lightning,
        EnergyType::Lightning,
        EnergyType::Fire,
    ];
    game.set_state(state);

    play_volkner(&mut game);

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert_eq!(
        choices.len(),
        2,
        "Should be able to choose either Electivire (slot 0) or Luxray (slot 1); got: {choices:?}"
    );

    // Choose Luxray, the benched target (slot 1).
    let luxray_choice = choices
        .iter()
        .find(|a| matches!(&a.action, SimpleAction::AttachTypedFromDiscard { in_play_idx, .. } if *in_play_idx == 1))
        .expect("Should have a choice targeting Luxray");
    game.apply_action(luxray_choice);

    let state = game.get_state_clone();
    let luxray = state.in_play_pokemon[0][1]
        .as_ref()
        .expect("Luxray should still be in play");
    assert_eq!(luxray.attached_energy.len(), 2);
    assert!(luxray
        .attached_energy
        .iter()
        .all(|e| *e == EnergyType::Lightning));

    assert_eq!(
        state.discard_energies[0],
        vec![EnergyType::Fire],
        "Both Lightning energies should be removed from discard, Fire should remain"
    );
}
