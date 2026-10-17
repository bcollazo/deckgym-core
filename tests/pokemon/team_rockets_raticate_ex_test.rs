use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game_with_board,
    Game,
};

/// Team Rocket's Raticate ex's Thieving Incisors: "Once during your turn, when you play this
/// Pokémon from your hand to evolve 1 of your Pokémon, you may move a random Energy from your
/// opponent's Active Pokémon to this Pokémon."
fn evolve_rattata_into_raticate_ex(seed: u64) -> Game<'static> {
    let mut game = get_initialized_game_with_board(
        seed,
        0,
        3,
        vec![PlayedCard::from_id(CardId::B4a058TeamRocketsRattata)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Grass])],
    );

    let mut state = game.get_state_clone();
    state.hands[0].clear();
    state.hands[0].push(get_card_by_enum(CardId::B4a059TeamRocketsRaticateEx));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Evolve {
            evolution: get_card_by_enum(CardId::B4a059TeamRocketsRaticateEx),
            in_play_idx: 0,
            from_deck: false,
        },
        is_stack: false,
    });

    game
}

#[test]
fn test_thieving_incisors_is_optional_and_steals_energy_on_evolve() {
    let mut game = evolve_rattata_into_raticate_ex(0);

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert_eq!(choices.len(), 2, "Thieving Incisors should be optional");
    assert!(choices.iter().any(|c| matches!(
        c.action,
        SimpleAction::MoveOpponentActiveEnergyToSelf { .. }
    )));
    assert!(choices
        .iter()
        .any(|c| matches!(c.action, SimpleAction::Noop)));

    let steal_choice = choices
        .into_iter()
        .find(|c| {
            matches!(
                c.action,
                SimpleAction::MoveOpponentActiveEnergyToSelf { .. }
            )
        })
        .expect("Thieving Incisors choice should be present");
    game.apply_action(&steal_choice);

    let state = game.get_state_clone();
    assert_eq!(state.get_active(0).attached_energy.len(), 1);
    assert_eq!(state.get_active(0).attached_energy[0], EnergyType::Grass);
    assert_eq!(state.get_active(1).attached_energy.len(), 1);
}

#[test]
fn test_thieving_incisors_declined_leaves_energy_unmoved() {
    let mut game = evolve_rattata_into_raticate_ex(0);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Noop,
        is_stack: true,
    });

    let state = game.get_state_clone();
    assert_eq!(state.get_active(0).attached_energy.len(), 0);
    assert_eq!(state.get_active(1).attached_energy.len(), 2);
}

#[test]
fn test_thieving_incisors_not_offered_when_opponent_active_has_no_energy() {
    let mut game = get_initialized_game_with_board(
        0,
        0,
        3,
        vec![PlayedCard::from_id(CardId::B4a058TeamRocketsRattata)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let mut state = game.get_state_clone();
    state.hands[0].clear();
    state.hands[0].push(get_card_by_enum(CardId::B4a059TeamRocketsRaticateEx));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Evolve {
            evolution: get_card_by_enum(CardId::B4a059TeamRocketsRaticateEx),
            in_play_idx: 0,
            from_deck: false,
        },
        is_stack: false,
    });

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(
        !choices.iter().any(|c| matches!(
            c.action,
            SimpleAction::MoveOpponentActiveEnergyToSelf { .. }
        )),
        "Thieving Incisors shouldn't be offered when the opponent's Active has no Energy"
    );
}
