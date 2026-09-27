use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

/// Resolves automatic single-choice actions (like the forced start-of-turn draw) until a real
/// decision point is reached.
fn advance_to_menu(game: &mut deckgym::Game) {
    loop {
        let (_actor, actions) = game.get_state_clone().generate_possible_actions();
        if let [only_action] = actions.as_slice() {
            if matches!(only_action.action, SimpleAction::DrawCard { .. }) {
                game.apply_action(only_action);
                continue;
            }
        }
        break;
    }
}

/// Player 1 (holding Snorlax) optionally attacks Wobbuffet, then hands the turn to player 0.
fn wobbuffet_game(opponent_attacks: bool) -> deckgym::Game<'static> {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.turn_count = 4;
    state.current_player = 1;
    state.set_board(
        vec![PlayedCard::from_id(CardId::A4086Wobbuffet)
            .with_energy(vec![EnergyType::Psychic, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax).with_energy(vec![EnergyType::Colorless; 4])],
    );
    game.set_state(state);

    if opponent_attacks {
        game.apply_action(&Action {
            actor: 1,
            action: attack_action(CardId::A1211Snorlax, 0),
            is_stack: false,
        });
    }
    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    advance_to_menu(&mut game);
    game
}

/// Wobbuffet - Reply Strongly: 30 damage, +50 "If this Pokémon was damaged by an attack during
/// your opponent's last turn while it was in the Active Spot."
#[test]
fn test_reply_strongly_boosted_after_being_attacked_last_turn() {
    let mut game = wobbuffet_game(true);
    assert_eq!(
        game.get_state_clone().get_active(0).get_remaining_hp(),
        20,
        "Wobbuffet should have taken Rollout's 70 damage"
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A4086Wobbuffet, 0),
        is_stack: false,
    });

    // Snorlax 150 HP - (30 + 50) = 70; neither Pokemon triggers Weakness here.
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 70);
}

#[test]
fn test_reply_strongly_base_damage_when_not_attacked_last_turn() {
    let mut game = wobbuffet_game(false);
    assert_eq!(
        game.get_state_clone().get_active(0).get_remaining_hp(),
        90,
        "Wobbuffet should be untouched"
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A4086Wobbuffet, 0),
        is_stack: false,
    });

    // Snorlax 150 HP - 30 = 120.
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 120);
}
