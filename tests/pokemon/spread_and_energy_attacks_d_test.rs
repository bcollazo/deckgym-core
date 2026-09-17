use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Forretress - Enormous Explosion: 100 to the Defending Pokémon, 100 to itself, and 50 to every
/// Benched Pokémon on both sides.
#[test]
fn test_forretress_enormous_explosion_hits_both_benches_and_itself() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B2b046Forretress).with_energy(vec![
                EnergyType::Metal,
                EnergyType::Metal,
                EnergyType::Metal,
            ]),
            PlayedCard::from_id(CardId::A1211Snorlax),
        ],
        vec![
            PlayedCard::from_id(CardId::A1211Snorlax),
            PlayedCard::from_id(CardId::A1211Snorlax),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2b046Forretress, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 50);
    assert_eq!(
        state.in_play_pokemon[1][1]
            .as_ref()
            .expect("opponent bench")
            .get_remaining_hp(),
        100
    );
    // Forretress has 100 HP and takes 100 to itself, so it knocks itself out and the Benched
    // Snorlax — already down 50 from the same explosion — is promoted into the Active Spot.
    assert_eq!(state.get_active(0).get_name(), "Snorlax");
    assert_eq!(state.get_active(0).get_remaining_hp(), 100);
}

/// Mimikyu - Shadow Hit: 60 to the Defending Pokémon, plus 20 to one of YOUR OWN Pokémon — the
/// attacker itself is a legal target.
#[test]
fn test_mimikyu_shadow_hit_can_damage_any_of_your_own_pokemon() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A3083Mimikyu)
                .with_energy(vec![EnergyType::Psychic, EnergyType::Colorless]),
            PlayedCard::from_id(CardId::A1211Snorlax),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A3083Mimikyu, 0),
        is_stack: false,
    });

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert_eq!(choices.len(), 2, "Mimikyu itself and its Bench are targets");

    // Pick the Benched Snorlax (index 1), leaving Mimikyu untouched.
    let bench_choice = choices
        .iter()
        .find(|choice| match &choice.action {
            SimpleAction::ApplyDamage { targets, .. } => targets[0].2 == 1,
            _ => false,
        })
        .expect("the Bench should be a target")
        .clone();
    game.apply_action(&bench_choice);

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 90);
    assert_eq!(state.get_active(0).get_remaining_hp(), 70);
    assert_eq!(
        state.in_play_pokemon[0][1]
            .as_ref()
            .expect("own bench")
            .get_remaining_hp(),
        130
    );
}

/// Archeops - Wild Spin: 20 to each of the opponent's Pokémon, and +20 to each on the next turn.
#[test]
fn test_archeops_wild_spin_spreads_damage_and_escalates_next_turn() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B1134Archeops).with_energy(vec![EnergyType::Fighting])],
        vec![
            // Venusaur ex is Weak to [R], not [F], so no weakness math interferes.
            PlayedCard::from_id(CardId::A1004VenusaurEx),
            PlayedCard::from_id(CardId::A1004VenusaurEx),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B1134Archeops, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 170);
    assert_eq!(
        state.in_play_pokemon[1][1]
            .as_ref()
            .expect("opponent bench")
            .get_remaining_hp(),
        170
    );

    // Next use of Wild Spin does 40 to each.
    let mut state = game.get_state_clone();
    state.current_player = 0;
    game.set_state(state);
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B1134Archeops, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 130);
    assert_eq!(
        state.in_play_pokemon[1][1]
            .as_ref()
            .expect("opponent bench")
            .get_remaining_hp(),
        130
    );
}

/// Uxie - Mind Boost: attach a [P] Energy from the Energy Zone to Mesprit or Azelf only.
#[test]
fn test_uxie_mind_boost_only_offers_mesprit_or_azelf() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A2075Uxie).with_energy(vec![EnergyType::Psychic]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A2076Mesprit),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A2075Uxie, 0),
        is_stack: false,
    });

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert_eq!(choices.len(), 1, "only Mesprit is an eligible target");
    game.apply_action(&choices[0].clone());

    let state = game.get_state_clone();
    assert_eq!(
        state.in_play_pokemon[0][2]
            .as_ref()
            .expect("Mesprit")
            .attached_energy,
        vec![EnergyType::Psychic]
    );
    assert_eq!(state.get_active(1).get_remaining_hp(), 130);
}

/// Sableye - Jeweled Gift: a random basic Energy is attached to a chosen Benched Pokémon.
#[test]
fn test_sableye_jeweled_gift_attaches_one_random_energy_to_a_benched_pokemon() {
    for seed in 0..10 {
        let mut game = deckgym::test_support::get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![
                PlayedCard::from_id(CardId::B3a040Sableye).with_energy(vec![EnergyType::Colorless]),
                PlayedCard::from_id(CardId::A1001Bulbasaur),
            ],
            vec![PlayedCard::from_id(CardId::A1211Snorlax)],
        );

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::B3a040Sableye, 0),
            is_stack: false,
        });

        let (_, choices) = game.get_state_clone().generate_possible_actions();
        assert_eq!(choices.len(), 1, "seed {seed}: only one Benched Pokémon");
        game.apply_action(&choices[0].clone());

        let state = game.get_state_clone();
        let bench = state.in_play_pokemon[0][1].as_ref().expect("bench");
        assert_eq!(
            bench.attached_energy.len(),
            1,
            "seed {seed}: exactly one Energy attached"
        );
        assert_ne!(
            bench.attached_energy[0],
            EnergyType::Colorless,
            "seed {seed}: Jeweled Gift only produces the 8 basic types"
        );
    }
}

/// Gyarados - Wild Swing: 20 damage, +40 for each Benched [W] Pokémon discarded.
#[test]
fn test_gyarados_wild_swing_boosts_damage_per_discarded_water_bench() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A4045Gyarados)
                .with_energy(vec![EnergyType::Water, EnergyType::Water]),
            PlayedCard::from_id(CardId::A1211Snorlax), // [C], not eligible
            PlayedCard::from_id(CardId::B4032Staryu),  // [W], eligible
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A4045Gyarados, 0),
        is_stack: false,
    });

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    // One eligible [W] Bench member => discard none, or discard it.
    assert_eq!(choices.len(), 2);

    let discard_one = choices
        .iter()
        .find(|choice| match &choice.action {
            SimpleAction::DiscardOwnBenchedGroupThenDamage {
                in_play_indices, ..
            } => in_play_indices.len() == 1,
            _ => false,
        })
        .expect("discarding the Benched Staryu should be offered")
        .clone();
    game.apply_action(&discard_one);
    game.play_until_stable();

    let state = game.get_state_clone();
    // 20 + 40 = 60
    assert_eq!(state.get_active(1).get_remaining_hp(), 90);
    assert!(
        state.in_play_pokemon[0][2].is_none(),
        "Staryu was discarded"
    );
    assert!(state.in_play_pokemon[0][1].is_some(), "Snorlax stays");
}

/// Wild Swing with nothing discarded deals only its base damage.
#[test]
fn test_gyarados_wild_swing_can_decline_the_discard() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A4045Gyarados)
                .with_energy(vec![EnergyType::Water, EnergyType::Water]),
            PlayedCard::from_id(CardId::B4032Staryu),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A4045Gyarados, 0),
        is_stack: false,
    });

    let (_, choices) = game.get_state_clone().generate_possible_actions();
    let discard_none = choices
        .iter()
        .find(|choice| match &choice.action {
            SimpleAction::DiscardOwnBenchedGroupThenDamage {
                in_play_indices, ..
            } => in_play_indices.is_empty(),
            _ => false,
        })
        .expect("discarding nothing should be offered")
        .clone();
    game.apply_action(&discard_none);
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 130);
    assert!(state.in_play_pokemon[0][1].is_some(), "Staryu stays");
}
