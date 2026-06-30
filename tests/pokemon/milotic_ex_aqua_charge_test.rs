use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::get_test_game_with_board,
};

/// Aqua Charge: "Once during your turn, you may take a [W] Energy from your Energy Zone and
/// attach it to this Pokémon."
#[test]
fn test_milotic_ex_aqua_charge_attaches_water_energy() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B3b015MiloticEx)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    // Milotic ex starts with no energy attached.
    assert!(game.get_state_clone().in_play_pokemon[0][0]
        .as_ref()
        .unwrap()
        .attached_energy
        .is_empty());

    let ability_action = Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    };
    game.apply_action(&ability_action);

    let milotic = game.get_state_clone().in_play_pokemon[0][0]
        .as_ref()
        .unwrap()
        .clone();
    assert_eq!(milotic.attached_energy, vec![EnergyType::Water]);

    // The ability is "once during your turn", so it should no longer be available.
    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    assert!(!actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::UseAbility { in_play_idx: 0 })));
}

/// Aqua Charge can be used from the Bench as well as the Active Spot.
#[test]
fn test_milotic_ex_aqua_charge_available_from_bench() {
    let game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::B3b015MiloticEx),
        ],
        vec![PlayedCard::from_id(CardId::A1002Ivysaur)],
    );

    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    assert!(actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::UseAbility { in_play_idx: 1 })));
}
