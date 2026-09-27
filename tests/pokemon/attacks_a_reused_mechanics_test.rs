//! Attacks whose effect text maps onto a `Mechanic` that already existed in the engine. The
//! interesting part of each of these is the mapping (and its parameters), so the tests drive the
//! attack through the public `Game` API and assert the observable board change.

use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
    Game,
};

fn attack(game: &mut Game<'static>, actor: usize, card_id: CardId, index: usize) {
    game.apply_action(&Action {
        actor,
        action: attack_action(card_id, index),
        is_stack: false,
    });
}

fn end_turn(game: &mut Game<'static>, actor: usize) {
    game.apply_action(&Action {
        actor,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
}

/// Musharna's Dream Dance puts BOTH Active Pokémon to sleep, not just the defender.
#[test]
fn test_musharna_dream_dance_sleeps_both_active_pokemon() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A4091Musharna)
            .with_energy(vec![EnergyType::Psychic, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::B4037WailordEx)],
    );

    attack(&mut game, 0, CardId::A4091Musharna, 0);

    let state = game.get_state_clone();
    assert!(
        state.get_active(0).is_asleep(),
        "Dream Dance should also put Musharna itself to sleep"
    );
    assert!(
        state.get_active(1).is_asleep(),
        "Dream Dance should put the Defending Pokémon to sleep"
    );
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        250 - 60,
        "Dream Dance still deals its 60 damage"
    );
}

/// Aurorus' Hail Prison discards exactly 2 [W] Energy from itself and paralyzes the defender.
#[test]
fn test_aurorus_hail_prison_discards_two_water_and_paralyzes() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2042Aurorus).with_energy(vec![
            EnergyType::Water,
            EnergyType::Water,
            EnergyType::Water,
        ])],
        vec![PlayedCard::from_id(CardId::B4037WailordEx)],
    );

    attack(&mut game, 0, CardId::B2042Aurorus, 0);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(0).attached_energy,
        vec![EnergyType::Water],
        "exactly 2 of the 3 attached [W] Energy should be discarded"
    );
    assert!(
        state.get_active(1).is_paralyzed(),
        "the Defending Pokémon is Paralyzed"
    );
}

/// Purrloin's Playful Knockdown discards every Tool from the opponent's Active Pokémon.
#[test]
fn test_purrloin_playful_knockdown_discards_opponent_active_tool() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A3b045Purrloin).with_energy(vec![EnergyType::Darkness])],
        vec![PlayedCard::from_id(CardId::B4037WailordEx)
            .with_tool(get_card_by_enum(CardId::A2148RockyHelmet))],
    );
    let discard_before = game.get_state_clone().discard_piles[1].len();

    attack(&mut game, 0, CardId::A3b045Purrloin, 0);

    let state = game.get_state_clone();
    assert!(
        state.get_active(1).attached_tool.is_none(),
        "the Tool should be gone from the Defending Pokémon"
    );
    assert_eq!(
        state.discard_piles[1].len(),
        discard_before + 1,
        "the discarded Tool lands in its owner's discard pile"
    );
}

/// Oranguru's Primate's Trap raises the Defending Pokémon's Retreat Cost by 1 [C], so a
/// Bulbasaur holding exactly 1 Energy (retreat cost 1) can no longer retreat.
#[test]
fn test_oranguru_primates_trap_raises_defender_retreat_cost() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A3140Oranguru)
            .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless])],
        vec![
            PlayedCard::from_id(CardId::B4037WailordEx),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
    );

    // Wailord ex's Retreat Cost is 4 [C]; give it exactly 4 so it can retreat today but not
    // after the attack raises the cost to 5.
    let mut state = game.get_state_clone();
    state.in_play_pokemon[1][0]
        .as_mut()
        .unwrap()
        .attached_energy = vec![EnergyType::Colorless; 4];
    game.set_state(state);

    attack(&mut game, 0, CardId::A3140Oranguru, 0);
    end_turn(&mut game, 0);

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 1);
    assert!(
        !choices
            .iter()
            .any(|choice| matches!(choice.action, SimpleAction::Retreat(_))),
        "Primate's Trap should push the Retreat Cost out of reach of 4 attached Energy"
    );
}

/// Team Rocket's Tinkaton's Pile-Driving Hammer is the same effect with 2 [C] instead of 1 [C]:
/// the Defending Pokémon can no longer pay for an attack it could otherwise afford.
#[test]
fn test_tinkaton_pile_driving_hammer_raises_defender_attack_cost() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B4a050TeamRocketsTinkaton).with_energy(vec![
                EnergyType::Metal,
                EnergyType::Metal,
                EnergyType::Colorless,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
    );

    attack(&mut game, 0, CardId::B4a050TeamRocketsTinkaton, 0);
    end_turn(&mut game, 0);

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 1);
    assert!(
        !choices
            .iter()
            .any(|choice| matches!(choice.action, SimpleAction::Attack(_))),
        "Vine Whip costs 2 Energy, and +2 [C] puts it out of reach of Bulbasaur's 2 Energy"
    );
}

/// Hariyama's Pivot Throw leaves the +50 vulnerability on Hariyama itself, so the opponent's
/// next attack hits it 50 harder.
#[test]
fn test_hariyama_pivot_throw_leaves_self_vulnerability() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4080Hariyama).with_energy(vec![
            EnergyType::Fighting,
            EnergyType::Fighting,
            EnergyType::Colorless,
        ])],
        // Blastoise ex survives Pivot Throw's 120, and its Surf (40) is not boosted by
        // Hariyama's [P] Weakness.
        vec![PlayedCard::from_id(CardId::A1056BlastoiseEx)
            .with_energy(vec![EnergyType::Water, EnergyType::Colorless])],
    );

    attack(&mut game, 0, CardId::B4080Hariyama, 0);
    end_turn(&mut game, 0);
    // Surf normally does 40; Hariyama (120 HP) should instead take 90.
    attack(&mut game, 1, CardId::A1056BlastoiseEx, 0);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(0).get_remaining_hp(),
        120 - 90,
        "Pivot Throw's drawback lands on Hariyama, adding 50 to the incoming 40"
    );
}

/// Espathra's Lumina Crash is the mirror image: the +50 lands on the DEFENDER, and applies on
/// Espathra's own next turn.
#[test]
fn test_espathra_lumina_crash_boosts_its_own_next_turn_damage() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B3a022Espathra)
            .with_energy(vec![EnergyType::Psychic, EnergyType::Psychic])],
        vec![PlayedCard::from_id(CardId::B4037WailordEx)],
    );

    attack(&mut game, 0, CardId::B3a022Espathra, 0);
    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        250 - 50,
        "the first Lumina Crash deals its plain 50"
    );

    end_turn(&mut game, 0);
    end_turn(&mut game, 1);
    attack(&mut game, 0, CardId::B3a022Espathra, 0);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        250 - 50 - 100,
        "on the next turn the Defending Pokémon takes 50 more from the same attack"
    );
}

/// Revavroom's Overacceleration boosts its own next Overacceleration by 70.
#[test]
fn test_revavroom_overacceleration_boosts_its_own_next_use() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B4115Revavroom).with_energy(vec![
                EnergyType::Metal,
                EnergyType::Metal,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::B4037WailordEx)],
    );

    attack(&mut game, 0, CardId::B4115Revavroom, 0);
    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        250 - 70
    );

    end_turn(&mut game, 0);
    end_turn(&mut game, 1);
    attack(&mut game, 0, CardId::B4115Revavroom, 0);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        250 - 70 - 140,
        "the second Overacceleration should do 70 + 70"
    );
}

/// Team Rocket's Sneasel's Group Beatdown flips one coin per Pokémon the attacker has in play
/// and deals 30 per heads — so with a full board the damage is a multiple of 30, capped at 4
/// heads, and is sometimes 0.
#[test]
fn test_team_rockets_sneasel_group_beatdown_scales_with_own_board() {
    let mut saw_zero = false;
    let mut saw_damage = false;

    for seed in 0..20 {
        let mut game = deckgym::test_support::get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![
                PlayedCard::from_id(CardId::B4a044TeamRocketsSneasel)
                    .with_energy(vec![EnergyType::Darkness, EnergyType::Colorless]),
                PlayedCard::from_id(CardId::A1033Charmander),
                PlayedCard::from_id(CardId::A1001Bulbasaur),
            ],
            vec![PlayedCard::from_id(CardId::B4037WailordEx)],
        );

        attack(&mut game, 0, CardId::B4a044TeamRocketsSneasel, 0);

        let damage = 250 - game.get_state_clone().get_active(1).get_remaining_hp();
        assert_eq!(damage % 30, 0, "damage should be a multiple of 30");
        assert!(damage <= 90, "at most 3 heads with 3 Pokémon in play");
        if damage == 0 {
            saw_zero = true;
        } else {
            saw_damage = true;
        }
    }

    assert!(saw_zero, "all-tails should sometimes deal nothing");
    assert!(saw_damage, "heads should sometimes deal damage");
}
