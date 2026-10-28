use super::*;
use deckgym::{card_ids::CardId, models::EnergyType};

const PAWMOTS: [CardId; 3] = [
    CardId::C1050Pawmot,
    CardId::C1142Pawmot,
    CardId::C1216Pawmot,
];

#[test]
fn electrocreation_attaches_lightning_to_the_active_lightning_pokemon() {
    for id in PAWMOTS {
        // Pawmot may be benched: the Ability targets the Active Spot.
        let mut game = get_test_game_with_board(
            vec![
                PlayedCard::from_id(CardId::A1094Pikachu),
                PlayedCard::from_id(id),
            ],
            vec![PlayedCard::from_id(CardId::A1055Blastoise)],
        );
        assert!(offers_ability(&game, 1), "{id:?}");
        use_ability(&mut game, 1);
        let state = game.get_state_clone();
        assert_eq!(
            state.get_active(0).attached_energy,
            vec![EnergyType::Lightning],
            "{id:?}"
        );
        assert!(!offers_ability(&game, 1), "{id:?}: once per turn");
    }
}

#[test]
fn electrocreation_is_not_offered_when_the_active_pokemon_is_not_lightning() {
    let game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1033Charmander),
            PlayedCard::from_id(CardId::C1050Pawmot),
        ],
        vec![PlayedCard::from_id(CardId::A1055Blastoise)],
    );
    assert!(!offers_ability(&game, 1));
}
