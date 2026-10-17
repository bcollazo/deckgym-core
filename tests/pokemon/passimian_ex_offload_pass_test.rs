use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game_with_board,
};

/// Passimian ex's Offload Pass: when it is Knocked Out in the Active Spot by an opponent's attack,
/// all of its [F] Energy moves to 1 of your Benched Pokémon (your choice); other Energy is lost.
#[test]
fn test_passimian_ex_offload_pass_moves_fighting_energy_to_chosen_bench_on_ko() {
    // Player 0's Active Passimian ex is at 30 HP with 2 [F] + 1 [C]; it has two Benched Pokémon.
    // Player 1's Bulbasaur (current player) KOs it with Vine Whip (40).
    let mut game = get_initialized_game_with_board(
        0,
        1, // current player = the attacker
        3,
        vec![
            PlayedCard::from_id(CardId::A3104PassimianEx)
                .with_energy(vec![
                    EnergyType::Fighting,
                    EnergyType::Fighting,
                    EnergyType::Colorless,
                ])
                .with_remaining_hp(30),
            PlayedCard::from_id(CardId::A1115Abra), // Bench idx 1 (Psychic — any type is valid)
            PlayedCard::from_id(CardId::A1143Machop), // Bench idx 2
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Grass])],
    );

    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::Attack(deckgym::test_support::nth_attack(CardId::A1001Bulbasaur, 0)),
        is_stack: false,
    });

    // Passimian ex was Knocked Out and discarded.
    let state = game.get_state_clone();
    assert!(state.in_play_pokemon[0][0].is_none());

    // Offload Pass offers Player 0 a choice of which Benched Pokémon receives all 2 [F] Energy.
    let (actor, actions) = state.generate_possible_actions();
    assert_eq!(actor, 0);
    let to_abra = SimpleAction::Attach {
        attachments: vec![(2, EnergyType::Fighting, 1)],
        is_turn_energy: false,
    };
    let to_machop = SimpleAction::Attach {
        attachments: vec![(2, EnergyType::Fighting, 2)],
        is_turn_energy: false,
    };
    assert!(actions.iter().any(|a| a.action == to_machop));
    let choice = actions
        .into_iter()
        .find(|a| a.action == to_abra)
        .expect("Offload Pass should let Player 0 send the [F] Energy to the benched Abra");
    game.apply_action(&choice);

    // Abra received exactly the 2 [F] Energy; the [C] Energy did not move (it was discarded).
    let state = game.get_state_clone();
    let abra = state.in_play_pokemon[0][1].as_ref().unwrap();
    assert_eq!(
        abra.attached_energy,
        vec![EnergyType::Fighting, EnergyType::Fighting]
    );
    assert!(!abra.attached_energy.contains(&EnergyType::Colorless));
}

/// Offload Pass is passive — it is never offered as a `UseAbility` action.
#[test]
fn test_passimian_ex_offload_pass_is_passive() {
    let game = get_initialized_game_with_board(
        0,
        0,
        3,
        vec![
            PlayedCard::from_id(CardId::A3104PassimianEx)
                .with_energy(vec![EnergyType::Fighting, EnergyType::Fighting]),
            PlayedCard::from_id(CardId::A1143Machop),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    assert!(!actions
        .iter()
        .any(|a| matches!(a.action, SimpleAction::UseAbility { in_play_idx: 0 })));
}
