use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Test Walking Wake B3a 053 - Sweeping Billow: discard an Energy from itself, and this
/// attack also does 20 damage to each of the opponent's Benched Pokémon.
#[test]
fn test_walking_wake_sweeping_billow() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B3a053WalkingWake)
            .with_energy(vec![EnergyType::Fire, EnergyType::Water])],
        vec![
            PlayedCard::from_id(CardId::B4197WailordEx),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3a053WalkingWake, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    // 60 fixed damage to the opponent's Active.
    assert_eq!(state.get_active(1).get_remaining_hp(), 250 - 60);
    // 20 damage to the opponent's Benched Bulbasaur.
    let benched_bulbasaur = state.in_play_pokemon[1][1]
        .as_ref()
        .expect("Bulbasaur should still be on the bench");
    assert_eq!(benched_bulbasaur.get_remaining_hp(), 70 - 20);
    // Exactly 1 of the 2 attached Energy was discarded.
    assert_eq!(state.get_active(0).attached_energy.len(), 1);
}
