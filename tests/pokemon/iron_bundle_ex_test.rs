use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

fn bulbasaur_with_hp(hp: u32) -> PlayedCard {
    PlayedCard::new(
        get_card_by_enum(CardId::A1001Bulbasaur),
        0,
        hp,
        vec![],
        false,
        vec![],
    )
}

/// Iron Bundle's first Cold Start deals 80 damage (60 + 20) and Paralyzes.
#[test]
fn test_iron_bundle_cold_start_first_attack_extra_damage_and_paralysis() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B3a013IronBundleEx).with_energy(vec![
                EnergyType::Water,
                EnergyType::Water,
                EnergyType::Colorless,
            ]),
        ],
        vec![bulbasaur_with_hp(200)],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3a013IronBundleEx, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    let opponent_hp = state.get_active(1).get_remaining_hp();
    assert_eq!(
        opponent_hp, 120,
        "Cold Start first attack should deal 80 damage (200 - 80 = 120)"
    );
    assert!(
        state.get_active(1).is_paralyzed(),
        "Cold Start first attack should Paralyze the opponent"
    );
}

/// Iron Bundle's second Cold Start (not first attack) deals 60 damage and no Paralysis.
#[test]
fn test_iron_bundle_cold_start_second_attack_base_damage_only() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;

    let mut iron_bundle = PlayedCard::from_id(CardId::B3a013IronBundleEx).with_energy(vec![
        EnergyType::Water,
        EnergyType::Water,
        EnergyType::Colorless,
    ]);
    // Simulate this Pokémon has already attacked since coming into play
    iron_bundle.has_attacked_since_play = true;

    state.set_board(vec![iron_bundle], vec![bulbasaur_with_hp(200)]);
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3a013IronBundleEx, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    let opponent_hp = state.get_active(1).get_remaining_hp();
    assert_eq!(
        opponent_hp, 140,
        "Subsequent Cold Start should deal only 60 damage (200 - 60 = 140)"
    );
    assert!(
        !state.get_active(1).is_paralyzed(),
        "Subsequent Cold Start should NOT Paralyze the opponent"
    );
}
