//! Protections whose card text names "your opponent's" attacks (and Abilities) do nothing against
//! damage that comes from the holder's own side. The cards tested here say:
//!
//! - Protective Poncho: "As long as the Pokémon this card is attached to is on your Bench,
//!   prevent all damage done to that Pokémon by your opponent's attacks and Abilities."
//! - Eiscue's Ice Face: "If this Pokémon has full HP, it takes -40 damage from attacks from your
//!   opponent's Pokémon."
//! - Falinks's Coordinated Unit: "If you have another Falinks in play, this Pokémon's attacks do
//!   +20 damage to your opponent's Active Pokémon, and this Pokémon takes -20 damage from attacks
//!   from your opponent's Pokémon."
//! - Unown's GUARD: "This Ability works if you have any Unown in play with an Ability other than
//!   GUARD. All of your Pokémon take -10 damage from attacks from your opponent's Pokémon."
//!
//! Each test hits the first Benched Pokémon of player 0 once with damage from player 1 and once
//! with damage from player 0's own Active Pokémon, through `ApplyDamage` with
//! `is_from_active_attack` set (the same step an attack's damage goes through).

use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::PlayedCard,
    test_support::get_test_game_with_board,
};

/// Player 0's board is `active` plus `bench`. `attacker` (an Active Bulbasaur) belongs to
/// `attacking_player`; the other player's Active is a plain Bulbasaur. Applies `damage` from the
/// attacker to player 0's first Benched Pokémon, and returns how much HP that Pokémon lost.
fn bench_damage_taken(bench: Vec<PlayedCard>, attacking_player: usize, damage: u32) -> u32 {
    let mut player_0 = vec![PlayedCard::from_id(CardId::A1001Bulbasaur)];
    player_0.extend(bench);
    let player_1 = vec![PlayedCard::from_id(CardId::A1001Bulbasaur)];
    let mut game = get_test_game_with_board(player_0, player_1);
    let mut state = game.get_state_clone();
    state.current_player = attacking_player;
    game.set_state(state);

    let hp_before = game.get_state_clone().get_remaining_hp(0, 1);
    game.apply_action(&Action {
        actor: attacking_player,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (attacking_player, 0),
            targets: vec![(damage, 0, 1)],
            is_from_active_attack: true,
        },
        is_stack: false,
    });
    let state = game.get_state_clone();
    hp_before - state.get_remaining_hp(0, 1)
}

#[test]
fn test_protective_poncho_only_blocks_the_opponents_damage() {
    let poncho_bulbasaur = || {
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_tool(get_card_by_enum(CardId::B2147ProtectivePoncho))]
    };
    assert_eq!(
        bench_damage_taken(poncho_bulbasaur(), 1, 30),
        0,
        "Protective Poncho must still stop an opponent's attack on the Benched holder"
    );
    assert_eq!(
        bench_damage_taken(poncho_bulbasaur(), 0, 30),
        30,
        "Protective Poncho only stops the opponent's damage, not the owner's own"
    );
}

#[test]
fn test_ice_face_only_cuts_the_opponents_attacks() {
    let eiscue = || vec![PlayedCard::from_id(CardId::B1080Eiscue)];
    assert_eq!(
        bench_damage_taken(eiscue(), 1, 50),
        10,
        "Ice Face still takes -40 from an opponent's attack while at full HP"
    );
    assert_eq!(
        bench_damage_taken(eiscue(), 0, 50),
        50,
        "Ice Face does not cut damage from the owner's own attack"
    );
}

#[test]
fn test_coordinated_unit_only_cuts_the_opponents_attacks() {
    let falinks = || {
        vec![
            PlayedCard::from_id(CardId::B2092Falinks),
            PlayedCard::from_id(CardId::B2092Falinks),
        ]
    };
    assert_eq!(
        bench_damage_taken(falinks(), 1, 40),
        20,
        "Coordinated Unit still takes -20 from an opponent's attack"
    );
    assert_eq!(
        bench_damage_taken(falinks(), 0, 40),
        40,
        "Coordinated Unit does not cut damage from the owner's own attack"
    );
}

#[test]
fn test_unown_guard_only_cuts_the_opponents_attacks() {
    // GUARD works because Unown CHECK is also in play; it covers all of the owner's Pokémon.
    let unowns = || {
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A4084Unown),
            PlayedCard::from_id(CardId::A2a034Unown),
        ]
    };
    assert_eq!(
        bench_damage_taken(unowns(), 1, 30),
        20,
        "Unown GUARD still takes -10 from an opponent's attack"
    );
    assert_eq!(
        bench_damage_taken(unowns(), 0, 30),
        30,
        "Unown GUARD does not cut damage from the owner's own attack"
    );
}
