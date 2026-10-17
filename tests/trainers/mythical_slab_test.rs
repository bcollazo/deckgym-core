use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, PlayedCard},
    test_support::get_test_game_with_board,
};

/// Plays Mythical Slab with `top` as the top card of the deck, followed by an Exeggcute, then
/// returns (hand, deck) after the effect resolved.
fn play_mythical_slab(top: CardId) -> (Vec<Card>, Vec<Card>) {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let mut state = game.get_state_clone();
    let slab = get_card_by_enum(CardId::A1a065MythicalSlab);
    state.hands[0] = vec![slab.clone()];
    state.decks[0].cards = vec![
        get_card_by_enum(top),
        get_card_by_enum(CardId::A1a001Exeggcute),
    ];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: slab.as_trainer(),
        },
        is_stack: false,
    });

    let state = game.get_state_clone();
    (state.hands[0].clone(), state.decks[0].cards.clone())
}

fn assert_kept(top: CardId) {
    let (hand, deck) = play_mythical_slab(top);
    assert_eq!(hand, vec![get_card_by_enum(top)]);
    assert_eq!(deck, vec![get_card_by_enum(CardId::A1a001Exeggcute)]);
}

fn assert_bottomed(top: CardId) {
    let (hand, deck) = play_mythical_slab(top);
    assert!(hand.is_empty());
    assert_eq!(
        deck,
        vec![
            get_card_by_enum(CardId::A1a001Exeggcute),
            get_card_by_enum(top),
        ]
    );
}

#[test]
fn test_mythical_slab_keeps_a_basic_psychic_pokemon() {
    assert_kept(CardId::A1115Abra);
}

#[test]
fn test_mythical_slab_keeps_a_stage_1_psychic_pokemon() {
    assert_kept(CardId::A1116Kadabra);
}

#[test]
fn test_mythical_slab_keeps_a_stage_2_psychic_pokemon() {
    assert_kept(CardId::A1117Alakazam);
}

#[test]
fn test_mythical_slab_bottoms_a_non_psychic_basic_pokemon() {
    assert_bottomed(CardId::A1001Bulbasaur);
}

#[test]
fn test_mythical_slab_bottoms_a_trainer() {
    assert_bottomed(CardId::PA001Potion);
}
