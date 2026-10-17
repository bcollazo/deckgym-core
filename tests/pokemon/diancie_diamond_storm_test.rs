use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Diancie - Diamond Storm: "Heal 20 damage from each of your [P] Pokémon." Only Psychic
/// Pokemon are healed; the Grass Bulbasaur on the Bench keeps its damage.
#[test]
fn test_diamond_storm_heals_only_psychic_pokemon() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B3067Diancie)
                .with_energy(vec![EnergyType::Psychic, EnergyType::Psychic])
                .with_remaining_hp(50),
            PlayedCard::from_id(CardId::A1a032MewEx).with_remaining_hp(100),
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_remaining_hp(30),
        ],
        vec![PlayedCard::from_id(CardId::B4197WailordEx)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3067Diancie, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 200);
    assert_eq!(
        state.get_active(0).get_remaining_hp(),
        70,
        "Diancie is [P] and heals 20"
    );
    assert_eq!(
        state.in_play_pokemon[0][1]
            .as_ref()
            .expect("Mew ex")
            .get_remaining_hp(),
        120,
        "the benched [P] Mew ex heals 20"
    );
    assert_eq!(
        state.in_play_pokemon[0][2]
            .as_ref()
            .expect("Bulbasaur")
            .get_remaining_hp(),
        30,
        "the [G] Bulbasaur is not healed"
    );
}
