use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

fn game_with(
    seed: u64,
    player_board: Vec<PlayedCard>,
    opponent_board: Vec<PlayedCard>,
) -> deckgym::Game<'static> {
    let mut game = get_initialized_game(seed);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;
    state.set_board(player_board, opponent_board);
    state.hands[1].clear();
    game.set_state(state);
    game
}

/// Guzzlord - Breakcore: "Flip a coin. If heads, discard your opponent's Active Pokémon."
/// Discarding is not a Knock Out, so no point is scored.
#[test]
fn test_breakcore_discards_opponent_active_on_heads_without_scoring() {
    let mut saw_heads = false;
    let mut saw_tails = false;

    for seed in 0..50 {
        let mut game = game_with(
            seed,
            vec![PlayedCard::from_id(CardId::B2109Guzzlord).with_energy(vec![
                EnergyType::Darkness,
                EnergyType::Darkness,
                EnergyType::Darkness,
                EnergyType::Colorless,
            ])],
            vec![
                PlayedCard::from_id(CardId::A1211Snorlax),
                PlayedCard::from_id(CardId::A1001Bulbasaur),
            ],
        );

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::B2109Guzzlord, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        if state.get_active(1).get_name() == "Snorlax" {
            saw_tails = true;
            assert!(state.discard_piles[1].is_empty());
        } else {
            saw_heads = true;
            assert_eq!(state.get_active(1).get_name(), "Bulbasaur");
            assert!(
                state.discard_piles[1]
                    .iter()
                    .any(|card| card.get_name() == "Snorlax"),
                "seed {seed}: discarded Snorlax should be in its owner's discard pile"
            );
            assert_eq!(
                state.points[0], 0,
                "seed {seed}: discarding is not a Knock Out, so no point is scored"
            );
        }
    }

    assert!(saw_heads && saw_tails, "both coin results should occur");
}

/// Fan Rotom - Spin Storm: "Flip a coin. If heads, put your opponent's Active Pokémon into
/// their hand."
#[test]
fn test_spin_storm_returns_opponent_active_to_hand_on_heads() {
    let mut saw_heads = false;
    let mut saw_tails = false;

    for seed in 0..50 {
        let mut game = game_with(
            seed,
            vec![PlayedCard::from_id(CardId::A2142FanRotom)
                .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless])],
            vec![
                PlayedCard::from_id(CardId::A1211Snorlax).with_energy(vec![EnergyType::Water]),
                PlayedCard::from_id(CardId::A1001Bulbasaur),
            ],
        );

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A2142FanRotom, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        if state.get_active(1).get_name() == "Snorlax" {
            saw_tails = true;
        } else {
            saw_heads = true;
            assert_eq!(state.get_active(1).get_name(), "Bulbasaur");
            assert!(
                state.hands[1]
                    .iter()
                    .any(|card| card.get_name() == "Snorlax"),
                "seed {seed}: Snorlax should be back in its owner's hand"
            );
            assert_eq!(
                state.discard_energies[1],
                vec![EnergyType::Water],
                "seed {seed}: attached Energy is discarded when the Pokemon returns to hand"
            );
        }
    }

    assert!(saw_heads && saw_tails, "both coin results should occur");
}

/// Chinchou - Luring Glow: "Flip a coin. If heads, switch in 1 of your opponent's Benched
/// Pokémon to the Active Spot."
#[test]
fn test_luring_glow_switches_in_a_benched_pokemon_on_heads() {
    let mut saw_heads = false;
    let mut saw_tails = false;

    for seed in 0..50 {
        let mut game = game_with(
            seed,
            vec![
                PlayedCard::from_id(CardId::PA095Chinchou).with_energy(vec![EnergyType::Lightning])
            ],
            vec![
                PlayedCard::from_id(CardId::A1211Snorlax),
                PlayedCard::from_id(CardId::A1001Bulbasaur),
            ],
        );

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::PA095Chinchou, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        match state.get_active(1).get_name().as_str() {
            "Snorlax" => saw_tails = true,
            "Bulbasaur" => {
                saw_heads = true;
                assert_eq!(
                    state.in_play_pokemon[1][1]
                        .as_ref()
                        .expect("Snorlax should be benched")
                        .get_name(),
                    "Snorlax"
                );
            }
            other => panic!("seed {seed}: unexpected active {other}"),
        }
    }

    assert!(saw_heads && saw_tails, "both coin results should occur");
}
