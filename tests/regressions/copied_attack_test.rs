use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{Card, EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game, get_test_game_with_board},
};

/// Finds an offered `Attack` action by its title. Copied attacks are now surfaced through the
/// unified `SimpleAction::Attack(Attack)`, so they are identified by the carried attack rather
/// than by a source/index.
fn find_attack_by_title(actions: &[Action], title: &str) -> Option<Action> {
    actions
        .iter()
        .find(|action| matches!(&action.action, SimpleAction::Attack(attack) if attack.title == title))
        .cloned()
}

fn attack_titles(actions: &[Action]) -> Vec<String> {
    actions
        .iter()
        .filter_map(|action| match &action.action {
            SimpleAction::Attack(attack) => Some(attack.title.clone()),
            _ => None,
        })
        .collect()
}

#[test]
fn test_genome_hacking_copies_simple_damage_attack() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1a032MewEx).with_energy(vec![
            EnergyType::Psychic,
            EnergyType::Psychic,
            EnergyType::Psychic,
        ])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 3;

    let hp_before = state.get_active(1).get_remaining_hp();
    let Card::Pokemon(opponent_active) = &state.get_active(1).card else {
        panic!("Expected opponent active to be a Pokemon");
    };
    let expected_damage = opponent_active.attacks[0].fixed_damage;
    let expected_title = opponent_active.attacks[0].title.clone();
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1a032MewEx, 1),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let (actor, actions) = state.generate_possible_actions();
    assert_eq!(actor, 0);

    let copied_attack = find_attack_by_title(&actions, &expected_title)
        .expect("Expected copied attack choice for opponent active's first attack");

    game.apply_action(&copied_attack);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        hp_before - expected_damage,
        "Genome Hacking should deal the copied attack's damage to the opponent active"
    );
}

#[test]
fn test_genome_hacking_uses_copied_attack_as_mew_ex_attack() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1a032MewEx).with_energy(vec![
                EnergyType::Psychic,
                EnergyType::Psychic,
                EnergyType::Psychic,
            ]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1115Abra)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1a032MewEx, 1),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let (actor, actions) = state.generate_possible_actions();
    assert_eq!(actor, 0);

    let copied_teleport = find_attack_by_title(&actions, "Teleport")
        .expect("Expected copied choice for Abra's Teleport");

    game.apply_action(&copied_teleport);

    let state = game.get_state_clone();
    let (actor, actions) = state.generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(
        actions.iter().any(|action| {
            matches!(
                action.action,
                SimpleAction::Activate {
                    player: 0,
                    in_play_idx: 1,
                }
            )
        }),
        "Copied Teleport should queue a switch choice for Mew ex's controller"
    );
    assert!(
        !actions
            .iter()
            .any(|action| { matches!(action.action, SimpleAction::Activate { player: 1, .. }) }),
        "Copied Teleport should not create switch choices for the opponent"
    );
}

#[test]
fn test_genome_hacking_only_offers_opponent_active_attacks() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1a032MewEx).with_energy(vec![
            EnergyType::Psychic,
            EnergyType::Psychic,
            EnergyType::Psychic,
        ])],
        vec![
            PlayedCard::from_id(CardId::A1115Abra),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1a032MewEx, 1),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let (actor, actions) = state.generate_possible_actions();
    assert_eq!(actor, 0);
    let titles = attack_titles(&actions);
    assert!(
        !titles.is_empty(),
        "Genome Hacking should offer the opponent active's attacks"
    );
    assert!(
        titles.iter().all(|title| title == "Teleport"),
        "Genome Hacking should only offer attacks from the opponent's Active Pokemon (Abra's Teleport), got {titles:?}"
    );
}

#[test]
fn test_copy_anything_offers_nothing_and_does_nothing_if_energy_does_not_match() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1205Ditto).with_energy(vec![EnergyType::Colorless])],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1036CharizardEx),
        ],
    );
    state.current_player = 0;
    state.turn_count = 3;

    let hp_before = state.get_active(1).get_remaining_hp();
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1205Ditto, 0),
        is_stack: false,
    });

    // With only 1 Colorless energy, Ditto cannot pay for any of the opponent's attacks, so no
    // copied-attack choice is offered and Copy Anything resolves to nothing.
    let state = game.get_state_clone();
    assert!(
        state.move_generation_stack.is_empty(),
        "Copy Anything should not offer unaffordable copied attacks"
    );
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        hp_before,
        "Copy Anything should do nothing when Ditto cannot pay for any copied attack"
    );
}

#[test]
fn test_copy_anything_copies_opponent_bench_attack_when_energy_matches() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1205Ditto)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
        vec![
            PlayedCard::from_id(CardId::A1115Abra),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );
    state.current_player = 0;
    state.turn_count = 3;

    let hp_before = state.get_active(1).get_remaining_hp();
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1205Ditto, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let (actor, actions) = state.generate_possible_actions();
    assert_eq!(actor, 0);

    let copied_bench_attack = find_attack_by_title(&actions, "Vine Whip")
        .expect("Copy Anything should offer a matching-energy attack from opponent bench");

    game.apply_action(&copied_bench_attack);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        hp_before - 40,
        "Copy Anything should resolve the copied opponent bench attack when Ditto has the required Energy"
    );
}

#[test]
fn test_copy_a_friend_uses_own_non_ex_bench_attacks_only() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B1a055Ditto)
                .with_energy(vec![EnergyType::Grass, EnergyType::Colorless]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1a032MewEx),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 3;

    let hp_before = state.get_active(1).get_remaining_hp();
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B1a055Ditto, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let (actor, actions) = state.generate_possible_actions();
    assert_eq!(actor, 0);

    let titles = attack_titles(&actions);
    assert!(
        titles.iter().any(|title| title == "Vine Whip"),
        "Copy a Friend should offer attacks from your non-ex Benched Pokemon, got {titles:?}"
    );
    assert!(
        !titles.iter().any(|title| title == "Psyshot"),
        "Copy a Friend should not offer attacks from Benched Pokemon ex, got {titles:?}"
    );

    let copied_bulbasaur_attack = find_attack_by_title(&actions, "Vine Whip")
        .expect("Expected copied attack from own non-ex bench");

    game.apply_action(&copied_bulbasaur_attack);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        hp_before - 40,
        "Copy a Friend should resolve the copied non-ex bench attack"
    );
}

#[test]
fn test_genome_hacking_filters_out_opponent_mew_ex_genome_hacking() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1a032MewEx).with_energy(vec![
            EnergyType::Psychic,
            EnergyType::Psychic,
            EnergyType::Psychic,
        ])],
        vec![PlayedCard::from_id(CardId::A1a032MewEx)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1a032MewEx, 1),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let (actor, actions) = state.generate_possible_actions();
    assert_eq!(actor, 0);
    let titles = attack_titles(&actions);
    assert!(
        titles.iter().any(|title| title == "Psyshot"),
        "Genome Hacking should still offer Mew ex's non-copy attack, got {titles:?}"
    );
    assert!(
        !titles.iter().any(|title| title == "Genome Hacking"),
        "Genome Hacking should filter out the opponent Mew ex's Genome Hacking option, got {titles:?}"
    );
}

#[test]
fn test_genome_hacking_does_not_offer_opponent_ditto_copy_anything() {
    let mut game = get_initialized_game(0);
    let state = game.get_state_clone();
    let mut state = state;
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1a032MewEx).with_energy(vec![
            EnergyType::Psychic,
            EnergyType::Psychic,
            EnergyType::Psychic,
        ])],
        vec![PlayedCard::from_id(CardId::A1205Ditto)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1a032MewEx, 1),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert!(
        state.move_generation_stack.is_empty(),
        "Genome Hacking should not queue copied-attack choices when the opponent Active only has Copy Anything"
    );
}

#[test]
fn test_copy_anything_does_not_offer_copy_attacks() {
    // Ditto (Copy Anything) copies from any opponent Pokemon. The opponent's Active Mew ex has a
    // normal attack (Psyshot) and a copy attack (Genome Hacking); the benched Ditto only has the
    // Copy Anything copy attack. Only the affordable, non-copy Psyshot should be offered.
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1205Ditto)
            .with_energy(vec![EnergyType::Psychic, EnergyType::Colorless])],
        vec![
            PlayedCard::from_id(CardId::A1a032MewEx),
            PlayedCard::from_id(CardId::A1205Ditto),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1205Ditto, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let (actor, actions) = state.generate_possible_actions();
    assert_eq!(actor, 0);
    let titles = attack_titles(&actions);
    assert_eq!(
        titles,
        vec!["Psyshot".to_string()],
        "Copy Anything should keep the opponent's affordable non-copy attack and filter out copy attacks, got {titles:?}"
    );
}

#[test]
fn test_genome_hacking_best_effort_discards_only_matching_typed_energy() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1a032MewEx).with_energy(vec![
            EnergyType::Psychic,
            EnergyType::Psychic,
            EnergyType::Psychic,
        ])],
        vec![
            PlayedCard::from_id(CardId::A1036CharizardEx),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1a032MewEx, 1),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let (_, actions) = state.generate_possible_actions();
    let copied_crimson_storm = find_attack_by_title(&actions, "Crimson Storm")
        .expect("Expected copied choice for Charizard ex's Crimson Storm");

    game.apply_action(&copied_crimson_storm);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(0).attached_energy,
        vec![EnergyType::Psychic, EnergyType::Psychic, EnergyType::Psychic],
        "Genome Hacking should not discard non-matching energy when the copied attack wants typed self-discard"
    );
    assert!(
        state.discard_energies[0].is_empty(),
        "Genome Hacking should not send unrelated energy to the discard pile"
    );
    assert_eq!(
        state.points[0], 2,
        "Copied Crimson Storm should still resolve its damage and KO the opponent Charizard ex"
    );
}

#[test]
fn test_genome_hacking_best_effort_discards_matching_energy_for_attackid_copy() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1a032MewEx).with_energy(vec![
                EnergyType::Water,
                EnergyType::Psychic,
                EnergyType::Psychic,
            ]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![
            PlayedCard::from_id(CardId::A2049PalkiaEx),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A1a032MewEx, 1),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let (_, actions) = state.generate_possible_actions();
    let copied_dimensional_storm = find_attack_by_title(&actions, "Dimensional Storm")
        .expect("Expected copied choice for Palkia ex's Dimensional Storm");

    game.apply_action(&copied_dimensional_storm);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(0).attached_energy,
        vec![EnergyType::Psychic, EnergyType::Psychic],
        "Copied legacy typed-discard attacks should only remove matching attached energy"
    );
    assert_eq!(
        state.discard_energies[0],
        vec![EnergyType::Water],
        "Only the matching Water energy should be discarded from Mew ex"
    );
    assert_eq!(
        state.points[0], 2,
        "Copied Dimensional Storm should still KO the opponent active"
    );

    let (actor, actions) = state.generate_possible_actions();
    assert_eq!(actor, 1);
    let promote_bulbasaur = actions
        .iter()
        .find(|action| {
            matches!(
                action.action,
                SimpleAction::Activate {
                    player: 1,
                    in_play_idx: 1,
                }
            )
        })
        .expect("Opponent should need to promote the damaged bench Bulbasaur")
        .clone();

    game.apply_action(&promote_bulbasaur);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).card.get_name(),
        "Bulbasaur",
        "The opponent bench should be promoted after the copied attack knocks out Palkia ex"
    );
}
