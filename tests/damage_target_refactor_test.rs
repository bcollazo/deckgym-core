use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::get_test_game_with_board,
};

/// Palkia ex's "Dimensional Storm" deals 150 damage to the opponent's Active
/// Pokémon and also 20 damage to each of the opponent's Benched Pokémon
/// (via the `also_bench_damage`/`palkia_dimensional_storm` codepath that was
/// refactored to use `DamageTarget`). It also discards 3 [W] Energy from
/// Palkia itself.
#[test]
fn palkia_ex_dimensional_storm_damages_active_and_opponent_bench() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A2049PalkiaEx).with_energy(vec![
            EnergyType::Water,
            EnergyType::Water,
            EnergyType::Water,
            EnergyType::Colorless,
        ])],
        vec![
            PlayedCard::from_id(CardId::A1036CharizardEx), // 180hp, weak to Water
            PlayedCard::from_id(CardId::A1001Bulbasaur),   // 70hp
            PlayedCard::from_id(CardId::A1001Bulbasaur),   // 70hp
        ],
    );

    let active_hp_before = game.get_state_clone().get_active(1).get_remaining_hp();
    let bench1_hp_before = game.get_state_clone().in_play_pokemon[1][1]
        .as_ref()
        .unwrap()
        .get_remaining_hp();
    let bench2_hp_before = game.get_state_clone().in_play_pokemon[1][2]
        .as_ref()
        .unwrap()
        .get_remaining_hp();

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(1), // Dimensional Storm
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        active_hp_before - 150 - 20, // 150 fixed damage + 20 weakness (Charizard ex is weak to Water)
        "Dimensional Storm should deal 150 (+20 weakness) damage to the opponent's Active Pokémon"
    );
    assert_eq!(
        state.in_play_pokemon[1][1]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        bench1_hp_before - 20,
        "Dimensional Storm should deal 20 damage to the opponent's first Benched Pokémon"
    );
    assert_eq!(
        state.in_play_pokemon[1][2]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        bench2_hp_before - 20,
        "Dimensional Storm should deal 20 damage to the opponent's second Benched Pokémon"
    );
    assert_eq!(
        state.in_play_pokemon[0][0]
            .as_ref()
            .unwrap()
            .attached_energy,
        vec![EnergyType::Colorless],
        "Dimensional Storm should discard 3 [W] Energy from Palkia ex"
    );
}

/// Spiritomb's "Swirling Disaster" deals 10 damage to each of the opponent's
/// Pokémon (Active and Benched), exercising the `damage_all_opponent_pokemon`
/// codepath that was refactored to use `DamageTarget`.
#[test]
fn spiritomb_swirling_disaster_damages_all_opponent_pokemon() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A2104Spiritomb).with_energy(vec![EnergyType::Colorless])],
        vec![
            PlayedCard::from_id(CardId::A1036CharizardEx), // 180hp, not weak to Darkness
            PlayedCard::from_id(CardId::A1001Bulbasaur),   // 70hp
            PlayedCard::from_id(CardId::A1001Bulbasaur),   // 70hp
        ],
    );

    let active_hp_before = game.get_state_clone().get_active(1).get_remaining_hp();
    let bench1_hp_before = game.get_state_clone().in_play_pokemon[1][1]
        .as_ref()
        .unwrap()
        .get_remaining_hp();
    let bench2_hp_before = game.get_state_clone().in_play_pokemon[1][2]
        .as_ref()
        .unwrap()
        .get_remaining_hp();

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0), // Swirling Disaster
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        active_hp_before - 10,
        "Swirling Disaster should deal 10 damage to the opponent's Active Pokémon"
    );
    assert_eq!(
        state.in_play_pokemon[1][1]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        bench1_hp_before - 10,
        "Swirling Disaster should deal 10 damage to the opponent's first Benched Pokémon"
    );
    assert_eq!(
        state.in_play_pokemon[1][2]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        bench2_hp_before - 10,
        "Swirling Disaster should deal 10 damage to the opponent's second Benched Pokémon"
    );
}
