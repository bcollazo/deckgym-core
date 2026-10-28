use super::*;

#[test]
fn find_a_friend_puts_a_colorless_pokemon_from_the_deck_into_hand() {
    let id = CardId::C1102Pidgey;
    let mut game = board(vec![ready(id, 0)], vec![durable()]);
    let mut state = game.get_state_clone();
    state.hands[0].clear();
    state.decks[0].cards = vec![
        get_card_by_enum(CardId::A1001Bulbasaur),
        get_card_by_enum(CardId::A1211Snorlax),
        get_card_by_enum(CardId::A1080Vaporeon),
    ];
    game.set_state(state);
    attack(&mut game, id, 0);
    let state = game.get_state_clone();
    assert_eq!(state.hands[0].len(), 1);
    assert_eq!(state.hands[0][0].get_name(), "Snorlax");
    assert_eq!(state.decks[0].cards.len(), 2);
}

#[test]
fn find_a_friend_does_nothing_without_a_colorless_pokemon_in_the_deck() {
    let id = CardId::C1102Pidgey;
    let mut game = board(vec![ready(id, 0)], vec![durable()]);
    let mut state = game.get_state_clone();
    state.hands[0].clear();
    state.decks[0].cards = vec![get_card_by_enum(CardId::A1001Bulbasaur)];
    game.set_state(state);
    attack(&mut game, id, 0);
    let state = game.get_state_clone();
    assert!(state.hands[0].is_empty());
    assert_eq!(state.decks[0].cards.len(), 1);
}
