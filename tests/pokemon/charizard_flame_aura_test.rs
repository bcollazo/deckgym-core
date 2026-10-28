use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::get_test_game_with_board,
};

fn attach(energy: EnergyType, in_play_idx: usize) -> Action {
    Action {
        actor: 0,
        action: SimpleAction::Attach {
            attachments: vec![(1, energy, in_play_idx)],
            is_turn_energy: true,
        },
        is_stack: false,
    }
}

#[test]
fn test_charizard_flame_aura_damages_opponent_active_on_fire_attach() {
    // Flame Aura: Whenever you attach a [R] Energy from your Energy Zone to this Pokémon,
    // do 30 damage to your opponent's Active Pokémon.
    for charizard in [CardId::C1018Charizard, CardId::C1136Charizard] {
        let mut game = get_test_game_with_board(
            vec![PlayedCard::from_id(charizard)],
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        );

        game.apply_action(&attach(EnergyType::Fire, 0));

        let state = game.get_state_clone();
        assert_eq!(state.get_active(0).attached_energy.len(), 1);
        assert_eq!(state.get_active(1).get_remaining_hp(), 40);
    }
}

#[test]
fn test_charizard_flame_aura_works_from_the_bench() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::C1018Charizard),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    game.apply_action(&attach(EnergyType::Fire, 1));

    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 40);
}

#[test]
fn test_charizard_flame_aura_ignores_non_fire_energy() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::C1018Charizard)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    game.apply_action(&attach(EnergyType::Water, 0));

    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 70);
}
