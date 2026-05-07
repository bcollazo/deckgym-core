use crate::deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

fn played_card_with_base_hp(card_id: CardId, base_hp: u32) -> PlayedCard {
    let card = get_card_by_enum(card_id);
    PlayedCard::new(card, 0, base_hp, vec![], false, vec![])
}

fn use_last_respects(discard_piles: [Vec<Card>; 2]) -> u32 {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.current_player = 0;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B2a053Houndstone)
            .with_energy(vec![EnergyType::Psychic, EnergyType::Colorless])],
        vec![played_card_with_base_hp(CardId::A1001Bulbasaur, 200)],
    );
    state.discard_piles = discard_piles;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    game.get_state_clone().get_active(1).get_remaining_hp()
}

#[test]
fn test_houndstone_last_respects_does_base_damage_without_psychic_pokemon_in_discard() {
    let remaining_hp = use_last_respects([vec![], vec![]]);

    assert_eq!(remaining_hp, 150);
}

#[test]
fn test_houndstone_last_respects_counts_only_own_psychic_pokemon_in_discard() {
    let remaining_hp = use_last_respects([
        vec![
            get_card_by_enum(CardId::A1128Mewtwo),
            get_card_by_enum(CardId::A1130Ralts),
            get_card_by_enum(CardId::A1120Gastly),
            get_card_by_enum(CardId::A1001Bulbasaur),
            get_card_by_enum(CardId::PA005PokeBall),
        ],
        vec![get_card_by_enum(CardId::A1131Kirlia)],
    ]);

    assert_eq!(remaining_hp, 90);
}
