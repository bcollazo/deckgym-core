use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

#[test]
fn test_audino_healing_light_heals_each_of_your_pokemon() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up board with damaged active Pokemon only
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B3140Audino)
                .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless])
                .with_damage(20),
        ],
        vec![PlayedCard::from_id(CardId::A1053Squirtle)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    game.set_state(state);

    // Verify initial state
    let initial_state = game.get_state_clone();
    let initial_audino_hp = initial_state.get_active(0).get_remaining_hp();
    let initial_opponent_hp = initial_state.get_active(1).get_remaining_hp();
    assert_eq!(initial_audino_hp, 60); // Audino has 80 HP, 20 damage = 60 remaining

    // Apply Healing Light attack (does 40 damage + heals 10 from each of your Pokemon)
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    // Opponent's active takes 40 damage
    let opponent_hp_after = state.get_active(1).get_remaining_hp();
    assert_eq!(opponent_hp_after, initial_opponent_hp - 40);

    // Your Audino should be healed 10 damage
    // Starting HP: 60, Healed 10 → 70
    let audino_hp_after = state.get_active(0).get_remaining_hp();
    assert_eq!(audino_hp_after, 70);
}

#[test]
fn test_mega_audino_ex_heartfelt_shine_heals_all_pokemon() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Set up board - just Mega Audino active to focus on healing the active Pokemon
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B3141MegaAudinoEx)
                .with_energy(vec![
                    EnergyType::Colorless,
                    EnergyType::Colorless,
                    EnergyType::Colorless,
                ])
                .with_damage(60),
        ],
        vec![PlayedCard::from_id(CardId::A1053Squirtle)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    game.set_state(state);

    // Apply Heartfelt Shine attack (does 90 damage + heals 30 from each of your Pokemon)
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    // Your Mega Audino (180 HP):  60 damage - 30 heal = 30 damage, remaining HP = 150
    assert_eq!(state.get_active(0).get_remaining_hp(), 150);
}
