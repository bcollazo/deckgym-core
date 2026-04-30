use crate::deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

fn played_card_with_base_hp(card_id: CardId, base_hp: u32) -> PlayedCard {
    let card = get_card_by_enum(card_id);
    PlayedCard::new(card, 0, base_hp, vec![], false, vec![])
}

#[test]
fn test_mega_sceptile_terminating_tail_discards_grass_and_poisons() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B3008MegaSceptileEx).with_energy(vec![
                EnergyType::Grass,
                EnergyType::Grass,
                EnergyType::Colorless,
            ]),
        ],
        vec![played_card_with_base_hp(CardId::A2119DialgaEx, 150)],
    );

    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let attacker = state.get_active(0);
    let defender = state.get_active(1);

    assert_eq!(attacker.attached_energy.len(), 2);
    assert_eq!(
        attacker
            .attached_energy
            .iter()
            .filter(|&&energy| energy == EnergyType::Grass)
            .count(),
        1,
        "Terminating Tail should leave exactly one Grass energy attached"
    );
    assert!(
        attacker.attached_energy.contains(&EnergyType::Colorless),
        "Terminating Tail should not discard non-Grass energy"
    );
    assert_eq!(
        defender.get_remaining_hp(),
        20,
        "Terminating Tail should deal 130 damage"
    );
    assert!(
        defender.is_poisoned(),
        "Terminating Tail should leave the opponent's Active Pokemon Poisoned"
    );
}
