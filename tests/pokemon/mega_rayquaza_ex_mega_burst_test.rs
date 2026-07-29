use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

const VENUSAUR_EX_MAX_HP: u32 = 190;

fn mega_rayquaza_ex_board(energy: Vec<EnergyType>) -> deckgym::Game<'static> {
    get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4120MegaRayquazaEx).with_energy(energy)],
        vec![PlayedCard::from_id(CardId::A1004VenusaurEx)],
    )
}

/// Mega Rayquaza ex's Mega Burst discards all [R] and [L] Energy attached to it and does
/// 50 damage for each Energy discarded that way.
#[test]
fn test_mega_rayquaza_ex_mega_burst_damage_per_discarded_energy() {
    let mut game = mega_rayquaza_ex_board(vec![
        EnergyType::Fire,
        EnergyType::Fire,
        EnergyType::Lightning,
    ]);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4120MegaRayquazaEx, 0),
        is_stack: false,
    });

    // 2 [R] + 1 [L] discarded → 50 × 3 = 150 damage.
    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        VENUSAUR_EX_MAX_HP - 150
    );
    assert!(state.get_active(0).attached_energy.is_empty());
    assert_eq!(state.discard_energies[0].len(), 3);
}

/// Mega Burst leaves Energy of other types attached, and they do not add damage.
#[test]
fn test_mega_rayquaza_ex_mega_burst_only_discards_fire_and_lightning() {
    let mut game = mega_rayquaza_ex_board(vec![
        EnergyType::Fire,
        EnergyType::Lightning,
        EnergyType::Water,
        EnergyType::Water,
    ]);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4120MegaRayquazaEx, 0),
        is_stack: false,
    });

    // 1 [R] + 1 [L] discarded → 50 × 2 = 100 damage; both [W] stay attached.
    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        VENUSAUR_EX_MAX_HP - 100
    );
    assert_eq!(
        state.get_active(0).attached_energy,
        vec![EnergyType::Water, EnergyType::Water]
    );
}
