use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard, StatusCondition},
    test_support::{attack_action, get_test_game_with_board},
};

const SNORLAX_HP: u32 = 150;
const VENUSAUR_EX_HP: u32 = 190;

fn attack(game: &mut deckgym::Game<'static>, card_id: CardId) {
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(card_id, 0),
        is_stack: false,
    });
}

/// Heatmor's Roasting Heat does 30, or 30 + 60 = 90 if the Defending Pokémon is Burned.
#[test]
fn test_roasting_heat_extra_damage_when_defender_burned() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A4037Heatmor)
            .with_energy(vec![EnergyType::Fire, EnergyType::Fire])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    let mut state = game.get_state_clone();
    state.apply_status_condition(1, 0, StatusCondition::Burned);
    game.set_state(state);

    attack(&mut game, CardId::A4037Heatmor);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        SNORLAX_HP - 90
    );
}

#[test]
fn test_roasting_heat_base_damage_when_defender_not_burned() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A4037Heatmor)
            .with_energy(vec![EnergyType::Fire, EnergyType::Fire])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    attack(&mut game, CardId::A4037Heatmor);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        SNORLAX_HP - 30
    );
}

/// Rotom's Assault Laser checks the OPPONENT's Active for a Pokémon Tool (20, or 20 + 30 = 50).
/// Giant Cape also gives the holder +20 HP, so the expected remaining HP accounts for that.
#[test]
fn test_assault_laser_extra_damage_when_defender_has_tool() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A2062Rotom).with_energy(vec![EnergyType::Lightning])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)
            .with_tool(get_card_by_enum(CardId::A2147GiantCape))],
    );

    attack(&mut game, CardId::A2062Rotom);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        SNORLAX_HP + 20 - 50
    );
}

#[test]
fn test_assault_laser_base_damage_when_defender_has_no_tool() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A2062Rotom).with_energy(vec![EnergyType::Lightning])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    attack(&mut game, CardId::A2062Rotom);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        SNORLAX_HP - 20
    );
}

/// Stoutland's Dangerous Bite does 70, or 70 + 70 = 140 against a Basic Pokémon.
#[test]
fn test_dangerous_bite_extra_damage_against_basic() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B1203Stoutland).with_energy(vec![
                EnergyType::Colorless,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    attack(&mut game, CardId::B1203Stoutland);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        SNORLAX_HP - 140
    );
}

#[test]
fn test_dangerous_bite_base_damage_against_evolution() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B1203Stoutland).with_energy(vec![
                EnergyType::Colorless,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::A1004VenusaurEx)],
    );

    attack(&mut game, CardId::B1203Stoutland);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        VENUSAUR_EX_HP - 70
    );
}

/// Kangaskhan's Cross-Cut is the mirror case: 20, or 20 + 40 = 60 against an Evolution Pokémon.
#[test]
fn test_cross_cut_extra_damage_against_evolution() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A4133Kangaskhan).with_energy(vec![EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1004VenusaurEx)],
    );

    attack(&mut game, CardId::A4133Kangaskhan);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        VENUSAUR_EX_HP - 60
    );
}

#[test]
fn test_cross_cut_base_damage_against_basic() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A4133Kangaskhan).with_energy(vec![EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    attack(&mut game, CardId::A4133Kangaskhan);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        SNORLAX_HP - 20
    );
}

/// Silvally's Gold Breaker does 60, or 60 + 90 = 150 against a Pokémon ex.
#[test]
fn test_gold_breaker_extra_damage_against_ex() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4144Silvally).with_energy(vec![
            EnergyType::Colorless,
            EnergyType::Colorless,
            EnergyType::Colorless,
        ])],
        vec![PlayedCard::from_id(CardId::A1004VenusaurEx)],
    );

    attack(&mut game, CardId::B4144Silvally);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        VENUSAUR_EX_HP - 150
    );
}

/// Seviper's Fateful Fang does 40, or 40 + 40 = 80 against Zangoose specifically.
/// Zangoose wears a Giant Cape (+20 HP) so the boosted hit does not Knock it Out.
#[test]
fn test_fateful_fang_extra_damage_against_zangoose() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A4a048Seviper)
            .with_energy(vec![EnergyType::Darkness, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A4a065Zangoose)
            .with_tool(get_card_by_enum(CardId::A2147GiantCape))],
    );

    attack(&mut game, CardId::A4a048Seviper);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        80 + 20 - 80
    );
}

#[test]
fn test_fateful_fang_base_damage_against_other_pokemon() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A4a048Seviper)
            .with_energy(vec![EnergyType::Darkness, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    attack(&mut game, CardId::A4a048Seviper);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        SNORLAX_HP - 40
    );
}

/// Scovillain's Red-Hot Headbutt does 60, or 60 + 40 = 100 against a [G] OR [M] Pokémon.
#[test]
fn test_red_hot_headbutt_extra_damage_against_metal() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2a013Scovillain)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::B3118Bronzong)],
    );

    attack(&mut game, CardId::B2a013Scovillain);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        120 - 100
    );
}

#[test]
fn test_red_hot_headbutt_base_damage_against_other_type() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2a013Scovillain)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    attack(&mut game, CardId::B2a013Scovillain);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        SNORLAX_HP - 60
    );
}

/// Swalot's Swallow Up does 30, or 30 + 80 = 110 when the Defending Pokémon has strictly less
/// remaining HP than Swalot itself (120 HP).
#[test]
fn test_swallow_up_extra_damage_when_defender_has_less_hp() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4099Swalot)
            .with_energy(vec![EnergyType::Darkness, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax).with_remaining_hp(115)],
    );

    attack(&mut game, CardId::B4099Swalot);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        115 - 110
    );
}

#[test]
fn test_swallow_up_base_damage_when_defender_has_more_hp() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4099Swalot)
            .with_energy(vec![EnergyType::Darkness, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    attack(&mut game, CardId::B4099Swalot);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        SNORLAX_HP - 30
    );
}

/// Marowak's Punish does 50, or 50 + 70 = 120 when the Defending Pokémon has "Team Rocket" in
/// its name.
#[test]
fn test_punish_extra_damage_against_team_rocket_pokemon() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4a036Marowak)
            .with_energy(vec![EnergyType::Fighting, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::B4a050TeamRocketsTinkaton)],
    );

    attack(&mut game, CardId::B4a036Marowak);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        140 - 120
    );
}

#[test]
fn test_punish_base_damage_against_non_team_rocket_pokemon() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4a036Marowak)
            .with_energy(vec![EnergyType::Fighting, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::B3118Bronzong)],
    );

    attack(&mut game, CardId::B4a036Marowak);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        120 - 50
    );
}
