//! Special Conditions, per the in-app Tips panel (transcribed 2026-09-08):
//!
//! - Asleep: "cannot attack or retreat. The player whose Pokémon is Asleep flips a coin during
//!   each Pokémon Checkup. If heads, the Pokémon recovers. If tails, it stays Asleep."
//! - Paralyzed: "cannot attack or retreat. After its owner's next turn, it recovers during
//!   Pokémon Checkup."
//! - Confused: "When Confused Pokémon attack, the owner must flip a coin. If heads, the attack
//!   works normally, but if tails, the attack doesn't happen and the player's turn ends."
//! - "Asleep, Paralyzed, and Confused cannot stack with each other. If one of these Special
//!   Conditions is applied, it replaces any of the others." Poisoned and Burned "can stack with
//!   other Special Conditions".
//! - "Only Active Pokémon can have Special Conditions applied to them. If an Active Pokémon
//!   returns to the Bench by retreating or some other way, it recovers from all Special
//!   Conditions." Evolving recovers from them too.

use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard, StatusCondition},
    test_support::{attack_action, get_test_game_with_board},
    Game, State,
};

/// Player 0: Squirtle (1 [W] Retreat Cost) holding two [W] Energy — enough to pay for both its
/// Water Gun attack and a retreat — with a Bulbasaur on the Bench to retreat into.
/// Player 1: a lone Bulbasaur.
fn game_with_squirtle_able_to_attack_and_retreat() -> Game<'static> {
    get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1053Squirtle)
                .with_energy(vec![EnergyType::Water, EnergyType::Water]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    )
}

/// Whether the current player is currently offered (an attack, a retreat).
fn offers_attack_and_retreat(state: &State) -> (bool, bool) {
    let (_, actions) = state.generate_possible_actions();
    (
        actions
            .iter()
            .any(|a| matches!(a.action, SimpleAction::Attack(_))),
        actions
            .iter()
            .any(|a| matches!(a.action, SimpleAction::Retreat(_))),
    )
}

fn end_turn(game: &mut Game<'static>) {
    let actor = game.get_state_clone().current_player;
    game.apply_action(&Action {
        actor,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    game.play_until_stable();
}

fn set_status(game: &mut Game<'static>, player: usize, status: StatusCondition) {
    let mut state = game.get_state_clone();
    state.apply_status_condition(player, 0, status);
    game.set_state(state);
}

// ---------------------------------------------------------------------------------------------
// Asleep and Paralyzed block attacking and retreating (in-app: "cannot attack or retreat").
// ---------------------------------------------------------------------------------------------

#[test]
fn test_a_healthy_active_is_offered_both_an_attack_and_a_retreat() {
    // Control for the two tests below: without a Special Condition both actions are generated,
    // so their absence there is really caused by the condition.
    let game = game_with_squirtle_able_to_attack_and_retreat();
    let (can_attack, can_retreat) = offers_attack_and_retreat(&game.get_state_clone());
    assert!(can_attack, "an unafflicted Squirtle can use Water Gun");
    assert!(can_retreat, "an unafflicted Squirtle can retreat");
}

#[test]
fn test_asleep_pokemon_cannot_attack_or_retreat() {
    let mut game = game_with_squirtle_able_to_attack_and_retreat();
    set_status(&mut game, 0, StatusCondition::Asleep);

    let (can_attack, can_retreat) = offers_attack_and_retreat(&game.get_state_clone());
    assert!(!can_attack, "in-app Tips: Asleep Pokémon cannot attack");
    assert!(!can_retreat, "in-app Tips: Asleep Pokémon cannot retreat");
}

#[test]
fn test_paralyzed_pokemon_cannot_attack_or_retreat() {
    let mut game = game_with_squirtle_able_to_attack_and_retreat();
    set_status(&mut game, 0, StatusCondition::Paralyzed);

    let (can_attack, can_retreat) = offers_attack_and_retreat(&game.get_state_clone());
    assert!(!can_attack, "in-app Tips: Paralyzed Pokémon cannot attack");
    assert!(
        !can_retreat,
        "in-app Tips: Paralyzed Pokémon cannot retreat"
    );
}

#[test]
fn test_confused_pokemon_may_still_attack_and_retreat() {
    // Confusion is resolved by a coin flip *when attacking*; it does not remove the option, and
    // it says nothing about retreating.
    let mut game = game_with_squirtle_able_to_attack_and_retreat();
    set_status(&mut game, 0, StatusCondition::Confused);

    let (can_attack, can_retreat) = offers_attack_and_retreat(&game.get_state_clone());
    assert!(
        can_attack,
        "a Confused Pokémon still gets to try its attack"
    );
    assert!(can_retreat, "Confusion does not stop a retreat");
}

// ---------------------------------------------------------------------------------------------
// Paralysis lasts through its owner's next turn.
// ---------------------------------------------------------------------------------------------

/// The ordinary case: player 0 paralyzes player 1's Active on player 0's turn. The Checkup that
/// ends player 0's turn must NOT clear it — otherwise Paralysis never denies an attack at all.
/// It denies player 1's whole next turn and is cleared by the Checkup that ends that turn.
#[test]
fn test_paralysis_denies_the_owners_next_turn_and_clears_after_it() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![
            PlayedCard::from_id(CardId::A1053Squirtle)
                .with_energy(vec![EnergyType::Water, EnergyType::Water]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );
    // Inflicted during player 0's turn, on player 1's Active.
    set_status(&mut game, 1, StatusCondition::Paralyzed);

    end_turn(&mut game); // Checkup ending player 0's turn
    let state = game.get_state_clone();
    assert_eq!(state.current_player, 1, "it should now be player 1's turn");
    assert!(
        state.get_active(1).is_paralyzed(),
        "the Checkup ending the *inflicting* player's turn must not clear Paralysis — in-app \
         Tips: it recovers only \"after its owner's next turn\""
    );

    let (can_attack, can_retreat) = offers_attack_and_retreat(&state);
    assert!(
        !can_attack && !can_retreat,
        "Paralysis must actually deny its owner's next turn"
    );

    end_turn(&mut game); // Checkup ending player 1's (the owner's) turn
    let state = game.get_state_clone();
    assert!(
        !state.get_active(1).is_paralyzed(),
        "Paralysis recovers during the Checkup after its owner's next turn"
    );
}

/// The edge case the rule text pins down: when a Pokémon is paralyzed during its *own* owner's
/// turn, that turn is not the "next" one. It stays Paralyzed through the intervening opponent
/// turn, denies the owner's following turn, and only then clears.
#[test]
fn test_paralysis_inflicted_on_the_owners_own_turn_survives_until_their_following_turn() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1053Squirtle)
                .with_energy(vec![EnergyType::Water, EnergyType::Water]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    // Player 0's own turn, player 0's own Active.
    set_status(&mut game, 0, StatusCondition::Paralyzed);

    end_turn(&mut game); // Checkup ending the turn it was inflicted on
    assert!(
        game.get_state_clone().get_active(0).is_paralyzed(),
        "the turn the Paralysis landed on is not \"its owner's next turn\""
    );

    end_turn(&mut game); // Checkup ending player 1's turn
    let state = game.get_state_clone();
    assert_eq!(state.current_player, 0);
    assert!(
        state.get_active(0).is_paralyzed(),
        "a Checkup ending the opponent's turn never clears the owner's Paralysis"
    );
    let (can_attack, can_retreat) = offers_attack_and_retreat(&state);
    assert!(
        !can_attack && !can_retreat,
        "the owner's following turn is the one Paralysis denies"
    );

    end_turn(&mut game); // Checkup ending the owner's following turn
    assert!(
        !game.get_state_clone().get_active(0).is_paralyzed(),
        "Paralysis clears in the Checkup after the owner's next turn"
    );
}

// ---------------------------------------------------------------------------------------------
// Exclusivity: Asleep / Paralyzed / Confused replace one another; Poisoned and Burned stack.
// ---------------------------------------------------------------------------------------------

#[test]
fn test_asleep_paralyzed_and_confused_replace_one_another() {
    for (first, applied) in [
        (StatusCondition::Paralyzed, StatusCondition::Asleep),
        (StatusCondition::Confused, StatusCondition::Asleep),
        (StatusCondition::Asleep, StatusCondition::Paralyzed),
        (StatusCondition::Confused, StatusCondition::Paralyzed),
        (StatusCondition::Asleep, StatusCondition::Confused),
        (StatusCondition::Paralyzed, StatusCondition::Confused),
    ] {
        // A fresh board per pair, so each assertion sees exactly one application on top of one
        // pre-existing condition.
        let game = game_with_squirtle_able_to_attack_and_retreat();
        let mut state = game.get_state_clone();
        state.apply_status_condition(0, 0, first);
        state.apply_status_condition(0, 0, applied);

        let active = state.get_active(0);
        assert_eq!(
            active.count_status_conditions(),
            1,
            "in-app Tips: applying {applied:?} replaces {first:?}"
        );
        let held = match applied {
            StatusCondition::Asleep => active.is_asleep(),
            StatusCondition::Paralyzed => active.is_paralyzed(),
            StatusCondition::Confused => active.is_confused(),
            _ => unreachable!(),
        };
        assert!(
            held,
            "the last-applied condition ({applied:?}) is the one kept"
        );
    }
}

#[test]
fn test_poisoned_and_burned_stack_with_everything() {
    let mut game = game_with_squirtle_able_to_attack_and_retreat();
    let mut state = game.get_state_clone();

    state.apply_status_condition(0, 0, StatusCondition::Poisoned);
    state.apply_status_condition(0, 0, StatusCondition::Burned);
    state.apply_status_condition(0, 0, StatusCondition::Asleep);

    let active = state.get_active(0);
    assert!(active.is_poisoned() && active.is_burned() && active.is_asleep());
    assert_eq!(
        active.count_status_conditions(),
        3,
        "Poisoned and Burned stack with other Special Conditions"
    );

    // Swapping Asleep for Paralyzed must leave Poison and Burn untouched.
    state.apply_status_condition(0, 0, StatusCondition::Paralyzed);
    let active = state.get_active(0);
    assert!(
        active.is_poisoned() && active.is_burned() && active.is_paralyzed() && !active.is_asleep(),
        "replacing Asleep with Paralyzed must not disturb Poisoned or Burned"
    );
    game.set_state(state);
}

// ---------------------------------------------------------------------------------------------
// Confusion: tails means the attack doesn't happen and the turn ends. No self-damage.
// ---------------------------------------------------------------------------------------------

#[test]
fn test_confusion_tails_cancels_the_attack_without_self_damage_and_ends_the_turn() {
    // The coin is resolved by the game's RNG, so sweep seeds until a tails branch shows up.
    let mut saw_tails = false;
    let mut saw_heads = false;
    for seed in 0..40u64 {
        let mut game = deckgym::test_support::get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![PlayedCard::from_id(CardId::A1053Squirtle)
                .with_energy(vec![EnergyType::Water, EnergyType::Water])],
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        );
        set_status(&mut game, 0, StatusCondition::Confused);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A1053Squirtle, 0), // Water Gun, 20
            is_stack: false,
        });

        let state = game.get_state_clone();
        let opponent_hp = state.get_active(1).get_remaining_hp();
        let own_hp = state.get_active(0).get_remaining_hp();

        // Either branch: the attack ends the turn, so the only thing left to do is end it.
        let (_, actions) = state.generate_possible_actions();
        assert_eq!(
            actions.len(),
            1,
            "after attacking, the turn ends regardless of the Confusion coin (seed {seed})"
        );
        assert!(matches!(actions[0].action, SimpleAction::EndTurn));

        if opponent_hp == 70 {
            saw_tails = true;
            assert_eq!(
                own_hp, 60,
                "in-app Tips: on tails \"the attack doesn't happen\" — Pocket's Confusion deals \
                 no self-damage (seed {seed})"
            );
        } else {
            saw_heads = true;
            assert_eq!(opponent_hp, 50, "on heads the attack works normally");
            assert_eq!(own_hp, 60, "Confusion never damages the attacker");
        }
    }
    assert!(
        saw_tails,
        "expected at least one tails branch across the seeds"
    );
    assert!(
        saw_heads,
        "expected at least one heads branch across the seeds"
    );
}

// ---------------------------------------------------------------------------------------------
// Special Conditions live only on the Active Pokémon.
// ---------------------------------------------------------------------------------------------

#[test]
fn test_special_conditions_are_never_applied_to_a_benched_pokemon() {
    let mut game = game_with_squirtle_able_to_attack_and_retreat();
    let mut state = game.get_state_clone();

    for status in [
        StatusCondition::Poisoned,
        StatusCondition::Burned,
        StatusCondition::Asleep,
        StatusCondition::Paralyzed,
        StatusCondition::Confused,
    ] {
        state.apply_status_condition(0, 1, status);
    }

    let benched = state.in_play_pokemon[0][1]
        .as_ref()
        .expect("Bulbasaur should be on the Bench");
    assert_eq!(
        benched.count_status_conditions(),
        0,
        "in-app Tips: \"Only Active Pokémon can have Special Conditions applied to them.\""
    );
    game.set_state(state);
}

#[test]
fn test_retreating_recovers_from_every_special_condition() {
    let mut game = game_with_squirtle_able_to_attack_and_retreat();
    let mut state = game.get_state_clone();
    for status in [
        StatusCondition::Poisoned,
        StatusCondition::Burned,
        StatusCondition::Confused,
    ] {
        state.apply_status_condition(0, 0, status);
    }
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Retreat(1),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let squirtle = state
        .enumerate_bench_pokemon(0)
        .find(|(_, p)| p.get_name() == "Squirtle")
        .expect("Squirtle should now be Benched")
        .1;
    assert_eq!(
        squirtle.count_status_conditions(),
        0,
        "\"If an Active Pokémon returns to the Bench by retreating or some other way, it \
         recovers from all Special Conditions.\""
    );
}

#[test]
fn test_evolving_recovers_from_every_special_condition() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1053Squirtle)],
    );
    let mut state = game.get_state_clone();
    for status in [
        StatusCondition::Poisoned,
        StatusCondition::Burned,
        StatusCondition::Paralyzed,
    ] {
        state.apply_status_condition(0, 0, status);
    }
    state.hands[0].clear();
    state.hands[0].push(get_card_by_enum(CardId::A1002Ivysaur));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Evolve {
            evolution: get_card_by_enum(CardId::A1002Ivysaur),
            in_play_idx: 0,
            from_deck: false,
        },
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.get_active(0).get_name(), "Ivysaur");
    assert_eq!(
        state.get_active(0).count_status_conditions(),
        0,
        "\"When a Pokémon evolves ... Special Conditions affecting the Pokémon end.\""
    );
}
