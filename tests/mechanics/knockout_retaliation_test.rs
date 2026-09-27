use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Player 0's Toucannon (140 HP, Colorless) uses Drill Peck (70 damage, [C]) into whatever
/// player 1 has in the Active Spot. Returns the resulting game.
fn toucannon_drill_peck_into(opponent_board: Vec<PlayedCard>) -> deckgym::Game<'static> {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A3135Toucannon).with_energy(vec![EnergyType::Colorless]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        opponent_board,
    );
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A3135Toucannon, 0),
        is_stack: false,
    });
    game
}

/// Pyukumuku's Innards Out: "If this Pokémon is in the Active Spot and is Knocked Out by damage
/// from an attack from your opponent's Pokémon, do 50 damage to the Attacking Pokémon."
#[test]
fn test_innards_out_damages_the_attacking_pokemon() {
    let game = toucannon_drill_peck_into(vec![
        PlayedCard::from_id(CardId::A3054Pyukumuku),
        PlayedCard::from_id(CardId::A1001Bulbasaur),
    ]);
    let state = game.get_state_clone();
    assert_eq!(
        state.in_play_pokemon[0][0]
            .as_ref()
            .expect("Toucannon should still be in play")
            .get_remaining_hp(),
        90,
        "Toucannon should have taken 50 damage from Innards Out"
    );
}

/// Innards Out only fires from the Active Spot: a Benched Pyukumuku knocked out by a spread
/// attack deals nothing back. Here Pyukumuku is Active but not Knocked Out, so no retaliation.
#[test]
fn test_innards_out_does_not_fire_without_a_knockout() {
    let game = toucannon_drill_peck_into(vec![
        PlayedCard::from_id(CardId::B1105Dusknoir),
        PlayedCard::from_id(CardId::A3054Pyukumuku),
    ]);
    let state = game.get_state_clone();
    assert_eq!(
        state.in_play_pokemon[0][0]
            .as_ref()
            .expect("Toucannon should still be in play")
            .get_remaining_hp(),
        140,
        "No Pyukumuku was Knocked Out in the Active Spot, so no damage comes back"
    );
}

/// Team Rocket's Electrode's Destiny Burst: same shape as Innards Out but for 70 damage.
#[test]
fn test_destiny_burst_damages_the_attacking_pokemon_for_70() {
    let game = toucannon_drill_peck_into(vec![
        PlayedCard::from_id(CardId::B4a020TeamRocketsElectrode),
        PlayedCard::from_id(CardId::A1001Bulbasaur),
    ]);
    let state = game.get_state_clone();
    assert_eq!(
        state.in_play_pokemon[0][0]
            .as_ref()
            .expect("Toucannon should still be in play")
            .get_remaining_hp(),
        70,
        "Toucannon should have taken 70 damage from Destiny Burst"
    );
}

/// Spiritomb's Final Scream: "…do 10 damage to each of your opponent's Pokémon." From the KO'd
/// Spiritomb's point of view, "your opponent" is the attacking player, so all of player 0's
/// in-play Pokémon take 10.
#[test]
fn test_final_scream_damages_every_attacking_side_pokemon() {
    let game = toucannon_drill_peck_into(vec![
        PlayedCard::from_id(CardId::B2103Spiritomb),
        PlayedCard::from_id(CardId::A1001Bulbasaur),
    ]);
    let state = game.get_state_clone();
    assert_eq!(
        state.in_play_pokemon[0][0]
            .as_ref()
            .expect("Toucannon should still be in play")
            .get_remaining_hp(),
        130
    );
    assert_eq!(
        state.in_play_pokemon[0][1]
            .as_ref()
            .expect("Benched Bulbasaur should still be in play")
            .get_remaining_hp(),
        60,
        "Final Scream hits the Bench too"
    );
}

/// Galarian Cursola's Perish Body: "If this Pokémon is in the Active Spot and is Knocked Out by
/// damage from an attack from your opponent's Pokémon, flip a coin. If heads, the Attacking
/// Pokémon is Knocked Out." Driven across seeds: sometimes Toucannon survives, sometimes it
/// goes down with Cursola.
#[test]
fn test_perish_body_can_knock_out_the_attacker() {
    let mut saw_attacker_knocked_out = false;
    let mut saw_attacker_survive = false;

    for seed in 0..40u64 {
        let mut game = deckgym::test_support::get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![
                PlayedCard::from_id(CardId::A3135Toucannon)
                    .with_energy(vec![EnergyType::Colorless]),
                PlayedCard::from_id(CardId::A1001Bulbasaur),
            ],
            vec![
                // 80 HP with 20 damage already on it: Drill Peck's 70 is lethal.
                PlayedCard::from_id(CardId::A4a035GalarianCursola).with_damage(20),
                PlayedCard::from_id(CardId::A1001Bulbasaur),
            ],
        );
        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A3135Toucannon, 0),
            is_stack: false,
        });

        let state = game.get_state_clone();
        let toucannon_alive = state.in_play_pokemon[0]
            .iter()
            .flatten()
            .any(|p| p.get_name() == "Toucannon");
        if toucannon_alive {
            saw_attacker_survive = true;
        } else {
            saw_attacker_knocked_out = true;
        }
    }

    assert!(
        saw_attacker_knocked_out,
        "expected at least one seed where Perish Body knocked out the attacker"
    );
    assert!(
        saw_attacker_survive,
        "expected at least one seed where the Perish Body coin came up tails"
    );
}

/// Glimmora's Shattering Crystal: "When this Pokémon is Knocked Out, flip a coin. If heads, your
/// opponent can't get any points for it."
#[test]
fn test_shattering_crystal_can_deny_the_knockout_point() {
    let mut saw_point_denied = false;
    let mut saw_point_scored = false;

    for seed in 0..40u64 {
        let mut game = deckgym::test_support::get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![
                PlayedCard::from_id(CardId::A3135Toucannon)
                    .with_energy(vec![EnergyType::Colorless]),
                PlayedCard::from_id(CardId::A1001Bulbasaur),
            ],
            vec![
                // 90 HP with 20 damage: Drill Peck's 70 is lethal (Glimmora is [D], Toucannon
                // is [C], so no weakness bonus).
                PlayedCard::from_id(CardId::B3a045Glimmora).with_damage(20),
                PlayedCard::from_id(CardId::A1001Bulbasaur),
            ],
        );
        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A3135Toucannon, 0),
            is_stack: false,
        });

        let state = game.get_state_clone();
        assert!(
            state.in_play_pokemon[1]
                .iter()
                .flatten()
                .all(|p| p.get_name() != "Glimmora"),
            "seed {seed}: Glimmora should be Knocked Out either way"
        );
        if state.points[0] == 0 {
            saw_point_denied = true;
        } else {
            assert_eq!(state.points[0], 1, "seed {seed}");
            saw_point_scored = true;
        }
    }

    assert!(
        saw_point_denied,
        "expected at least one seed where Shattering Crystal denied the point"
    );
    assert!(
        saw_point_scored,
        "expected at least one seed where the point was scored normally"
    );
}
