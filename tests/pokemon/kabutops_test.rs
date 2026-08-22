use deckgym::{
    actions::Action,
    card_ids::CardId::{self},
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_leech_life_heals_equals_to_damage_dealt() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.current_player = 0;
    state.turn_count = 1;

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1159Kabutops).with_energy(vec![EnergyType::Fighting])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
    );
    game.set_state(state);

    // Opponent attacks first: Bulbasaur damages Kabutos for 30 dmg
    let mut state = game.get_state_clone();
    state.current_player = 1;
    game.set_state(state);
    game.apply_action(&Action {
        actor: 1,
        action: attack_action(CardId::A1001Bulbasaur, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let hp_after_opponent_attacks = state.get_active(0).get_remaining_hp();
    assert!(hp_after_opponent_attacks < 140);

    // Own Kabutops attacks and heals by "Leech lilfe"
    let mut state = game.get_state_clone();
    state.current_player = 0;
    game.set_state(state);
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1159Kabutops, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();

    // Bulbasaur took 50 damage
    assert_eq!(state.get_active(1).get_remaining_hp(), 20);
    // Leech Life: heals equals 50 dmg
    assert_eq!(
        state.get_active(0).get_remaining_hp(),
        hp_after_opponent_attacks + 50
    );
}

#[test]
fn test_leech_life_heals_equals_to_damage_dealt_considers_reducing_damage_item() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.current_player = 0;
    state.turn_count = 1;

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1159Kabutops).with_energy(vec![EnergyType::Fighting])],
        vec![PlayedCard::from_id(CardId::B4197WailordEx)
            .with_energy(vec![
                EnergyType::Water,
                EnergyType::Water,
                EnergyType::Water,
                EnergyType::Water,
            ])
            .with_tool(get_card_by_enum(CardId::B1219HeavyHelmet))],
    );
    game.set_state(state);

    // Opponent attacks first: Wailord Ex damages Kabutops for 100 dmg
    let mut state = game.get_state_clone();
    state.current_player = 1;
    game.set_state(state);
    game.apply_action(&Action {
        actor: 1,
        action: attack_action(CardId::B4197WailordEx, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let hp_after_opponent_attacks = state.get_active(0).get_remaining_hp();
    assert!(hp_after_opponent_attacks < 120);

    // Own Kabutops attacks and heals by "Leech life" - 20 dmg (due Heavy Helmet effect)
    let mut state = game.get_state_clone();
    state.current_player = 0;
    game.set_state(state);
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1159Kabutops, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();

    // Wailord Ex took 30 damage
    assert_eq!(state.get_active(1).get_remaining_hp(), 220);
    // Leech Life: heals Kabutops for 30 dmg
    assert_eq!(
        state.get_active(0).get_remaining_hp(),
        hp_after_opponent_attacks + 30
    );
}

#[test]
fn test_leech_life_heals_equals_to_damage_dealt_with_weakness() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.current_player = 0;
    state.turn_count = 1;

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1159Kabutops).with_energy(vec![EnergyType::Fighting])],
        vec![
            PlayedCard::from_id(CardId::A4b376PikachuEx)
                .with_energy(vec![EnergyType::Lightning, EnergyType::Lightning]),
            PlayedCard::from_id(CardId::A1097Magnemite),
            PlayedCard::from_id(CardId::A1097Magnemite),
            PlayedCard::from_id(CardId::A1097Magnemite),
        ],
    );
    game.set_state(state);

    // Opponent attacks first: Pikachu Ex damages Kabutops for 90 dmg
    let mut state = game.get_state_clone();
    state.current_player = 1;
    game.set_state(state);
    game.apply_action(&Action {
        actor: 1,
        action: attack_action(CardId::A4b376PikachuEx, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let hp_after_opponent_attacks = state.get_active(0).get_remaining_hp();
    assert!(hp_after_opponent_attacks < 120);

    // Own Kabutops attacks and heals by "Leech life" + 20 dmg
    let mut state = game.get_state_clone();
    state.current_player = 0;
    game.set_state(state);
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1159Kabutops, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();

    // Pikachu Ex took 70 damage
    assert_eq!(state.get_active(1).get_remaining_hp(), 50);
    // Leech Life: heals Kabutops for 70 dmg
    assert_eq!(
        state.get_active(0).get_remaining_hp(),
        hp_after_opponent_attacks + 70
    );
}
