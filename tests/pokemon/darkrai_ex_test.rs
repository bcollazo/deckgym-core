use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

#[test]
fn test_darkrai_ex_nightmare_aura() {
    // Darkrai ex's Nightmare Aura: Whenever you attach a Darkness Energy from your Energy Zone to this Pokémon, do 20 damage to your opponent's Active Pokémon.

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up Darkrai ex vs Bulbasaur
    state.set_board(
        vec![PlayedCard::from_id(CardId::A2110DarkraiEx)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    game.set_state(state);

    // Attach Darkness energy from Energy Zone to Darkrai ex
    let attach_action = Action {
        actor: 0,
        action: SimpleAction::Attach {
            attachments: vec![(1, EnergyType::Darkness, 0)],
            is_turn_energy: true,
        },
        is_stack: false,
    };

    game.apply_action(&attach_action);
    let state = game.get_state_clone();

    // Check that Darkrai ex has the energy attached
    assert_eq!(
        state.get_active(0).attached_energy.len(),
        1,
        "Darkrai ex should have 1 energy attached"
    );

    // Check that opponent's active took 20 damage
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        50,
        "Opponent's active should have taken 20 damage (70 - 20 = 50)"
    );
}

#[test]
fn test_darkrai_ex_nightmare_aura_only_darkness() {
    // Test that non-Darkness energy doesn't trigger the ability

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A2110DarkraiEx)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    game.set_state(state);

    // Attach Fire energy from Energy Zone to Darkrai ex
    let attach_action = Action {
        actor: 0,
        action: SimpleAction::Attach {
            attachments: vec![(1, EnergyType::Fire, 0)],
            is_turn_energy: true,
        },
        is_stack: false,
    };

    game.apply_action(&attach_action);
    let state = game.get_state_clone();

    // Check that opponent's active did NOT take damage
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        70,
        "Opponent's active should not have taken damage from non-Darkness energy"
    );
}

/// Darkrai ex on the Active Spot against a 70-HP Bulbasaur; `energy` attached to it by an effect
/// (`is_turn_energy: false`, e.g. Misty or an Ability) rather than as the turn's attachment.
fn effect_attach(energy: EnergyType) -> deckgym::State {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.set_board(
        vec![PlayedCard::from_id(CardId::A2110DarkraiEx)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    game.set_state(state);
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attach {
            attachments: vec![(1, energy, 0)],
            is_turn_energy: false,
        },
        is_stack: false,
    });
    game.get_state_clone()
}

#[test]
fn test_darkrai_ex_nightmare_aura_triggers_on_effect_attachment_from_zone() {
    // Nightmare Aura reads "attach a [D] Energy from your Energy Zone", like Charizard's Flame
    // Aura: an effect that attaches from the Energy Zone triggers it, not only the turn's
    // attachment.
    let state = effect_attach(EnergyType::Darkness);
    assert_eq!(state.get_active(0).attached_energy.len(), 1);
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        50,
        "Opponent's active should have taken 20 damage (70 - 20 = 50)"
    );
}

#[test]
fn test_darkrai_ex_nightmare_aura_ignores_non_darkness_effect_attachment() {
    let state = effect_attach(EnergyType::Water);
    assert_eq!(state.get_active(0).attached_energy.len(), 1);
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        70,
        "Opponent's active should not have taken damage from a non-Darkness Energy"
    );
}

#[test]
fn test_darkrai_ex_nightmare_aura_ko_triggers_promotion() {
    // If Nightmare Aura KOs the opponent's Active and they have a bench,
    // promotion should be queued.

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Darkrai ex vs Bulbasaur at 20 HP with a benched Pokemon
    state.set_board(
        vec![PlayedCard::from_id(CardId::A2110DarkraiEx)],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_remaining_hp(20),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );
    state.current_player = 0;
    game.set_state(state);

    // Attach Darkness energy from Energy Zone to Darkrai ex
    let attach_action = Action {
        actor: 0,
        action: SimpleAction::Attach {
            attachments: vec![(1, EnergyType::Darkness, 0)],
            is_turn_energy: true,
        },
        is_stack: false,
    };

    game.apply_action(&attach_action);
    let state = game.get_state_clone();

    // Promotion should be queued for the opponent (player 1)
    let (actor, actions) = state.generate_possible_actions();
    let has_promotion = actor == 1
        && actions
            .iter()
            .any(|a| matches!(a.action, SimpleAction::Activate { .. }));

    assert!(
        has_promotion,
        "Expected promotion to be queued after Darkrai Aura KOs active"
    );
}
