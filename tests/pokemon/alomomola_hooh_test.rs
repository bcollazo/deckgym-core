use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

fn damaged_playable(card_id: CardId, damage: u32) -> PlayedCard {
    PlayedCard::new(get_card_by_enum(card_id), damage, 70, vec![], false, vec![])
}

#[test]
fn test_alomomola_heals_each_benched_pokemon() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B4045Alomomola)
            .with_energy(vec![EnergyType::Water, EnergyType::Water])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    // Two damaged benched Pokémon (40 remaining each)
    state.in_play_pokemon[0][1] = Some(damaged_playable(CardId::A1001Bulbasaur, 30));
    state.in_play_pokemon[0][2] = Some(damaged_playable(CardId::A1002Ivysaur, 30));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4045Alomomola, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    // Both healed by 10: 40 -> 50
    let bench1 = state.in_play_pokemon[0][1].as_ref().unwrap();
    let bench2 = state.in_play_pokemon[0][2].as_ref().unwrap();
    assert_eq!(bench1.get_remaining_hp(), 50);
    assert_eq!(bench2.get_remaining_hp(), 50);
}

#[test]
fn test_ho_oh_sacred_fire_heals_only_benched_basic() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B1032HoOh).with_energy(vec![
            EnergyType::Fire,
            EnergyType::Fire,
            EnergyType::Colorless,
            EnergyType::Colorless,
        ])],
        vec![PlayedCard::from_id(CardId::B4197WailordEx)],
    );
    // Basic Bulbasaur and evolved Ivysaur, both damaged (40 remaining)
    state.in_play_pokemon[0][1] = Some(damaged_playable(CardId::A1001Bulbasaur, 30));
    state.in_play_pokemon[0][2] = Some(damaged_playable(CardId::A1002Ivysaur, 30));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B1032HoOh, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    // Basic healed by 30: 40 -> 70; evolved NOT healed (stays 40)
    let basic = state.in_play_pokemon[0][1].as_ref().unwrap();
    let evolved = state.in_play_pokemon[0][2].as_ref().unwrap();
    assert_eq!(
        basic.get_remaining_hp(),
        70,
        "Basic benched should be healed"
    );
    assert_eq!(
        evolved.get_remaining_hp(),
        40,
        "Evolved benched should NOT be healed"
    );
}
