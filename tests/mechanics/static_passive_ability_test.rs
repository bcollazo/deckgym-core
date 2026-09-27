use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard, StatusCondition},
    test_support::get_test_game_with_board,
};

/// Hoothoot's Insomnia: "This Pokémon can't be Asleep." Other Special Conditions still land.
#[test]
fn test_insomnia_blocks_sleep_but_not_other_conditions() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A4140Hoothoot)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let mut state = game.get_state_clone();
    state.apply_status_condition(0, 0, StatusCondition::Asleep);
    state.apply_status_condition(0, 0, StatusCondition::Poisoned);
    game.set_state(state);

    let state = game.get_state_clone();
    assert!(!state.get_active(0).is_asleep(), "Insomnia blocks Asleep");
    assert!(
        state.get_active(0).is_poisoned(),
        "Insomnia only blocks Asleep"
    );
}

/// Lilligant's Toughness Aroma: "Each of your [G] Pokémon gets +20 HP."
#[test]
fn test_toughness_aroma_gives_grass_pokemon_extra_hp() {
    let game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::B1018Lilligant),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let state = game.get_state_clone();
    assert_eq!(
        state.in_play_pokemon[0][0]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        90,
        "Bulbasaur is [G]: 70 + 20"
    );
    assert_eq!(
        state.in_play_pokemon[0][2]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        60,
        "Charmander is [R] and unaffected"
    );
    assert_eq!(
        state.in_play_pokemon[1][0]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        70,
        "the opponent's [G] Pokémon is unaffected"
    );
}

#[test]
fn test_toughness_aroma_disappears_when_lilligant_leaves_play() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::B1018Lilligant),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    assert_eq!(
        game.get_state_clone().in_play_pokemon[0][0]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        90
    );

    let mut state = game.get_state_clone();
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    game.set_state(state);

    assert_eq!(
        game.get_state_clone().in_play_pokemon[0][0]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        70,
        "the +20 HP goes away with Lilligant"
    );
}

/// Claydol's Heal Block: "Pokémon (both yours and your opponent's) can't be healed."
#[test]
fn test_heal_block_stops_potion_healing_on_both_sides() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_damage(30),
            PlayedCard::from_id(CardId::A3a031Claydol),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur).with_damage(30)],
    );
    let mut state = game.get_state_clone();
    state.hands[0].clear();
    state.hands[0].push(get_card_by_enum(CardId::PA001Potion));
    game.set_state(state);

    let potion = match get_card_by_enum(CardId::PA001Potion) {
        deckgym::models::Card::Trainer(trainer_card) => trainer_card,
        _ => unreachable!(),
    };
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: potion,
        },
        is_stack: false,
    });
    // Resolve the Potion's target choice if one is offered.
    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    if let Some(heal) = choices
        .iter()
        .find(|c| matches!(c.action, SimpleAction::Heal { .. }))
    {
        assert_eq!(actor, 0);
        let heal = heal.clone();
        game.apply_action(&heal);
    }

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(0).get_remaining_hp(),
        40,
        "Heal Block should have stopped the Potion from healing anything"
    );
}

/// Cherubi's En-fruits-iastic: "If this Pokémon has a Pokémon Tool attached, attacks used by this
/// Pokémon cost 1 less [G] Energy." Sweets Relay costs a single [G], so with a Tool attached
/// Cherubi can use it with no Energy at all.
#[test]
fn test_en_fruits_iastic_discounts_a_grass_energy_when_a_tool_is_attached() {
    let attacks_offered = |card: PlayedCard| {
        let game = get_test_game_with_board(
            vec![card],
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        );
        let (_, choices) = game.get_state_clone().generate_possible_actions();
        choices
            .iter()
            .any(|c| matches!(c.action, SimpleAction::Attack(_)))
    };

    assert!(
        !attacks_offered(PlayedCard::from_id(CardId::A4023Cherubi)),
        "no Energy and no Tool: Sweets Relay is unaffordable"
    );
    assert!(
        attacks_offered(
            PlayedCard::from_id(CardId::A4023Cherubi)
                .with_tool(get_card_by_enum(CardId::A2148RockyHelmet))
        ),
        "the Tool discounts Sweets Relay's only [G] Energy"
    );
    assert!(attacks_offered(
        PlayedCard::from_id(CardId::A4023Cherubi).with_energy(vec![EnergyType::Grass])
    ));
}

/// Regigigas's Seal of Antiquity: "If you don't have Regirock, Regice, and Registeel on your
/// Bench, this Pokémon can't attack."
#[test]
fn test_seal_of_antiquity_needs_all_three_regis_on_the_bench() {
    let can_attack = |bench: Vec<PlayedCard>| {
        let mut board = vec![
            PlayedCard::from_id(CardId::B3134Regigigas).with_energy(vec![
                EnergyType::Colorless,
                EnergyType::Colorless,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
        ];
        board.extend(bench);
        let game =
            get_test_game_with_board(board, vec![PlayedCard::from_id(CardId::A1001Bulbasaur)]);
        let (_, choices) = game.get_state_clone().generate_possible_actions();
        choices
            .iter()
            .any(|c| matches!(c.action, SimpleAction::Attack(_)))
    };

    assert!(!can_attack(vec![]), "no Regis on the bench: can't attack");
    assert!(
        !can_attack(vec![
            PlayedCard::from_id(CardId::B3077Regirock),
            PlayedCard::from_id(CardId::B3045Regice),
        ]),
        "two of three is not enough"
    );
    assert!(can_attack(vec![
        PlayedCard::from_id(CardId::B3077Regirock),
        PlayedCard::from_id(CardId::B3045Regice),
        PlayedCard::from_id(CardId::B3116Registeel),
    ]));
}
