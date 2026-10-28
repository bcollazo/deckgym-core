use super::*;

#[test]
fn poke_mart_guy_draws_two_random_eligible_copies_without_replacement() {
    let eligible = [
        CardId::C1120SuperPotion,
        CardId::C1121MaxRepel,
        CardId::C1122FullHeal,
    ];
    let mut seen = std::collections::HashSet::new();
    for seed in 0..40 {
        let mut g = deckgym::test_support::get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![pokemon("A1 001")],
            vec![pokemon("A1 211")],
        );
        let mut s = g.get_state_clone();
        s.decks[0].cards = eligible
            .iter()
            .copied()
            .chain([CardId::PA001Potion])
            .map(get_card_by_enum)
            .collect();
        g.set_state(s);
        play(
            &mut g,
            if seed % 2 == 0 {
                CardId::C1129PokeMartGuy
            } else {
                CardId::C1169PokeMartGuy
            },
        );
        let s = g.get_state_clone();
        assert_eq!(s.hands[0].len(), 2);
        assert_eq!(s.decks[0].cards.len(), 2);
        assert!(s.hands[0]
            .iter()
            .all(|c| eligible.iter().any(|id| get_card_by_enum(*id) == *c)));
        assert!(s.decks[0]
            .cards
            .contains(&get_card_by_enum(CardId::PA001Potion)));
        let mut names: Vec<_> = s.hands[0].iter().map(|c| c.get_id()).collect();
        names.sort();
        seen.insert(names);
    }
    assert_eq!(seen.len(), 3);
}
#[test]
fn poke_mart_guy_handles_zero_one_or_two_identical_matches() {
    for count in 0..=2 {
        let mut g = board(vec![pokemon("A1 001")]);
        let mut s = g.get_state_clone();
        s.decks[0].cards = vec![get_card_by_enum(CardId::C1120SuperPotion); count];
        g.set_state(s);
        play(&mut g, CardId::C1129PokeMartGuy);
        let s = g.get_state_clone();
        assert_eq!(s.hands[0].len(), count);
        assert!(s.decks[0].cards.is_empty());
    }
}
