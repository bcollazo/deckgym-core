use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game_with_board, get_test_game_with_board},
};

/// Victini's "Victory Star": "Once during your turn, after you flip any coins for an attack of
/// 1 of your [R] Pokémon, you may ignore all results of those coin flips and begin flipping
/// those coins again. You can't use more than 1 Victory Star Ability each turn."
///
/// After a coin-flip attack from an ally [R] Pokémon resolves, with an unused Victini in play,
/// the engine should offer exactly two follow-up choices: decline (SimpleAction::Noop, keep the
/// already-flipped result) or accept (SimpleAction::UseAbility on Victini, re-flip from scratch).
/// This should NOT be offered when the attack has no coin flip at all, even if the attacker is
/// Victini itself.
#[test]
fn test_victory_star_offers_reflip_choice_only_for_ally_fire_coin_flip_attacks() {
    // Ponyta's Stomp: 10 fixed damage, "Flip a coin. If heads, this attack does 30 more damage."
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1a010Ponyta).with_energy(vec![EnergyType::Fire]),
            PlayedCard::from_id(CardId::B3025Victini),
        ],
        vec![PlayedCard::from_id(CardId::A1053Squirtle)],
    );
    let mut state = game.get_state_clone();
    state.current_player = 0;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1a010Ponyta, 0),
        is_stack: false,
    });

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(
        actor, 0,
        "Attacking player should be offered the reflip decision"
    );
    assert_eq!(
        choices.len(),
        2,
        "Expected exactly a decline/accept pair, got: {:?}",
        choices.iter().map(|c| &c.action).collect::<Vec<_>>()
    );
    assert!(
        choices
            .iter()
            .any(|c| matches!(c.action, SimpleAction::Noop)),
        "Should be able to decline and keep the original coin result"
    );
    assert!(
        choices
            .iter()
            .any(|c| matches!(c.action, SimpleAction::UseAbility { in_play_idx: 1 })),
        "Should be able to invoke Victini's Victory Star (bench idx 1) to re-flip"
    );
}

#[test]
fn test_victory_star_not_offered_for_attacks_without_a_coin_flip() {
    // Victini's own "V-Flame" attack has no effect text, i.e. no coin flip to reflip, even
    // though Victini itself is a [R] Pokemon and is in play.
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B3025Victini)
            .with_energy(vec![EnergyType::Fire, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1053Squirtle)],
    );
    let mut state = game.get_state_clone();
    state.current_player = 0;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3025Victini, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    // V-Flame does a flat 40 damage; no coin flip means no reflip decision should be queued.
    assert_eq!(state.get_active(1).get_remaining_hp(), 20);
    assert!(
        state.move_generation_stack.is_empty(),
        "No reflip decision should be queued for a coin-less attack"
    );
}

/// Declining Victory Star (SimpleAction::Noop) must exactly reproduce the coin result that was
/// already flipped -- it is a pure "keep what happened" replay, not a second independent flip
/// and not a forced-heads effect (unlike the Will trainer card). We verify this by comparing,
/// for the same seed, a board with an (always-declined) Victini present against a baseline board
/// with no Victini at all: the resulting damage must match for every seed, and across enough
/// seeds we should see both the heads (40 dmg) and tails (10 dmg) results -- proving decline
/// doesn't skew the original distribution.
#[test]
fn test_declining_victory_star_reproduces_original_coin_result() {
    let mut saw_heads_damage = false;
    let mut saw_tails_damage = false;

    for seed in 0..50 {
        // Baseline: no Victini in play at all.
        let mut baseline_game = get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![PlayedCard::from_id(CardId::A1a010Ponyta).with_energy(vec![EnergyType::Fire])],
            vec![PlayedCard::from_id(CardId::A1053Squirtle)],
        );
        baseline_game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A1a010Ponyta, 0),
            is_stack: false,
        });
        baseline_game.play_until_stable();
        let baseline_hp = baseline_game
            .get_state_clone()
            .get_active(1)
            .get_remaining_hp();

        // With Victini present, but always declining the reflip.
        let mut game = get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![
                PlayedCard::from_id(CardId::A1a010Ponyta).with_energy(vec![EnergyType::Fire]),
                PlayedCard::from_id(CardId::B3025Victini),
            ],
            vec![PlayedCard::from_id(CardId::A1053Squirtle)],
        );
        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A1a010Ponyta, 0),
            is_stack: false,
        });
        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Noop,
            is_stack: true,
        });
        game.play_until_stable();
        let declined_hp = game.get_state_clone().get_active(1).get_remaining_hp();

        assert_eq!(
            declined_hp, baseline_hp,
            "Seed {seed}: declining Victory Star should exactly reproduce the original flip"
        );

        match declined_hp {
            50 => saw_tails_damage = true, // 60 - 10
            20 => saw_heads_damage = true, // 60 - 40
            other => panic!("Seed {seed}: unexpected resulting HP {other}"),
        }
    }

    assert!(
        saw_heads_damage,
        "Expected to see the heads (40 dmg) result at least once across 50 seeds"
    );
    assert!(
        saw_tails_damage,
        "Expected to see the tails (10 dmg) result at least once across 50 seeds \
         (if this fails, declining may be incorrectly forcing heads like Will does)"
    );
}

/// Accepting Victory Star must produce a genuinely NEW, independent coin roll -- not a replay of
/// the sequence that was just rejected. Two things are asserted across many seeds:
///   1. Both outcomes (heads = 40 dmg, tails = 10 dmg) occur after reflipping, proving the coin
///      is actually re-rolled rather than fixed.
///   2. For at least some seeds, the reflipped result DIFFERS from what declining would have
///      produced on that same seed -- proving the reflip is independent of the original flip.
/// Snorlax (150 HP, not Weak to [R]) absorbs either result without a knockout or weakness math.
#[test]
fn test_accepting_victory_star_reflips_with_independent_coins() {
    let mut saw_heads_after_reflip = false;
    let mut saw_tails_after_reflip = false;
    let mut saw_reflip_change_the_result = false;

    for seed in 0..200 {
        let board = || {
            (
                vec![
                    PlayedCard::from_id(CardId::A1a010Ponyta).with_energy(vec![EnergyType::Fire]),
                    PlayedCard::from_id(CardId::B3025Victini),
                ],
                vec![PlayedCard::from_id(CardId::A1211Snorlax)],
            )
        };

        // Path A: decline the reflip (keep the original coin).
        let (player_board, opponent_board) = board();
        let mut declined_game =
            get_initialized_game_with_board(seed, 0, 3, player_board, opponent_board);
        declined_game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A1a010Ponyta, 0),
            is_stack: false,
        });
        declined_game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Noop,
            is_stack: true,
        });
        declined_game.play_until_stable();
        let declined_hp = declined_game
            .get_state_clone()
            .get_active(1)
            .get_remaining_hp();

        // Path B: accept the reflip (fresh coin) from the identical starting seed/board.
        let (player_board, opponent_board) = board();
        let mut reflip_game =
            get_initialized_game_with_board(seed, 0, 3, player_board, opponent_board);
        reflip_game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A1a010Ponyta, 0),
            is_stack: false,
        });
        reflip_game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::UseAbility { in_play_idx: 1 },
            is_stack: true,
        });
        reflip_game.play_until_stable();
        let reflip_hp = reflip_game
            .get_state_clone()
            .get_active(1)
            .get_remaining_hp();

        match reflip_hp {
            140 => saw_tails_after_reflip = true, // 150 - 10
            110 => saw_heads_after_reflip = true, // 150 - 40
            other => panic!("Seed {seed}: unexpected HP after reflip: {other}"),
        }
        if reflip_hp != declined_hp {
            saw_reflip_change_the_result = true;
        }
    }

    assert!(
        saw_heads_after_reflip && saw_tails_after_reflip,
        "Reflip should yield both heads and tails across 200 seeds \
         (heads seen: {saw_heads_after_reflip}, tails seen: {saw_tails_after_reflip})"
    );
    assert!(
        saw_reflip_change_the_result,
        "Reflipping should sometimes change the outcome vs. declining; if it never does, \
         the 'reflip' is replaying the original coin instead of rolling a new one"
    );
}

/// "You can't use more than 1 Victory Star Ability each turn." After the ability is used once,
/// a second eligible [R] coin-flip attack in the same turn must resolve immediately with no
/// reflip prompt. Driven purely through public API: attack, accept the reflip, then attack again
/// (Mega Kangaskhan-style repeat attacks aren't needed -- we re-issue the attack action directly)
/// and assert the follow-up decision is gone.
#[test]
fn test_victory_star_can_only_be_used_once_per_turn() {
    let mut game = get_initialized_game_with_board(
        7,
        0,
        3,
        vec![
            PlayedCard::from_id(CardId::A1a010Ponyta).with_energy(vec![EnergyType::Fire]),
            PlayedCard::from_id(CardId::B3025Victini),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    // First attack: the prompt should appear, and we accept it.
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1a010Ponyta, 0),
        is_stack: false,
    });
    let (_, choices) = game.get_state_clone().generate_possible_actions();
    assert!(
        choices
            .iter()
            .any(|c| matches!(c.action, SimpleAction::UseAbility { in_play_idx: 1 })),
        "First eligible attack should offer Victory Star"
    );
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 1 },
        is_stack: true,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    assert!(
        state.move_generation_stack.is_empty(),
        "Reflip should have fully resolved"
    );

    // Second eligible attack in the SAME turn: no prompt this time.
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1a010Ponyta, 0),
        is_stack: false,
    });
    let (_, choices) = game.get_state_clone().generate_possible_actions();
    assert!(
        !choices
            .iter()
            .any(|c| matches!(c.action, SimpleAction::UseAbility { in_play_idx: 1 })),
        "Victory Star should not be offered twice in one turn, got: {:?}",
        choices.iter().map(|c| &c.action).collect::<Vec<_>>()
    );
}
