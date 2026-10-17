use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

#[test]
fn test_glittering_gift_attaches_psychic_energy_to_two_benched() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B3b031Carbink).with_energy(vec![EnergyType::Psychic]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1033Charmander),
            PlayedCard::from_id(CardId::A1053Squirtle),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3b031Carbink, 0),
        is_stack: false,
    });

    // With 3 benched Pokemon there should be C(3,2) = 3 pair choices.
    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert_eq!(choices.len(), 3);
    assert!(choices.iter().all(|choice| matches!(
        &choice.action,
        SimpleAction::Attach { attachments, .. } if attachments.len() == 2
    )));

    // Pick the first pair and verify both Pokemon got a Psychic Energy.
    let chosen = choices[0].clone();
    let targets: Vec<usize> = match &chosen.action {
        SimpleAction::Attach { attachments, .. } => {
            attachments.iter().map(|(_, _, idx)| *idx).collect()
        }
        _ => unreachable!(),
    };
    game.apply_action(&chosen);

    let state = game.get_state_clone();
    for idx in targets {
        assert_eq!(
            state.in_play_pokemon[0][idx]
                .as_ref()
                .unwrap()
                .attached_energy,
            vec![EnergyType::Psychic],
            "each chosen benched Pokemon should have received a [P] Energy"
        );
    }
}
