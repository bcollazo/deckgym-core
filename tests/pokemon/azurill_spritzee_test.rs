use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

fn damaged_bench_bulbasaur(damage: u32) -> PlayedCard {
    let bulbasaur = get_card_by_enum(CardId::A1001Bulbasaur);
    PlayedCard::new(bulbasaur, damage, 70, vec![], false, vec![])
}

#[test]
fn test_azurill_splash_heals_50_from_one_benched() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![PlayedCard::from_id(CardId::A4a063Azurill)],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    // One damaged benched Pokémon (40 remaining) -> the heal choice is forced
    state.in_play_pokemon[0][1] = Some(damaged_bench_bulbasaur(30));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A4a063Azurill, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    let benched = state.in_play_pokemon[0][1].as_ref().expect("benched");
    // 40 + 50 = 90, capped at 70 max HP
    assert_eq!(benched.get_remaining_hp(), 70);
}

#[test]
fn test_spritzee_fairy_wind_heals_20_from_one_pokemon() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B1a035Spritzee).with_energy(vec![EnergyType::Psychic])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    // One damaged benched Pokémon (40 remaining) -> the heal choice is forced
    state.in_play_pokemon[0][1] = Some(damaged_bench_bulbasaur(30));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B1a035Spritzee, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    let benched = state.in_play_pokemon[0][1].as_ref().expect("benched");
    // 40 + 20 = 60
    assert_eq!(benched.get_remaining_hp(), 60);
}
