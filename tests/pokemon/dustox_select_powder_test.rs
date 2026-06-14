use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard, StatusCondition},
    test_support::get_initialized_game,
};

/// Dustox's Select Powder: "Choose either Poisoned or Confused. Your opponent's
/// Active Pokémon is now affected by that Special Condition." (60 damage).
#[test]
fn test_dustox_select_powder_offers_poison_and_confuse_then_poisons() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B1007Dustox).with_energy(vec![EnergyType::Grass])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        10,
        "Select Powder should deal 60 damage (70 HP Bulbasaur -> 10)"
    );

    let (actor, choices) = state.generate_possible_actions();
    assert_eq!(actor, 0);
    assert_eq!(
        choices.len(),
        2,
        "Player should choose between Poisoned and Confused"
    );
    assert!(choices.iter().any(|c| matches!(
        c.action,
        SimpleAction::ApplyStatusToOpponentActive {
            condition: StatusCondition::Poisoned
        }
    )));
    assert!(choices.iter().any(|c| matches!(
        c.action,
        SimpleAction::ApplyStatusToOpponentActive {
            condition: StatusCondition::Confused
        }
    )));

    let poison_choice = choices
        .iter()
        .find(|c| {
            matches!(
                c.action,
                SimpleAction::ApplyStatusToOpponentActive {
                    condition: StatusCondition::Poisoned
                }
            )
        })
        .cloned()
        .expect("Expected a Poisoned choice");
    game.apply_action(&poison_choice);

    let state = game.get_state_clone();
    assert!(
        state.get_active(1).is_poisoned(),
        "Choosing Poisoned should poison the opponent's Active Pokémon"
    );
    assert!(
        !state.get_active(1).is_confused(),
        "The opponent's Active should not also be Confused"
    );
}

#[test]
fn test_dustox_select_powder_can_confuse_opponent_active() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B1007Dustox).with_energy(vec![EnergyType::Grass])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let (_, choices) = game.get_state_clone().generate_possible_actions();
    let confuse_choice = choices
        .iter()
        .find(|c| {
            matches!(
                c.action,
                SimpleAction::ApplyStatusToOpponentActive {
                    condition: StatusCondition::Confused
                }
            )
        })
        .cloned()
        .expect("Expected a Confused choice");
    game.apply_action(&confuse_choice);

    assert!(
        game.get_state_clone().get_active(1).is_confused(),
        "Choosing Confused should confuse the opponent's Active Pokémon"
    );
}
