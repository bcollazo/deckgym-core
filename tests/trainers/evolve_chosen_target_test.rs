//! Quick-Grow Extract and Wallace: "Choose 1 of your … Pokémon in play. Put a random … Pokémon
//! from your deck that evolves from that Pokémon onto that Pokémon." The Pokémon is the player's
//! choice; only the evolution card is random.

use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard, TrainerCard},
    test_support::get_initialized_game,
    Game,
};

fn trainer(card_id: CardId) -> TrainerCard {
    get_card_by_enum(card_id).as_trainer()
}

/// A game (from `seed`) on turn 3 with `board` in play for player 0, `deck` as its deck and `card`
/// in hand.
fn game_with(seed: u64, board: Vec<PlayedCard>, deck: Vec<CardId>, card: &TrainerCard) -> Game<'static> {
    let mut game = get_initialized_game(seed);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;
    state.set_board(board, vec![PlayedCard::from_id(CardId::A1001Bulbasaur)]);
    state.decks[0].cards = deck.into_iter().map(get_card_by_enum).collect();
    state.hands[0] = vec![Card::Trainer(card.clone())];
    game.set_state(state);
    game
}

fn play(game: &mut Game<'static>, card: &TrainerCard) {
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: card.clone(),
        },
        is_stack: false,
    });
}

fn name_at(game: &Game<'static>, in_play_idx: usize) -> String {
    game.get_state_clone().in_play_pokemon[0][in_play_idx]
        .as_ref()
        .expect("a Pokémon there")
        .get_name()
}

#[test]
fn quick_grow_extract_lets_the_player_choose_which_pokemon_evolves() {
    // Caterpie (Active) and Treecko (Bench); the deck has 2 Metapod and 1 Grovyle. Before, the
    // target was drawn with the evolution: Caterpie 2/3 of the time, Treecko 1/3.
    let extract = trainer(CardId::B1a067QuickGrowExtract);
    for seed in 0..8 {
        let mut game = game_with(
            seed,
            vec![
                PlayedCard::from_id(CardId::B3b001Caterpie),
                PlayedCard::from_id(CardId::B3005Treecko),
            ],
            vec![CardId::B3b002Metapod, CardId::B3b002Metapod, CardId::B3006Grovyle],
            &extract,
        );
        play(&mut game, &extract);

        // One choice per Pokémon that can evolve.
        let (actor, actions) = game.get_state_clone().generate_possible_actions();
        assert_eq!(actor, 0);
        let choices: Vec<SimpleAction> = actions.iter().map(|a| a.action.clone()).collect();
        assert_eq!(
            choices,
            vec![
                SimpleAction::EvolveIntoRandomFromDeck {
                    in_play_idx: 0,
                    energy: EnergyType::Grass,
                },
                SimpleAction::EvolveIntoRandomFromDeck {
                    in_play_idx: 1,
                    energy: EnergyType::Grass,
                },
            ]
        );

        // Choosing Treecko evolves Treecko, never Caterpie.
        game.apply_action(&actions[1]);
        assert_eq!(name_at(&game, 1), "Grovyle");
        assert_eq!(name_at(&game, 0), "Caterpie");
        assert_eq!(game.get_state_clone().decks[0].cards.len(), 2);
    }
}

#[test]
fn quick_grow_extract_with_one_pokemon_to_choose_evolves_it_at_once() {
    // A single Pokémon can evolve: no decision, as before.
    let extract = trainer(CardId::B1a067QuickGrowExtract);
    let mut game = game_with(
        0,
        vec![PlayedCard::from_id(CardId::B3005Treecko)],
        vec![CardId::B3006Grovyle],
        &extract,
    );
    play(&mut game, &extract);
    assert_eq!(name_at(&game, 0), "Grovyle");
    assert!(game.get_state_clone().move_generation_stack.is_empty());
}

#[test]
fn wallace_lets_the_player_choose_which_pokemon_evolves() {
    // Staryu (50 HP, Active) and Magikarp (30 HP, Bench); the deck has Starmie and Gyarados.
    let wallace = trainer(CardId::B3b068Wallace);
    let mut game = game_with(
        0,
        vec![
            PlayedCard::from_id(CardId::A1074Staryu),
            PlayedCard::from_id(CardId::A1077Magikarp),
        ],
        vec![CardId::A1075Starmie, CardId::A1078Gyarados],
        &wallace,
    );
    play(&mut game, &wallace);
    let (_, actions) = game.get_state_clone().generate_possible_actions();
    let choices: Vec<SimpleAction> = actions.iter().map(|a| a.action.clone()).collect();
    assert_eq!(
        choices,
        vec![
            SimpleAction::EvolveIntoRandomFromDeck {
                in_play_idx: 0,
                energy: EnergyType::Water,
            },
            SimpleAction::EvolveIntoRandomFromDeck {
                in_play_idx: 1,
                energy: EnergyType::Water,
            },
        ]
    );
    game.apply_action(&actions[1]);
    assert_eq!(name_at(&game, 1), "Gyarados");
    assert_eq!(name_at(&game, 0), "Staryu");
}
