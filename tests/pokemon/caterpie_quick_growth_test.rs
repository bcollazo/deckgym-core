use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

/// Caterpie (B3b 001/B3b 091) Quick Growth:
/// "At the end of your opponent's turn, if this Pokémon is in the Active Spot,
/// put a random card from your deck that evolves from this Pokémon onto this
/// Pokémon to evolve it."
#[test]
fn test_quick_growth_evolves_caterpie_at_end_of_opponent_turn() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::B3b001Caterpie)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    state.current_player = 1;
    // Put exactly one Metapod (evolves from Caterpie) in player 0's deck.
    state.decks[0].cards = vec![get_card_by_enum(CardId::B3b002Metapod)];
    game.set_state(state);

    // Opponent (player 1) ends their turn without attacking.
    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    let active = state.get_active(0);
    assert!(
        matches!(&active.card, Card::Pokemon(p) if p.name == "Metapod"),
        "Caterpie should have evolved into Metapod via Quick Growth; got {:?}",
        active.card.get_name()
    );
    assert!(
        state.decks[0].cards.is_empty(),
        "Metapod should have been removed from the deck"
    );
}

#[test]
fn test_quick_growth_no_op_when_no_evolution_in_deck() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::B3b001Caterpie)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    state.current_player = 1;
    // No Metapod in the deck — only unrelated cards.
    state.decks[0].cards = vec![get_card_by_enum(CardId::A1001Bulbasaur)];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    let active = state.get_active(0);
    assert!(
        matches!(&active.card, Card::Pokemon(p) if p.name == "Caterpie"),
        "Caterpie should stay unevolved when no Metapod is in deck; got {:?}",
        active.card.get_name()
    );
}

/// The full-art variant (B3b 091) has the identical ability and should behave
/// the same way.
#[test]
fn test_quick_growth_full_art_variant_evolves() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::B3b091Caterpie)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    state.current_player = 1;
    state.decks[0].cards = vec![get_card_by_enum(CardId::B3b002Metapod)];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    let active = state.get_active(0);
    assert!(
        matches!(&active.card, Card::Pokemon(p) if p.name == "Metapod"),
        "Full-art Caterpie should also evolve via Quick Growth; got {:?}",
        active.card.get_name()
    );
}

/// The "Hook" attack (10 damage, Grass energy) should deal damage normally.
#[test]
fn test_caterpie_hook_attack_deals_10_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::B3b001Caterpie).with_energy(vec![EnergyType::Grass])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    game.set_state(state);

    let opponent_hp_before = game.get_state_clone().get_active(1).get_remaining_hp();

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3b001Caterpie, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        opponent_hp_before - 10,
        "Hook should deal exactly 10 damage"
    );
}
