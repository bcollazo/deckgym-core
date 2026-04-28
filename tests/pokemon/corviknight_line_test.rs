use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

fn attack_once(attacker: PlayedCard, defender: PlayedCard) -> deckgym::State {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;
    state.set_board(vec![attacker], vec![defender]);
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    game.get_state_clone()
}

#[test]
fn test_rookidee_pluck_discards_opponent_active_tool() {
    let state = attack_once(
        PlayedCard::from_id(CardId::B3145Rookidee).with_energy(vec![EnergyType::Colorless]),
        PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_tool(get_card_by_enum(CardId::A2147GiantCape)),
    );

    let defender = state.get_active(1);
    assert!(defender.attached_tool.is_none());
    assert_eq!(defender.get_remaining_hp(), 60);
    assert!(state.discard_piles[1].contains(&get_card_by_enum(CardId::A2147GiantCape)));
}

#[test]
fn test_corvisquire_joust_discards_opponent_active_tool() {
    let state = attack_once(
        PlayedCard::from_id(CardId::B3146Corvisquire)
            .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless]),
        PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_tool(get_card_by_enum(CardId::A2147GiantCape)),
    );

    let defender = state.get_active(1);
    assert!(defender.attached_tool.is_none());
    assert_eq!(defender.get_remaining_hp(), 40);
    assert!(state.discard_piles[1].contains(&get_card_by_enum(CardId::A2147GiantCape)));
}

#[test]
fn test_corviknight_ex_air_crash_discards_opponent_active_energy() {
    let state = attack_once(
        PlayedCard::from_id(CardId::B3124CorviknightEx).with_energy(vec![
            EnergyType::Metal,
            EnergyType::Metal,
            EnergyType::Metal,
        ]),
        PlayedCard::from_id(CardId::B3124CorviknightEx).with_energy(vec![EnergyType::Metal]),
    );

    let defender = state.get_active(1);
    assert!(defender.attached_energy.is_empty());
    assert_eq!(state.discard_energies[1], vec![EnergyType::Metal]);
}

#[test]
fn test_corviknight_iron_wings_discards_energy_and_reduces_next_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B1175Corviknight).with_energy(vec![
                EnergyType::Metal,
                EnergyType::Metal,
                EnergyType::Colorless,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::B3124CorviknightEx)],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let corviknight = state.get_active(0);
    assert_eq!(corviknight.attached_energy, vec![EnergyType::Colorless]);
    assert_eq!(
        state.discard_energies[0],
        vec![EnergyType::Metal, EnergyType::Metal]
    );

    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (1, 0),
            targets: vec![(30, 0, 0)],
            is_from_active_attack: true,
        },
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.get_active(0).get_remaining_hp(), 140);
}
