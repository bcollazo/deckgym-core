use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_evil_admonition_base_damage_when_no_opponent_abilities() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B1149Honchkrow)
            .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless])],
        // Both opponent Pokémon lack an Ability.
        vec![
            PlayedCard::from_id(CardId::A1211Snorlax),
            PlayedCard::from_id(CardId::A1211Snorlax),
        ],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B1149Honchkrow, 0),
        is_stack: false,
    });
    game.play_until_stable();

    // Evil Admonition: 40 base + 0 extra (no opponent Pokémon with an Ability).
    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        150 - 40
    );
}

#[test]
fn test_evil_admonition_extra_damage_per_opponent_ability() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B1149Honchkrow)
            .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless])],
        // Active Snorlax has no Ability, but two benched Weezing each have one.
        vec![
            PlayedCard::from_id(CardId::A1211Snorlax),
            PlayedCard::from_id(CardId::A1177Weezing),
            PlayedCard::from_id(CardId::A1177Weezing),
        ],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B1149Honchkrow, 0),
        is_stack: false,
    });
    game.play_until_stable();

    // Evil Admonition: 40 base + 40 * 2 (two opponent Pokémon with an Ability) = 120.
    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        150 - 120
    );
}
