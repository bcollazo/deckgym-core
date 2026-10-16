//! Effects whose card text limits them to damage "from attacks" leave every other kind of damage
//! alone.
//!
//! Official **Detailed battle FAQ**, "Why didn't Mimikyu ex's Disguise Ability prevent damage?":
//! Disguise does not prevent damage that doesn't come from attacks. Damage from Special
//! Conditions (such as Poisoned or Burned), from Abilities (such as Greninja's Water Shuriken)
//! and from Tools (such as Rocky Helmet) all get through. The same reading applies to any
//! reduction or prevention whose text names attacks. The cards tested here say:
//!
//! - Heavy Helmet: "it takes -20 damage from attacks from your opponent's Pokémon".
//! - Cascoon's Harden: "prevent all damage done to this Pokémon by attacks if that damage is 40
//!   or less".
//! - Shinx's Hide: "prevent all damage from—and effects of—attacks done to this Pokémon".
//! - Carracosta's Blocking Shell: "Prevent all damage done to this Pokémon by attacks from Basic
//!   Pokémon".
//!
//! Steel Apron and Metal Core Barrier ("takes -10 / -50 damage from attacks from your opponent's
//! Pokémon") already behave this way; the last test pins that down so this file also shows they
//! are unchanged.
//!
//! The non-attack damage used below is Poison (10) and Burn (20) at Pokémon Checkup, Darkrai's
//! Bad Dreams (20, an Ability) at the end of the turn, and Greninja's Water Shuriken ("Once during
//! your turn, you may do 20 damage to 1 of your opponent's Pokémon", also an Ability).

use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    effects::CardEffect,
    models::{EnergyType, PlayedCard, StatusCondition},
    test_support::{attack_action, get_test_game_with_board},
    Game,
};

const POISON_DAMAGE: u32 = 10;
const BURN_DAMAGE: u32 = 20;
const BAD_DREAMS_DAMAGE: u32 = 20;

fn end_turn(game: &mut Game<'static>, actor: usize) {
    game.apply_action(&Action {
        actor,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
}

/// Player 1 uses the first attack of their Active `attacker` against player 0's Active.
fn opponent_attacks(game: &mut Game<'static>, attacker: CardId) {
    let mut state = game.get_state_clone();
    state.current_player = 1;
    game.set_state(state);
    game.apply_action(&Action {
        actor: 1,
        action: attack_action(attacker, 0),
        is_stack: false,
    });
}

/// Player 0's Active `holder` faces player 1's Darkrai, is given `conditions`, and player 0 ends
/// their turn. Returns how much damage the holder took over that end of turn: Poison and Burn
/// at Pokémon Checkup, and Bad Dreams if it is Asleep.
fn damage_over_end_of_turn(holder: PlayedCard, conditions: &[StatusCondition]) -> u32 {
    let mut game = get_test_game_with_board(
        vec![holder],
        vec![PlayedCard::from_id(CardId::B2b040Darkrai)],
    );
    let mut state = game.get_state_clone();
    for condition in conditions {
        state.apply_status_condition(0, 0, *condition);
    }
    game.set_state(state);

    let hp_before = game.get_state_clone().get_remaining_hp(0, 0);
    end_turn(&mut game, 0);
    hp_before - game.get_state_clone().get_remaining_hp(0, 0)
}

/// Poison, Burn and Bad Dreams each go through whatever protection `holder` carries.
fn assert_non_attack_damage_goes_through(holder: impl Fn() -> PlayedCard, protection: &str) {
    assert_eq!(
        damage_over_end_of_turn(holder(), &[StatusCondition::Poisoned]),
        POISON_DAMAGE,
        "{protection} only covers attacks, so it must not stop the Poison damage at Checkup"
    );
    assert_eq!(
        damage_over_end_of_turn(holder(), &[StatusCondition::Burned]),
        BURN_DAMAGE,
        "{protection} only covers attacks, so it must not stop the Burn damage at Checkup"
    );
    assert_eq!(
        damage_over_end_of_turn(holder(), &[StatusCondition::Asleep]),
        BAD_DREAMS_DAMAGE,
        "{protection} only covers attacks, so it must not stop Bad Dreams (an Ability)"
    );
}

#[test]
fn test_heavy_helmet_cuts_an_opponents_attack_but_not_other_damage() {
    // Snorlax's Retreat Cost is 4, so Heavy Helmet's -20 is switched on.
    let helmet_snorlax = || {
        PlayedCard::from_id(CardId::A1211Snorlax)
            .with_tool(get_card_by_enum(CardId::B1219HeavyHelmet))
    };

    // An opponent's attack is still cut by 20: Vine Whip does 40, so Snorlax takes 20.
    let mut game = get_test_game_with_board(
        vec![helmet_snorlax()],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
    );
    opponent_attacks(&mut game, CardId::A1001Bulbasaur);
    assert_eq!(
        game.get_state_clone().get_remaining_hp(0, 0),
        150 - 20,
        "Heavy Helmet should still take 20 off a 40-damage attack from the opponent"
    );

    // Poison, Burn and Bad Dreams are not damage from attacks, so they are not cut.
    assert_non_attack_damage_goes_through(helmet_snorlax, "Heavy Helmet");

    // "from attacks from your opponent's Pokémon": damage an attack of yours does to your own
    // Benched Pokémon is not cut either.
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            helmet_snorlax(),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (0, 0),
            targets: vec![(30, 0, 1)],
            is_from_active_attack: true,
        },
        is_stack: false,
    });
    assert_eq!(
        game.get_state_clone().get_remaining_hp(0, 1),
        150 - 30,
        "Heavy Helmet only cuts attacks from the opponent's Pokémon, not the player's own"
    );
}

#[test]
fn test_heavy_helmet_does_not_cut_greninjas_water_shuriken() {
    // Water Shuriken is an Ability, not an attack, so the full 20 lands on a Heavy Helmet holder
    // (Snorlax's Retreat Cost is 4).
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1089Greninja),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)
            .with_tool(get_card_by_enum(CardId::B1219HeavyHelmet))],
    );
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 1 },
        is_stack: false,
    });

    // The Ability now asks which of the opponent's Pokémon to hit; choose Snorlax.
    let (_actor, choices) = game.get_state_clone().generate_possible_actions();
    let hit_snorlax = choices
        .into_iter()
        .find(|choice| {
            matches!(
                &choice.action,
                SimpleAction::ApplyDamage { targets, .. } if targets.as_slice() == [(20, 1, 0)]
            )
        })
        .expect("Water Shuriken should offer 20 damage to the opponent's Active");
    game.apply_action(&hit_snorlax);

    assert_eq!(
        game.get_state_clone().get_remaining_hp(1, 0),
        150 - 20,
        "Water Shuriken is an Ability, so Heavy Helmet must not take anything off its 20 damage"
    );
}

#[test]
fn test_harden_prevents_attack_damage_but_not_other_damage() {
    let hardened_cascoon = || {
        let mut cascoon = PlayedCard::from_id(CardId::B1006Cascoon);
        cascoon.add_effect(CardEffect::PreventDamageIfLessOrEqual { threshold: 40 }, 1);
        cascoon
    };

    // Attack damage of 40 or less is still prevented: Vine Whip does 40.
    let mut game = get_test_game_with_board(
        vec![hardened_cascoon()],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
    );
    opponent_attacks(&mut game, CardId::A1001Bulbasaur);
    assert_eq!(
        game.get_state_clone().get_remaining_hp(0, 0),
        80,
        "Harden should still prevent a 40-damage attack"
    );

    // Poison, Burn and Bad Dreams are all 40 or less, but they are not damage by attacks.
    assert_non_attack_damage_goes_through(hardened_cascoon, "Harden");
}

#[test]
fn test_hide_prevents_attack_damage_but_not_other_damage() {
    let hidden_shinx = || {
        let mut shinx = PlayedCard::from_id(CardId::A2058Shinx);
        shinx.add_effect(CardEffect::PreventAllDamageAndEffects, 1);
        shinx
    };

    // All damage from an attack is still prevented.
    let mut game = get_test_game_with_board(
        vec![hidden_shinx()],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
    );
    opponent_attacks(&mut game, CardId::A1001Bulbasaur);
    assert_eq!(
        game.get_state_clone().get_remaining_hp(0, 0),
        60,
        "Hide should still prevent all damage from an attack"
    );

    // Poison, Burn and Bad Dreams are not attacks, so Hide does not stop them.
    assert_non_attack_damage_goes_through(hidden_shinx, "Hide");
}

#[test]
fn test_blocking_shell_prevents_basic_attack_damage_but_not_ability_damage() {
    let shielded_carracosta = || {
        let mut carracosta = PlayedCard::from_id(CardId::B1067Carracosta);
        carracosta.add_effect(CardEffect::PreventDamageFromBasic, 1);
        carracosta
    };

    // Damage from a Basic Pokémon's attack is still prevented (Mewtwo ex's Psychic Sphere: 50).
    let mut game = get_test_game_with_board(
        vec![shielded_carracosta()],
        vec![PlayedCard::from_id(CardId::A1129MewtwoEx)
            .with_energy(vec![EnergyType::Psychic, EnergyType::Colorless])],
    );
    opponent_attacks(&mut game, CardId::A1129MewtwoEx);
    assert_eq!(
        game.get_state_clone().get_remaining_hp(0, 0),
        150,
        "Blocking Shell should still prevent all damage from a Basic Pokémon's attack"
    );

    // Darkrai is a Basic Pokémon, but Bad Dreams is an Ability, not an attack. (Carracosta is a
    // Stage 2, so its own Poison and Burn were never "from a Basic Pokémon"; Bad Dreams is the
    // damage this wording can wrongly catch.)
    assert_eq!(
        damage_over_end_of_turn(shielded_carracosta(), &[StatusCondition::Asleep]),
        BAD_DREAMS_DAMAGE,
        "Blocking Shell only covers attacks, so it must not stop Bad Dreams (an Ability)"
    );
}

#[test]
fn test_steel_apron_and_metal_core_barrier_are_unchanged() {
    // Metal Core Barrier: "The [M] Pokémon this card is attached to takes -50 damage from
    // attacks from your opponent's Pokémon."
    let barrier_registeel = || {
        PlayedCard::from_id(CardId::A2112Registeel)
            .with_tool(get_card_by_enum(CardId::B2148MetalCoreBarrier))
    };
    let mut game = get_test_game_with_board(
        vec![barrier_registeel()],
        vec![PlayedCard::from_id(CardId::A1211Snorlax).with_energy(vec![EnergyType::Colorless; 4])],
    );
    opponent_attacks(&mut game, CardId::A1211Snorlax); // Rollout, 70
    assert_eq!(
        game.get_state_clone().get_remaining_hp(0, 0),
        110 - (70 - 50),
        "Metal Core Barrier should still take 50 off an opponent's attack"
    );
    assert_eq!(
        damage_over_end_of_turn(barrier_registeel(), &[StatusCondition::Poisoned]),
        POISON_DAMAGE,
        "Metal Core Barrier should still leave Poison damage alone"
    );

    // Steel Apron: "The [M] Pokémon this card is attached to takes -10 damage from attacks from
    // your opponent's Pokémon". It also stops Special Conditions, so the non-attack damage here
    // is Deceptive Needle's 10 at the end of its owner's turn.
    let apron_meltan = || {
        PlayedCard::from_id(CardId::A1181Meltan)
            .with_tool(get_card_by_enum(CardId::A4153SteelApron))
    };
    let mut game = get_test_game_with_board(
        vec![apron_meltan()],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
    );
    opponent_attacks(&mut game, CardId::A1001Bulbasaur); // Vine Whip, 40
    assert_eq!(
        game.get_state_clone().get_remaining_hp(0, 0),
        60 - (40 - 10),
        "Steel Apron should still take 10 off an opponent's attack"
    );

    let mut game = get_test_game_with_board(
        vec![apron_meltan()],
        vec![PlayedCard::from_id(CardId::A2096Murkrow)
            .with_tool(get_card_by_enum(CardId::B4148DeceptiveNeedle))],
    );
    let mut state = game.get_state_clone();
    state.current_player = 1;
    game.set_state(state);
    end_turn(&mut game, 1);
    assert_eq!(
        game.get_state_clone().get_remaining_hp(0, 0),
        60 - 10,
        "Steel Apron should still leave Deceptive Needle's damage alone"
    );
}
