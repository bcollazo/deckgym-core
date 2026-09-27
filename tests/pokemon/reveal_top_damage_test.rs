use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

const VENUSAUR_EX_HP: u32 = 190;

/// Golurk's Heavy Rocket does 60 damage for each of the top 3 cards of your deck that is a
/// Pokémon with a Retreat Cost of 3 or more. Snorlax retreats for 4, Bulbasaur for 1.
#[test]
fn test_heavy_rocket_damage_per_heavy_pokemon_revealed() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B1136Golurk)
            .with_energy(vec![EnergyType::Fighting, EnergyType::Fighting])],
        vec![PlayedCard::from_id(CardId::A1004VenusaurEx)],
    );
    let mut state = game.get_state_clone();
    state.decks[0].cards = vec![
        get_card_by_enum(CardId::A1211Snorlax),
        get_card_by_enum(CardId::A1211Snorlax),
        get_card_by_enum(CardId::A1001Bulbasaur),
        get_card_by_enum(CardId::A1211Snorlax),
    ];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B1136Golurk, 0),
        is_stack: false,
    });

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        VENUSAUR_EX_HP - 120
    );
}

/// No matching cards among the top 3 means no damage at all.
#[test]
fn test_heavy_rocket_does_nothing_without_heavy_pokemon() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B1136Golurk)
            .with_energy(vec![EnergyType::Fighting, EnergyType::Fighting])],
        vec![PlayedCard::from_id(CardId::A1004VenusaurEx)],
    );
    let mut state = game.get_state_clone();
    state.decks[0].cards = vec![
        get_card_by_enum(CardId::A1001Bulbasaur),
        get_card_by_enum(CardId::A1001Bulbasaur),
        get_card_by_enum(CardId::A1001Bulbasaur),
    ];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B1136Golurk, 0),
        is_stack: false,
    });

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        VENUSAUR_EX_HP
    );
}

/// Team Rocket's Wobbuffet's Rocket Frenzy does 30 damage for each "Team Rocket" Pokémon among
/// the top 6 cards of your deck.
#[test]
fn test_rocket_frenzy_damage_per_team_rocket_pokemon_revealed() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::PB091TeamRocketsWobbuffet)
            .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1004VenusaurEx)],
    );
    let mut state = game.get_state_clone();
    state.decks[0].cards = vec![
        get_card_by_enum(CardId::B4a050TeamRocketsTinkaton),
        get_card_by_enum(CardId::A1001Bulbasaur),
        get_card_by_enum(CardId::B4a050TeamRocketsTinkaton),
        get_card_by_enum(CardId::A1001Bulbasaur),
        get_card_by_enum(CardId::B4a050TeamRocketsTinkaton),
        get_card_by_enum(CardId::A1001Bulbasaur),
        get_card_by_enum(CardId::B4a050TeamRocketsTinkaton),
    ];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::PB091TeamRocketsWobbuffet, 0),
        is_stack: false,
    });

    // Only the top 6 are revealed, so the fourth Tinkaton does not count.
    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        VENUSAUR_EX_HP - 90
    );
}
