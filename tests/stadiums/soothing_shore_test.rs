use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard, TrainerCard},
    test_support::get_initialized_game,
};

fn trainer_from_id(card_id: CardId) -> TrainerCard {
    match get_card_by_enum(card_id) {
        Card::Trainer(trainer_card) => trainer_card,
        _ => panic!("Expected trainer card"),
    }
}

fn end_turn(game: &mut deckgym::Game<'static>, actor: usize) {
    game.apply_action(&Action {
        actor,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
}

#[test]
fn test_soothing_shore_heals_only_water_energy_pokemon_of_player_ending_turn() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            // Active has [W] Energy attached -> should heal 20.
            PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_energy(vec![EnergyType::Water])
                .with_remaining_hp(30),
            // Benched, but no [W] Energy -> should not heal.
            PlayedCard::from_id(CardId::A1033Charmander)
                .with_energy(vec![EnergyType::Fire])
                .with_remaining_hp(30),
        ],
        vec![
            // Opponent has [W] Energy too, but it is not their turn ending -> no heal.
            PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_energy(vec![EnergyType::Water])
                .with_remaining_hp(30),
        ],
    );
    state.current_player = 0;
    state.turn_count = 3;
    state.hands[0] = vec![get_card_by_enum(CardId::B4154SoothingShore)];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: trainer_from_id(CardId::B4154SoothingShore),
        },
        is_stack: false,
    });
    assert!(
        game.get_state_clone().active_stadium.is_some(),
        "Soothing Shore should be playable and become the active stadium"
    );

    end_turn(&mut game, 0);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_remaining_hp(0, 0),
        50,
        "Active with [W] Energy should heal 20 at the end of its owner's turn"
    );
    assert_eq!(
        state.get_remaining_hp(0, 1),
        30,
        "Benched Pokemon without [W] Energy should not heal"
    );
    assert_eq!(
        state.get_remaining_hp(1, 0),
        30,
        "Opponent's Pokemon should not heal at the end of the other player's turn"
    );
}

#[test]
fn test_soothing_shore_heals_the_opponent_when_their_turn_ends() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Water])
            .with_remaining_hp(30)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Water])
            .with_remaining_hp(30)],
    );
    state.current_player = 1;
    state.turn_count = 4;
    state.active_stadium = Some(get_card_by_enum(CardId::B4154SoothingShore));
    game.set_state(state);

    end_turn(&mut game, 1);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_remaining_hp(1, 0),
        50,
        "Stadium effects apply to both players: player 1 heals when player 1's turn ends"
    );
    assert_eq!(
        state.get_remaining_hp(0, 0),
        30,
        "Player 0 should not heal at the end of player 1's turn"
    );
}
