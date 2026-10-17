use deckgym::{
    actions::SimpleAction, card_ids::CardId, models::PlayedCard,
    test_support::get_test_game_with_board,
};

#[test]
fn test_discard_fossil_generated_once_per_fossil() {
    let game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1216HelixFossil),
        ],
        vec![PlayedCard::from_id(CardId::A1202Chansey)],
    );

    let (_, actions) = game.get_state_clone().generate_possible_actions();
    let discard_fossil_actions: Vec<_> = actions
        .iter()
        .filter(|a| matches!(a.action, SimpleAction::DiscardFossil { .. }))
        .collect();

    assert_eq!(discard_fossil_actions.len(), 1);
    assert_eq!(
        discard_fossil_actions[0].action,
        SimpleAction::DiscardFossil { in_play_idx: 1 }
    );
}
