use super::*;

#[test]
fn crushing_rampage_discards_only_the_top_two_of_own_deck() {
    let id = CardId::C1085MegaTyranitarEx;
    for count in 0..=3 {
        let mut game = board(0, vec![ready(id, 0)], vec![durable()]);
        let mut s = game.get_state_clone();
        let cards: Vec<_> = [
            CardId::PA001Potion,
            CardId::PA005PokeBall,
            CardId::A1001Bulbasaur,
        ]
        .iter()
        .take(count)
        .map(|id| get_card_by_enum(*id))
        .collect();
        s.decks[0].cards = cards.clone();
        s.discard_piles[0].clear();
        let opponent = s.decks[1].cards.clone();
        game.set_state(s);
        attack(&mut game, id, 0);
        let s = game.get_state_clone();
        assert_eq!(s.get_active(1).public_details().damage, 180);
        assert_eq!(s.decks[0].cards, cards[count.min(2)..]);
        assert_eq!(s.discard_piles[0], cards[..count.min(2)]);
        assert_eq!(s.decks[1].cards, opponent);
    }
}
