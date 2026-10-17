use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::PlayedCard,
    test_support::get_test_game_with_board,
};

/// Sylveon's Soothing Ribbon: "Once during your turn, if this Pokémon has a Pokémon Tool
/// attached, you may heal 30 damage from 1 of your Pokémon."
#[test]
fn test_sylveon_soothing_ribbon_heals_when_tool_attached() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B3b030Sylveon)
            .with_tool(get_card_by_enum(CardId::A2148RockyHelmet))
            .with_remaining_hp(70)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    });

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    let heal_action = choices
        .into_iter()
        .find(|a| matches!(a.action, SimpleAction::Heal { in_play_idx: 0, .. }))
        .expect("Soothing Ribbon should offer to heal Sylveon");
    game.apply_action(&heal_action);

    let state = game.get_state_clone();
    assert_eq!(state.get_active(0).get_remaining_hp(), 100);
}

#[test]
fn test_sylveon_soothing_ribbon_unusable_without_tool() {
    let game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B3b030Sylveon).with_remaining_hp(70)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let (_actor, choices) = game.get_state_clone().generate_possible_actions();
    assert!(!choices
        .iter()
        .any(|a| matches!(a.action, SimpleAction::UseAbility { in_play_idx: 0 })));
}
