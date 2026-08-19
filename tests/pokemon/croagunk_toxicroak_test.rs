use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_croagunk_flips_one_coin_per_pokemon_in_play() {
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![
                PlayedCard::from_id(CardId::A2107Croagunk)
                    .with_energy(vec![EnergyType::Darkness, EnergyType::Darkness]),
                PlayedCard::from_id(CardId::A1001Bulbasaur),
                PlayedCard::from_id(CardId::A1002Ivysaur),
            ],
            vec![PlayedCard::from_id(CardId::A1211Snorlax)],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A2107Croagunk, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        let remaining = state.get_active(1).get_remaining_hp();
        // 3 Pokémon in play -> 3 coins -> damage {0, 20, 40, 60}
        assert!(
            [150, 130, 110, 90].contains(&remaining),
            "seed {seed}: 3 coins of 20, got remaining {remaining}"
        );
    }
}

#[test]
fn test_toxicroak_flips_one_coin_per_pokemon_in_play() {
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![
                PlayedCard::from_id(CardId::A2108Toxicroak)
                    .with_energy(vec![EnergyType::Darkness, EnergyType::Darkness]),
                PlayedCard::from_id(CardId::A1001Bulbasaur),
            ],
            vec![PlayedCard::from_id(CardId::A1211Snorlax)],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A2108Toxicroak, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        let remaining = state.get_active(1).get_remaining_hp();
        // 2 Pokémon in play -> 2 coins -> damage {0, 40, 80}
        assert!(
            [150, 110, 70].contains(&remaining),
            "seed {seed}: 2 coins of 40, got remaining {remaining}"
        );
    }
}
