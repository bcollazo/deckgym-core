use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game_with_board},
};

/// Meowth's "Carefree Steps": "If any damage is done to this Pokémon by attacks, flip a coin. If
/// heads, prevent that damage."
///
/// The key behavior (and the bug this fixes): on a heads flip only the *damage* to Meowth is
/// prevented — the rest of the attack (here, the Poison from Grimer's Poison Gas) still happens.
/// Previously the heads branch dropped the whole attack, including its effects.
///
/// We drive the attack across many RNG seeds and assert:
/// - Meowth is ALWAYS poisoned (the secondary effect resolves regardless of the coin), and
/// - Meowth's remaining HP is sometimes full (heads, damage prevented) and sometimes reduced
///   (tails, 10 damage), proving the prevention coin actually fires.
#[test]
fn test_meowth_carefree_steps_prevents_only_damage_not_effects() {
    let mut saw_prevented = false;
    let mut saw_damaged = false;

    for seed in 0..40u64 {
        // Grimer (attacker) vs Meowth with Carefree Steps (defender).
        let mut game = get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![PlayedCard::from_id(CardId::A1174Grimer).with_energy(vec![EnergyType::Darkness])],
            vec![PlayedCard::from_id(CardId::B2124Meowth)],
        );

        // Poison Gas: 10 damage and the opponent's Active Pokémon is now Poisoned.
        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A1174Grimer, 0),
            is_stack: false,
        });

        let state = game.get_state_clone();
        let meowth = state.get_active(1);

        // The Poison effect must always land, even on the heads (damage-prevented) branch.
        assert!(
            meowth.is_poisoned(),
            "seed {seed}: Meowth should be Poisoned even when its damage is prevented"
        );

        match meowth.get_remaining_hp() {
            50 => saw_prevented = true, // Heads: damage to Meowth prevented.
            40 => saw_damaged = true,   // Tails: 10 damage dealt.
            other => panic!("seed {seed}: unexpected Meowth HP {other}"),
        }
    }

    assert!(
        saw_prevented,
        "expected at least one seed where Carefree Steps prevented the damage"
    );
    assert!(
        saw_damaged,
        "expected at least one seed where the damage went through"
    );
}

/// Carefree Steps applies wherever the Meowth is — including the Bench — and each Meowth flips
/// its own independent coin. Palkia ex's Dimensional Storm does 150 to the Active and 20 to every
/// Benched Pokémon, so against two Benched Meowth there should be two independent coin flips.
///
/// Across seeds we expect to observe every combination — both prevented, both hit, and (crucially)
/// the mixed cases where one Meowth is prevented and the other is not. A single shared coin could
/// never produce a mixed outcome, so seeing one proves the flips are independent and per-Pokémon.
#[test]
fn test_carefree_steps_applies_to_benched_meowth_with_independent_flips() {
    let mut saw_both_prevented = false;
    let mut saw_both_hit = false;
    let mut saw_mixed = false;

    for seed in 0..60u64 {
        // Palkia ex (attacker) vs a tanky Active (survives 150) backed by two Benched Meowth.
        let mut game = get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![PlayedCard::from_id(CardId::A2049PalkiaEx).with_energy(vec![
                EnergyType::Water,
                EnergyType::Water,
                EnergyType::Water,
                EnergyType::Water,
            ])],
            vec![
                // 190 HP, survives the 150 so no promotion muddies the bench slots.
                PlayedCard::from_id(CardId::A1004VenusaurEx),
                PlayedCard::from_id(CardId::B2124Meowth),
                PlayedCard::from_id(CardId::B2124Meowth),
            ],
        );

        // Dimensional Storm is Palkia ex's second attack (index 1).
        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A2049PalkiaEx, 1),
            is_stack: false,
        });

        let state = game.get_state_clone();

        // The Active (no Carefree Steps) always takes the full 150.
        assert_eq!(
            state.get_active(1).get_remaining_hp(),
            190 - 150,
            "seed {seed}: opponent Active should always take full Dimensional Storm damage"
        );

        // Each Benched Meowth is either untouched (50) or took 20 (30).
        let hp1 = state.in_play_pokemon[1][1]
            .as_ref()
            .expect("first Benched Meowth should still be in play")
            .get_remaining_hp();
        let hp2 = state.in_play_pokemon[1][2]
            .as_ref()
            .expect("second Benched Meowth should still be in play")
            .get_remaining_hp();

        for hp in [hp1, hp2] {
            assert!(
                hp == 50 || hp == 30,
                "seed {seed}: unexpected Benched Meowth HP {hp}"
            );
        }

        match (hp1, hp2) {
            (50, 50) => saw_both_prevented = true,
            (30, 30) => saw_both_hit = true,
            _ => saw_mixed = true,
        }
    }

    assert!(
        saw_both_prevented,
        "expected a seed where both Benched Meowth prevented the damage"
    );
    assert!(
        saw_both_hit,
        "expected a seed where both Benched Meowth took the damage"
    );
    assert!(
        saw_mixed,
        "expected a mixed seed (one Meowth prevented, one not), proving two independent coin flips"
    );
}
