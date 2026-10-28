use std::collections::BTreeSet;

use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game_with_board},
    Game,
};

/// Meowth's "Carefree Steps": "If any damage is done to this Pokémon by attacks, flip a coin. If
/// heads, prevent that damage."
///
/// These attacks deliver their damage through a choice queued after the attack (a target pick, a
/// discard pick), not through the attack's own damage. That damage is still attack damage, so the
/// coin has to flip there too. Each test sweeps seeds and records which HP values the Meowth ends
/// on: a coin that never flips leaves a single value.
fn meowth_with_hp(hp: u32) -> PlayedCard {
    PlayedCard::new(
        get_card_by_enum(CardId::B2124Meowth),
        0,
        hp,
        vec![],
        false,
        vec![],
    )
}

/// Applies the first currently offered action that satisfies `pick`; returns false when none does.
fn apply_first_matching(game: &mut Game<'static>, pick: impl Fn(&SimpleAction) -> bool) -> bool {
    let (actor, actions) = game.get_state_clone().generate_possible_actions();
    match actions.into_iter().find(|a| pick(&a.action)) {
        Some(action) => {
            game.apply_action(&Action {
                actor,
                action: action.action,
                is_stack: action.is_stack,
            });
            true
        }
        None => false,
    }
}

fn remaining_hp(game: &Game<'static>, player: usize, in_play_idx: usize) -> u32 {
    game.get_state_clone().in_play_pokemon[player][in_play_idx]
        .as_ref()
        .expect("Pokémon should still be in play")
        .get_remaining_hp()
}

fn start_attack(game: &mut Game<'static>, card_id: CardId, index: usize) {
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(card_id, index),
        is_stack: false,
    });
}

/// Heatmor's Tongue Whip: "This attack does 30 damage to 1 of your opponent's Benched Pokémon."
#[test]
fn test_carefree_steps_flips_for_tongue_whip_bench_damage() {
    let mut outcomes = BTreeSet::new();
    for seed in 0..40u64 {
        let mut game = get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![PlayedCard::from_id(CardId::B1044Heatmor).with_energy(vec![EnergyType::Fire])],
            vec![
                PlayedCard::from_id(CardId::A1004VenusaurEx),
                PlayedCard::from_id(CardId::B2124Meowth),
            ],
        );
        start_attack(&mut game, CardId::B1044Heatmor, 0);
        assert!(apply_first_matching(&mut game, |a| matches!(
            a,
            SimpleAction::ApplyDamage { targets, .. } if targets[0].2 == 1
        )));
        outcomes.insert(remaining_hp(&game, 1, 1));
    }
    assert_eq!(outcomes, BTreeSet::from([20, 50]));
}

/// Vespiquen ex's Chase Order: "You may discard 1 of your Benched Basic [G] Pokémon. If you do,
/// this attack does 70 more damage." Both the plain hit and the boosted hit are queued choices.
#[test]
fn test_carefree_steps_flips_for_chase_order_with_and_without_discard() {
    for discard in [false, true] {
        let mut outcomes = BTreeSet::new();
        for seed in 0..40u64 {
            let mut game = get_initialized_game_with_board(
                seed,
                0,
                3,
                vec![
                    PlayedCard::from_id(CardId::B4011VespiquenEx)
                        .with_energy(vec![EnergyType::Grass, EnergyType::Grass]),
                    PlayedCard::from_id(CardId::A1001Bulbasaur),
                ],
                vec![meowth_with_hp(200)],
            );
            start_attack(&mut game, CardId::B4011VespiquenEx, 0);
            assert!(apply_first_matching(&mut game, |a| match a {
                SimpleAction::DiscardOwnBenchedThenDamage { .. } => discard,
                SimpleAction::ApplyDamage { .. } => !discard,
                _ => false,
            }));
            if discard {
                assert!(apply_first_matching(&mut game, |a| matches!(
                    a,
                    SimpleAction::ApplyDamage { .. }
                )));
            }
            outcomes.insert(remaining_hp(&game, 1, 0));
        }
        let expected = if discard { [60, 200] } else { [130, 200] };
        assert_eq!(outcomes, BTreeSet::from(expected), "discard: {discard}");
    }
}

/// Gyarados's Wild Swing: "You may discard any number of your Benched [W] Pokémon. This attack
/// does 40 more damage for each Benched Pokémon you discarded in this way." The damage is queued
/// after the discard choice, including when nothing is discarded.
#[test]
fn test_carefree_steps_flips_for_wild_swing_with_and_without_discard() {
    for discard in [false, true] {
        let mut outcomes = BTreeSet::new();
        for seed in 0..40u64 {
            let mut game = get_initialized_game_with_board(
                seed,
                0,
                3,
                vec![
                    PlayedCard::from_id(CardId::A4045Gyarados).with_energy(vec![
                        EnergyType::Water,
                        EnergyType::Water,
                        EnergyType::Water,
                    ]),
                    PlayedCard::from_id(CardId::A1053Squirtle),
                ],
                vec![meowth_with_hp(200)],
            );
            start_attack(&mut game, CardId::A4045Gyarados, 0);
            assert!(apply_first_matching(&mut game, |a| match a {
                SimpleAction::DiscardOwnBenchedGroupThenDamage {
                    in_play_indices, ..
                } => {
                    in_play_indices.is_empty() != discard
                }
                _ => false,
            }));
            assert!(apply_first_matching(&mut game, |a| matches!(
                a,
                SimpleAction::ApplyDamage { .. }
            )));
            outcomes.insert(remaining_hp(&game, 1, 0));
        }
        let expected = if discard { [140, 200] } else { [180, 200] };
        assert_eq!(outcomes, BTreeSet::from(expected), "discard: {discard}");
    }
}

/// Mega Kangaskhan ex's Double-Punching Family: "This attack is used twice in a row. The second
/// attack does 40 damage." The second punch is a separate instance of damage, so it flips its own
/// coin: all four heads/tails combinations show up.
#[test]
fn test_carefree_steps_flips_separately_for_each_kangaskhan_punch() {
    let mut outcomes = BTreeSet::new();
    for seed in 0..80u64 {
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
            vec![meowth_with_hp(200)],
        );
        start_attack(&mut game, CardId::B2127MegaKangaskhanEx, 0);
        game.play_until_stable();
        outcomes.insert(remaining_hp(&game, 1, 0));
    }
    assert_eq!(outcomes, BTreeSet::from([80, 120, 160, 200]));
}
