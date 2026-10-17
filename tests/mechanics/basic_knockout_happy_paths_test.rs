use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard, StatusCondition},
    state::GameOutcome,
    test_support::{attack_action, get_test_game_with_board},
};

fn end_turn(game: &mut deckgym::Game<'static>, actor: usize) {
    game.apply_action(&Action {
        actor,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    game.play_until_stable();
}

/// Baseline happy path: a plain attack that knocks out the opponent's Active Pokémon should
/// promote their Bench Pokémon and award points, leaving the new Active at full, undamaged HP.
#[test]
fn test_basic_attack_kos_active_and_promotes_bench() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
        vec![
            PlayedCard::from_id(CardId::A1033Charmander).with_remaining_hp(30),
            PlayedCard::from_id(CardId::A1189Rattata),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1001Bulbasaur, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_name(),
        "Rattata",
        "Rattata should have been promoted after Charmander (30 HP remaining) was KO'd by Vine Whip"
    );
    assert_eq!(
        state.get_remaining_hp(1, 0),
        40,
        "The promoted Rattata should be at full, undamaged HP"
    );
    assert_eq!(
        state.points[0], 1,
        "A regular knockout should award 1 point"
    );
    assert_eq!(state.winner, None, "The game should still be ongoing");
}

/// An attack whose own damage is lethal AND which also inflicts a status condition on the same
/// hit should still result in a clean knockout (the status-application code path shouldn't
/// interfere with, or short-circuit, the knockout check for the same attack).
#[test]
fn test_lethal_attack_that_also_inflicts_status_still_results_in_knockout() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B1036MegaBlazikenEx)
            .with_energy(vec![EnergyType::Fire, EnergyType::Fire])],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1189Rattata),
        ],
    );

    // Mega Burning: 120 damage (well past Bulbasaur's 70 HP) and burns the target — the target
    // dies from the attack's own damage, the Burned status is just along for the ride.
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B1036MegaBlazikenEx, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_name(),
        "Rattata",
        "Rattata should have been promoted after Bulbasaur was KO'd by the lethal, burn-inflicting attack"
    );
    assert_eq!(
        state.get_remaining_hp(1, 0),
        40,
        "The promoted Rattata should be untouched by the attack that KO'd Bulbasaur"
    );
    assert_eq!(
        state.points[0], 1,
        "A regular knockout should award 1 point"
    );
    assert_eq!(state.winner, None, "The game should still be ongoing");
}

/// Rocky Helmet's counterattack damage can knock out the attacker itself. If the attacker has no
/// Bench Pokémon left, that immediately ends the game for the defending player — this exercises
/// the *immediate* (non-deferred) knockout path used during attack resolution, which this PR's
/// end-of-turn/checkup changes should not have touched.
#[test]
fn test_rocky_helmet_counterattack_kos_attacker_and_ends_game() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])
            .with_remaining_hp(20)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_tool(get_card_by_enum(CardId::A2148RockyHelmet))],
    );

    // Vine Whip deals 40 to the Rocky Helmet holder (30 HP remaining, survives), but Rocky
    // Helmet's 20 counterattack damage knocks out the attacker (20 HP remaining) in return.
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1001Bulbasaur, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(
        state.winner,
        Some(GameOutcome::Win(1)),
        "Player 0's only Pokemon was KO'd by Rocky Helmet with no Bench to promote, ending the game"
    );
    assert_eq!(
        state.points[1], 1,
        "Player 1 should have scored a knockout point from the counterattack KO"
    );
}

/// Hoopa ex's Shadow Bullet (30 damage) followed by Deceptive Needle's end-of-turn 10 damage
/// should cleanly knock out a plain 40 HP Pokémon with no relevant abilities — the simplest case
/// the deferred end-of-turn knockout check needs to still get right.
#[test]
fn test_deceptive_needle_kos_plain_40_hp_pokemon_after_hoopa_ex_attack() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4103HoopaEx)
            .with_energy(vec![EnergyType::Darkness])
            .with_tool(get_card_by_enum(CardId::B4148DeceptiveNeedle))],
        vec![
            PlayedCard::from_id(CardId::A1189Rattata),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4103HoopaEx, 0),
        is_stack: false,
    });
    game.play_until_stable();

    assert_eq!(
        game.get_state_clone().get_remaining_hp(1, 0),
        10,
        "Shadow Bullet should leave Rattata (40 HP) at 10 remaining HP"
    );

    // TODO: attacking should really auto-sequence into ending the turn — the player shouldn't
    // have to separately decide to end turn after there's nothing left to do but that.
    end_turn(&mut game, 0);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_name(),
        "Bulbasaur",
        "Bulbasaur should have been promoted after Deceptive Needle's 10 damage knocked out Rattata"
    );
    assert_eq!(
        state.points[0], 1,
        "A regular knockout should award 1 point"
    );
    assert_eq!(state.winner, None, "The game should still be ongoing");
}

/// Sanity check in the other direction: a non-lethal attack should leave the defender as the
/// Active Pokémon, undamaged beyond the attack itself — nothing should get knocked out early.
#[test]
fn test_non_lethal_attack_does_not_knock_out_defender() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1001Bulbasaur, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_name(),
        "Charmander",
        "Charmander should survive a non-lethal Vine Whip and remain Active"
    );
    assert_eq!(
        state.get_remaining_hp(1, 0),
        20,
        "60 HP - 40 damage = 20 HP remaining"
    );
    assert_eq!(state.points[0], 0, "No knockout should have occurred");
    assert_eq!(state.winner, None);
}

/// Plain Burn damage during Pokémon Checkup, with no healing ability in play to save it, should
/// still cleanly knock out the Pokémon and promote the Bench — the basic case the deferred
/// checkup-knockout check needs to get right on its own, without any other effect involved.
#[test]
fn test_plain_burn_checkup_kos_without_any_saving_ability() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![
            PlayedCard::from_id(CardId::A1033Charmander).with_remaining_hp(20),
            PlayedCard::from_id(CardId::A1189Rattata),
        ],
    );
    let mut state = game.get_state_clone();
    state.apply_status_condition(1, 0, StatusCondition::Burned);
    game.set_state(state);

    end_turn(&mut game, 0);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_name(),
        "Rattata",
        "Rattata should have been promoted after Burn (20 damage) knocked out Charmander (20 HP remaining)"
    );
    assert_eq!(
        state.points[0], 1,
        "The opponent scores the point even though the knockout was from Burn, not an attack"
    );
    assert_eq!(state.winner, None);
}

/// Same as the Burn case above, but for Poison — the other checkup-damage status condition.
#[test]
fn test_plain_poison_checkup_kos_without_any_saving_ability() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![
            PlayedCard::from_id(CardId::A1033Charmander).with_remaining_hp(10),
            PlayedCard::from_id(CardId::A1189Rattata),
        ],
    );
    let mut state = game.get_state_clone();
    state.apply_status_condition(1, 0, StatusCondition::Poisoned);
    game.set_state(state);

    end_turn(&mut game, 0);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_name(),
        "Rattata",
        "Rattata should have been promoted after Poison (10 damage) knocked out Charmander (10 HP remaining)"
    );
    assert_eq!(
        state.points[0], 1,
        "The opponent scores the point even though the knockout was from Poison, not an attack"
    );
    assert_eq!(state.winner, None);
}
