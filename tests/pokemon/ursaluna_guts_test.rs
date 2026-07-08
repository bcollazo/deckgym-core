use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game_with_board},
};

/// Ursaluna's "Guts": "If this Pokémon would be Knocked Out by damage from an attack, flip a
/// coin. If heads, this Pokémon is not Knocked Out, and its remaining HP becomes 10."
///
/// We drive a lethal attack across many RNG seeds and assert that Ursaluna sometimes survives
/// at exactly 10 HP (heads) and is sometimes Knocked Out (tails).
#[test]
fn test_guts_coin_flip_can_survive_lethal_attack_at_10_hp() {
    let mut saw_survived = false;
    let mut saw_knocked_out = false;

    for seed in 0..40u64 {
        // Bulbasaur's Vine Whip (40) is lethal on an Ursaluna at 30 remaining HP.
        let mut game = get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
            vec![
                PlayedCard::from_id(CardId::B3b058Ursaluna).with_remaining_hp(30),
                PlayedCard::from_id(CardId::A1033Charmander),
            ],
        );

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A1001Bulbasaur, 0),
            is_stack: false,
        });

        let state = game.get_state_clone();
        if state.points[0] > 0 {
            // Tails: Ursaluna was Knocked Out and the attacker scored a point.
            saw_knocked_out = true;
        } else {
            let ursaluna = state.get_active(1);
            assert_eq!(ursaluna.get_name(), "Ursaluna", "seed {seed}");
            assert_eq!(
                ursaluna.get_remaining_hp(),
                10,
                "seed {seed}: Guts survival should leave Ursaluna at exactly 10 HP"
            );
            saw_survived = true;
        }
    }

    assert!(
        saw_survived,
        "expected at least one seed where Guts saved Ursaluna"
    );
    assert!(
        saw_knocked_out,
        "expected at least one seed where Ursaluna was Knocked Out"
    );
}

/// Even when Guts saves Ursaluna, the attack still damaged it — so on-damage triggers like
/// Rocky Helmet's 20 counterattack damage must fire on every branch (verified in-game: the
/// helmet damage resolves before the Guts result).
#[test]
fn test_guts_survival_still_triggers_rocky_helmet() {
    let mut saw_survived = false;
    let mut saw_knocked_out = false;

    for seed in 0..40u64 {
        let mut game = get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
            vec![
                PlayedCard::from_id(CardId::B3b058Ursaluna)
                    .with_tool(get_card_by_enum(CardId::A2148RockyHelmet))
                    .with_remaining_hp(30),
                PlayedCard::from_id(CardId::A1033Charmander),
            ],
        );

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A1001Bulbasaur, 0),
            is_stack: false,
        });

        let state = game.get_state_clone();
        assert_eq!(
            state.get_active(0).get_remaining_hp(),
            70 - 20,
            "seed {seed}: Rocky Helmet should hit the attacker whether or not Guts saves Ursaluna"
        );
        if state.points[0] > 0 {
            saw_knocked_out = true;
        } else {
            assert_eq!(state.get_active(1).get_remaining_hp(), 10, "seed {seed}");
            saw_survived = true;
        }
    }

    assert!(saw_survived && saw_knocked_out);
}

/// Mega Kangaskhan ex's Double-Punching Family attacks twice. If Guts saves Ursaluna from the
/// first hit (80), the second hit (40) is a new knock out threat and triggers its own,
/// independent Guts coin flip (verified in-game).
#[test]
fn test_guts_flips_again_on_kangaskhans_second_punch() {
    let mut saw_survived_both_hits = false;
    let mut saw_knocked_out = false;

    for seed in 0..60u64 {
        // Ursaluna at 80 remaining HP: the first punch is exactly lethal, and after a heads
        // (10 HP remaining) the 40-damage second punch is lethal again.
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
            ],
            vec![
                PlayedCard::from_id(CardId::B3b058Ursaluna).with_remaining_hp(80),
                PlayedCard::from_id(CardId::A1033Charmander),
            ],
        );

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::B2127MegaKangaskhanEx, 0),
            is_stack: false,
        });

        // Resolve the queued follow-ups: the defender's promotion (if Ursaluna was Knocked
        // Out) and the second punch's ApplyDamage.
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
        if state.points[0] > 0 {
            saw_knocked_out = true;
        } else {
            // Surviving both hits requires two independent heads: 80 -> 10 (first Guts flip),
            // then 40 vs 10 HP -> 10 (second Guts flip).
            assert_eq!(state.get_active(1).get_name(), "Ursaluna", "seed {seed}");
            assert_eq!(state.get_active(1).get_remaining_hp(), 10, "seed {seed}");
            saw_survived_both_hits = true;
        }
    }

    assert!(
        saw_survived_both_hits,
        "expected at least one seed where Guts saved Ursaluna from both punches"
    );
    assert!(saw_knocked_out);
}

/// Non-lethal damage must not trigger the Guts coin flip: damage should apply normally on
/// every seed.
#[test]
fn test_guts_does_not_trigger_on_non_lethal_damage() {
    for seed in 0..10u64 {
        let mut game = get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
            vec![PlayedCard::from_id(CardId::B3b058Ursaluna)],
        );

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A1001Bulbasaur, 0),
            is_stack: false,
        });

        let state = game.get_state_clone();
        assert_eq!(
            state.get_active(1).get_remaining_hp(),
            160 - 40,
            "seed {seed}: non-lethal damage should apply normally"
        );
    }
}
