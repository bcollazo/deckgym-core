use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard, TrainerCard},
    test_support::get_initialized_game,
};

fn make_sada_trainer_card() -> TrainerCard {
    match get_card_by_enum(CardId::B3a072ProfessorSada) {
        Card::Trainer(tc) => tc,
        _ => panic!("Expected trainer card"),
    }
}

fn play_sada(game: &mut deckgym::Game) {
    let trainer_card = make_sada_trainer_card();
    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    };
    game.apply_action(&play_action);
}

/// Professor Sada cannot be played when there are no Ancient Pokémon on the board.
#[test]
fn test_cannot_play_sada_no_ancient_pokemon() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    state.hands[0].clear();
    let trainer_card = make_sada_trainer_card();
    state.hands[0].push(Card::Trainer(trainer_card));
    state.discard_energies[0] = vec![EnergyType::Fire, EnergyType::Water, EnergyType::Grass];
    game.set_state(state);

    let (_, actions) = game.get_state_clone().generate_possible_actions();
    let can_play = actions
        .iter()
        .any(|a| matches!(&a.action, SimpleAction::Play { .. }));
    assert!(
        !can_play,
        "Should not be able to play Sada without Ancient Pokémon, actions: {actions:?}"
    );
}

/// Professor Sada cannot be played when the discard has no energies at all.
#[test]
fn test_cannot_play_sada_empty_discard() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![PlayedCard::from_id(CardId::B3a003BruteBonnet)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    state.hands[0].clear();
    let trainer_card = make_sada_trainer_card();
    state.hands[0].push(Card::Trainer(trainer_card));
    state.discard_energies[0] = vec![]; // empty
    game.set_state(state);

    let (_, actions) = game.get_state_clone().generate_possible_actions();
    let can_play = actions
        .iter()
        .any(|a| matches!(&a.action, SimpleAction::Play { .. }));
    assert!(
        !can_play,
        "Should not be able to play Sada with empty discard, actions: {actions:?}"
    );
}

/// With only 1 distinct energy type in the discard, Sada attaches that 1 energy.
/// The player chooses which Ancient Pokémon receives it.
#[test]
fn test_sada_one_type_gives_one_choice_per_ancient_slot() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B3a003BruteBonnet),
            PlayedCard::from_id(CardId::B3a034GreatTusk),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    state.hands[0].clear();
    let trainer_card = make_sada_trainer_card();
    state.hands[0].push(Card::Trainer(trainer_card));
    // 3 Fire but only 1 distinct type
    state.discard_energies[0] = vec![EnergyType::Fire, EnergyType::Fire, EnergyType::Fire];
    game.set_state(state);

    play_sada(&mut game);

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    let sada_choices: Vec<_> = choices
        .iter()
        .filter(|a| matches!(&a.action, SimpleAction::SadaAttach { .. }))
        .collect();
    // C(1,1) × 2 slots = 2 choices (attach Fire to active, or to bench)
    assert_eq!(
        sada_choices.len(),
        2,
        "1 type × 2 ancient slots = 2 choices; got: {sada_choices:?}"
    );
    assert!(
        sada_choices
            .iter()
            .all(|a| matches!(&a.action, SimpleAction::SadaAttach { assignments } if assignments.len() == 1)),
        "Each choice should attach exactly 1 energy"
    );
}

/// With 2 distinct energy types, Sada attaches both (one of each).
#[test]
fn test_sada_two_types_attaches_both() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![PlayedCard::from_id(CardId::B3a003BruteBonnet)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    state.hands[0].clear();
    let trainer_card = make_sada_trainer_card();
    state.hands[0].push(Card::Trainer(trainer_card));
    state.discard_energies[0] = vec![EnergyType::Fire, EnergyType::Water];
    game.set_state(state);

    play_sada(&mut game);

    // Only 1 ancient slot, so both energies must go there: 1 choice
    let (_, choices) = game.get_state_clone().generate_possible_actions();
    let sada_choices: Vec<_> = choices
        .iter()
        .filter(|a| matches!(&a.action, SimpleAction::SadaAttach { .. }))
        .collect();
    assert_eq!(
        sada_choices.len(),
        1,
        "1 ancient slot, 2 types → 1 choice; got: {sada_choices:?}"
    );
    assert!(
        matches!(&sada_choices[0].action, SimpleAction::SadaAttach { assignments } if assignments.len() == 2),
        "Should attach 2 energies"
    );

    game.apply_action(sada_choices[0]);

    let state = game.get_state_clone();
    assert_eq!(
        state.discard_energies[0].len(),
        0,
        "Both discard energies should be consumed"
    );
    let active_energies = &state.in_play_pokemon[0][0]
        .as_ref()
        .unwrap()
        .attached_energy;
    assert_eq!(active_energies.len(), 2);
}

/// Playing Professor Sada with one Ancient Pokémon and exactly 3 energy types presents
/// exactly 1 assignment (all 3 energies go to the only Ancient Pokémon).
#[test]
fn test_sada_one_ancient_three_types_gives_one_choice() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![PlayedCard::from_id(CardId::B3a003BruteBonnet)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    state.hands[0].clear();
    let trainer_card = make_sada_trainer_card();
    state.hands[0].push(Card::Trainer(trainer_card));
    state.discard_energies[0] = vec![EnergyType::Fire, EnergyType::Water, EnergyType::Grass];
    game.set_state(state);

    play_sada(&mut game);

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    let sada_choices: Vec<_> = choices
        .iter()
        .filter(|a| matches!(&a.action, SimpleAction::SadaAttach { .. }))
        .collect();
    assert_eq!(
        sada_choices.len(),
        1,
        "1 ancient slot × 1 type combo × 1^3 = 1 choice; got: {sada_choices:?}"
    );
}

/// Playing Professor Sada with two Ancient Pokémon and exactly 3 energy types presents
/// exactly 8 assignments (2^3 ways to assign 3 energies to 2 targets).
#[test]
fn test_sada_two_ancient_three_types_gives_eight_choices() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B3a003BruteBonnet),
            PlayedCard::from_id(CardId::B3a034GreatTusk),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    state.hands[0].clear();
    let trainer_card = make_sada_trainer_card();
    state.hands[0].push(Card::Trainer(trainer_card));
    state.discard_energies[0] = vec![EnergyType::Fire, EnergyType::Water, EnergyType::Grass];
    game.set_state(state);

    play_sada(&mut game);

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    let sada_choices: Vec<_> = choices
        .iter()
        .filter(|a| matches!(&a.action, SimpleAction::SadaAttach { .. }))
        .collect();
    assert_eq!(
        sada_choices.len(),
        8,
        "2 ancient slots × 1 combo × 2^3 = 8 choices; got: {sada_choices:?}"
    );
}

/// After choosing an assignment, the energies are removed from discard
/// and attached to the correct Pokémon.
#[test]
fn test_sada_attaches_energies_and_clears_discard() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B3a003BruteBonnet), // active — slot 0
            PlayedCard::from_id(CardId::B3a034GreatTusk),   // bench — slot 1
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    state.hands[0].clear();
    let trainer_card = make_sada_trainer_card();
    state.hands[0].push(Card::Trainer(trainer_card));
    state.discard_energies[0] = vec![EnergyType::Fire, EnergyType::Water, EnergyType::Grass];
    game.set_state(state);

    play_sada(&mut game);

    // Pick the assignment that puts all 3 energies on the active (slot 0)
    let (_, choices) = game.get_state_clone().generate_possible_actions();
    let all_to_active = choices.iter().find(|a| {
        matches!(
            &a.action,
            SimpleAction::SadaAttach { assignments }
            if assignments.iter().all(|(_, idx)| *idx == 0)
        )
    });
    assert!(
        all_to_active.is_some(),
        "Should have an all-to-active choice"
    );

    game.apply_action(all_to_active.unwrap());

    let state = game.get_state_clone();
    assert!(
        state.discard_energies[0].is_empty(),
        "All 3 energies should have been removed from discard"
    );

    let active = state.in_play_pokemon[0][0]
        .as_ref()
        .expect("Active should exist");
    assert_eq!(active.attached_energy.len(), 3);
    let attached: std::collections::HashSet<_> = active.attached_energy.iter().collect();
    assert!(attached.contains(&EnergyType::Fire));
    assert!(attached.contains(&EnergyType::Water));
    assert!(attached.contains(&EnergyType::Grass));
}

/// The full-art variant (B3a 087) behaves identically.
#[test]
fn test_sada_full_art_variant_works() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![PlayedCard::from_id(CardId::B3a003BruteBonnet)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    state.hands[0].clear();
    let full_art_card = match get_card_by_enum(CardId::B3a087ProfessorSada) {
        Card::Trainer(tc) => tc,
        _ => panic!("Expected trainer card"),
    };
    state.hands[0].push(Card::Trainer(full_art_card.clone()));
    state.discard_energies[0] = vec![EnergyType::Fire, EnergyType::Water, EnergyType::Grass];
    game.set_state(state);

    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: full_art_card,
        },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let (_, choices) = game.get_state_clone().generate_possible_actions();
    let sada_choices: Vec<_> = choices
        .iter()
        .filter(|a| matches!(&a.action, SimpleAction::SadaAttach { .. }))
        .collect();
    assert_eq!(
        sada_choices.len(),
        1,
        "Full-art variant should work identically"
    );
}
