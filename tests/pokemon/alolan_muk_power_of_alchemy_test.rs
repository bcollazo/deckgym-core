//! Alolan Muk (B2 097 / B2 173) — Power of Alchemy: "Basic Pokémon in play (both yours and your
//! opponent's) have no Abilities."
//!
//! Alolan Muk is a Stage 1, so it never suppresses its own Ability. Suppression covers both
//! activated Basic Abilities (no `UseAbility` action is offered) and passive ones (damage
//! reduction, retreat cost, ...), on both players' boards, and lifts as soon as Muk leaves play.

use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
    Game,
};

fn shaymin_ability_offered(game: &Game<'static>) -> bool {
    let (_, actions) = game.get_state_clone().generate_possible_actions();
    actions
        .iter()
        .any(|a| matches!(a.action, SimpleAction::UseAbility { in_play_idx: 1 }))
}

/// Shaymin's Fragrant Flower Garden is an activated Basic Ability. With an Alolan Muk on the
/// opponent's board it is gone, so the engine offers no `UseAbility` action for it.
#[test]
fn test_power_of_alchemy_removes_an_activated_basic_ability() {
    let game_with_opponent = |opponent: CardId| {
        get_test_game_with_board(
            vec![
                PlayedCard::from_id(CardId::A1202Chansey),
                PlayedCard::from_id(CardId::A2022Shaymin),
            ],
            vec![PlayedCard::from_id(opponent)],
        )
    };

    assert!(
        shaymin_ability_offered(&game_with_opponent(CardId::A1202Chansey)),
        "Shaymin's Ability should normally be offered"
    );
    assert!(
        !shaymin_ability_offered(&game_with_opponent(CardId::B2097AlolanMuk)),
        "Power of Alchemy should remove the activated Ability of a Basic Pokemon"
    );
}

/// The clause "both yours and your opponent's" means Muk also switches off Basic Abilities on its
/// own side of the board — and, being a Stage 1, it keeps its own Ability while doing so.
#[test]
fn test_power_of_alchemy_also_removes_basic_abilities_on_its_own_side() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1202Chansey),
            PlayedCard::from_id(CardId::A2022Shaymin),
            PlayedCard::from_id(CardId::B2097AlolanMuk),
        ],
        vec![PlayedCard::from_id(CardId::A1202Chansey)],
    );
    let mut state = game.get_state_clone();
    state.hands[0].clear();
    game.set_state(state);

    assert!(
        !shaymin_ability_offered(&game),
        "Power of Alchemy should remove Basic Abilities on its controller's board too"
    );
}

fn regirock_remaining_hp_after_gentle_slap(muk_on_attacker_bench: bool) -> u32 {
    let mut player_board = vec![PlayedCard::from_id(CardId::A1202Chansey).with_energy(vec![
        EnergyType::Colorless,
        EnergyType::Colorless,
        EnergyType::Colorless,
    ])];
    if muk_on_attacker_bench {
        player_board.push(PlayedCard::from_id(CardId::B2097AlolanMuk));
    }
    let mut game = get_test_game_with_board(
        player_board,
        vec![PlayedCard::from_id(CardId::A2087Regirock)],
    );
    let mut state = game.get_state_clone();
    state.hands[0].clear();
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1202Chansey, 0),
        is_stack: false,
    });
    game.play_until_stable();

    game.get_state_clone().get_active(1).get_remaining_hp()
}

/// Regirock's Exoskeleton ("This Pokémon takes -20 damage from attacks") is a passive Basic
/// Ability. Chansey's Gentle Slap does 60; with Exoskeleton that is 40, and with Power of Alchemy
/// in play Exoskeleton is gone so the full 60 lands.
#[test]
fn test_power_of_alchemy_removes_a_passive_basic_ability() {
    assert_eq!(
        regirock_remaining_hp_after_gentle_slap(false),
        80,
        "Exoskeleton should normally reduce Gentle Slap's 60 damage to 40 (120 HP - 40)"
    );
    assert_eq!(
        regirock_remaining_hp_after_gentle_slap(true),
        60,
        "Power of Alchemy should remove Exoskeleton, so all 60 damage lands (120 HP - 60)"
    );
}

/// Melmetal's Hard Coat is the same -20 damage reduction on a Stage 1, which Power of Alchemy
/// does not touch.
#[test]
fn test_power_of_alchemy_does_not_affect_evolved_pokemon_abilities() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1202Chansey).with_energy(vec![
                EnergyType::Colorless,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
            PlayedCard::from_id(CardId::B2097AlolanMuk),
        ],
        vec![PlayedCard::from_id(CardId::A1182Melmetal)],
    );
    let mut state = game.get_state_clone();
    state.hands[0].clear();
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1202Chansey, 0),
        is_stack: false,
    });
    game.play_until_stable();

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        90,
        "Melmetal is a Stage 1, so Hard Coat still reduces the 60 damage to 40 (130 HP - 40)"
    );
}

/// Suppression is not sticky: once Alolan Muk leaves play, Basic Abilities come back.
#[test]
fn test_basic_abilities_return_when_alolan_muk_leaves_play() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1202Chansey),
            PlayedCard::from_id(CardId::A2022Shaymin),
        ],
        vec![PlayedCard::from_id(CardId::B2097AlolanMuk)],
    );
    assert!(
        !shaymin_ability_offered(&game),
        "Shaymin's Ability should be suppressed while Alolan Muk is in play"
    );

    let mut state = game.get_state_clone();
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1202Chansey),
            PlayedCard::from_id(CardId::A2022Shaymin),
        ],
        vec![PlayedCard::from_id(CardId::A1202Chansey)],
    );
    game.set_state(state);

    assert!(
        shaymin_ability_offered(&game),
        "Shaymin's Ability should come back once Alolan Muk leaves play"
    );
}

/// The B2 173 full-art print carries the same Ability.
#[test]
fn test_power_of_alchemy_full_art_print_suppresses_too() {
    let game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1202Chansey),
            PlayedCard::from_id(CardId::A2022Shaymin),
        ],
        vec![PlayedCard::from_id(CardId::B2173AlolanMuk)],
    );
    assert!(
        !shaymin_ability_offered(&game),
        "B2 173 Alolan Muk should suppress Basic Abilities just like B2 097"
    );
}

/// Roaring Moon's Ancient Roar triggers from the `on_bench_from_hand` hook rather than from
/// regular ability move generation. Roaring Moon is a Basic, so Power of Alchemy switches it off
/// there too.
#[test]
fn test_power_of_alchemy_removes_a_basic_on_bench_from_hand_ability() {
    let ancient_roar_offered = |opponent_active: CardId| {
        let mut game = get_test_game_with_board(
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
            vec![
                PlayedCard::from_id(opponent_active),
                PlayedCard::from_id(CardId::A1057Psyduck),
            ],
        );
        let mut state = game.get_state_clone();
        state.hands[0].clear();
        state.hands[0].push(deckgym::database::get_card_by_enum(
            CardId::B3a047RoaringMoon,
        ));
        game.set_state(state);

        let (_, actions) = game.get_state_clone().generate_possible_actions();
        let place = actions
            .into_iter()
            .find(|a| matches!(&a.action, SimpleAction::Place(c, _) if c.get_name() == "Roaring Moon"))
            .expect("Roaring Moon should be placeable on the bench");
        game.apply_action(&place);

        let (_, actions) = game.get_state_clone().generate_possible_actions();
        actions
            .iter()
            .any(|a| matches!(a.action, SimpleAction::UseAbility { .. }))
    };

    assert!(
        ancient_roar_offered(CardId::A1053Squirtle),
        "Ancient Roar should normally be offered when Roaring Moon is benched"
    );
    assert!(
        !ancient_roar_offered(CardId::B2097AlolanMuk),
        "Power of Alchemy should remove Ancient Roar, a Basic Pokemon's on-bench Ability"
    );
}

/// Teal Mask Ogerpon ex's Soothing Wind cures Special Conditions the moment it enters play. That
/// on-entry hook reads the card rather than the board, so it needs the same suppression.
#[test]
fn test_power_of_alchemy_removes_the_on_entry_cure_of_a_basic_ability() {
    let cured_on_entry = |opponent_active: CardId| {
        let mut game = get_test_game_with_board(
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur).with_energy(vec![EnergyType::Grass])],
            vec![PlayedCard::from_id(opponent_active)],
        );
        let mut state = game.get_state_clone();
        state.apply_status_condition(0, 0, deckgym::models::StatusCondition::Poisoned);
        assert!(state.get_active(0).is_poisoned());

        let ogerpon = deckgym::database::get_card_by_enum(CardId::B2017TealMaskOgerponEx);
        state.hands[0].clear();
        state.hands[0].push(ogerpon.clone());
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Place(ogerpon, 1),
            is_stack: false,
        });

        !game.get_state_clone().get_active(0).is_poisoned()
    };

    assert!(
        cured_on_entry(CardId::A1053Squirtle),
        "Soothing Wind should normally cure the Poison when Ogerpon enters play"
    );
    assert!(
        !cured_on_entry(CardId::B2097AlolanMuk),
        "Power of Alchemy should remove Soothing Wind, so the Poison stays"
    );
}
