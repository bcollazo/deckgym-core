use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

const VENUSAUR_EX_MAX_HP: u32 = 190;
const BULBASAUR_MAX_HP: u32 = 70;

/// Toxtricity ex's Damaging Spark: 90 damage to the Defending Pokémon, plus 30 damage to EACH of
/// the opponent's Benched Pokémon that already has damage on it. Undamaged Benched Pokémon are
/// untouched.
#[test]
fn test_toxtricity_ex_damaging_spark_hits_only_damaged_bench() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B4a106ToxtricityEx).with_energy(vec![
                EnergyType::Lightning,
                EnergyType::Lightning,
                EnergyType::Colorless,
            ]),
        ],
        vec![
            PlayedCard::from_id(CardId::A1004VenusaurEx),
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_remaining_hp(BULBASAUR_MAX_HP - 10),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4a106ToxtricityEx, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        VENUSAUR_EX_MAX_HP - 90
    );
    assert_eq!(
        state.in_play_pokemon[1][1]
            .as_ref()
            .expect("Damaged benched Bulbasaur should still be in play")
            .get_remaining_hp(),
        BULBASAUR_MAX_HP - 10 - 30
    );
    assert_eq!(
        state.in_play_pokemon[1][2]
            .as_ref()
            .expect("Undamaged benched Bulbasaur should still be in play")
            .get_remaining_hp(),
        BULBASAUR_MAX_HP
    );
}
