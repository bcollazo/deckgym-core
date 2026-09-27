use deckgym::{
    actions::SimpleAction,
    card_ids::CardId,
    database::get_card_by_enum,
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

/// Squirt Bottle: "Discard a [R] Energy from your opponent's Active Pokémon."
#[test]
fn test_squirt_bottle_discards_one_fire_energy() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![
            PlayedCard::from_id(CardId::A1033Charmander).with_energy(vec![
                EnergyType::Fire,
                EnergyType::Fire,
                EnergyType::Water,
            ]),
        ],
    );

    give_and_play(&mut game, CardId::A4152SquirtBottle);

    let energy = game.get_state_clone().get_active(1).attached_energy.clone();
    assert_eq!(energy.len(), 2, "Exactly one Energy should be discarded");
    assert_eq!(
        energy.iter().filter(|e| **e == EnergyType::Fire).count(),
        1,
        "Squirt Bottle should discard a [R] Energy specifically"
    );
    assert!(energy.contains(&EnergyType::Water));
}

/// Squirt Bottle is unplayable when the opponent's Active has no [R] Energy.
#[test]
fn test_squirt_bottle_requires_fire_energy_on_opponent_active() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1053Squirtle).with_energy(vec![EnergyType::Water])],
    );
    let card = trainer_from_id(CardId::A4152SquirtBottle);
    let mut state = game.get_state_clone();
    state.hands[0] = vec![Card::Trainer(card.clone())];
    game.set_state(state);

    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    assert!(
        !actions.iter().any(|a| matches!(
            &a.action,
            SimpleAction::Play { trainer_card } if trainer_card.id == card.id
        )),
        "Squirt Bottle should not be playable without a [R] Energy to discard"
    );
}

/// Fishing Net: "Put a random Basic [W] Pokémon from your discard pile into your hand."
#[test]
fn test_fishing_net_recovers_a_basic_water_pokemon() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    let mut state = game.get_state_clone();
    state.discard_piles[0] = vec![
        get_card_by_enum(CardId::A1053Squirtle),
        get_card_by_enum(CardId::A1033Charmander),
    ];
    game.set_state(state);

    give_and_play(&mut game, CardId::A3143FishingNet);

    let state = game.get_state_clone();
    assert!(
        state.hands[0].iter().any(|c| c.get_name() == "Squirtle"),
        "Fishing Net should recover the Basic [W] Pokémon"
    );
    assert!(
        state.discard_piles[0]
            .iter()
            .any(|c| c.get_name() == "Charmander"),
        "The non-[W] Pokémon should stay in the discard pile"
    );
}

/// Fishing Net is unplayable with no Basic [W] Pokémon in the discard pile.
#[test]
fn test_fishing_net_requires_a_basic_water_pokemon_in_discard() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    let card = trainer_from_id(CardId::A3143FishingNet);
    let mut state = game.get_state_clone();
    state.discard_piles[0] = vec![get_card_by_enum(CardId::A1033Charmander)];
    state.hands[0] = vec![Card::Trainer(card.clone())];
    game.set_state(state);

    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    assert!(
        !actions.iter().any(|a| matches!(
            &a.action,
            SimpleAction::Play { trainer_card } if trainer_card.id == card.id
        )),
        "Fishing Net needs a Basic [W] Pokémon in the discard pile"
    );
}

/// Pokémon Flute: "Put a Basic Pokémon from your opponent's discard pile onto their Bench."
#[test]
fn test_pokemon_flute_benches_a_basic_from_opponent_discard() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    let mut state = game.get_state_clone();
    state.discard_piles[1] = vec![
        get_card_by_enum(CardId::A1053Squirtle),
        get_card_by_enum(CardId::A1004VenusaurEx), // Stage 2, not eligible
    ];
    game.set_state(state);

    give_and_play(&mut game, CardId::A1a064PokemonFlute);

    let (actor, actions) = game.get_state_clone().generate_possible_actions();
    assert_eq!(
        actor, 0,
        "The Pokémon Flute player chooses which Basic to bench"
    );
    let choices: Vec<&deckgym::actions::Action> = actions
        .iter()
        .filter(|a| {
            matches!(
                a.action,
                SimpleAction::BenchOpponentPokemonFromDiscard { .. }
            )
        })
        .collect();
    assert_eq!(
        choices.len(),
        1,
        "Only the Basic Squirtle should be an eligible target"
    );
    let choice = choices[0].clone();
    game.apply_action(&choice);

    let state = game.get_state_clone();
    assert!(
        state
            .enumerate_bench_pokemon(1)
            .any(|(_, p)| p.get_name() == "Squirtle"),
        "Squirtle should now be on the opponent's Bench"
    );
    assert!(
        !state.discard_piles[1]
            .iter()
            .any(|c| c.get_name() == "Squirtle"),
        "Squirtle should have left the discard pile"
    );
}

/// Prank Spinner: "A card from among both player's hands is chosen at random, revealed to the
/// other player, and shuffled into its owner's deck."
#[test]
fn test_prank_spinner_moves_one_card_from_a_hand_into_its_owners_deck() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    let spinner = trainer_from_id(CardId::B1213PrankSpinner);
    let mut state = game.get_state_clone();
    state.hands[0] = vec![
        Card::Trainer(spinner.clone()),
        get_card_by_enum(CardId::PA001Potion),
    ];
    state.hands[1] = vec![get_card_by_enum(CardId::PA006RedCard)];
    let decks_before = [state.decks[0].cards.len(), state.decks[1].cards.len()];
    game.set_state(state);

    play_trainer(&mut game, 0, spinner);

    let state = game.get_state_clone();
    let hands_after = state.hands[0].len() + state.hands[1].len();
    let decks_after = [state.decks[0].cards.len(), state.decks[1].cards.len()];
    // Playing the Spinner itself removes 1 card from hand; the effect removes 1 more.
    assert_eq!(hands_after, 1, "Exactly one card should have left a hand");
    assert_eq!(
        (decks_after[0] - decks_before[0]) + (decks_after[1] - decks_before[1]),
        1,
        "The chosen card should have gone into its owner's deck"
    );
}

/// Hitting Hammer: "Flip 2 coins. If both of them are heads, discard a random Energy from your
/// opponent's Active Pokémon." Over many seeds we should see both outcomes.
#[test]
fn test_hitting_hammer_discards_energy_only_on_double_heads() {
    let mut discarded = 0;
    let mut untouched = 0;
    for seed in 0..40u64 {
        let mut game = deckgym::test_support::get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
            vec![PlayedCard::from_id(CardId::A1033Charmander)
                .with_energy(vec![EnergyType::Fire, EnergyType::Fire])],
        );
        give_and_play(&mut game, CardId::B1215HittingHammer);
        match game.get_state_clone().get_active(1).attached_energy.len() {
            1 => discarded += 1,
            2 => untouched += 1,
            other => panic!("Unexpected energy count {other}"),
        }
    }
    assert!(
        discarded > 0,
        "Double heads should sometimes discard an Energy"
    );
    assert!(
        untouched > 0,
        "Anything short of double heads should leave the Energy alone"
    );
}
