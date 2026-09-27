use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::get_test_game_with_board,
};

fn offers_attack(game: &deckgym::Game, title: &str) -> bool {
    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    actions.iter().any(
        |action| matches!(&action.action, SimpleAction::Attack(attack) if attack.title == title),
    )
}

/// Boltund - Defiant Spark normally costs [L][C][C], but "If this Pokémon has damage on it,
/// this attack can be used for 1 [L] Energy."
#[test]
fn test_defiant_spark_costs_one_lightning_only_while_damaged() {
    let undamaged = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A4a031Boltund).with_energy(vec![EnergyType::Lightning])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    assert!(
        !offers_attack(&undamaged, "Defiant Spark"),
        "an undamaged Boltund still owes the full [L][C][C]"
    );

    let damaged = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A4a031Boltund)
            .with_energy(vec![EnergyType::Lightning])
            .with_damage(10)],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    assert!(
        offers_attack(&damaged, "Defiant Spark"),
        "a damaged Boltund can attack for a single [L]"
    );
}

#[test]
fn test_defiant_spark_deals_its_damage_under_the_reduced_cost() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A4a031Boltund)
            .with_energy(vec![EnergyType::Lightning])
            .with_damage(10)],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: deckgym::test_support::attack_action(CardId::A4a031Boltund, 0),
        is_stack: false,
    });

    // 150 - 70 = 80.
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 80);
}

/// Veluza - Shedding Spiral normally costs [W][C][C][C], but "If you have no cards in your
/// deck, this attack can be used for 1 [W] Energy."
#[test]
fn test_shedding_spiral_costs_one_water_only_with_an_empty_deck() {
    let stocked = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2a031Veluza).with_energy(vec![EnergyType::Water])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    assert!(
        !stocked.get_state_clone().decks[0].cards.is_empty(),
        "the test deck should start with cards"
    );
    assert!(
        !offers_attack(&stocked, "Shedding Spiral"),
        "with cards left in the deck Veluza owes the full [W][C][C][C]"
    );

    let mut empty = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2a031Veluza).with_energy(vec![EnergyType::Water])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    let mut state = empty.get_state_clone();
    state.decks[0].cards.clear();
    empty.set_state(state);
    assert!(
        offers_attack(&empty, "Shedding Spiral"),
        "with an empty deck Veluza can attack for a single [W]"
    );
}
