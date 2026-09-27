use deckgym::{
    actions::SimpleAction,
    card_ids::CardId,
    models::{Card, EnergyType, PlayedCard},
    test_support::{get_test_game_with_board, play_trainer, trainer_from_id},
    Game,
};

fn give_and_play(game: &mut Game<'static>, card_id: CardId) {
    let trainer = trainer_from_id(card_id);
    let mut state = game.get_state_clone();
    state.hands[0] = vec![Card::Trainer(trainer.clone())];
    game.set_state(state);
    play_trainer(game, 0, trainer);
}

fn is_playable(game: &Game<'static>, trainer_id: &str) -> bool {
    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    actions.iter().any(|a| {
        matches!(&a.action, SimpleAction::Play { trainer_card } if trainer_card.id == trainer_id)
    })
}

/// Lt. Surge: "Move all [L] Energy from your Benched Pokémon to your Raichu, Electrode, or
/// Electabuzz in the Active Spot." Only Lightning Energy moves; other types stay put.
#[test]
fn test_lt_surge_moves_all_lightning_energy_to_active_raichu() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1095Raichu).with_energy(vec![EnergyType::Lightning]),
            PlayedCard::from_id(CardId::A1100Electrode)
                .with_energy(vec![EnergyType::Lightning, EnergyType::Lightning]),
            PlayedCard::from_id(CardId::A1053Squirtle)
                .with_energy(vec![EnergyType::Lightning, EnergyType::Water]),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );

    give_and_play(&mut game, CardId::A1226LtSurge);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(0).attached_energy.len(),
        4,
        "Raichu should end with its own 1 plus the 3 Lightning from the Bench"
    );
    assert!(state.in_play_pokemon[0][1]
        .as_ref()
        .unwrap()
        .attached_energy
        .is_empty());
    assert_eq!(
        state.in_play_pokemon[0][2]
            .as_ref()
            .unwrap()
            .attached_energy,
        vec![EnergyType::Water],
        "Non-Lightning Energy should stay on the Bench"
    );
}

/// Lt. Surge needs one of its three named Pokémon in the Active Spot.
#[test]
fn test_lt_surge_requires_a_named_active_pokemon() {
    let lt_surge = trainer_from_id(CardId::A1226LtSurge);

    let mut wrong_active = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1100Electrode).with_energy(vec![EnergyType::Lightning]),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    let mut state = wrong_active.get_state_clone();
    state.hands[0] = vec![Card::Trainer(lt_surge.clone())];
    wrong_active.set_state(state);
    assert!(
        !is_playable(&wrong_active, &lt_surge.id),
        "Lt. Surge should not be playable with Bulbasaur active"
    );

    let mut no_bench_energy = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1095Raichu),
            PlayedCard::from_id(CardId::A1053Squirtle).with_energy(vec![EnergyType::Water]),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    let mut state = no_bench_energy.get_state_clone();
    state.hands[0] = vec![Card::Trainer(lt_surge.clone())];
    no_bench_energy.set_state(state);
    assert!(
        !is_playable(&no_bench_energy, &lt_surge.id),
        "Lt. Surge should not be playable with no [L] Energy on the Bench"
    );
}

/// Juggler: "You can use this card only if your Pokémon in play have 3 or more different types of
/// Energy attached. Move all Energy from each of your Benched Pokémon to your Active Pokémon."
#[test]
fn test_juggler_moves_every_bench_energy_to_the_active() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_energy(vec![EnergyType::Grass]),
            PlayedCard::from_id(CardId::A1033Charmander).with_energy(vec![EnergyType::Fire]),
            PlayedCard::from_id(CardId::A1053Squirtle)
                .with_energy(vec![EnergyType::Water, EnergyType::Lightning]),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );

    give_and_play(&mut game, CardId::B2151Juggler);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(0).attached_energy.len(),
        4,
        "All 3 Bench Energy should join the Active's own 1"
    );
    for bench_idx in 1..=2 {
        assert!(
            state.in_play_pokemon[0][bench_idx]
                .as_ref()
                .unwrap()
                .attached_energy
                .is_empty(),
            "Bench slot {bench_idx} should be emptied"
        );
    }
}

/// Juggler's play condition: fewer than 3 distinct Energy types in play makes it unplayable.
#[test]
fn test_juggler_requires_three_distinct_energy_types() {
    let juggler = trainer_from_id(CardId::B2151Juggler);
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_energy(vec![EnergyType::Grass]),
            PlayedCard::from_id(CardId::A1033Charmander).with_energy(vec![EnergyType::Fire]),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    let mut state = game.get_state_clone();
    state.hands[0] = vec![Card::Trainer(juggler.clone())];
    game.set_state(state);
    assert!(
        !is_playable(&game, &juggler.id),
        "Two Energy types is not enough for Juggler"
    );

    let mut state = game.get_state_clone();
    state.in_play_pokemon[0][1]
        .as_mut()
        .unwrap()
        .attached_energy
        .push(EnergyType::Water);
    game.set_state(state);
    assert!(
        is_playable(&game, &juggler.id),
        "Three Energy types should make Juggler playable"
    );
}

/// Acerola: "Choose 1 of your Palossand or Mimikyu that has damage on it, and move 40 of its
/// damage to your opponent's Active Pokémon."
#[test]
fn test_acerola_moves_40_damage_to_the_opponents_active() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A3082Palossand).with_damage(60)],
        vec![PlayedCard::from_id(CardId::A1202Chansey)],
    );

    give_and_play(&mut game, CardId::A3148Acerola);

    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    let move_damage = actions
        .iter()
        .find(|a| matches!(a.action, SimpleAction::MoveDamageToOpponentActive { .. }))
        .expect("Acerola should offer to move Palossand's damage")
        .clone();
    game.apply_action(&move_damage);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(0).get_remaining_hp(),
        130 - 20,
        "40 of Palossand's 60 damage should have moved off it"
    );
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        120 - 40,
        "The opponent's Active should have taken the 40 damage"
    );
}

/// Acerola only names Palossand and Mimikyu, and only ones that already have damage.
#[test]
fn test_acerola_requires_a_damaged_named_pokemon() {
    let acerola = trainer_from_id(CardId::A3148Acerola);
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_damage(40),
            PlayedCard::from_id(CardId::A3082Palossand),
        ],
        vec![PlayedCard::from_id(CardId::A1202Chansey)],
    );
    let mut state = game.get_state_clone();
    state.hands[0] = vec![Card::Trainer(acerola.clone())];
    game.set_state(state);
    assert!(
        !is_playable(&game, &acerola.id),
        "A damaged Bulbasaur and an undamaged Palossand are both invalid targets"
    );
}
