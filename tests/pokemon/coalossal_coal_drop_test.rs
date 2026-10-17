use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

/// Coalossal's Coal Drop deals 100 damage, or 100 + 50 = 150 damage when the
/// opponent's Active Pokémon is a [G] Pokémon.
#[test]
fn test_coal_drop_extra_damage_against_grass_defender() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Venusaur ex is a [G] Pokémon with 190 HP, weak to Fire (so no weakness bonus
    // against Coalossal, a [F] Pokémon).
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B1147Coalossal).with_energy(vec![
                EnergyType::Fighting,
                EnergyType::Fighting,
                EnergyType::Fighting,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::A1004VenusaurEx)],
    );
    state.current_player = 0;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B1147Coalossal, 0),
        is_stack: false,
    });

    // Venusaur ex 190 HP - 150 (100 base + 50 [G] bonus) = 40.
    let hp = game.get_state_clone().get_active(1).get_remaining_hp();
    assert_eq!(
        hp, 40,
        "Coal Drop should deal 150 damage to a [G] Active Pokémon"
    );
}

/// Against a non-[G] defender, Coal Drop deals only its base 100 damage.
#[test]
fn test_coal_drop_base_damage_against_non_grass_defender() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Dusknoir is a [P] Pokémon with 150 HP, weak to Darkness.
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B1147Coalossal).with_energy(vec![
                EnergyType::Fighting,
                EnergyType::Fighting,
                EnergyType::Fighting,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::B1105Dusknoir)],
    );
    state.current_player = 0;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B1147Coalossal, 0),
        is_stack: false,
    });

    // Dusknoir 150 HP - 100 (base damage only) = 50.
    let hp = game.get_state_clone().get_active(1).get_remaining_hp();
    assert_eq!(
        hp, 50,
        "Coal Drop should deal only base 100 damage to a non-[G] Active Pokémon"
    );
}
