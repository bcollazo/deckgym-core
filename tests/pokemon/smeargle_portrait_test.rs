use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::PlayedCard,
    test_support::get_test_game_with_board,
    Game,
};

/// Smeargle's Portrait: "Once during your turn, if this Pokémon is in the Active Spot, you may
/// look at a random Supporter card from your opponent's hand. Use the effect of that card as the
/// effect of this Ability."
fn game_with_smeargle(opponent_hand: Vec<CardId>) -> Game<'static> {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2130Smeargle)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let mut state = game.get_state_clone();
    state.hands[1] = opponent_hand
        .into_iter()
        .map(get_card_by_enum)
        .collect::<Vec<_>>();
    game.set_state(state);
    game
}

fn offers_portrait(game: &Game<'_>) -> bool {
    let (_, choices) = game.get_state_clone().generate_possible_actions();
    choices
        .iter()
        .any(|c| matches!(c.action, SimpleAction::UseAbility { in_play_idx: 0 }))
}

#[test]
fn test_portrait_is_offered_only_when_the_opponent_holds_a_usable_supporter() {
    assert!(offers_portrait(&game_with_smeargle(vec![
        CardId::A1223Giovanni
    ])));
    assert!(
        !offers_portrait(&game_with_smeargle(vec![CardId::A1001Bulbasaur])),
        "no Supporter in the opponent's hand"
    );
    assert!(
        !offers_portrait(&game_with_smeargle(vec![])),
        "empty opponent hand"
    );
}

/// Copying Professor's Research ("Draw 2 cards.") should draw for Smeargle's owner, and must
/// leave the copied card in the opponent's hand.
#[test]
fn test_portrait_copies_the_supporters_effect_without_taking_the_card() {
    let mut game = game_with_smeargle(vec![CardId::A1219Erika]);
    let mut state = game.get_state_clone();
    state.in_play_pokemon[0][0] = Some(PlayedCard::from_id(CardId::B2130Smeargle));
    // Erika: "Heal 50 damage from 1 of your [G] Pokémon." Give the owner a damaged [G] target.
    state.in_play_pokemon[0][1] = Some(PlayedCard::from_id(CardId::A1001Bulbasaur).with_damage(50));
    game.set_state(state);

    assert!(offers_portrait(&game));
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    });

    // Resolve Erika's heal target choice if one is offered.
    let (_, choices) = game.get_state_clone().generate_possible_actions();
    if let Some(heal) = choices
        .iter()
        .find(|c| matches!(c.action, SimpleAction::Heal { .. }))
    {
        let heal = heal.clone();
        game.apply_action(&heal);
    }

    let state = game.get_state_clone();
    assert_eq!(
        state.in_play_pokemon[0][1]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        70,
        "Erika's effect should have healed the owner's Bulbasaur"
    );
    assert_eq!(
        state.hands[1].len(),
        1,
        "the opponent keeps the Supporter that was only looked at"
    );
}
