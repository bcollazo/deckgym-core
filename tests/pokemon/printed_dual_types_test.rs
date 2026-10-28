use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard},
    test_support::get_test_game_with_board,
};

#[test]
fn printed_dual_types_are_preserved_for_every_alternate_printing() {
    for id in [
        CardId::C1042MegaGreninjaEx,
        CardId::C1155MegaGreninjaEx,
        CardId::C1173MegaGreninjaEx,
    ] {
        let card = get_card_by_enum(id);
        assert_eq!(
            card.get_types(),
            vec![EnergyType::Water, EnergyType::Darkness]
        );
        assert!(card.is_type(EnergyType::Darkness));
        assert!(!card.is_type(EnergyType::Fire));
    }
    for id in [
        CardId::C1056MegaMewtwoXEx,
        CardId::C1157MegaMewtwoXEx,
        CardId::C1175MegaMewtwoXEx,
        CardId::C1185MegaMewtwoXEx,
    ] {
        let card = get_card_by_enum(id);
        assert_eq!(
            card.get_types(),
            vec![EnergyType::Psychic, EnergyType::Fighting]
        );
        let json = serde_json::to_string(&card).unwrap();
        assert!(json.contains("secondary_energy_type"));
        let decoded: Card = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.get_types(), card.get_types());
    }
}

#[test]
fn legacy_single_type_json_still_deserializes_without_an_extra_field() {
    let card = get_card_by_enum(CardId::A1001Bulbasaur);
    let json = serde_json::to_string(&card).unwrap();
    assert!(!json.contains("secondary_energy_type"));
    let decoded: Card = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.get_types(), vec![EnergyType::Grass]);
}

#[test]
fn printed_secondary_type_triggers_weakness_in_the_damage_pipeline() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::C1056MegaMewtwoXEx)],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (0, 0),
            targets: vec![(40, 1, 0)],
            is_from_active_attack: true,
        },
        is_stack: false,
    });
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 90);
}

#[test]
fn inferred_deck_energy_includes_both_printed_types_and_explicit_energy_takes_precedence() {
    let inferred = deckgym::Deck::from_string("1 C1 042").unwrap();
    assert_eq!(
        inferred.energy_types(),
        &[EnergyType::Water, EnergyType::Darkness]
    );
    let explicit = deckgym::Deck::from_string("Energy: Water\n1 C1 042").unwrap();
    assert_eq!(explicit.energy_types(), &[EnergyType::Water]);
}
