//! "Attacks used by the Defending Pokémon do -N damage" (Cubone's Growl, Bonsly's Teary Attack,
//! Clefable's Moonblast, ...): the cut belongs to the Pokémon that was hit, not to the Active Spot,
//! and it is an effect on the **Attacking** Pokémon, so it comes off in step 2 of the official
//! Detailed battle FAQ's damage order — before Weakness (see `damage_order_test`).
//!
//! Cubone, Growl: "During your opponent's next turn, attacks used by the Defending Pokémon do -20
//! damage."

use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
    Game,
};

fn act(game: &mut Game<'static>, actor: usize, action: SimpleAction) {
    game.apply_action(&Action {
        actor,
        action,
        is_stack: false,
    });
}

/// Cubone (60 HP, weak to [G]) uses Growl on the opponent's Active, then passes the turn.
fn growl_then_pass(opponent_board: Vec<PlayedCard>) -> Game<'static> {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1151Cubone).with_energy(vec![EnergyType::Colorless])],
        opponent_board,
    );
    act(&mut game, 0, attack_action(CardId::A1151Cubone, 0));
    act(&mut game, 0, SimpleAction::EndTurn);
    game
}

fn cubone_hp(game: &Game<'static>) -> u32 {
    game.get_state_clone().get_remaining_hp(0, 0)
}

/// Bulbasaur's Vine Whip (40) hits Cubone, which is weak to [G], with Bounded Field in play.
/// Growl's -20 is on the attacker, so it is inside what Weakness doubles: `(40 - 20) × 2 = 40`.
/// Counted after Weakness it would be `40 × 2 - 20 = 60`, which is all of Cubone's HP.
#[test]
fn test_growl_cut_comes_off_before_weakness() {
    let mut game = growl_then_pass(vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
        .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])]);
    let mut state = game.get_state_clone();
    state.set_active_stadium(get_card_by_enum(CardId::B3155BoundedField));
    game.set_state(state);

    act(&mut game, 1, attack_action(CardId::A1001Bulbasaur, 0));

    assert_eq!(
        cubone_hp(&game),
        60 - 40,
        "(40 - 20 from Growl) × 2 for Weakness under Bounded Field = 40 damage"
    );
}

/// The cut stays on the Pokémon that was hit. If the Defending Pokémon retreats, the Pokémon that
/// takes its place is not the one Growl was used on, so its attack is not reduced.
#[test]
fn test_growl_cut_does_not_follow_the_active_spot() {
    let mut game = growl_then_pass(vec![
        PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless]),
        PlayedCard::from_id(CardId::A1033Charmander).with_energy(vec![EnergyType::Fire]),
    ]);

    act(&mut game, 1, SimpleAction::Retreat(1));
    act(&mut game, 1, attack_action(CardId::A1033Charmander, 0));

    assert_eq!(
        cubone_hp(&game),
        60 - 30,
        "Charmander's Ember (30) was not hit by Growl, so it is not reduced"
    );
}

/// While the Pokémon that was hit stays in the Active Spot, all of its attacks are cut.
#[test]
fn test_growl_cut_reduces_the_defending_pokemons_attack() {
    let mut game = growl_then_pass(vec![
        PlayedCard::from_id(CardId::A1033Charmander).with_energy(vec![EnergyType::Fire])
    ]);

    act(&mut game, 1, attack_action(CardId::A1033Charmander, 0));

    assert_eq!(
        cubone_hp(&game),
        60 - 10,
        "Ember (30) used by the Defending Pokémon does -20 damage"
    );
}

/// Weedle's Sting (20) into Cubone, which is weak to [G]: Growl's -20 takes the attack to 0 in step 2.
/// An attack reduced to 0 does no damage, so Weakness has nothing to add to (not `0 + 20 = 20`).
/// Anything cut below 0 is floored to 0 first and does no damage either.
#[test]
fn test_growl_cut_to_zero_does_no_damage_even_against_weakness() {
    let mut game = growl_then_pass(vec![
        PlayedCard::from_id(CardId::A1008Weedle).with_energy(vec![EnergyType::Grass])
    ]);

    act(&mut game, 1, attack_action(CardId::A1008Weedle, 0));

    assert_eq!(
        cubone_hp(&game),
        60,
        "Sting 20 - Growl 20 = 0: no damage, so Weakness adds nothing"
    );
}
