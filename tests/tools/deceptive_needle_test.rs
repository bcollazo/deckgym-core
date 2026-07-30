use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
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
fn test_deceptive_needle_damages_opponent_active_at_end_of_holders_turn() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        // Murkrow is a [D] Pokemon, so the needle's condition is met.
        vec![PlayedCard::from_id(CardId::A2096Murkrow)
            .with_tool(get_card_by_enum(CardId::B4148DeceptiveNeedle))],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    game.set_state(state);

    let opponent_hp_before = game.get_state_clone().get_remaining_hp(1, 0);

    end_turn(&mut game, 0);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_remaining_hp(1, 0),
        opponent_hp_before - 10,
        "Deceptive Needle should do 10 damage to the opponent's Active at the end of your turn"
    );
}

#[test]
fn test_deceptive_needle_does_nothing_on_a_non_darkness_holder() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        // Bulbasaur is [G], not [D], so the needle stays dormant.
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_tool(get_card_by_enum(CardId::B4148DeceptiveNeedle))],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    game.set_state(state);

    let opponent_hp_before = game.get_state_clone().get_remaining_hp(1, 0);

    end_turn(&mut game, 0);

    assert_eq!(
        game.get_state_clone().get_remaining_hp(1, 0),
        opponent_hp_before,
        "A non-[D] holder should not trigger Deceptive Needle"
    );
}

#[test]
fn test_deceptive_needle_does_nothing_from_the_bench_or_on_the_opponents_turn() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_energy(vec![EnergyType::Grass]),
            // [D] holder, but benched rather than in the Active Spot.
            PlayedCard::from_id(CardId::A2096Murkrow)
                .with_tool(get_card_by_enum(CardId::B4148DeceptiveNeedle)),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    game.set_state(state);

    let opponent_hp_before = game.get_state_clone().get_remaining_hp(1, 0);

    // End player 0's turn: the needle is on the bench, so nothing happens.
    end_turn(&mut game, 0);
    assert_eq!(
        game.get_state_clone().get_remaining_hp(1, 0),
        opponent_hp_before,
        "A benched holder should not trigger Deceptive Needle"
    );

    // End player 1's turn: the needle only fires at the end of *its owner's* turn, and
    // player 0 (the owner) must not take damage from their own tool either.
    let player_0_hp_before = game.get_state_clone().get_remaining_hp(0, 0);
    end_turn(&mut game, 1);
    let state = game.get_state_clone();
    assert_eq!(
        state.get_remaining_hp(0, 0),
        player_0_hp_before,
        "Deceptive Needle should not fire at the end of the opponent's turn"
    );
}
