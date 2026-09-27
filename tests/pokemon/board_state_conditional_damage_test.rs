use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard, StatusCondition},
    test_support::{attack_action, get_test_game_with_board},
};

const SNORLAX_HP: u32 = 150;

fn attack(game: &mut deckgym::Game<'static>, card_id: CardId) {
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(card_id, 0),
        is_stack: false,
    });
}

/// Grafaiai's Colorful Attack counts distinct Energy types across ALL of your Pokémon in play,
/// not just the attacker: 30, or 30 + 60 = 90 with 3 or more different types attached.
#[test]
fn test_colorful_attack_counts_energy_across_the_whole_board() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B2a070Grafaiai).with_energy(vec![EnergyType::Darkness]),
            PlayedCard::from_id(CardId::A1033Charmander).with_energy(vec![EnergyType::Fire]),
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_energy(vec![EnergyType::Water]),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    attack(&mut game, CardId::B2a070Grafaiai);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        SNORLAX_HP - 90
    );
}

#[test]
fn test_colorful_attack_base_damage_with_only_two_energy_types() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B2a070Grafaiai).with_energy(vec![EnergyType::Darkness]),
            PlayedCard::from_id(CardId::A1033Charmander).with_energy(vec![EnergyType::Fire]),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    attack(&mut game, CardId::B2a070Grafaiai);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        SNORLAX_HP - 30
    );
}

/// Bronzong's Psychic Resonance looks at the opponent's whole board, Bench included:
/// 50, or 50 + 50 = 100 if the opponent has any [P] Pokémon in play.
#[test]
fn test_psychic_resonance_extra_damage_for_benched_psychic_pokemon() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B3118Bronzong)
            .with_energy(vec![EnergyType::Metal, EnergyType::Colorless])],
        vec![
            PlayedCard::from_id(CardId::A1211Snorlax),
            PlayedCard::from_id(CardId::A1129MewtwoEx),
        ],
    );

    attack(&mut game, CardId::B3118Bronzong);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        SNORLAX_HP - 100
    );
}

#[test]
fn test_psychic_resonance_base_damage_without_psychic_pokemon() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B3118Bronzong)
            .with_energy(vec![EnergyType::Metal, EnergyType::Colorless])],
        vec![
            PlayedCard::from_id(CardId::A1211Snorlax),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
    );

    attack(&mut game, CardId::B3118Bronzong);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        SNORLAX_HP - 50
    );
}

/// Grumpig's Swaying Dance does 40, or 40 + 40 = 80 on an exactly-even opponent hand size.
#[test]
fn test_swaying_dance_extra_damage_on_even_opponent_hand() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2b032Grumpig).with_energy(vec![EnergyType::Psychic])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    let mut state = game.get_state_clone();
    state.hands[1].truncate(4);
    game.set_state(state);

    attack(&mut game, CardId::B2b032Grumpig);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        SNORLAX_HP - 80
    );
}

#[test]
fn test_swaying_dance_base_damage_on_odd_opponent_hand() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2b032Grumpig).with_energy(vec![EnergyType::Psychic])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    let mut state = game.get_state_clone();
    state.hands[1].truncate(3);
    game.set_state(state);

    attack(&mut game, CardId::B2b032Grumpig);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        SNORLAX_HP - 40
    );
}

/// Buzzwole's Ground Beat does 40, or 40 + 40 = 80 when the opponent has exactly 1 point.
#[test]
fn test_ground_beat_extra_damage_when_opponent_has_one_point() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2014Buzzwole)
            .with_energy(vec![EnergyType::Grass, EnergyType::Grass])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    let mut state = game.get_state_clone();
    state.points[1] = 1;
    game.set_state(state);

    attack(&mut game, CardId::B2014Buzzwole);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        SNORLAX_HP - 80
    );
}

#[test]
fn test_ground_beat_base_damage_when_opponent_has_two_points() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2014Buzzwole)
            .with_energy(vec![EnergyType::Grass, EnergyType::Grass])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    let mut state = game.get_state_clone();
    state.points[1] = 2;
    game.set_state(state);

    attack(&mut game, CardId::B2014Buzzwole);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        SNORLAX_HP - 40
    );
}

/// Team Rocket's Muk's Poison Absorption always does 80, and additionally heals 60 from itself
/// when the Defending Pokémon is Poisoned.
#[test]
fn test_poison_absorption_heals_when_defender_poisoned() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4a041TeamRocketsMuk)
            .with_energy(vec![
                EnergyType::Darkness,
                EnergyType::Darkness,
                EnergyType::Colorless,
            ])
            .with_remaining_hp(40)],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    let mut state = game.get_state_clone();
    state.apply_status_condition(1, 0, StatusCondition::Poisoned);
    game.set_state(state);

    attack(&mut game, CardId::B4a041TeamRocketsMuk);

    let state = game.get_state_clone();
    assert_eq!(state.get_active(0).get_remaining_hp(), 100);
    assert_eq!(state.get_active(1).get_remaining_hp(), SNORLAX_HP - 80);
}

#[test]
fn test_poison_absorption_does_not_heal_when_defender_not_poisoned() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4a041TeamRocketsMuk)
            .with_energy(vec![
                EnergyType::Darkness,
                EnergyType::Darkness,
                EnergyType::Colorless,
            ])
            .with_remaining_hp(40)],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    attack(&mut game, CardId::B4a041TeamRocketsMuk);

    let state = game.get_state_clone();
    assert_eq!(state.get_active(0).get_remaining_hp(), 40);
    assert_eq!(state.get_active(1).get_remaining_hp(), SNORLAX_HP - 80);
}
