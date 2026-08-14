use deckgym::{
    actions::SimpleAction,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::get_test_game_with_board,
};

/// Abomasnow's Vigor Link: "If you have Arceus or Arceus ex in play, attacks used by this
/// Pokémon cost 1 less [C] Energy." Mega Punch normally needs [W][W][C]; with the discount it
/// should be usable with just [W][W].
#[test]
fn test_abomasnow_vigor_link_discounts_attack_cost_with_arceus_in_play() {
    let game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A2a021Abomasnow)
                .with_energy(vec![EnergyType::Water, EnergyType::Water]),
            PlayedCard::from_id(CardId::A2a071ArceusEx),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(choices
        .iter()
        .any(|choice| matches!(&choice.action, SimpleAction::Attack(attack) if attack.title == "Mega Punch")));
}

#[test]
fn test_abomasnow_vigor_link_no_discount_without_arceus() {
    let game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A2a021Abomasnow)
            .with_energy(vec![EnergyType::Water, EnergyType::Water])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(!choices
        .iter()
        .any(|choice| matches!(&choice.action, SimpleAction::Attack(attack) if attack.title == "Mega Punch")));
}
