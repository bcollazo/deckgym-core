use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game_with_board},
};

/// Bouncy Body: when the Active Jellicent is damaged by an opponent's attack, its owner
/// takes a [W] Energy from their Energy Zone and attaches it to a Benched Pokémon of
/// their choice.
#[test]
fn test_bouncy_body_attaches_water_energy_to_chosen_bench() {
    let mut game = get_initialized_game_with_board(
        0,
        0,
        3,
        vec![PlayedCard::from_id(CardId::B1105Dusknoir)
            .with_energy(vec![EnergyType::Psychic, EnergyType::Colorless])],
        vec![
            PlayedCard::from_id(CardId::B1069Jellicent),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B1105Dusknoir, 0),
        is_stack: false,
    });

    // Jellicent's owner now chooses which Benched Pokémon receives the Energy.
    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 1, "Jellicent's owner makes the Bouncy Body choice");
    assert_eq!(
        choices.len(),
        2,
        "Bouncy Body should offer one choice per Benched Pokémon"
    );
    assert!(
        choices
            .iter()
            .all(|choice| matches!(choice.action, SimpleAction::Attach { .. })),
        "Bouncy Body choices should all be Energy attachments"
    );

    let charmander_choice = choices
        .iter()
        .find(|choice| {
            matches!(
                &choice.action,
                SimpleAction::Attach { attachments, .. }
                    if attachments == &vec![(1, EnergyType::Water, 2)]
            )
        })
        .expect("Charmander on the bench should be a Bouncy Body target")
        .clone();
    game.apply_action(&charmander_choice);

    let state = game.get_state_clone();
    assert_eq!(
        state.in_play_pokemon[1][2]
            .as_ref()
            .expect("Charmander should still be benched")
            .attached_energy,
        vec![EnergyType::Water],
        "the chosen Benched Pokémon should get a [W] Energy"
    );
    assert!(
        state.in_play_pokemon[1][1]
            .as_ref()
            .expect("Bulbasaur should still be benched")
            .attached_energy
            .is_empty(),
        "only the chosen Benched Pokémon should get Energy"
    );
    assert!(
        state.get_active(1).attached_energy.is_empty(),
        "Bouncy Body attaches to the Bench, never to Jellicent itself"
    );
}

/// With no Benched Pokémon there is nowhere to put the Energy, so no choice is offered.
#[test]
fn test_bouncy_body_does_nothing_without_a_bench() {
    let mut game = get_initialized_game_with_board(
        0,
        0,
        3,
        vec![PlayedCard::from_id(CardId::B1105Dusknoir)
            .with_energy(vec![EnergyType::Psychic, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::B1069Jellicent)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B1105Dusknoir, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    // Jellicent has 120 HP and is weak to Lightning, so Hammer In's 80 damage leaves 40.
    assert_eq!(state.get_active(1).get_remaining_hp(), 40);
    let (actor, choices) = state.generate_possible_actions();
    assert_eq!(
        actor, 0,
        "no Bouncy Body choice should be queued for Jellicent's owner"
    );
    assert!(choices
        .iter()
        .all(|choice| !matches!(choice.action, SimpleAction::Attach { .. })));
}
