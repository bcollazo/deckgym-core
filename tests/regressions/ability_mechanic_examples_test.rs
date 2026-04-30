use crate::deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::PlayedCard,
    test_support::get_test_game_with_board,
};

#[test]
fn test_weezing_gas_leak_poisons_opponent_active() {
    // Weezing's Gas Leak: Once during your turn, if this Pokémon is in the Active Spot,
    // you may make your opponent's Active Pokémon Poisoned.
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1177Weezing)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let ability_action = Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    };
    game.apply_action(&ability_action);

    let state = game.get_state_clone();
    let opponent_active = state.in_play_pokemon[1][0]
        .as_ref()
        .expect("Opponent active should be in play");
    assert!(
        opponent_active.is_poisoned(),
        "Weezing's Gas Leak should poison the opponent's Active Pokémon"
    );
}

#[test]
fn test_indeedee_ex_watch_over_heals_active() {
    // Indeedee ex's Watch Over: Once during your turn, you may heal 20 damage from your Active Pokémon.
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B1121IndeedeeEx).with_remaining_hp(80)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let ability_action = Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    };
    game.apply_action(&ability_action);

    let state = game.get_state_clone();
    let active = state.in_play_pokemon[0][0]
        .as_ref()
        .expect("Indeedee ex should still be in play");
    assert_eq!(
        active.get_remaining_hp(),
        100,
        "Watch Over should heal 20 damage from Active Pokémon"
    );
}

#[test]
fn test_pidgeot_drive_off_forces_opponent_switch() {
    // Pidgeot's Drive Off: Once during your turn, you may switch out your opponent's Active Pokémon
    // to the Bench. (Your opponent chooses the new Active Pokémon.)
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1188Pidgeot)],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1002Ivysaur),
        ],
    );

    let ability_action = Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    };
    game.apply_action(&ability_action);

    // After using Drive Off, the opponent should be asked to choose a new active Pokémon
    let state = game.get_state_clone();
    let (_actor, actions) = state.generate_possible_actions();
    let has_activate = actions
        .iter()
        .any(|a| matches!(a.action, SimpleAction::Activate { player: 1, .. }));
    assert!(
        has_activate,
        "Drive Off should prompt opponent to choose a new Active Pokémon"
    );
}
