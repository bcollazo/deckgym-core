use super::*;

#[test]
fn signs_of_evolution_puts_only_an_eevee_evolution_into_hand() {
    let id = CardId::C1107Eevee;
    let mut game = board(vec![ready(id, 0)], vec![durable()]);
    let mut state = game.get_state_clone();
    state.hands[0].clear();
    state.decks[0].cards = vec![
        get_card_by_enum(CardId::A1001Bulbasaur),
        get_card_by_enum(CardId::A1206Eevee),
        get_card_by_enum(CardId::A1080Vaporeon),
    ];
    game.set_state(state);
    attack(&mut game, id, 0);
    let state = game.get_state_clone();
    assert_eq!(state.hands[0].len(), 1);
    assert_eq!(state.hands[0][0].get_name(), "Vaporeon");
    assert_eq!(state.decks[0].cards.len(), 2);
}
