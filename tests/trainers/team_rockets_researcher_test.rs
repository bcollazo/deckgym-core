use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, PlayedCard},
    test_support::get_initialized_game,
};

/// Sets up a board with Team Rocket's Researcher in hand and `deck_cards` as the whole deck,
/// then plays it.
fn play_researcher(seed: u64, deck_cards: Vec<Card>) -> deckgym::State {
    let researcher = get_card_by_enum(CardId::B4a069TeamRocketsResearcher).as_trainer();

    let mut game = get_initialized_game(seed);
    let mut state = game.get_state_clone();
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    state.hands[0] = vec![Card::Trainer(researcher.clone())];
    state.decks[0].cards = deck_cards;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: researcher,
        },
        is_stack: false,
    });

    game.get_state_clone()
}

#[test]
fn test_researcher_never_pulls_non_team_rocket_pokemon() {
    // Deck only has non-"Team Rocket" cards, so no heads-count should ever pull anything.
    let bulbasaur = get_card_by_enum(CardId::A1001Bulbasaur);
    let cynthia = get_card_by_enum(CardId::A2152Cynthia);
    let deck_cards = vec![bulbasaur, cynthia];

    for seed in 0..15 {
        let state = play_researcher(seed, deck_cards.clone());
        assert_eq!(
            state.decks[0].cards.len(),
            deck_cards.len(),
            "Seed {seed}: no card should leave the deck without a Team Rocket Pokemon"
        );
        assert!(
            state.hands[0].is_empty(),
            "Seed {seed}: hand should stay empty when no Team Rocket Pokemon exists in deck"
        );
    }
}

#[test]
fn test_researcher_can_pull_multiple_team_rocket_pokemon_on_heads_streak() {
    let grunt_mon_a = get_card_by_enum(CardId::B4a038TeamRocketsEkans);
    let grunt_mon_b = get_card_by_enum(CardId::B4a040TeamRocketsGrimer);
    let deck_cards = vec![grunt_mon_a.clone(), grunt_mon_b.clone()];

    let mut max_pulled = 0;
    for seed in 0..60 {
        let state = play_researcher(seed, deck_cards.clone());
        let pulled = state.hands[0].len();
        // Every card actually placed in hand must be one of the eligible Team Rocket Pokemon.
        for card in &state.hands[0] {
            assert!(
                deck_cards.contains(card),
                "Seed {seed}: only Team Rocket Pokemon from the deck should be pulled"
            );
        }
        assert_eq!(
            state.decks[0].cards.len() + pulled,
            deck_cards.len(),
            "Seed {seed}: pulled cards should have left the deck"
        );
        max_pulled = max_pulled.max(pulled);
    }

    assert_eq!(
        max_pulled,
        deck_cards.len(),
        "Researcher should be able to pull every eligible Team Rocket Pokemon on a heads streak"
    );
}
