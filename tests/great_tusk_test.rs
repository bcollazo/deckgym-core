use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::get_test_game_with_board,
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
            PlayedCard::from_id(CardId::B1a065Furfrou),
        ],
        vec![PlayedCard::from_id(CardId::B1a065Furfrou)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });
}
