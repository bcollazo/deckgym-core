use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game_with_board},
    Game,
};

/// Cradily (B4 007) – Stick and Absorb: 70 damage, "Heal 30 damage from this Pokémon. During your
/// opponent's next turn, the Defending Pokémon can't retreat."
fn use_stick_and_absorb() -> Game<'static> {
    let mut game = get_initialized_game_with_board(
        0,
        0,
        3,
        vec![PlayedCard::from_id(CardId::B4007Cradily)
            .with_energy(vec![EnergyType::Grass, EnergyType::Grass])
            .with_damage(50)],
        vec![
            PlayedCard::from_id(CardId::A1211Snorlax).with_energy(vec![
                EnergyType::Colorless,
                EnergyType::Colorless,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4007Cradily, 0),
        is_stack: false,
    });

    game
}

#[test]
fn test_stick_and_absorb_damages_and_heals_self() {
    let game = use_stick_and_absorb();
    let state = game.get_state_clone();

    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        80,
        "Stick and Absorb should deal 70 damage (Snorlax 150 -> 80)"
    );
    assert_eq!(
        state.get_active(0).get_remaining_hp(),
        130,
        "Stick and Absorb should heal 30 from Cradily (150 - 50 + 30 = 130)"
    );
}

#[test]
fn test_stick_and_absorb_blocks_defender_retreat_next_turn() {
    let mut game = use_stick_and_absorb();

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    game.play_until_stable();

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 1, "It should be the opponent's turn");
    assert!(
        !choices
            .iter()
            .any(|a| matches!(a.action, SimpleAction::Retreat(_))),
        "The Defending Pokémon should not be able to retreat during the opponent's next turn"
    );
}

/// Control: without Stick and Absorb the same board can retreat, proving the assertion above is
/// driven by the attack's effect and not by a missing retreat precondition.
#[test]
fn test_defender_can_retreat_without_stick_and_absorb() {
    let mut game = get_initialized_game_with_board(
        0,
        0,
        3,
        vec![PlayedCard::from_id(CardId::B4007Cradily)
            .with_energy(vec![EnergyType::Grass, EnergyType::Grass])
            .with_damage(50)],
        vec![
            PlayedCard::from_id(CardId::A1211Snorlax).with_energy(vec![
                EnergyType::Colorless,
                EnergyType::Colorless,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    game.play_until_stable();

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 1, "It should be the opponent's turn");
    assert!(
        choices
            .iter()
            .any(|a| matches!(a.action, SimpleAction::Retreat(_))),
        "Snorlax should be able to retreat when it wasn't hit by Stick and Absorb"
    );
}
