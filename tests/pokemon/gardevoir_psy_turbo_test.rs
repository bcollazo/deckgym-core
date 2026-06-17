use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

#[test]
fn test_gardevoir_psy_turbo_attaches_energy_to_benched_psychic_pokemon() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B2065Gardevoir)
                .with_energy(vec![EnergyType::Psychic, EnergyType::Psychic]),
            PlayedCard::from_id(CardId::A1130Ralts),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2065Gardevoir, 0),
        is_stack: false,
    });

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    // Only the Psychic-type Ralts on the bench should be a valid attach target,
    // not the Colorless/Fire Charmander.
    assert!(!choices.is_empty());
    assert!(choices.iter().all(|choice| {
        matches!(
            &choice.action,
            SimpleAction::Attach { attachments, .. }
                if attachments.iter().all(|(_, _, in_play_idx)| *in_play_idx == 1)
        )
    }));

    let chosen = choices[0].clone();
    game.apply_action(&chosen);

    let state = game.get_state_clone();
    let ralts = state.in_play_pokemon[0][1]
        .as_ref()
        .expect("Ralts should still be on the bench");
    assert_eq!(ralts.attached_energy.len(), 2);
}
