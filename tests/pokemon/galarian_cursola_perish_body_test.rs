use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game_with_board},
};

/// Galarian Cursola's "Perish Body": "If this Pokémon is in the Active Spot and is Knocked Out
/// by damage from an attack from your opponent's Pokémon, flip a coin. If heads, the Attacking
/// Pokémon is Knocked Out."
///
/// We drive a lethal attack across many RNG seeds and assert that the attacker is sometimes
/// Knocked Out in return (heads) and sometimes survives (tails), while Cursola is Knocked Out
/// and scores for the attacker on every seed.
#[test]
fn test_perish_body_flips_on_lethal_active_ko() {
    let mut saw_attacker_knocked_out = false;
    let mut saw_attacker_survived = false;

    for seed in 0..40u64 {
        // Bulbasaur's Vine Whip (40) is lethal on a Cursola at 40 remaining HP.
        let mut game = get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![
                PlayedCard::from_id(CardId::A1001Bulbasaur)
                    .with_energy(vec![EnergyType::Grass, EnergyType::Colorless]),
                PlayedCard::from_id(CardId::A1033Charmander),
            ],
            vec![
                PlayedCard::from_id(CardId::A4a035GalarianCursola).with_remaining_hp(40),
                PlayedCard::from_id(CardId::A4a034GalarianCorsola),
            ],
        );

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A1001Bulbasaur, 0),
            is_stack: false,
        });

        let state = game.get_state_clone();
        // Cursola is Knocked Out on every seed and the attacker scores its point.
        assert!(
            state.in_play_pokemon[1][0].is_none(),
            "seed {seed}: Cursola should be Knocked Out"
        );
        assert_eq!(state.points[0], 1, "seed {seed}");

        if state.in_play_pokemon[0][0].is_none() {
            // Heads: Perish Body Knocked Out the attacking Bulbasaur; the defender scores.
            assert_eq!(
                state.points[1], 1,
                "seed {seed}: defender should score for the attacker's KO"
            );
            saw_attacker_knocked_out = true;
        } else {
            // Tails: the attacker is untouched (Perish Body deals no damage).
            let bulbasaur = state.get_active(0);
            assert_eq!(bulbasaur.get_name(), "Bulbasaur", "seed {seed}");
            assert_eq!(bulbasaur.get_remaining_hp(), 70, "seed {seed}");
            assert_eq!(state.points[1], 0, "seed {seed}");
            saw_attacker_survived = true;
        }
    }

    assert!(
        saw_attacker_knocked_out,
        "expected at least one seed where Perish Body Knocked Out the attacker"
    );
    assert!(
        saw_attacker_survived,
        "expected at least one seed where the attacker survived the flip"
    );
}

/// Perish Body is passive: it must never be offered as a usable ability action.
#[test]
fn test_perish_body_is_passive() {
    let game = get_initialized_game_with_board(
        0,
        0,
        3,
        vec![PlayedCard::from_id(CardId::A4a035GalarianCursola)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let (_, actions) = game.get_state_clone().generate_possible_actions();
    assert!(
        !actions
            .iter()
            .any(|a| matches!(a.action, SimpleAction::UseAbility { .. })),
        "Perish Body should never be offered as a UseAbility action"
    );
}

/// Perish Body only triggers from the Active Spot: a benched Cursola Knocked Out by spread
/// damage must not flip, so the attacker survives on every seed.
#[test]
fn test_perish_body_does_not_trigger_on_bench_ko() {
    for seed in 0..20u64 {
        // Lurantis's Petal Blizzard does 20 to each of the opponent's Pokémon: lethal on the
        // benched Cursola at 20 HP, non-lethal on the active Charmander.
        let mut game = get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![PlayedCard::from_id(CardId::A3015Lurantis).with_energy(vec![EnergyType::Grass])],
            vec![
                PlayedCard::from_id(CardId::A1033Charmander),
                PlayedCard::from_id(CardId::A4a035GalarianCursola).with_remaining_hp(20),
            ],
        );

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A3015Lurantis, 0),
            is_stack: false,
        });

        let state = game.get_state_clone();
        assert!(
            state.in_play_pokemon[1][1].is_none(),
            "seed {seed}: benched Cursola should be Knocked Out"
        );
        assert_eq!(state.points[0], 1, "seed {seed}");
        assert_eq!(
            state.get_active(1).get_remaining_hp(),
            60 - 20,
            "seed {seed}"
        );
        // No Perish Body flip from the bench: Lurantis is never Knocked Out.
        assert!(state.in_play_pokemon[0][0].is_some(), "seed {seed}");
        assert_eq!(state.points[1], 0, "seed {seed}");
    }
}

/// Non-lethal damage must not trigger Perish Body: damage applies normally on every seed and
/// the attacker is never at risk.
#[test]
fn test_perish_body_does_not_trigger_on_non_lethal_damage() {
    for seed in 0..10u64 {
        let mut game = get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
            vec![PlayedCard::from_id(CardId::A4a035GalarianCursola)],
        );

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A1001Bulbasaur, 0),
            is_stack: false,
        });

        let state = game.get_state_clone();
        assert_eq!(
            state.get_active(1).get_remaining_hp(),
            80 - 40,
            "seed {seed}: non-lethal damage should apply normally"
        );
        assert_eq!(state.get_active(0).get_remaining_hp(), 70, "seed {seed}");
        assert_eq!(state.points, [0, 0], "seed {seed}");
    }
}

/// Damage queued through the move-generation stack is still "damage from an attack": if
/// Cursola is promoted after the first punch and then Knocked Out by Mega Kangaskhan ex's
/// second punch, Perish Body flips — and on heads the Mega KO (3 points) wins the defender
/// the game.
#[test]
fn test_perish_body_flips_on_kangaskhan_second_punch() {
    let mut saw_attacker_knocked_out = false;
    let mut saw_attacker_survived = false;

    for seed in 0..40u64 {
        let mut game = get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![
                PlayedCard::from_id(CardId::B2127MegaKangaskhanEx).with_energy(vec![
                    EnergyType::Colorless,
                    EnergyType::Colorless,
                    EnergyType::Colorless,
                ]),
                PlayedCard::from_id(CardId::A1033Charmander),
            ],
            vec![
                PlayedCard::from_id(CardId::A1033Charmander).with_remaining_hp(30),
                PlayedCard::from_id(CardId::A4a035GalarianCursola).with_remaining_hp(40),
                // A spare Pokémon so the game doesn't end when Cursola is Knocked Out.
                PlayedCard::from_id(CardId::A4a034GalarianCorsola),
            ],
        );

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::B2127MegaKangaskhanEx, 0),
            is_stack: false,
        });

        // Resolve the queued follow-ups: the defender's promotion (always pick Cursola so the
        // second punch faces Perish Body) and the second punch's ApplyDamage (which Knocks Out
        // the promoted Cursola and flips Perish Body).
        loop {
            let state = game.get_state_clone();
            if state.winner.is_some() {
                break;
            }
            let (_, choices) = state.generate_possible_actions();
            let follow_up = choices
                .iter()
                .find(|choice| {
                    matches!(
                        choice.action,
                        SimpleAction::Activate {
                            player: 1,
                            in_play_idx: 1,
                        }
                    )
                })
                .or_else(|| {
                    choices.iter().find(|choice| {
                        matches!(
                            choice.action,
                            SimpleAction::ApplyDamage { .. } | SimpleAction::Activate { .. }
                        )
                    })
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
        // Both punches always land: Charmander then the promoted Cursola are Knocked Out.
        assert_eq!(state.points[0], 2, "seed {seed}");

        if state.points[1] > 0 {
            // Heads: Perish Body Knocked Out Mega Kangaskhan ex — worth 3 points, so the
            // defender wins outright.
            assert_eq!(state.points[1], 3, "seed {seed}");
            assert!(state.winner.is_some(), "seed {seed}");
            saw_attacker_knocked_out = true;
        } else {
            assert!(
                state.in_play_pokemon[0][0].is_some(),
                "seed {seed}: Kangaskhan should survive a tails flip"
            );
            saw_attacker_survived = true;
        }
    }

    assert!(
        saw_attacker_knocked_out,
        "expected at least one seed where Perish Body Knocked Out Mega Kangaskhan ex"
    );
    assert!(
        saw_attacker_survived,
        "expected at least one seed where Mega Kangaskhan ex survived both flips"
    );
}
