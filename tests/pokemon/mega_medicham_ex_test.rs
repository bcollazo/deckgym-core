use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

// Mega Medicham ex's "Chakra Fist" (base 100 damage):
// "If this Pokémon has any [P] Energy attached, this attack does 40 more damage. This attack's
// damage isn't affected by any effects on your opponent's Active Pokémon."
//
// The second clause is the same effect Sawk's "Brick Break" has, so Chakra Fist shares the
// "ignore effects on the opponent's Active Pokémon" behavior. These tests drive the attack through
// the public `Game` API and assert on the resulting HP.

fn medicham_with(energy: Vec<EnergyType>) -> PlayedCard {
    PlayedCard::from_id(CardId::PB029MegaMedichamEx).with_energy(energy)
}

fn chakra_fist() -> Action {
    Action {
        actor: 0,
        action: attack_action(CardId::PB029MegaMedichamEx, 0),
        is_stack: false,
    }
}

/// With a [P] Energy attached, Chakra Fist does 100 + 40 = 140.
#[test]
fn test_mega_medicham_chakra_fist_psychic_bonus() {
    let mut game = get_test_game_with_board(
        vec![medicham_with(vec![
            EnergyType::Fighting,
            EnergyType::Colorless,
            EnergyType::Psychic,
        ])],
        vec![PlayedCard::from_id(CardId::A1004VenusaurEx)],
    );

    game.apply_action(&chakra_fist());

    let state = game.get_state_clone();
    // Venusaur ex has 190 HP (not weak to Fighting); 190 - 140 = 50.
    assert_eq!(state.get_active(1).get_remaining_hp(), 50);
}

/// Without any [P] Energy attached, Chakra Fist does its base 100.
#[test]
fn test_mega_medicham_chakra_fist_no_psychic_bonus() {
    let mut game = get_test_game_with_board(
        vec![medicham_with(vec![
            EnergyType::Fighting,
            EnergyType::Colorless,
            EnergyType::Colorless,
        ])],
        vec![PlayedCard::from_id(CardId::A1004VenusaurEx)],
    );

    game.apply_action(&chakra_fist());

    let state = game.get_state_clone();
    // 190 - 100 = 90.
    assert_eq!(state.get_active(1).get_remaining_hp(), 90);
}

/// Like Sawk's Brick Break, Chakra Fist ignores effects on the opponent's Active Pokémon.
/// Cloyster's "Shell Armor" (`ReduceDamageFromAttacks{10}`) is bypassed: it takes the full 100.
#[test]
fn test_mega_medicham_chakra_fist_bypasses_cloyster_shell_armor() {
    let mut game = get_test_game_with_board(
        vec![medicham_with(vec![
            EnergyType::Fighting,
            EnergyType::Colorless,
            EnergyType::Colorless,
        ])],
        vec![PlayedCard::from_id(CardId::A1067Cloyster)],
    );

    game.apply_action(&chakra_fist());

    let state = game.get_state_clone();
    // Cloyster has 120 HP; Shell Armor would leave it at 30. Chakra Fist ignores it → 20.
    assert_eq!(state.get_active(1).get_remaining_hp(), 20);
}

/// Oricorio's "Safeguard" (`PreventAllDamageFromEx`) normally blocks all damage from a Pokémon ex
/// (see `test_oricorio_safeguard_prevents_damage_from_ex_attack`). Mega Medicham ex is a Pokémon
/// ex, but Chakra Fist ignores effects on the opponent's Active Pokémon, so Safeguard is bypassed
/// and Oricorio (70 HP) still gets Knocked Out by the 100 damage — earning player 0 a point.
#[test]
fn test_mega_medicham_chakra_fist_damages_oricorio_through_safeguard() {
    let mut game = get_test_game_with_board(
        vec![medicham_with(vec![
            EnergyType::Fighting,
            EnergyType::Colorless,
            EnergyType::Colorless,
        ])],
        // Bench a backup so the Knock Out doesn't end the game.
        vec![
            PlayedCard::from_id(CardId::A3066Oricorio),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );

    game.apply_action(&chakra_fist());

    let state = game.get_state_clone();
    // If Safeguard had prevented the damage, Oricorio would survive and no point would be scored.
    assert_eq!(
        state.points[0], 1,
        "Chakra Fist must bypass Safeguard and Knock Out Oricorio"
    );
}
