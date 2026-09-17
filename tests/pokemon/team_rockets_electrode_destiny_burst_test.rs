use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Team Rocket's Electrode's "Destiny Burst": "If this Pokémon is in the Active Spot and is
/// Knocked Out by damage from an attack from your opponent's Pokémon, do 70 damage to the
/// Attacking Pokémon."
///
/// Electrode (70 HP) is set to exactly the Vine Whip's 40 damage away from fainting, and the
/// attacking Bulbasaur also has exactly 70 HP, so Destiny Burst's retaliation knocks it out too
/// in the very same attack — verifying the retaliation resolves immediately rather than being
/// deferred to a later Pokemon Checkup.
#[test]
fn test_destiny_burst_knocks_out_attacker_in_same_attack() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_energy(vec![EnergyType::Grass, EnergyType::Colorless]),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
        vec![
            PlayedCard::from_id(CardId::B4a020TeamRocketsElectrode).with_remaining_hp(40),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1001Bulbasaur, 0),
        is_stack: false,
    });

    // Resolve the promotions both players now need (Electrode from Vine Whip, Bulbasaur from
    // Destiny Burst's retaliation).
    loop {
        let (_, choices) = game.get_state_clone().generate_possible_actions();
        let follow_up = choices.iter().find(|choice| {
            matches!(
                choice.action,
                SimpleAction::ApplyDamage { .. } | SimpleAction::Activate { .. }
            )
        });
        match follow_up {
            Some(action) => {
                let action = action.clone();
                game.apply_action(&action);
            }
            None => break,
        }
    }

    let state = game.get_state_clone();
    assert_eq!(
        state.points[0], 1,
        "Player 0 should score a point for knocking out Electrode"
    );
    assert_eq!(
        state.points[1], 1,
        "Player 1 should score a point for Destiny Burst knocking out Bulbasaur"
    );
    assert_eq!(state.get_active(0).get_name(), "Charmander");
    assert_eq!(state.get_active(1).get_name(), "Charmander");
}

/// Destiny Burst must not trigger when the hit is not lethal, unlike an always-on counterattack
/// ability (e.g. Rocky Helmet).
#[test]
fn test_destiny_burst_does_not_trigger_on_non_lethal_damage() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::B4a020TeamRocketsElectrode)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1001Bulbasaur, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        70 - 40,
        "Electrode should just take the Vine Whip damage"
    );
    assert_eq!(
        state.get_active(0).get_remaining_hp(),
        70,
        "Destiny Burst should not trigger when Electrode survives the hit"
    );
}
