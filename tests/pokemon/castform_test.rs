use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

#[test]
fn test_castform_blow_through_gets_stadium_damage_bonus() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B3133Castform).with_energy(vec![EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.active_stadium = Some(get_card_by_enum(CardId::B2155PeculiarPlaza));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let final_state = game.get_state_clone();
    assert_eq!(
        final_state.get_active(1).get_remaining_hp(),
        30,
        "Blow Through should deal 40 damage when a Stadium is in play"
    );
}

#[test]
fn test_castform_rainy_absorbing_heals_only_with_stadium() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B3040CastformRainyForm)
            .with_energy(vec![EnergyType::Water])
            .with_remaining_hp(30)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.active_stadium = Some(get_card_by_enum(CardId::B2155PeculiarPlaza));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let final_state = game.get_state_clone();
    assert_eq!(
        final_state.get_active(0).get_remaining_hp(),
        50,
        "Rainy Absorbing should heal 20 when a Stadium is in play"
    );
    assert_eq!(
        final_state.get_active(1).get_remaining_hp(),
        40,
        "Rainy Absorbing should still deal its base 30 damage"
    );
}

#[test]
fn test_castform_sunny_scorching_burns_only_with_stadium() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B3024CastformSunnyForm).with_energy(vec![EnergyType::Fire])
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.active_stadium = Some(get_card_by_enum(CardId::B2155PeculiarPlaza));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let final_state = game.get_state_clone();
    assert!(
        final_state.get_active(1).is_burned(),
        "Sunny Scorching should Burn the opponent's Active when a Stadium is in play"
    );
}

#[test]
fn test_castform_snowy_chilling_sleeps_only_with_stadium() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B3041CastformSnowyForm)
            .with_energy(vec![EnergyType::Water, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.active_stadium = Some(get_card_by_enum(CardId::B2155PeculiarPlaza));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let final_state = game.get_state_clone();
    assert!(
        final_state.get_active(1).is_asleep(),
        "Snowy Chilling should make the opponent's Active Asleep when a Stadium is in play"
    );
}
