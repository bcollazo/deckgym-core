//! Attacks from the attacks-a coverage batch whose effect only shows up later, in a hook: during
//! damage calculation, on an Energy Zone attach, on a retreat, or in move generation.

use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
    Game,
};

fn attack(game: &mut Game<'static>, actor: usize, card_id: CardId, index: usize) {
    game.apply_action(&Action {
        actor,
        action: attack_action(card_id, index),
        is_stack: false,
    });
}

/// Ends `actor`'s turn and settles the automatic start-of-turn actions (the draw), so that the
/// next `generate_possible_actions` returns the new player's real choices.
fn end_turn(game: &mut Game<'static>, actor: usize) {
    game.apply_action(&Action {
        actor,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    game.play_until_stable();
}

// -------------------------------------------------------------------------------------------
// Aegislash - Superb Shield (damage reduction, but only against Pokemon ex)
// -------------------------------------------------------------------------------------------

/// Superb Shield takes 80 off an attack from the opponent's Pokémon ex.
#[test]
fn test_aegislash_superb_shield_reduces_damage_from_pokemon_ex() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B2120Aegislash).with_energy(vec![
                EnergyType::Metal,
                EnergyType::Metal,
                EnergyType::Metal,
            ]),
        ],
        // Blastoise ex's Surf does 40; Aegislash (140 HP) is Weak to [R], not [W].
        vec![PlayedCard::from_id(CardId::A1056BlastoiseEx)
            .with_energy(vec![EnergyType::Water, EnergyType::Colorless])],
    );

    attack(&mut game, 0, CardId::B2120Aegislash, 0);
    end_turn(&mut game, 0);
    attack(&mut game, 1, CardId::A1056BlastoiseEx, 0);

    assert_eq!(
        game.get_state_clone().get_active(0).get_remaining_hp(),
        140,
        "40 damage from a Pokemon ex, minus 80, floors at 0"
    );
}

/// It does nothing against an attacker that is not a Pokémon ex.
#[test]
fn test_aegislash_superb_shield_does_not_reduce_damage_from_non_ex() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B2120Aegislash).with_energy(vec![
                EnergyType::Metal,
                EnergyType::Metal,
                EnergyType::Metal,
            ]),
        ],
        // Machoke is not an ex, is not Weak to [M], and survives Superb Shield's 80. Its Strength
        // is [F], so Aegislash's [R] Weakness does not apply either.
        vec![PlayedCard::from_id(CardId::A1144Machoke)
            .with_energy(vec![EnergyType::Fighting, EnergyType::Fighting])],
    );

    attack(&mut game, 0, CardId::B2120Aegislash, 0);
    end_turn(&mut game, 0);
    attack(&mut game, 1, CardId::A1144Machoke, 0);

    assert_eq!(
        game.get_state_clone().get_active(0).get_remaining_hp(),
        140 - 50,
        "Superb Shield only blunts attacks from the opponent's Pokemon ex"
    );
}

// -------------------------------------------------------------------------------------------
// Gothitelle - Stellar Cradle (Asleep on Energy Zone attach)
// -------------------------------------------------------------------------------------------

/// Feeding the trapped Defending Pokémon from the Energy Zone puts it to Sleep.
#[test]
fn test_gothitelle_stellar_cradle_sleeps_defender_on_energy_attach() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B1114Gothitelle)
            .with_energy(vec![EnergyType::Psychic, EnergyType::Psychic])],
        vec![PlayedCard::from_id(CardId::B4037WailordEx)],
    );

    attack(&mut game, 0, CardId::B1114Gothitelle, 0);
    end_turn(&mut game, 0);

    assert!(
        !game.get_state_clone().get_active(1).is_asleep(),
        "nothing happens until they actually attach"
    );

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 1);
    let attach = choices
        .iter()
        .find(|choice| {
            matches!(&choice.action, SimpleAction::Attach { attachments, is_turn_energy: true }
                if attachments.iter().any(|(_, _, idx)| *idx == 0))
        })
        .expect("the opponent should be able to attach their turn Energy to their Active")
        .clone();
    game.apply_action(&attach);

    assert!(
        game.get_state_clone().get_active(1).is_asleep(),
        "attaching from the Energy Zone should spring Stellar Cradle"
    );
}

/// Ending the turn without attaching leaves the defender awake.
#[test]
fn test_gothitelle_stellar_cradle_does_nothing_without_an_attach() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B1114Gothitelle)
            .with_energy(vec![EnergyType::Psychic, EnergyType::Psychic])],
        vec![PlayedCard::from_id(CardId::B4037WailordEx)],
    );

    attack(&mut game, 0, CardId::B1114Gothitelle, 0);
    end_turn(&mut game, 0);
    end_turn(&mut game, 1);

    assert!(!game.get_state_clone().get_active(1).is_asleep());
}

// -------------------------------------------------------------------------------------------
// Galarian Stunfisk - Snapping Trap (damage on the opponent's retreat)
// -------------------------------------------------------------------------------------------

/// Retreating into a Snapping Trap hits whatever the opponent promotes.
#[test]
fn test_galarian_stunfisk_snapping_trap_hits_the_new_active_on_retreat() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2117GalarianStunfisk)
            .with_energy(vec![EnergyType::Metal, EnergyType::Colorless])],
        vec![
            PlayedCard::from_id(CardId::B4037WailordEx).with_energy(vec![EnergyType::Colorless; 4]),
            PlayedCard::from_id(CardId::A1056BlastoiseEx),
        ],
    );

    attack(&mut game, 0, CardId::B2117GalarianStunfisk, 0);
    end_turn(&mut game, 0);

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 1);
    let retreat = choices
        .iter()
        .find(|choice| matches!(choice.action, SimpleAction::Retreat(1)))
        .expect("Wailord ex has 4 Energy for its 4 [C] Retreat Cost")
        .clone();
    game.apply_action(&retreat);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_name(),
        "Blastoise ex",
        "the Benched Pokemon was promoted"
    );
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        180 - 40,
        "the newly promoted Pokemon takes the trap's 40"
    );
}

/// The trap only lasts for the opponent's next turn, and only while Stunfisk holds the Active
/// Spot: retreating a turn later costs nothing.
#[test]
fn test_galarian_stunfisk_snapping_trap_expires_after_one_turn() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2117GalarianStunfisk)
            .with_energy(vec![EnergyType::Metal, EnergyType::Colorless])],
        vec![
            PlayedCard::from_id(CardId::B4037WailordEx).with_energy(vec![EnergyType::Colorless; 4]),
            PlayedCard::from_id(CardId::A1056BlastoiseEx),
        ],
    );

    attack(&mut game, 0, CardId::B2117GalarianStunfisk, 0);
    end_turn(&mut game, 0);
    end_turn(&mut game, 1); // let the trap's turn pass without retreating
    end_turn(&mut game, 0);

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 1);
    let retreat = choices
        .iter()
        .find(|choice| matches!(choice.action, SimpleAction::Retreat(1)))
        .expect("Wailord ex should still be able to retreat")
        .clone();
    game.apply_action(&retreat);

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_name(), "Blastoise ex");
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        180,
        "the trap should have expired"
    );
}

// -------------------------------------------------------------------------------------------
// Malamar - Evolution Jammer (no evolving from hand)
// -------------------------------------------------------------------------------------------

/// Evolution Jammer removes the opponent's Evolve actions for their next turn only, without
/// touching anything else they can do.
#[test]
fn test_malamar_evolution_jammer_blocks_evolving_from_hand_for_one_turn() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B3112Malamar)
            .with_energy(vec![EnergyType::Darkness, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let ivysaur: Vec<Card> = vec![get_card_by_enum(CardId::A1002Ivysaur)];
    let mut state = game.get_state_clone();
    state.hands[1] = ivysaur.clone();
    game.set_state(state);

    attack(&mut game, 0, CardId::B3112Malamar, 0);
    end_turn(&mut game, 0);

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 1);
    assert!(
        !choices
            .iter()
            .any(|choice| matches!(choice.action, SimpleAction::Evolve { .. })),
        "Ivysaur should not be playable onto Bulbasaur while jammed: {choices:?}"
    );

    // A turn later the jam is gone and the same evolution is offered again.
    end_turn(&mut game, 1);
    end_turn(&mut game, 0);

    let mut state = game.get_state_clone();
    state.hands[1] = ivysaur;
    game.set_state(state);

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 1);
    assert!(
        choices
            .iter()
            .any(|choice| matches!(choice.action, SimpleAction::Evolve { .. })),
        "the jam should only last one turn: {choices:?}"
    );
}

/// Evolution Jammer is aimed at the opponent, so it must not stop the attacker evolving on their
/// own next turn.
#[test]
fn test_malamar_evolution_jammer_does_not_block_its_own_side() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B3112Malamar)
                .with_energy(vec![EnergyType::Darkness, EnergyType::Colorless]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::B4037WailordEx)],
    );

    attack(&mut game, 0, CardId::B3112Malamar, 0);
    end_turn(&mut game, 0);
    end_turn(&mut game, 1);

    let mut state = game.get_state_clone();
    state.hands[0] = vec![get_card_by_enum(CardId::A1002Ivysaur)];
    game.set_state(state);

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(
        choices
            .iter()
            .any(|choice| matches!(choice.action, SimpleAction::Evolve { .. })),
        "Malamar's own side can still evolve: {choices:?}"
    );
}
