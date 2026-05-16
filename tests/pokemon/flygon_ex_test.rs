use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

#[test]
fn test_flygon_ex_dragon_pulse_discards_top_card_of_own_deck() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::B3126FlygonEx).with_energy(vec![
            EnergyType::Grass,
            EnergyType::Fighting,
            EnergyType::Colorless,
        ])],
        vec![PlayedCard::from_id(CardId::A1053Squirtle)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    game.set_state(state);

    let deck_size_before = game.get_state_clone().decks[0].cards.len();
    let discard_size_before = game.get_state_clone().discard_piles[0].len();

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.decks[0].cards.len(),
        deck_size_before - 1,
        "Dragon Pulse should discard the top card of Flygon ex's deck"
    );
    assert_eq!(
        state.discard_piles[0].len(),
        discard_size_before + 1,
        "Dragon Pulse should put the discarded card into Flygon ex's discard pile"
    );
}

#[test]
fn test_flygon_ex_sand_slammer_damages_all_opponent_pokemon_on_checkup() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Player 0: Squirtle (active), Bulbasaur (bench)
    // Player 1: Flygon ex (active, with Sand Slammer), Bulbasaur (bench)
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1053Squirtle),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![
            PlayedCard::from_id(CardId::B3126FlygonEx),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );
    state.current_player = 0;
    state.turn_count = 3;
    game.set_state(state);

    let squirtle_hp = game.get_state_clone().get_remaining_hp(0, 0);
    let bulbasaur_hp = game.get_state_clone().get_remaining_hp(0, 1);

    // End player 0's turn — checkup fires, Sand Slammer deals 10 to all player 0's Pokémon
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_remaining_hp(0, 0),
        squirtle_hp - 10,
        "Sand Slammer should deal 10 damage to opponent's active Pokémon"
    );
    assert_eq!(
        state.get_remaining_hp(0, 1),
        bulbasaur_hp - 10,
        "Sand Slammer should deal 10 damage to opponent's benched Pokémon"
    );
}
