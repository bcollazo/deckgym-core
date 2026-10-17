use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    effects::CardEffect,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
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

fn end_turn(game: &mut deckgym::Game, actor: usize) {
    game.apply_action(&Action {
        actor,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    advance_to_menu(game);
}

fn octillery_game() -> deckgym::Game<'static> {
    get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A4056Octillery)
            .with_energy(vec![EnergyType::Water, EnergyType::Water])],
        vec![
            PlayedCard::from_id(CardId::A1211Snorlax).with_energy(vec![EnergyType::Colorless; 4]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    )
}

/// Octillery's Octazooka deals its damage and leaves a coin-flip attack block on the Defending
/// Pokemon that outlives a single turn (unlike Magnezone's Mirror Shot).
#[test]
fn test_octazooka_block_outlives_a_single_turn() {
    let mut game = octillery_game();

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A4056Octillery, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        100,
        "Octazooka should still deal its 50 damage"
    );
    assert!(
        state
            .get_active(1)
            .has_effect(&CardEffect::CoinFlipToBlockAttack),
        "Octazooka should leave a CoinFlipToBlockAttack effect on the defender"
    );

    // Two full turn cycles later the effect is still there: it lasts until the Defending
    // Pokemon leaves the Active Spot, not just for the opponent's next turn.
    end_turn(&mut game, 0);
    end_turn(&mut game, 1);
    end_turn(&mut game, 0);

    assert!(
        game.get_state_clone()
            .get_active(1)
            .has_effect(&CardEffect::CoinFlipToBlockAttack),
        "Octazooka's block should not expire while the defender stays Active"
    );
}

/// The block is cleared once the Defending Pokemon leaves the Active Spot.
#[test]
fn test_octazooka_block_is_cleared_when_defender_leaves_active_spot() {
    let mut game = octillery_game();

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A4056Octillery, 0),
        is_stack: false,
    });
    end_turn(&mut game, 0);

    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::Retreat(1),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_name(), "Bulbasaur");
    let benched_snorlax = state.in_play_pokemon[1][1]
        .as_ref()
        .expect("Snorlax should be on the bench after retreating");
    assert!(
        !benched_snorlax.has_effect(&CardEffect::CoinFlipToBlockAttack),
        "Leaving the Active Spot should clear Octazooka's block"
    );
}
