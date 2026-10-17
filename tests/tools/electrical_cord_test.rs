use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::get_test_game_with_board,
    Game, State,
};

/// Player 0's Active Pikachu holds Electrical Cord and 2 Lightning Energy; the opponent knocks it
/// out with an attack. Returns the state afterwards.
fn knock_out_cord_holder(bench: Vec<PlayedCard>) -> State {
    let mut board = vec![PlayedCard::from_id(CardId::A1094Pikachu)
        .with_energy(vec![EnergyType::Lightning, EnergyType::Lightning])
        .with_tool(get_card_by_enum(CardId::A3a065ElectricalCord))];
    board.extend(bench);
    let mut game: Game<'static> =
        get_test_game_with_board(board, vec![PlayedCard::from_id(CardId::A1143Machop)]);
    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (1, 0),
            targets: vec![(100, 0, 0)],
            is_from_active_attack: true,
        },
        is_stack: false,
    });
    game.get_state_clone()
}

/// Electrical Cord: "...move 2 [L] Energy from that Pokémon and attach 1 Energy each to 2 of your
/// Benched Pokémon."
#[test]
fn test_electrical_cord_moves_one_energy_to_each_of_two_benched_pokemon() {
    let state = knock_out_cord_holder(vec![
        PlayedCard::from_id(CardId::A1033Charmander),
        PlayedCard::from_id(CardId::A1001Bulbasaur),
    ]);

    assert_eq!(
        state.in_play_pokemon[0][1]
            .as_ref()
            .unwrap()
            .attached_energy,
        vec![EnergyType::Lightning]
    );
    assert_eq!(
        state.in_play_pokemon[0][2]
            .as_ref()
            .unwrap()
            .attached_energy,
        vec![EnergyType::Lightning]
    );
    assert!(state.discard_energies[0].is_empty());
}

/// With only 1 Benched Pokémon the second Energy cannot move, so it stays on the Knocked Out
/// Pokémon and is discarded with it instead of disappearing.
#[test]
fn test_electrical_cord_with_one_benched_pokemon_discards_the_energy_it_cannot_move() {
    let state = knock_out_cord_holder(vec![PlayedCard::from_id(CardId::A1033Charmander)]);

    assert_eq!(
        state.in_play_pokemon[0][1]
            .as_ref()
            .unwrap()
            .attached_energy,
        vec![EnergyType::Lightning]
    );
    assert_eq!(
        state.discard_energies[0],
        vec![EnergyType::Lightning],
        "The Energy that could not move should be discarded, not lost"
    );
}
