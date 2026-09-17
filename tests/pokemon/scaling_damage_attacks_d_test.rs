use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Mewtwo (B4) - Psychic: 10 damage, +40 for each Energy on the Defending Pokémon.
#[test]
fn test_mewtwo_psychic_scales_with_defender_energy() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4070Mewtwo).with_energy(vec![
            EnergyType::Psychic,
            EnergyType::Psychic,
            EnergyType::Psychic,
        ])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)
            .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless])],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4070Mewtwo, 0),
        is_stack: false,
    });
    game.play_until_stable();

    // 10 + 2 * 40 = 90
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 60);
}

/// Tangrowth - Grass Knot: 10 damage, +40 for each Energy in the defender's Retreat Cost.
#[test]
fn test_tangrowth_grass_knot_scales_with_defender_retreat_cost() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A4005Tangrowth).with_energy(vec![
                EnergyType::Grass,
                EnergyType::Grass,
                EnergyType::Grass,
            ]),
        ],
        // Snorlax has a 4-Energy Retreat Cost.
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A4005Tangrowth, 0),
        is_stack: false,
    });
    game.play_until_stable();

    // 10 + 4 * 40 = 170, capped by Snorlax's 150 HP -> knocked out.
    assert!(game.get_state_clone().in_play_pokemon[1][0].is_none());
}

/// Team Rocket's Arbok - Shadow Seeker: 70 damage, +10 per Energy in the defender's Retreat Cost.
#[test]
fn test_team_rockets_arbok_shadow_seeker_scales_with_retreat_cost() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B4a039TeamRocketsArbok).with_energy(vec![
                EnergyType::Darkness,
                EnergyType::Darkness,
                EnergyType::Darkness,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4a039TeamRocketsArbok, 0),
        is_stack: false,
    });
    game.play_until_stable();

    // 70 + 4 * 10 = 110
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 40);
}

/// Team Rocket's Persian - Dangerous Rogue: 10 damage, +40 per opposing Benched Pokémon.
#[test]
fn test_team_rockets_persian_dangerous_rogue_scales_with_opponent_bench() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4a061TeamRocketsPersian)
            .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless])],
        vec![
            PlayedCard::from_id(CardId::A1211Snorlax),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4a061TeamRocketsPersian, 0),
        is_stack: false,
    });
    game.play_until_stable();

    // 10 + 2 * 40 = 90
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 60);
}

/// Leafeon - Leaf Blast: 10 damage, +20 for each [G] Energy attached to it.
#[test]
fn test_leafeon_leaf_blast_scales_with_own_grass_energy() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A3b002Leafeon).with_energy(vec![
            EnergyType::Grass,
            EnergyType::Grass,
            EnergyType::Colorless,
        ])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A3b002Leafeon, 0),
        is_stack: false,
    });
    game.play_until_stable();

    // 10 + 2 * 20 = 50 (the Colorless Energy does not count)
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 100);
}
