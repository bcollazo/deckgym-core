use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_lilligant_toughness_aroma_boosts_own_grass_pokemon_only() {
    let mut game = get_initialized_game(0);
    game.play_until_stable();

    let mut state = game.get_state_clone();
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B1018Lilligant), // [G], 80 HP -> 100 (self included)
            PlayedCard::from_id(CardId::A1030Lilligant), // [G], 100 HP -> 120
            PlayedCard::from_id(CardId::A1033Charmander), // [R], 60 HP -> unaffected
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    state.current_player = 0;
    game.set_state(state);

    let state = game.get_state_clone();
    assert_eq!(
        state.in_play_pokemon[0][0]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        100
    );
    assert_eq!(
        state.in_play_pokemon[0][1]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        120
    );
    assert_eq!(
        state.in_play_pokemon[0][2]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        60
    );
}

#[test]
fn test_lilligant_toughness_aroma_bonus_disappears_once_lilligant_is_knocked_out() {
    let mut game = get_initialized_game(0);
    game.play_until_stable();

    let mut state = game.get_state_clone();
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B1018Lilligant).with_remaining_hp(30), // KOable by 1 Ember
            PlayedCard::from_id(CardId::A1001Bulbasaur),                       // [G], 70 HP
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander).with_energy(vec![EnergyType::Fire])],
    );
    state.current_player = 1;
    game.set_state(state);

    // Sanity check: benched Bulbasaur currently benefits from Lilligant's team-wide bonus.
    let state = game.get_state_clone();
    assert_eq!(
        state.in_play_pokemon[0][1]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        90
    );

    game.apply_action(&Action {
        actor: 1,
        action: attack_action(CardId::A1033Charmander, 0),
        is_stack: false,
    });

    // Lilligant was K.O.'d by the attack, so Bulbasaur should lose the team-wide
    // bonus and be back to its base 70 HP.
    let state = game.get_state_clone();
    assert!(state.in_play_pokemon[0][0].is_none());
    assert_eq!(
        state.in_play_pokemon[0][1]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        70
    );
}
