use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Minun - Buddy Spark: 30 damage, and with Plusle on the Bench also 10 damage to each of the
/// opponent's Benched Pokemon.
#[test]
fn test_buddy_spark_hits_the_opponents_bench_with_plusle_benched() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B2053Minun).with_energy(vec![EnergyType::Lightning]),
            PlayedCard::from_id(CardId::B2052Plusle),
        ],
        vec![
            PlayedCard::from_id(CardId::A1211Snorlax),
            PlayedCard::from_id(CardId::A1211Snorlax),
            PlayedCard::from_id(CardId::A1211Snorlax),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2053Minun, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 120);
    for bench_idx in [1, 2] {
        assert_eq!(
            state.in_play_pokemon[1][bench_idx]
                .as_ref()
                .expect("benched Snorlax")
                .get_remaining_hp(),
            140,
            "each benched Pokemon should take 10"
        );
    }
}

#[test]
fn test_buddy_spark_spares_the_bench_without_plusle() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B2053Minun).with_energy(vec![EnergyType::Lightning]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![
            PlayedCard::from_id(CardId::A1211Snorlax),
            PlayedCard::from_id(CardId::A1211Snorlax),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2053Minun, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 120);
    assert_eq!(
        state.in_play_pokemon[1][1]
            .as_ref()
            .expect("benched Snorlax")
            .get_remaining_hp(),
        150,
        "the bench should be untouched without Plusle"
    );
}
