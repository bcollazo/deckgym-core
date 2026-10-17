use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard},
    test_support::get_initialized_game,
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

fn can_retreat(game: &deckgym::Game, player: usize) -> bool {
    let (actor, actions) = game.get_state_clone().generate_possible_actions();
    actor == player
        && actions
            .iter()
            .any(|a| matches!(a.action, SimpleAction::Retreat(_)))
}

fn end_turn(game: &mut deckgym::Game, actor: usize) {
    game.apply_action(&Action {
        actor,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    advance_to_menu(game);
}

/// Team Rocket's Goo-zooka: "Until the end of your opponent's next turn, your opponent's Active
/// Pokémon's Retreat Cost is 1 more."
#[test]
fn test_goozooka_blocks_retreat_during_opponents_next_turn_then_expires() {
    let goozooka = get_card_by_enum(CardId::B4a068TeamRocketsGoozooka).as_trainer();

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    // Blastoise's Retreat Cost is 3 Colorless; give the opponent exactly that much energy so a
    // 1-more increase is the difference between being able to retreat or not.
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![
            PlayedCard::from_id(CardId::A1055Blastoise).with_energy(vec![EnergyType::Colorless; 3]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );
    state.current_player = 0;
    state.turn_count = 3;
    state.hands[0] = vec![Card::Trainer(goozooka.clone())];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: goozooka,
        },
        is_stack: false,
    });

    // Player 0 ends their turn -> becomes the opponent's (player 1) next turn.
    end_turn(&mut game, 0);
    assert!(
        !can_retreat(&game, 1),
        "Opponent should not be able to retreat with only 3 energy while Retreat Cost is +1"
    );

    // Opponent ends their turn without retreating -> back to player 0.
    end_turn(&mut game, 1);
    // Player 0 ends their turn -> the opponent's *second* next turn, where the effect expired.
    end_turn(&mut game, 0);
    assert!(
        can_retreat(&game, 1),
        "Retreat Cost increase should have expired after the opponent's next turn ended"
    );
}
