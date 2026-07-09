use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::get_test_game_with_board,
};

/// Lunala ex's Psychic Connect: "Once during your turn, you may move all [P] Energy from 1 of your
/// Benched [P] Pokémon to your Active Pokémon." The Active may be any type, only [P] Energy moves,
/// and it can be used only once per turn.
#[test]
fn test_lunala_ex_psychic_connect_moves_all_psychic_energy_to_any_active() {
    // Active is a Grass Pokémon (not Psychic) to confirm the destination is not type-restricted.
    // Lunala ex is benched at idx 1; the benched Abra at idx 2 holds 2 [P] + 1 [C].
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A3087LunalaEx),
            PlayedCard::from_id(CardId::A1115Abra).with_energy(vec![
                EnergyType::Psychic,
                EnergyType::Psychic,
                EnergyType::Colorless,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::A1002Ivysaur)],
    );

    // Use Lunala ex's ability from the Bench.
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 1 },
        is_stack: false,
    });

    // The only offered move drains both [P] Energy from Abra (idx 2) into the Active (idx 0).
    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    let move_energy = actions
        .into_iter()
        .find(|a| {
            matches!(
                a.action,
                SimpleAction::MoveEnergy {
                    from_in_play_idx: 2,
                    to_in_play_idx: 0,
                    energy_type: EnergyType::Psychic,
                    amount: 2,
                }
            )
        })
        .expect("Psychic Connect should offer moving both [P] Energy from Abra to the Active");
    game.apply_action(&move_energy);

    let state = game.get_state_clone();
    // The Grass Active received both [P] Energy (destination is not type-restricted).
    let active = state.in_play_pokemon[0][0].as_ref().unwrap();
    assert_eq!(
        active
            .attached_energy
            .iter()
            .filter(|&&e| e == EnergyType::Psychic)
            .count(),
        2,
    );
    // Only the [P] Energy moved; Abra kept its [C] Energy.
    let abra = state.in_play_pokemon[0][2].as_ref().unwrap();
    assert_eq!(abra.attached_energy, vec![EnergyType::Colorless]);

    // Once per turn: Lunala ex's ability is no longer offered after use.
    let (_actor, actions) = state.generate_possible_actions();
    assert!(!actions
        .iter()
        .any(|a| matches!(a.action, SimpleAction::UseAbility { in_play_idx: 1 })));
}

/// Psychic Connect is unavailable when no benched [P] Pokémon has [P] Energy to move.
#[test]
fn test_lunala_ex_psychic_connect_unavailable_without_psychic_energy_source() {
    let game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A3087LunalaEx),
            PlayedCard::from_id(CardId::A1115Abra), // benched [P] Pokémon but no Energy attached
        ],
        vec![PlayedCard::from_id(CardId::A1002Ivysaur)],
    );

    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    assert!(!actions
        .iter()
        .any(|a| matches!(a.action, SimpleAction::UseAbility { in_play_idx: 0 })));
}
