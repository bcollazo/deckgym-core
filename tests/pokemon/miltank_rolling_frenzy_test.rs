use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Miltank - Rolling Frenzy: "Until this Pokémon leaves the Active Spot, this Pokémon's Rolling
/// Frenzy attack does +30 damage. This effect stacks."
#[test]
fn test_miltank_rolling_frenzy_stacks_while_it_stays_active() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A4a062Miltank).with_energy(vec![EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    for expected_remaining in [140, 100, 30] {
        let mut state = game.get_state_clone();
        state.current_player = 0;
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A4a062Miltank, 0),
            is_stack: false,
        });
        game.play_until_stable();

        assert_eq!(
            game.get_state_clone().get_active(1).get_remaining_hp(),
            expected_remaining
        );
    }
}

/// The bonus is tied to the Active Spot: switching Miltank out clears the stacked effect.
#[test]
fn test_miltank_rolling_frenzy_bonus_is_lost_when_it_leaves_the_active_spot() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A4a062Miltank).with_energy(vec![EnergyType::Colorless]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A4a062Miltank, 0),
        is_stack: false,
    });
    game.play_until_stable();
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 140);

    // Swap Miltank to the Bench and back, which clears effects on it.
    let mut state = game.get_state_clone();
    state.current_player = 0;
    game.set_state(state);
    game.apply_action(&Action {
        actor: 0,
        action: deckgym::actions::SimpleAction::Activate {
            player: 0,
            in_play_idx: 1,
        },
        is_stack: false,
    });
    let mut state = game.get_state_clone();
    state.current_player = 0;
    game.set_state(state);
    game.apply_action(&Action {
        actor: 0,
        action: deckgym::actions::SimpleAction::Activate {
            player: 0,
            in_play_idx: 1,
        },
        is_stack: false,
    });

    let mut state = game.get_state_clone();
    state.current_player = 0;
    game.set_state(state);
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A4a062Miltank, 0),
        is_stack: false,
    });
    game.play_until_stable();

    // Back to the unboosted 10 damage.
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 130);
}
