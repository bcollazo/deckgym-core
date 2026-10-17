use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

fn tinkaton_board() -> deckgym::Game<'static> {
    get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B2a074Tinkaton).with_energy(vec![
                EnergyType::Metal,
                EnergyType::Metal,
                EnergyType::Colorless,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::A1004VenusaurEx)],
    )
}

/// After using Gigaton Hammer, Tinkaton can't use Gigaton Hammer during its next turn.
#[test]
fn test_tinkaton_cannot_use_gigaton_hammer_the_turn_after() {
    let mut game = tinkaton_board();

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2a074Tinkaton, 0),
        is_stack: false,
    });

    // Pass through the opponent's turn to get back to Tinkaton's next turn.
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });

    let (actor, actions) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(
        !actions
            .iter()
            .any(|action| matches!(action.action, SimpleAction::Attack(_))),
        "Tinkaton should not be able to use Gigaton Hammer the turn after using it"
    );
}

#[test]
fn test_tinkaton_gigaton_hammer_deals_140_damage() {
    let mut game = tinkaton_board();

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2a074Tinkaton, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 190 - 140);
}
