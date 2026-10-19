//! Retreat Cost is read as it currently is, on the side of the Pokémon that has it.
//!
//! - Heavy Helmet: "If the Pokémon this card is attached to has a Retreat Cost of 3 or more, it
//!   takes -20 damage from attacks from your opponent's Pokémon."
//! - Peculiar Plaza: "The Retreat Cost of each [P] Pokémon in play (both yours and your
//!   opponent's) is 2 less."
//! - Ariados, Trap Territory: "Your opponent's Active Pokémon's Retreat Cost is 1 more."
//! - Tangrowth, Grass Knot: "This attack does 40 more damage for each Energy in your opponent's
//!   Active Pokémon's Retreat Cost."

use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
    Game,
};

fn with_helmet(card_id: CardId) -> PlayedCard {
    PlayedCard::from_id(card_id).with_tool(get_card_by_enum(CardId::B1219HeavyHelmet))
}

/// Player 0 attacks with Bulbasaur's Vine Whip (40 damage). Returns the HP the opponent's Active
/// loses. `attacker_side` is player 0's Bench, `holder_side` is player 1's board (Active first).
fn vine_whip_damage(
    attacker_side: Vec<PlayedCard>,
    holder_side: Vec<PlayedCard>,
    plaza: bool,
) -> u32 {
    let mut player_0 = vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
        .with_energy(vec![EnergyType::Grass, EnergyType::Grass])];
    player_0.extend(attacker_side);
    let mut game = get_test_game_with_board(player_0, holder_side);
    if plaza {
        let mut state = game.get_state_clone();
        state.set_active_stadium(get_card_by_enum(CardId::B2155PeculiarPlaza));
        game.set_state(state);
    }
    let before = game.get_state_clone().get_remaining_hp(1, 0);
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1001Bulbasaur, 0),
        is_stack: false,
    });
    game.play_until_stable();
    before - game.get_state_clone().get_remaining_hp(1, 0)
}

/// Slowbro has a printed Retreat Cost of 3 and is [P]; Peculiar Plaza takes it to 1, so the
/// Helmet does nothing.
#[test]
fn test_heavy_helmet_does_not_cut_when_peculiar_plaza_lowers_the_retreat_cost() {
    let holder = || vec![with_helmet(CardId::A1119Slowbro)];
    assert_eq!(vine_whip_damage(vec![], holder(), false), 20);
    assert_eq!(vine_whip_damage(vec![], holder(), true), 40);
}

/// Mewtwo has a printed Retreat Cost of 2; the opponent's Ariados makes it 3, so the Helmet cuts.
/// An Ariados on the holder's own side does not touch the holder's Retreat Cost.
#[test]
fn test_heavy_helmet_cuts_when_the_opponents_ariados_raises_the_retreat_cost_to_3() {
    let holder = || vec![with_helmet(CardId::A1128Mewtwo)];
    let ariados = || vec![PlayedCard::from_id(CardId::B1a006Ariados)];
    assert_eq!(vine_whip_damage(vec![], holder(), false), 40);
    assert_eq!(vine_whip_damage(ariados(), holder(), false), 20);

    let mut own_ariados = holder();
    own_ariados.extend(ariados());
    assert_eq!(vine_whip_damage(vec![], own_ariados, false), 40);
}

/// Trap Territory names the Active Pokémon, so it does not raise a Benched holder's cost.
#[test]
fn test_heavy_helmet_on_the_bench_ignores_ariados() {
    let mut player_0 = vec![
        PlayedCard::from_id(CardId::A1001Bulbasaur),
        PlayedCard::from_id(CardId::B1a006Ariados),
    ];
    let player_1 = vec![
        PlayedCard::from_id(CardId::A1001Bulbasaur),
        with_helmet(CardId::A1128Mewtwo),
    ];
    let mut game: Game<'static> = get_test_game_with_board(std::mem::take(&mut player_0), player_1);
    let before = game.get_state_clone().get_remaining_hp(1, 1);
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (0, 0),
            targets: vec![(40, 1, 1)],
            is_from_active_attack: true,
        },
        is_stack: false,
    });
    assert_eq!(before - game.get_state_clone().get_remaining_hp(1, 1), 40);
}

/// Grass Knot counts the defender's Retreat Cost: the attacker's Ariados adds 1 to it, the
/// defender's own Ariados does not. Whimsicott ex's printed cost is 1.
#[test]
fn test_grass_knot_counts_the_attackers_ariados_not_the_defenders() {
    let grass_knot_damage = |attacker_ariados: bool, defender_ariados: bool| {
        let mut player_0 = vec![
            PlayedCard::from_id(CardId::A4005Tangrowth).with_energy(vec![
                EnergyType::Grass,
                EnergyType::Grass,
                EnergyType::Grass,
            ]),
        ];
        let mut player_1 = vec![PlayedCard::from_id(CardId::B1016WhimsicottEx)];
        if attacker_ariados {
            player_0.push(PlayedCard::from_id(CardId::B1a006Ariados));
        }
        if defender_ariados {
            player_1.push(PlayedCard::from_id(CardId::B1a006Ariados));
        }
        let mut game = get_test_game_with_board(player_0, player_1);
        let before = game.get_state_clone().get_remaining_hp(1, 0);
        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A4005Tangrowth, 0),
            is_stack: false,
        });
        game.play_until_stable();
        before - game.get_state_clone().get_remaining_hp(1, 0)
    };
    assert_eq!(grass_knot_damage(false, false), 50);
    assert_eq!(grass_knot_damage(false, true), 50);
    assert_eq!(grass_knot_damage(true, false), 90);
}
