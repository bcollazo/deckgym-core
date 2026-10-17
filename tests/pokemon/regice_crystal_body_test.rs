//! Regice (A2 034) — Crystal Body: "Prevent all effects of attacks used by your opponent's
//! Pokémon done to this Pokémon."
//!
//! Rules interpretation used by these tests (and by the implementation): an attack's *effects*
//! are the Special Conditions and the `CardEffect`s the attack imposes on the targeted Pokémon.
//! Damage and the resulting Knock Out are explicitly NOT effects, so Crystal Body never reduces
//! damage. The shield is also scoped to the *opponent's* attacks: anything Regice's own player
//! causes (including a Tool such as Poison Barb punishing Regice for attacking) still lands.

use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    effects::CardEffect,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game_with_board, get_test_game_with_board},
    Game,
};

fn attack(game: &mut Game<'static>, actor: usize, card_id: CardId, index: usize) {
    game.apply_action(&Action {
        actor,
        action: attack_action(card_id, index),
        is_stack: false,
    });
}

/// Grimer's Poison Gas does 10 damage and Poisons the Defending Pokémon. Against Regice the
/// damage still lands (damage is not an "effect"), but the Poison is prevented.
#[test]
fn test_crystal_body_prevents_status_condition_from_opponent_attack_but_not_the_damage() {
    let poison_gas_against = |defender: CardId| {
        let mut game = get_initialized_game_with_board(
            0,
            1,
            3,
            vec![PlayedCard::from_id(defender)],
            vec![PlayedCard::from_id(CardId::A1174Grimer).with_energy(vec![EnergyType::Darkness])],
        );
        let mut state = game.get_state_clone();
        state.hands[1].clear();
        game.set_state(state);

        attack(&mut game, 1, CardId::A1174Grimer, 0);
        game.play_until_stable();

        let state = game.get_state_clone();
        (
            state.get_active(0).is_poisoned(),
            state.get_active(0).get_remaining_hp(),
        )
    };

    let (chansey_poisoned, _) = poison_gas_against(CardId::A1202Chansey);
    assert!(
        chansey_poisoned,
        "Poison Gas should normally Poison the Defending Pokemon"
    );

    let (regice_poisoned, regice_hp) = poison_gas_against(CardId::A2034Regice);
    assert!(
        !regice_poisoned,
        "Crystal Body should prevent the Poison from the opponent's attack"
    );
    assert_eq!(
        regice_hp, 100,
        "Crystal Body must not prevent the attack's damage (110 HP - 10)"
    );
}

/// Umbreon's Dark Binding does 40 damage and, if the Defending Pokémon is a Basic Pokémon, leaves
/// a `CannotAttack` effect on it. Regice is a Basic, so it would normally be affected.
#[test]
fn test_crystal_body_prevents_card_effect_from_opponent_attack_but_not_the_damage() {
    let dark_binding_against = |defender: CardId| {
        let mut game = get_initialized_game_with_board(
            0,
            1,
            3,
            vec![PlayedCard::from_id(defender)],
            vec![PlayedCard::from_id(CardId::A3b043Umbreon).with_energy(vec![EnergyType::Darkness])],
        );
        let mut state = game.get_state_clone();
        state.hands[1].clear();
        game.set_state(state);

        attack(&mut game, 1, CardId::A3b043Umbreon, 0);
        game.play_until_stable();

        let state = game.get_state_clone();
        (
            state.get_active(0).has_effect(&CardEffect::CannotAttack),
            state.get_active(0).get_remaining_hp(),
        )
    };

    let (chansey_blocked, _) = dark_binding_against(CardId::A1202Chansey);
    assert!(
        chansey_blocked,
        "Dark Binding should normally stop a Basic Defending Pokemon from attacking"
    );

    let (regice_blocked, regice_hp) = dark_binding_against(CardId::A2034Regice);
    assert!(
        !regice_blocked,
        "Crystal Body should prevent the CannotAttack effect from the opponent's attack"
    );
    assert_eq!(
        regice_hp, 70,
        "Crystal Body must not prevent the attack's damage (110 HP - 40)"
    );
}

/// Crystal Body only shields Regice from the *opponent's* attacks. Poison Barb Poisons whoever
/// attacks its holder — that happens on Regice's own turn, as a consequence of Regice's own
/// attack, so it is not an effect of an opponent's attack and still applies.
#[test]
fn test_crystal_body_does_not_shield_regice_on_its_own_turn() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A2034Regice)
            .with_energy(vec![EnergyType::Water, EnergyType::Water])],
        vec![PlayedCard::from_id(CardId::A1202Chansey)
            .with_tool(get_card_by_enum(CardId::A3146PoisonBarb))],
    );
    let mut state = game.get_state_clone();
    state.hands[0].clear();
    game.set_state(state);

    attack(&mut game, 0, CardId::A2034Regice, 0);
    game.play_until_stable();

    assert!(
        game.get_state_clone().get_active(0).is_poisoned(),
        "Poison Barb is not an effect of an opponent's attack, so Crystal Body must not stop it"
    );
}

/// Regice's own attack keeps working normally against a shielded-looking board: sanity check that
/// the ability is purely defensive and generates no ability action of its own.
#[test]
fn test_crystal_body_is_passive_and_offers_no_ability_action() {
    let game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A2034Regice)],
        vec![PlayedCard::from_id(CardId::A1202Chansey)],
    );

    let (_, actions) = game.get_state_clone().generate_possible_actions();
    assert!(
        !actions
            .iter()
            .any(|a| matches!(a.action, SimpleAction::UseAbility { .. })),
        "Crystal Body is passive and should never be offered as an action"
    );
}
