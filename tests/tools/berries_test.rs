use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{PlayedCard, StatusCondition},
    test_support::get_test_game_with_board,
    Game, State,
};

fn end_turn(game: &mut Game<'static>) {
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    game.play_until_stable();
}

fn game_with(player_board: Vec<PlayedCard>) -> Game<'static> {
    get_test_game_with_board(
        player_board,
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    )
}

/// Lum Berry: "At the end of each turn, if the Pokémon this card is attached to is affected by any
/// Special Conditions, it recovers from all of them, and discard this card."
#[test]
fn test_lum_berry_cures_conditions_and_discards_itself() {
    let mut game = game_with(vec![PlayedCard::from_id(CardId::A1202Chansey)
        .with_tool(get_card_by_enum(CardId::A2149LumBerry))]);
    let mut state = game.get_state_clone();
    state.apply_status_condition(0, 0, StatusCondition::Confused);
    state.apply_status_condition(0, 0, StatusCondition::Poisoned);
    game.set_state(state);

    end_turn(&mut game);

    let state: State = game.get_state_clone();
    let chansey = state.get_active(0);
    assert!(!chansey.is_confused(), "Lum Berry should cure Confused");
    assert!(!chansey.is_poisoned(), "Lum Berry should cure Poisoned");
    assert!(
        chansey.attached_tools.is_empty(),
        "Lum Berry should discard itself after firing"
    );
    assert!(
        state.discard_piles[0]
            .iter()
            .any(|c| c.get_name() == "Lum Berry"),
        "Lum Berry should be in the discard pile"
    );
}

/// With no Special Conditions, Lum Berry stays attached.
#[test]
fn test_lum_berry_stays_attached_without_special_conditions() {
    let mut game = game_with(vec![PlayedCard::from_id(CardId::A1202Chansey)
        .with_tool(get_card_by_enum(CardId::A2149LumBerry))]);

    end_turn(&mut game);

    assert!(
        game.get_state_clone().get_active(0).has_tool_attached(),
        "Lum Berry should remain attached while its holder is healthy"
    );
}

/// Sitrus Berry: "At the end of each turn, if the Pokémon this card is attached to has half of its
/// maximum HP or less remaining, heal 30 damage from it. If you do, discard this card."
#[test]
fn test_sitrus_berry_heals_30_at_half_hp_and_discards_itself() {
    // Chansey has 120 HP; at 60 remaining it is exactly at half, which qualifies.
    let mut game = game_with(vec![PlayedCard::from_id(CardId::A1202Chansey)
        .with_remaining_hp(60)
        .with_tool(get_card_by_enum(CardId::B1218SitrusBerry))]);

    end_turn(&mut game);

    let state = game.get_state_clone();
    let chansey = state.get_active(0);
    assert_eq!(
        chansey.get_remaining_hp(),
        90,
        "Sitrus Berry should heal 30 damage"
    );
    assert!(
        chansey.attached_tools.is_empty(),
        "Sitrus Berry should discard itself after healing"
    );
}

/// Above half HP, Sitrus Berry does nothing and stays attached.
#[test]
fn test_sitrus_berry_does_nothing_above_half_hp() {
    let mut game = game_with(vec![PlayedCard::from_id(CardId::A1202Chansey)
        .with_remaining_hp(70)
        .with_tool(get_card_by_enum(CardId::B1218SitrusBerry))]);

    end_turn(&mut game);

    let state = game.get_state_clone();
    assert_eq!(state.get_active(0).get_remaining_hp(), 70);
    assert!(
        state.get_active(0).has_tool_attached(),
        "Sitrus Berry should stay attached above half HP"
    );
}

/// Both berries say "at the end of each turn", so they also fire on the opponent's turn — here on
/// a Pokémon belonging to the player who is not ending the turn.
#[test]
fn test_sitrus_berry_fires_at_the_end_of_the_opponents_turn_too() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1202Chansey)
            .with_remaining_hp(50)
            .with_tool(get_card_by_enum(CardId::B1218SitrusBerry))],
    );

    // Player 0 ends their turn; the berry belongs to player 1.
    end_turn(&mut game);

    let state = game.get_state_clone();
    let chansey = state.in_play_pokemon[1][0]
        .as_ref()
        .expect("Opponent active should still be there");
    assert_eq!(chansey.get_remaining_hp(), 80);
    assert!(chansey.attached_tools.is_empty());
}
