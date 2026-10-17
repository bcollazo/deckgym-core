use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard},
    test_support::get_test_game_with_board,
    Game,
};

/// Sets up `pre_evolution` as player 0's Active Pokémon with `evolution` in hand, then evolves.
/// The on-evolve ability is left pending on the move-generation stack.
fn evolve_into(
    pre_evolution: CardId,
    evolution: CardId,
    opponent_bench: Vec<PlayedCard>,
) -> Game<'static> {
    let mut opponent_board = vec![PlayedCard::from_id(CardId::A1001Bulbasaur)];
    opponent_board.extend(opponent_bench);
    let mut game =
        get_test_game_with_board(vec![PlayedCard::from_id(pre_evolution)], opponent_board);
    let mut state = game.get_state_clone();
    state.hands[0].clear();
    state.hands[0].push(get_card_by_enum(evolution));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Evolve {
            evolution: get_card_by_enum(evolution),
            in_play_idx: 0,
            from_deck: false,
        },
        is_stack: false,
    });
    game
}

fn use_pending_ability(game: &mut Game<'static>) {
    let (_, choices) = game.get_state_clone().generate_possible_actions();
    let action = choices
        .iter()
        .find(|c| matches!(c.action, SimpleAction::UseAbility { .. }))
        .expect("the on-evolve ability should be offered")
        .clone();
    game.apply_action(&action);
}

/// Galarian Perrserker's Dig Up: "…you may put 2 random Pokémon Tool cards from your discard pile
/// into your hand."
#[test]
fn test_dig_up_pulls_two_tools_from_the_discard_pile() {
    let mut game = evolve_into(
        CardId::B2110GalarianMeowth,
        CardId::B2111GalarianPerrserker,
        vec![],
    );
    let mut state = game.get_state_clone();
    state.hands[0].clear();
    state.discard_piles[0] = vec![
        get_card_by_enum(CardId::A2148RockyHelmet),
        get_card_by_enum(CardId::A2147GiantCape),
        get_card_by_enum(CardId::A1001Bulbasaur),
    ];
    game.set_state(state);

    use_pending_ability(&mut game);

    let state = game.get_state_clone();
    assert_eq!(state.hands[0].len(), 2, "two Tools should come back");
    assert!(state.hands[0].iter().all(|card| matches!(
        card,
        Card::Trainer(t) if t.trainer_card_type == deckgym::models::TrainerType::Tool
    )));
    assert_eq!(
        state.discard_piles[0].len(),
        1,
        "only the non-Tool card stays in the discard pile"
    );
}

/// Raticate's Treasure Collecting: "…look at the top 4 cards of your deck and put all Item cards
/// you find there into your hand. Shuffle the other cards back into your deck."
#[test]
fn test_treasure_collecting_takes_items_from_the_top_four() {
    let mut game = evolve_into(CardId::B4129Rattata, CardId::B4130Raticate, vec![]);
    let mut state = game.get_state_clone();
    state.hands[0].clear();
    state.decks[0].cards = vec![
        get_card_by_enum(CardId::PA001Potion),
        get_card_by_enum(CardId::A1001Bulbasaur),
        get_card_by_enum(CardId::PA001Potion),
        get_card_by_enum(CardId::A1001Bulbasaur),
        // 5th card is out of range and must be untouched by the "look at 4".
        get_card_by_enum(CardId::PA001Potion),
    ];
    game.set_state(state);

    use_pending_ability(&mut game);

    let state = game.get_state_clone();
    assert_eq!(
        state.hands[0].len(),
        2,
        "the two Potions in the top 4 go to hand"
    );
    assert!(state.hands[0]
        .iter()
        .all(|card| card.get_name() == "Potion"));
    assert_eq!(
        state.decks[0].cards.len(),
        3,
        "the two non-Items plus the untouched 5th card stay in the deck"
    );
}

/// Delcatty's Search for Friends: the player picks which Supporter comes back.
#[test]
fn test_search_for_friends_offers_each_discarded_supporter() {
    let mut game = evolve_into(CardId::B1193Skitty, CardId::B1194Delcatty, vec![]);
    let mut state = game.get_state_clone();
    state.hands[0].clear();
    state.discard_piles[0] = vec![
        get_card_by_enum(CardId::A1219Erika),
        get_card_by_enum(CardId::PA001Potion),
    ];
    game.set_state(state);

    use_pending_ability(&mut game);

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    let put_back: Vec<_> = choices
        .iter()
        .filter(|c| matches!(c.action, SimpleAction::PutDiscardCardInHand { .. }))
        .cloned()
        .collect();
    assert_eq!(put_back.len(), 1, "only the Supporter is eligible");
    game.apply_action(&put_back[0]);

    let state = game.get_state_clone();
    assert!(state.hands[0].iter().any(|c| c.get_name() == "Erika"));
    assert_eq!(state.discard_piles[0].len(), 1);
}

/// Polteageist's Refreshing Tea: the opponent shuffles their hand away and redraws one card per
/// point they still need.
#[test]
fn test_refreshing_tea_redraws_by_remaining_points() {
    let mut game = evolve_into(CardId::B2074Sinistea, CardId::B2075Polteageist, vec![]);
    let mut state = game.get_state_clone();
    state.hands[1] = vec![
        get_card_by_enum(CardId::A1001Bulbasaur),
        get_card_by_enum(CardId::A1001Bulbasaur),
        get_card_by_enum(CardId::A1001Bulbasaur),
        get_card_by_enum(CardId::A1001Bulbasaur),
        get_card_by_enum(CardId::A1001Bulbasaur),
    ];
    state.points[1] = 1;
    game.set_state(state);

    use_pending_ability(&mut game);

    let state = game.get_state_clone();
    assert_eq!(
        state.hands[1].len(),
        2,
        "opponent needs 2 more points, so they redraw 2 cards"
    );
}

/// Samurott's Stance: prevents all damage done to it until the end of the opponent's next turn.
#[test]
fn test_stance_prevents_damage_until_end_of_opponents_next_turn() {
    let mut game = evolve_into(CardId::B4043Dewott, CardId::B4044Samurott, vec![]);
    use_pending_ability(&mut game);

    // Hand the turn over and let the opponent attack Samurott.
    let mut state = game.get_state_clone();
    state.current_player = 1;
    state.turn_count = 4;
    state.in_play_pokemon[1][0] =
        Some(PlayedCard::from_id(CardId::A3135Toucannon).with_energy(vec![EnergyType::Colorless]));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 1,
        action: deckgym::test_support::attack_action(CardId::A3135Toucannon, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(0).get_remaining_hp(),
        150,
        "Stance should have prevented all of Drill Peck's damage"
    );
}

/// Poltchageist's Hospitality triggers when it is put onto the Bench from hand.
#[test]
fn test_hospitality_heals_the_active_grass_pokemon_from_the_bench() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur).with_damage(30)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let mut state = game.get_state_clone();
    state.hands[0].clear();
    state.hands[0].push(get_card_by_enum(CardId::B4017Poltchageist));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Place(get_card_by_enum(CardId::B4017Poltchageist), 1),
        is_stack: false,
    });
    use_pending_ability(&mut game);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(0).get_remaining_hp(),
        70 - 10,
        "Hospitality heals 20 of the 30 damage"
    );
}
