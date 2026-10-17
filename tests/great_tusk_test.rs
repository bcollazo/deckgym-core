use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Repro: "Receiving Pokemon should be there when modifying damage"
/// (src/hooks/core.rs:861, damage_effect_mutation -> handle_damage_only -> modify_damage).
///
/// Great Tusk's "Shaking Stomp" also deals 20 damage to each of the
/// *attacker's own* Benched Pokémon. `damage_effect_mutation` hardcodes the
/// target player as the opponent, so when the attacker has a Benched
/// Pokémon at an index where the opponent has none, it panics.
#[test]
fn great_tusk_shaking_stomp_self_bench_damage_does_not_panic() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B3a034GreatTusk)
                .with_energy(vec![EnergyType::Fighting, EnergyType::Fighting]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::B3a036KoraidonEx)],
    );

    let bench_hp_before = game.get_state_clone().in_play_pokemon[0][1]
        .as_ref()
        .unwrap()
        .get_remaining_hp();
    let opponent_active_hp_before = game.get_state_clone().get_active(1).get_remaining_hp();

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3a034GreatTusk, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.in_play_pokemon[0][1]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        bench_hp_before - 20,
        "Shaking Stomp should deal 20 damage to the attacker's own Benched Pokémon"
    );
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        opponent_active_hp_before - 80,
        "Shaking Stomp should deal 80 damage to the opponent's Active Pokémon"
    );
}

/// Furfrou's "Fur Coat" ability ("This Pokémon takes -20 damage from attacks.")
/// should also absorb the 20 self-bench damage from Great Tusk's Shaking Stomp,
/// resulting in 0 damage taken.
#[test]
fn great_tusk_shaking_stomp_self_bench_damage_absorbed_by_fur_coat() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B3a034GreatTusk)
                .with_energy(vec![EnergyType::Fighting, EnergyType::Fighting]),
            PlayedCard::from_id(CardId::B1a065Furfrou),
        ],
        vec![PlayedCard::from_id(CardId::B3a036KoraidonEx)],
    );

    let bench_hp_before = game.get_state_clone().in_play_pokemon[0][1]
        .as_ref()
        .unwrap()
        .get_remaining_hp();

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3a034GreatTusk, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.in_play_pokemon[0][1]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        bench_hp_before,
        "Fur Coat should absorb the 20 self-bench damage from Shaking Stomp, resulting in 0 damage taken"
    );
}
