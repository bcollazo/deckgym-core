use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard},
    test_support::get_test_game_with_board,
};

fn play_quick_grow_extract_with_caterpie_and_treecko() -> deckgym::Game<'static> {
    // Quick-Grow Extract: "Choose 1 of your [G] Pokémon in play. Put a random [G] Pokémon from
    // your deck that evolves from that Pokémon onto that Pokémon to evolve it."
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B3b001Caterpie),
            PlayedCard::from_id(CardId::B3005Treecko),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let mut state = game.get_state_clone();
    state.decks[0].cards = vec![
        get_card_by_enum(CardId::B3b002Metapod),
        get_card_by_enum(CardId::B3006Grovyle),
    ];
    let quick_grow_extract = get_card_by_enum(CardId::B1a067QuickGrowExtract).as_trainer();
    state.hands[0] = vec![Card::Trainer(quick_grow_extract.clone())];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: quick_grow_extract,
        },
        is_stack: false,
    });
    game
}

#[test]
fn test_quick_grow_extract_lets_player_choose_target() {
    let game = play_quick_grow_extract_with_caterpie_and_treecko();

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    let choices: Vec<_> = choices.into_iter().map(|a| a.action).collect();
    assert_eq!(
        choices,
        vec![
            SimpleAction::EvolveRandomFromDeck {
                in_play_idx: 0,
                energy_type: EnergyType::Grass,
            },
            SimpleAction::EvolveRandomFromDeck {
                in_play_idx: 1,
                energy_type: EnergyType::Grass,
            },
        ]
    );
}

#[test]
fn test_quick_grow_extract_evolves_chosen_pokemon() {
    let mut game = play_quick_grow_extract_with_caterpie_and_treecko();

    let (_, choices) = game.get_state_clone().generate_possible_actions();
    let evolve_treecko = choices
        .into_iter()
        .find(|a| {
            matches!(
                a.action,
                SimpleAction::EvolveRandomFromDeck { in_play_idx: 1, .. }
            )
        })
        .expect("Treecko should be a selectable target");
    game.apply_action(&evolve_treecko);

    let state = game.get_state_clone();
    assert_eq!(state.get_active(0).get_name(), "Caterpie");
    assert_eq!(
        state.in_play_pokemon[0][1].as_ref().unwrap().get_name(),
        "Grovyle"
    );
    assert_eq!(
        state.decks[0].cards,
        vec![get_card_by_enum(CardId::B3b002Metapod)]
    );
}
