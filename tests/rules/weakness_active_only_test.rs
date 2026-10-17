//! In-app Tips, "Use your Pokémon's attacks": "**Don't apply Weakness for Benched Pokémon.**"
//!
//! Weakness is a modifier on damage dealt to the opponent's **Active** Pokémon only. Spread
//! attacks that also hit the Bench deal their unmodified damage there, and the Bounded Field
//! Stadium (which turns Weakness into ×2 total damage) is likewise Active-only because it only
//! ever amplifies a Weakness that applies.

use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
    State,
};

/// Lurantis' Petal Blizzard: "This attack does 20 damage to each of your opponent's Pokémon."
/// Lurantis is [G]; Spiritomb ([D], 80 HP) is weak to [G].
fn game_with_lurantis_vs_spiritombs() -> deckgym::Game<'static> {
    get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A3015Lurantis).with_energy(vec![EnergyType::Grass])],
        vec![
            PlayedCard::from_id(CardId::A2104Spiritomb),
            PlayedCard::from_id(CardId::A2104Spiritomb),
        ],
    )
}

fn petal_blizzard() -> Action {
    Action {
        actor: 0,
        action: attack_action(CardId::A3015Lurantis, 0),
        is_stack: false,
    }
}

/// Damage taken, read off the public `get_remaining_hp` against the card's printed HP.
fn damage_on(state: &State, player: usize, idx: usize, printed_hp: u32) -> u32 {
    printed_hp
        - state.in_play_pokemon[player][idx]
            .as_ref()
            .expect("Pokemon should still be in play")
            .get_remaining_hp()
}

const SPIRITOMB_HP: u32 = 80;
const BULBASAUR_HP: u32 = 70;

#[test]
fn test_weakness_applies_to_active_but_not_to_benched_pokemon() {
    let mut game = game_with_lurantis_vs_spiritombs();
    game.apply_action(&petal_blizzard());
    let state = game.get_state_clone();

    assert_eq!(
        damage_on(&state, 1, 0, SPIRITOMB_HP),
        40,
        "the Active Spiritomb is weak to [G], so it takes 20 + 20 Weakness"
    );
    assert_eq!(
        damage_on(&state, 1, 1, SPIRITOMB_HP),
        20,
        "in-app Tips: \"Don't apply Weakness for Benched Pokémon\" — the Benched Spiritomb \
         takes the attack's plain 20"
    );
}

#[test]
fn test_bounded_field_doubles_weakness_damage_only_on_the_active() {
    let mut game = game_with_lurantis_vs_spiritombs();
    let mut state = game.get_state_clone();
    // Bounded Field: "...damage from attacks against Pokémon that have Weakness is ×2 instead
    // of the usual increase" (non-Mega-ex attackers only; Lurantis qualifies).
    state.set_active_stadium(get_card_by_enum(CardId::B3155BoundedField));
    game.set_state(state);

    game.apply_action(&petal_blizzard());
    let state = game.get_state_clone();

    assert_eq!(
        damage_on(&state, 1, 0, SPIRITOMB_HP),
        40,
        "Bounded Field turns the Active's Weakness into ×2 of 20"
    );
    assert_eq!(
        damage_on(&state, 1, 1, SPIRITOMB_HP),
        20,
        "Bounded Field only amplifies a Weakness that applies, and Weakness never applies to a \
         Benched Pokémon"
    );
}

/// A single-target attack against the opponent's Active is unaffected by the Active-only rule:
/// this guards the fix from over-reaching and switching Weakness off everywhere.
#[test]
fn test_weakness_still_applies_to_a_plain_active_attack() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A3014Fomantis).with_energy(vec![EnergyType::Grass])],
        vec![PlayedCard::from_id(CardId::A2104Spiritomb)],
    );
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A3014Fomantis, 0), // Leafage, 20
        is_stack: false,
    });
    let state = game.get_state_clone();
    assert_eq!(
        damage_on(&state, 1, 0, SPIRITOMB_HP),
        40,
        "Fomantis' Leafage does 20 + 20 Weakness to the [G]-weak Active Spiritomb"
    );
}

/// Guard against a regression where an attack that only ever hits the Bench somehow picks up
/// Weakness: Mew ex's Psychic Dive style "damage to a Benched Pokémon" path is exercised here
/// through the same spread attack, but asserting on the *no Active in the way* framing.
#[test]
fn test_bench_damage_is_unmodified_when_active_is_not_weak() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A3015Lurantis).with_energy(vec![EnergyType::Grass])],
        vec![
            // Bulbasaur is [G] and weak to [R]; it is not weak to Lurantis' [G].
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A2104Spiritomb),
        ],
    );
    game.apply_action(&petal_blizzard());
    let state = game.get_state_clone();

    assert_eq!(
        damage_on(&state, 1, 0, BULBASAUR_HP),
        20,
        "Active Bulbasaur is not [G]-weak"
    );
    assert_eq!(
        damage_on(&state, 1, 1, SPIRITOMB_HP),
        20,
        "the Benched [G]-weak Spiritomb still takes only the plain 20"
    );
    let _ = SimpleAction::EndTurn;
}
