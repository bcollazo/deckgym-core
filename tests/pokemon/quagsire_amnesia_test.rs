use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Quagsire's Amnesia locks out one of the Defending Pokémon's attacks for the opponent's next
/// turn. Snorlax only has Rollout, so the random pick is forced and Snorlax cannot attack.
#[test]
fn test_amnesia_blocks_the_defenders_attack_next_turn() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B3b037Quagsire).with_energy(vec![
                EnergyType::Fighting,
                EnergyType::Fighting,
                EnergyType::Colorless,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax).with_energy(vec![
            EnergyType::Colorless,
            EnergyType::Colorless,
            EnergyType::Colorless,
            EnergyType::Colorless,
        ])],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3b037Quagsire, 0),
        is_stack: false,
    });
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });

    game.play_until_stable();

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 1);
    assert!(
        !choices
            .iter()
            .any(|choice| matches!(&choice.action, SimpleAction::Attack(attack) if attack.title == "Rollout")),
        "Rollout should be locked out by Amnesia"
    );
}

/// Without Amnesia the same board lets Snorlax use Rollout, so the test above is meaningful.
#[test]
fn test_defender_can_attack_without_amnesia() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B3b037Quagsire).with_energy(vec![
                EnergyType::Fighting,
                EnergyType::Fighting,
                EnergyType::Colorless,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax).with_energy(vec![
            EnergyType::Colorless,
            EnergyType::Colorless,
            EnergyType::Colorless,
            EnergyType::Colorless,
        ])],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });

    game.play_until_stable();

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 1);
    assert!(
        choices
            .iter()
            .any(|choice| matches!(&choice.action, SimpleAction::Attack(attack) if attack.title == "Rollout")),
        "Rollout should be available when Amnesia was not used"
    );
}
