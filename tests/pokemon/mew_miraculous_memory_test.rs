use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Mew's Miraculous Memory picks an attack at random from the Pokémon in the opponent's hand and
/// deck and uses it. With Bulbasaur as the only Pokémon there, Vine Whip (40) is forced.
#[test]
fn test_miraculous_memory_uses_an_attack_from_the_opponents_deck() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2b030Mew)
            .with_energy(vec![EnergyType::Psychic, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    let mut state = game.get_state_clone();
    state.hands[1].clear();
    state.decks[1].cards = vec![get_card_by_enum(CardId::A1001Bulbasaur)];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2b030Mew, 0),
        is_stack: false,
    });

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert_eq!(choices.len(), 1, "the copied attack is chosen at random");
    assert!(
        matches!(&choices[0].action, SimpleAction::Attack(attack) if attack.title == "Vine Whip")
    );

    game.apply_action(&choices[0].clone());

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        150 - 40
    );
}

/// With no Pokémon in the opponent's hand or deck there is no attack to copy, so Miraculous
/// Memory does nothing.
#[test]
fn test_miraculous_memory_does_nothing_without_opponent_pokemon() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2b030Mew)
            .with_energy(vec![EnergyType::Psychic, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    let mut state = game.get_state_clone();
    state.hands[1].clear();
    state.decks[1].cards.clear();
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2b030Mew, 0),
        is_stack: false,
    });

    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 150);
}
