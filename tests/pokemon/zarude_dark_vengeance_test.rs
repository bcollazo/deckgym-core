use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game, get_test_game_with_board},
};

/// Plays a full turn cycle in which player 1 Knocks Out player 0's Active `ko_victim` with
/// Venusaur ex's Razor Leaf (60 damage), player 0 promotes Zarude, and then Zarude uses Dark
/// Vengeance on the (still undamaged, 190 HP) Venusaur ex. Returns Venusaur ex's remaining HP.
fn venusaur_hp_after_zarude_revenge(ko_victim: CardId) -> u32 {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.set_board(
        vec![
            PlayedCard::from_id(ko_victim),
            PlayedCard::from_id(CardId::B3114Zarude)
                .with_energy(vec![EnergyType::Darkness, EnergyType::Darkness]),
        ],
        vec![
            PlayedCard::from_id(CardId::A1004VenusaurEx).with_energy(vec![
                EnergyType::Grass,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
        ],
    );
    state.current_player = 1;
    state.turn_count = 4;
    game.set_state(state);

    // Player 1 Knocks Out player 0's Active Pokemon.
    game.apply_action(&Action {
        actor: 1,
        action: attack_action(CardId::A1004VenusaurEx, 0),
        is_stack: false,
    });

    // Player 0 promotes Zarude, its only Benched Pokemon.
    game.play_until_stable();
    assert_eq!(game.get_state_clone().get_active(0).get_name(), "Zarude");

    // Player 1 ends the turn, so the Knock Out now counts as "during your opponent's last turn".
    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    game.play_until_stable();

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3114Zarude, 0),
        is_stack: false,
    });

    game.get_state_clone().get_remaining_hp(1, 0)
}

/// Without a Knock Out last turn, Dark Vengeance deals its base 40 damage.
#[test]
fn test_zarude_dark_vengeance_base_damage() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B3114Zarude)
            .with_energy(vec![EnergyType::Darkness, EnergyType::Darkness])],
        vec![PlayedCard::from_id(CardId::A1004VenusaurEx)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3114Zarude, 0),
        is_stack: false,
    });

    assert_eq!(
        game.get_state_clone().get_remaining_hp(1, 0),
        150,
        "Dark Vengeance should deal 40 damage without a Knock Out last turn (190 - 40 = 150)"
    );
}

/// A Darkness Pokemon Knocked Out during the opponent's last turn adds 80 damage.
#[test]
fn test_zarude_dark_vengeance_extra_damage_after_darkness_knockout() {
    assert_eq!(
        venusaur_hp_after_zarude_revenge(CardId::A1164Ekans),
        70,
        "Dark Vengeance should deal 120 damage after a Darkness Pokemon was Knocked Out (190 - 120 = 70)"
    );
}

/// A Knock Out of a non-Darkness Pokemon does not trigger the bonus.
#[test]
fn test_zarude_dark_vengeance_ignores_non_darkness_knockout() {
    assert_eq!(
        venusaur_hp_after_zarude_revenge(CardId::A1053Squirtle),
        150,
        "Dark Vengeance should deal only 40 damage after a non-Darkness Pokemon was Knocked Out"
    );
}
