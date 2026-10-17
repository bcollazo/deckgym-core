use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::PlayedCard,
    test_support::get_initialized_game,
};

fn end_turn(game: &mut deckgym::Game<'static>, actor: usize) {
    game.apply_action(&Action {
        actor,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
}

#[test]
fn test_leftovers_heals_active_holder_at_end_of_its_owners_turn() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_damage(50)
            .with_tool(get_card_by_enum(CardId::A3b067Leftovers))],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    game.set_state(state);

    // Bulbasaur has 70 HP, so 50 damage leaves 20.
    assert_eq!(game.get_state_clone().get_remaining_hp(0, 0), 20);

    end_turn(&mut game, 0);

    assert_eq!(
        game.get_state_clone().get_remaining_hp(0, 0),
        30,
        "Leftovers should heal 10 from its Active holder at the end of its owner's turn"
    );
}

#[test]
fn test_leftovers_does_nothing_from_the_bench_or_on_the_opponents_turn() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1033Charmander),
            // Holder is benched rather than in the Active Spot.
            PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_damage(50)
                .with_tool(get_card_by_enum(CardId::A3b067Leftovers)),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    game.set_state(state);

    end_turn(&mut game, 0);
    assert_eq!(
        game.get_state_clone().get_remaining_hp(0, 1),
        20,
        "A benched holder should not be healed by Leftovers"
    );

    // Player 1's turn ending must not heal player 0's Pokémon either.
    end_turn(&mut game, 1);
    assert_eq!(
        game.get_state_clone().get_remaining_hp(0, 1),
        20,
        "Leftovers should not heal at the end of the opponent's turn"
    );
}
