use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game_with_board},
};

/// Hisuian Goodra's "Securely Sheltered": "If any damage is done to this Pokémon by attacks,
/// flip a coin. If heads, this Pokémon takes -80 damage from that attack."
///
/// A mirror Hisuian Goodra attacks with Heavy Impact (120 fixed damage). The defending Goodra
/// (150 HP) either takes the full 120 (tails, 30 HP left) or 120-80=40 (heads, 110 HP left).
/// Driving the attack across many seeds we must observe both branches, proving the reduction
/// coin actually fires and reduces (rather than prevents) the damage.
#[test]
fn test_securely_sheltered_reduces_damage_on_heads() {
    let mut saw_reduced = false;
    let mut saw_full_damage = false;

    for seed in 0..40u64 {
        let mut game = get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![
                PlayedCard::from_id(CardId::B3b050HisuianGoodra).with_energy(vec![
                    EnergyType::Water,
                    EnergyType::Metal,
                    EnergyType::Water,
                ]),
            ],
            vec![PlayedCard::from_id(CardId::B3b050HisuianGoodra)],
        );

        // Heavy Impact: 120 fixed damage, no effect.
        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::B3b050HisuianGoodra, 0),
            is_stack: false,
        });

        let state = game.get_state_clone();
        match state.get_active(1).get_remaining_hp() {
            110 => saw_reduced = true,    // Heads: 120 - 80 = 40 damage.
            30 => saw_full_damage = true, // Tails: full 120 damage.
            other => panic!("seed {seed}: unexpected defending Goodra HP {other}"),
        }
    }

    assert!(
        saw_reduced,
        "expected at least one seed where Securely Sheltered reduced the damage by 80"
    );
    assert!(
        saw_full_damage,
        "expected at least one seed where the full damage went through"
    );
}

/// When the incoming damage is 80 or less, a heads flip reduces it to 0 — but only the damage:
/// secondary attack effects still land. Grimer's Poison Gas (10 damage + Poison) against
/// Hisuian Goodra must always poison it, while its HP is either untouched (heads) or -10 (tails).
#[test]
fn test_securely_sheltered_floors_at_zero_and_keeps_attack_effects() {
    let mut saw_no_damage = false;
    let mut saw_damaged = false;

    for seed in 0..40u64 {
        let mut game = get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![PlayedCard::from_id(CardId::A1174Grimer).with_energy(vec![EnergyType::Darkness])],
            vec![PlayedCard::from_id(CardId::B3b050HisuianGoodra)],
        );

        // Poison Gas: 10 damage and the opponent's Active Pokémon is now Poisoned.
        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A1174Grimer, 0),
            is_stack: false,
        });

        let state = game.get_state_clone();
        let goodra = state.get_active(1);

        assert!(
            goodra.is_poisoned(),
            "seed {seed}: Goodra should be Poisoned even when its damage is reduced to 0"
        );

        match goodra.get_remaining_hp() {
            150 => saw_no_damage = true, // Heads: 10 - 80 floors at 0 damage.
            140 => saw_damaged = true,   // Tails: 10 damage dealt.
            other => panic!("seed {seed}: unexpected Goodra HP {other}"),
        }
    }

    assert!(
        saw_no_damage,
        "expected at least one seed where the damage was reduced to nothing"
    );
    assert!(
        saw_damaged,
        "expected at least one seed where the damage went through"
    );
}

/// Bastiodon's "Guarded Grill" shares the same effect template with -100 instead of -80.
/// Heavy Impact's 120 becomes 20 on heads (140 HP left of 160) or stays 120 on tails (40 left).
#[test]
fn test_guarded_grill_reduces_damage_by_100() {
    let mut saw_reduced = false;
    let mut saw_full_damage = false;

    for seed in 0..40u64 {
        let mut game = get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![
                PlayedCard::from_id(CardId::B3b050HisuianGoodra).with_energy(vec![
                    EnergyType::Water,
                    EnergyType::Metal,
                    EnergyType::Water,
                ]),
            ],
            vec![PlayedCard::from_id(CardId::A2114Bastiodon)],
        );

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::B3b050HisuianGoodra, 0),
            is_stack: false,
        });

        let state = game.get_state_clone();
        match state.get_active(1).get_remaining_hp() {
            140 => saw_reduced = true,    // Heads: 120 - 100 = 20 damage.
            40 => saw_full_damage = true, // Tails: full 120 damage.
            other => panic!("seed {seed}: unexpected Bastiodon HP {other}"),
        }
    }

    assert!(
        saw_reduced,
        "expected at least one seed where Guarded Grill reduced the damage by 100"
    );
    assert!(
        saw_full_damage,
        "expected at least one seed where the full damage went through"
    );
}
