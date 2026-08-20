use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
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

#[test]
fn test_lilligant_toughness_aroma_stacks_when_second_lilligant_enters() {
    // Verifies two things:
    //  1. Bonus stacks: two Toughness Aroma sources → each Grass Pokémon gets +40 HP.
    //  2. Entering after: a Grass Pokémon placed AFTER Lilligant is already in play
    //     immediately receives the (now doubled) bonus.
    let mut game = get_initialized_game(0);
    game.play_until_stable();

    let mut state = game.get_state_clone();
    // Start with only B1018Lilligant (80 HP) in the active slot. Self-bonus gives it 100 HP.
    state.set_board(
        vec![PlayedCard::from_id(CardId::B1018Lilligant)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    state.current_player = 0;

    // Put B1329Lilligant (full-art reprint, identical ability) in hand so it can be placed.
    let b1329_card = get_card_by_enum(CardId::B1329Lilligant);
    state.hands[0].push(b1329_card.clone());
    game.set_state(state);

    // Sanity: before placing B1329, B1018 only gets +20 from its own ability.
    assert_eq!(
        game.get_state_clone().in_play_pokemon[0][0]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        100 // 80 + 20
    );

    // Place B1329Lilligant on bench slot 1. This triggers the catch-all bonus refresh.
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Place(b1329_card, 1),
        is_stack: false,
    });

    // Now two Toughness Aroma sources are in play (+20 each → +40 per Grass Pokémon).
    // Both Lilligants are Grass, so both get +40 HP: 80 + 40 = 120.
    let state = game.get_state_clone();
    assert_eq!(
        state.in_play_pokemon[0][0]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        120 // 80 + 40
    );
    assert_eq!(
        state.in_play_pokemon[0][1]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        120 // 80 + 40 — entered AFTER B1018, immediately gets doubled bonus
    );
}
