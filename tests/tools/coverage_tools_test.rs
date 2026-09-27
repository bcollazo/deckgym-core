use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game_with_board, get_test_game_with_board},
    Game,
};

fn attack(game: &mut Game<'static>, actor: usize, card_id: CardId, index: usize) {
    game.apply_action(&Action {
        actor,
        action: attack_action(card_id, index),
        is_stack: false,
    });
}

/// Beastite: "Attacks used by the Ultra Beast this card is attached to do +10 damage to your
/// opponent's Active Pokémon for each point you have gotten."
#[test]
fn test_beastite_adds_10_damage_per_point() {
    // Nihilego's first attack does a fixed 30. With 2 points, Beastite adds 20.
    let damage_with_points = |points: u8| {
        let mut game = get_test_game_with_board(
            vec![PlayedCard::from_id(CardId::A3a042Nihilego)
                .with_energy(vec![EnergyType::Darkness, EnergyType::Colorless])
                .with_tool(get_card_by_enum(CardId::A3a066Beastite))],
            vec![PlayedCard::from_id(CardId::A1202Chansey)],
        );
        let mut state = game.get_state_clone();
        state.points[0] = points;
        game.set_state(state);
        attack(&mut game, 0, CardId::A3a042Nihilego, 0);
        120 - game.get_state_clone().get_remaining_hp(1, 0)
    };

    let base = damage_with_points(0);
    assert_eq!(
        damage_with_points(2),
        base + 20,
        "Beastite should add +10 damage per point"
    );
}

/// Beastite only boosts Ultra Beasts.
#[test]
fn test_beastite_does_nothing_on_a_non_ultra_beast() {
    let damage_with_points = |points: u8| {
        let mut game = get_test_game_with_board(
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])
                .with_tool(get_card_by_enum(CardId::A3a066Beastite))],
            vec![PlayedCard::from_id(CardId::A1202Chansey)],
        );
        let mut state = game.get_state_clone();
        state.points[0] = points;
        game.set_state(state);
        attack(&mut game, 0, CardId::A1001Bulbasaur, 0);
        120 - game.get_state_clone().get_remaining_hp(1, 0)
    };

    assert_eq!(damage_with_points(0), damage_with_points(2));
}

/// Rescue Scarf: "If the Pokémon this card is attached to is Knocked Out by damage from an attack
/// from your opponent's Pokémon, put it into your hand instead of the discard pile."
#[test]
fn test_rescue_scarf_returns_the_knocked_out_pokemon_to_hand() {
    let mut game = get_initialized_game_with_board(
        0,
        1,
        3,
        vec![
            PlayedCard::from_id(CardId::A1033Charmander)
                .with_remaining_hp(10)
                .with_tool(get_card_by_enum(CardId::A4155RescueScarf)),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![
            PlayedCard::from_id(CardId::A1004VenusaurEx).with_energy(vec![
                EnergyType::Grass,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
        ],
    );
    let mut state = game.get_state_clone();
    state.hands[0].clear();
    game.set_state(state);

    attack(&mut game, 1, CardId::A1004VenusaurEx, 0);

    let state = game.get_state_clone();
    assert!(
        state.hands[0].iter().any(|c| c.get_name() == "Charmander"),
        "Rescue Scarf should put the Knocked Out Charmander into its owner's hand"
    );
    assert!(
        !state.discard_piles[0]
            .iter()
            .any(|c| c.get_name() == "Charmander"),
        "Charmander should not be in the discard pile"
    );
    assert!(
        state.discard_piles[0]
            .iter()
            .any(|c| c.get_name() == "Rescue Scarf"),
        "The Rescue Scarf itself still goes to the discard pile"
    );
    assert_eq!(
        state.points[1], 1,
        "The opponent still scores the point for the knockout"
    );
}

/// Without Rescue Scarf the same Pokémon lands in the discard pile.
#[test]
fn test_knocked_out_pokemon_goes_to_discard_without_rescue_scarf() {
    let mut game = get_initialized_game_with_board(
        0,
        1,
        3,
        vec![
            PlayedCard::from_id(CardId::A1033Charmander).with_remaining_hp(10),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![
            PlayedCard::from_id(CardId::A1004VenusaurEx).with_energy(vec![
                EnergyType::Grass,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
        ],
    );
    let mut state = game.get_state_clone();
    state.hands[0].clear();
    game.set_state(state);

    attack(&mut game, 1, CardId::A1004VenusaurEx, 0);

    let state = game.get_state_clone();
    assert!(state.discard_piles[0]
        .iter()
        .any(|c| c.get_name() == "Charmander"));
    assert!(!state.hands[0].iter().any(|c| c.get_name() == "Charmander"));
}

/// Lucky Mittens: "Whenever your opponent's Pokémon is Knocked Out by damage from an attack used
/// by the Pokémon this card is attached to, draw a card."
#[test]
fn test_lucky_mittens_draws_a_card_on_knockout() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1004VenusaurEx)
            .with_energy(vec![
                EnergyType::Grass,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ])
            .with_tool(get_card_by_enum(CardId::B1220LuckyMittens))],
        vec![
            PlayedCard::from_id(CardId::A1033Charmander).with_remaining_hp(10),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );
    let mut state = game.get_state_clone();
    state.hands[0].clear();
    game.set_state(state);

    attack(&mut game, 0, CardId::A1004VenusaurEx, 0);

    assert_eq!(
        game.get_state_clone().hands[0].len(),
        1,
        "Lucky Mittens should draw a card when its holder knocks a Pokémon out"
    );
}

/// No knockout means no draw.
#[test]
fn test_lucky_mittens_does_not_draw_without_a_knockout() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1004VenusaurEx)
            .with_energy(vec![
                EnergyType::Grass,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ])
            .with_tool(get_card_by_enum(CardId::B1220LuckyMittens))],
        vec![PlayedCard::from_id(CardId::A1202Chansey)],
    );
    let mut state = game.get_state_clone();
    state.hands[0].clear();
    game.set_state(state);

    attack(&mut game, 0, CardId::A1004VenusaurEx, 0);

    assert!(game.get_state_clone().hands[0].is_empty());
}

/// Dark Pendant: "If the [D] Pokémon this card is attached to is in the Active Spot and is damaged
/// by an attack from your opponent's Pokémon, your opponent reveals a random card from their hand
/// and shuffles it into their deck."
#[test]
fn test_dark_pendant_shuffles_a_card_from_the_attackers_hand() {
    let mut game = get_initialized_game_with_board(
        0,
        1,
        3,
        vec![PlayedCard::from_id(CardId::A3a042Nihilego)
            .with_tool(get_card_by_enum(CardId::A4154DarkPendant))],
        vec![
            PlayedCard::from_id(CardId::A1004VenusaurEx).with_energy(vec![
                EnergyType::Grass,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
        ],
    );
    let mut state = game.get_state_clone();
    state.hands[1] = vec![
        get_card_by_enum(CardId::PA001Potion),
        get_card_by_enum(CardId::PA005PokeBall),
    ];
    let deck_before = state.decks[1].cards.len();
    game.set_state(state);

    attack(&mut game, 1, CardId::A1004VenusaurEx, 0);
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(
        state.hands[1].len(),
        1,
        "Dark Pendant should take one card out of the attacker's hand"
    );
    assert_eq!(
        state.decks[1].cards.len(),
        deck_before + 1,
        "That card should be shuffled into the attacker's deck"
    );
}

/// Dark Pendant only fires for a [D] holder.
#[test]
fn test_dark_pendant_does_nothing_on_a_non_dark_pokemon() {
    let mut game = get_initialized_game_with_board(
        0,
        1,
        3,
        vec![PlayedCard::from_id(CardId::A1202Chansey)
            .with_tool(get_card_by_enum(CardId::A4154DarkPendant))],
        vec![
            PlayedCard::from_id(CardId::A1004VenusaurEx).with_energy(vec![
                EnergyType::Grass,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
        ],
    );
    let mut state = game.get_state_clone();
    state.hands[1] = vec![get_card_by_enum(CardId::PA001Potion)];
    game.set_state(state);

    attack(&mut game, 1, CardId::A1004VenusaurEx, 0);
    game.play_until_stable();

    assert_eq!(game.get_state_clone().hands[1].len(), 1);
}

/// Memory Light: "The Pokémon this card is attached to can use any attack from its previous
/// Evolutions." Ivysaur evolved from Bulbasaur should be offered Bulbasaur's Vine Whip.
#[test]
fn test_memory_light_grants_previous_evolution_attacks() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    let mut state = game.get_state_clone();
    state.hands[0] = vec![get_card_by_enum(CardId::A1002Ivysaur)];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Evolve {
            evolution: get_card_by_enum(CardId::A1002Ivysaur),
            in_play_idx: 0,
            from_deck: false,
        },
        is_stack: false,
    });

    // Give Ivysaur enough Energy for Vine Whip (Grass + Colorless) but not for Razor Leaf.
    let mut state = game.get_state_clone();
    let ivysaur = state.in_play_pokemon[0][0].as_mut().unwrap();
    ivysaur.attached_energy = vec![EnergyType::Grass, EnergyType::Colorless];
    game.set_state(state);

    let attack_titles = |game: &Game<'static>| -> Vec<String> {
        let (_actor, actions) = game.get_state_clone().generate_possible_actions();
        actions
            .iter()
            .filter_map(|a| match &a.action {
                SimpleAction::Attack(attack) => Some(attack.title.clone()),
                _ => None,
            })
            .collect()
    };

    assert!(
        !attack_titles(&game).contains(&"Vine Whip".to_string()),
        "Without Memory Light, Ivysaur should not have Bulbasaur's Vine Whip"
    );

    let mut state = game.get_state_clone();
    state.in_play_pokemon[0][0].as_mut().unwrap().attached_tool =
        Some(get_card_by_enum(CardId::A4a068MemoryLight));
    game.set_state(state);

    assert!(
        attack_titles(&game).contains(&"Vine Whip".to_string()),
        "Memory Light should grant Ivysaur its previous Evolution's attack"
    );
}

/// Clear Veil: "Prevent all effects of attacks used by your opponent's Pokémon done to the Pokémon
/// this card is attached to." Here the effect is the Special Condition from Weezing's attack.
#[test]
fn test_clear_veil_prevents_a_special_condition_from_an_opponents_attack() {
    let with_veil = |veil: bool| {
        let mut defender = PlayedCard::from_id(CardId::A1202Chansey);
        if veil {
            defender = defender.with_tool(get_card_by_enum(CardId::B4149ClearVeil));
        }
        let mut game = get_initialized_game_with_board(
            0,
            1,
            3,
            vec![defender],
            vec![PlayedCard::from_id(CardId::A1174Grimer).with_energy(vec![EnergyType::Darkness])],
        );
        let mut state = game.get_state_clone();
        state.hands[1].clear();
        game.set_state(state);

        attack(&mut game, 1, CardId::A1174Grimer, 0);
        game.play_until_stable();
        game.get_state_clone().get_active(0).is_poisoned()
    };

    assert!(
        with_veil(false),
        "Weezing's attack should normally poison the defender"
    );
    assert!(
        !with_veil(true),
        "Clear Veil should prevent the poison from the opponent's attack"
    );
}

/// Clear Veil is a Tool like any other: it can be attached and shows up as an attachment choice.
#[test]
fn test_clear_veil_is_an_attachable_tool() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    let clear_veil = match get_card_by_enum(CardId::B4149ClearVeil) {
        Card::Trainer(tc) => tc,
        _ => panic!("Clear Veil should be a Trainer card"),
    };
    let mut state = game.get_state_clone();
    state.hands[0] = vec![Card::Trainer(clear_veil.clone())];
    game.set_state(state);

    deckgym::test_support::play_trainer(&mut game, 0, clear_veil);

    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    let attach = actions
        .iter()
        .find(|a| matches!(a.action, SimpleAction::AttachTool { .. }))
        .expect("Clear Veil should offer an attachment target")
        .clone();
    game.apply_action(&attach);

    assert!(game.get_state_clone().get_active(0).attached_tool.is_some());
}
