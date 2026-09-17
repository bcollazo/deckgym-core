use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Kingambit - Overlord's Blade: 60 damage, +40 for each of your own Pokémon Knocked Out this game.
#[test]
fn test_kingambit_overlords_blade_scales_with_your_own_knockouts() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B3a043Kingambit)
            .with_energy(vec![EnergyType::Darkness, EnergyType::Darkness])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    // No losses yet: the printed 60.
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3a043Kingambit, 0),
        is_stack: false,
    });
    game.play_until_stable();
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 90);

    // Give player 0 a Benched Pokémon and knock it out, then attack again.
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.in_play_pokemon[0][1] =
        Some(PlayedCard::from_id(CardId::A1001Bulbasaur).with_remaining_hp(10));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (1, 0),
            targets: vec![(50, 0, 1)],
            is_from_active_attack: false,
        },
        is_stack: false,
    });

    let mut state = game.get_state_clone();
    state.current_player = 0;
    game.set_state(state);
    assert!(
        game.get_state_clone().in_play_pokemon[0][1].is_none(),
        "the Benched Bulbasaur should have been knocked out"
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3a043Kingambit, 0),
        is_stack: false,
    });

    // 60 + 1 * 40 = 100
    assert_eq!(game.get_state_clone().in_play_pokemon[1][0], None);
}

/// Hisuian Basculegion - Soul Counter: 50 damage, +50 for each point the opponent got last turn.
#[test]
fn test_hisuian_basculegion_soul_counter_without_opponent_points() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4a018HisuianBasculegion)
            .with_energy(vec![EnergyType::Water, EnergyType::Water])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4a018HisuianBasculegion, 0),
        is_stack: false,
    });
    game.play_until_stable();

    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 100);
}

/// The opponent scoring a point during their turn boosts the next Soul Counter by 50.
#[test]
fn test_hisuian_basculegion_soul_counter_scales_with_points_scored_last_turn() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B4a018HisuianBasculegion)
                .with_energy(vec![EnergyType::Water, EnergyType::Water])
                .with_remaining_hp(10),
            PlayedCard::from_id(CardId::B4a018HisuianBasculegion)
                .with_energy(vec![EnergyType::Water, EnergyType::Water]),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    // Hand the turn to the opponent, who knocks out the Active Basculegion for a point.
    let mut state = game.get_state_clone();
    state.current_player = 1;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (1, 0),
            targets: vec![(50, 0, 0)],
            is_from_active_attack: true,
        },
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(state.points[1], 1, "the opponent should have scored");

    // The opponent ends their turn, which closes their point ledger for that turn.
    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    game.play_until_stable();
    assert_eq!(game.get_state_clone().current_player, 0);

    // Back on our turn, Soul Counter is 50 + 1 * 50 = 100.
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4a018HisuianBasculegion, 0),
        is_stack: false,
    });

    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 50);
}

/// Toxicroak - Toxic: the Poison it inflicts deals 20 at Checkup instead of the usual 10.
#[test]
fn test_toxicroak_toxic_poison_deals_twenty_at_checkup() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A2a052Toxicroak).with_energy(vec![EnergyType::Darkness])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A2a052Toxicroak, 0),
        is_stack: false,
    });
    let state = game.get_state_clone();
    assert!(state.get_active(1).is_poisoned());
    // Toxic itself does no damage.
    assert_eq!(state.get_active(1).get_remaining_hp(), 150);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });

    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 130);
}

/// Toxapex - Severe Poison: 40 at Checkup instead of 10.
#[test]
fn test_toxapex_severe_poison_deals_forty_at_checkup() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B3b047ToxapEx)
            .with_energy(vec![EnergyType::Darkness, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3b047ToxapEx, 0),
        is_stack: false,
    });
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });

    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 110);
}

/// An ordinary Poison applied later resets the Checkup damage back to 10.
#[test]
fn test_ordinary_poison_after_severe_poison_is_back_to_ten() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B3b047ToxapEx)
            .with_energy(vec![EnergyType::Darkness, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3b047ToxapEx, 0),
        is_stack: false,
    });

    // Re-apply Poison the ordinary way before Checkup runs.
    let mut state = game.get_state_clone();
    state.apply_status_condition(1, 0, deckgym::models::StatusCondition::Poisoned);
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });

    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 140);
}

/// Mesprit - Supreme Blast: unusable unless Uxie AND Azelf are on the Bench.
#[test]
fn test_mesprit_supreme_blast_requires_uxie_and_azelf_on_the_bench() {
    let game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A2076Mesprit).with_energy(vec![
                EnergyType::Psychic,
                EnergyType::Psychic,
                EnergyType::Psychic,
            ]),
            PlayedCard::from_id(CardId::A2075Uxie),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    let (_, choices) = game.get_state_clone().generate_possible_actions();
    assert!(
        !choices.iter().any(|choice| matches!(
            &choice.action,
            SimpleAction::Attack(attack) if attack.title == "Supreme Blast"
        )),
        "Supreme Blast must not be offered with only Uxie on the Bench"
    );
}

/// With both on the Bench, Supreme Blast is usable and discards all of Mesprit's Energy.
#[test]
fn test_mesprit_supreme_blast_discards_all_energy_when_usable() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A2076Mesprit).with_energy(vec![
                EnergyType::Psychic,
                EnergyType::Psychic,
                EnergyType::Psychic,
            ]),
            PlayedCard::from_id(CardId::A2075Uxie),
            PlayedCard::from_id(CardId::A2077Azelf),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    let (_, choices) = game.get_state_clone().generate_possible_actions();
    assert!(
        choices.iter().any(|choice| matches!(
            &choice.action,
            SimpleAction::Attack(attack) if attack.title == "Supreme Blast"
        )),
        "Supreme Blast should be offered once Uxie and Azelf are both benched"
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A2076Mesprit, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert!(
        state.in_play_pokemon[1][0].is_none(),
        "160 knocks out Snorlax"
    );
    assert!(state.get_active(0).attached_energy.is_empty());
}
