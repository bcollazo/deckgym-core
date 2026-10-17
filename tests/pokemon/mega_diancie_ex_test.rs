use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Brilliant Storm: 40 + 20 for each [P] Energy attached to all of your Pokémon.
/// With no Psychic energy on your side, it should deal only the base 40 damage.
#[test]
fn test_mega_diancie_ex_brilliant_storm_no_psychic_energy() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B3b032MegaDiancieEx).with_energy(vec![
                EnergyType::Colorless,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let action = Action {
        actor: 0,
        action: attack_action(CardId::B3b032MegaDiancieEx, 0),
        is_stack: false,
    };
    game.apply_action(&action);

    let state = game.get_state_clone();
    // Bulbasaur has 70 HP, 40 damage should leave 30 HP
    let bulbasaur = state.get_active(1);
    assert_eq!(
        bulbasaur.get_remaining_hp(),
        70 - 40,
        "Brilliant Storm should deal 40 base damage with no Psychic energy"
    );
}

/// With 2 [P] Energy across your Pokémon, Brilliant Storm should deal 40 + 40 = 80 damage.
#[test]
fn test_mega_diancie_ex_brilliant_storm_with_psychic_energy() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B3b032MegaDiancieEx).with_energy(vec![
                EnergyType::Psychic,
                EnergyType::Psychic,
                EnergyType::Colorless,
            ]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![
            // Snorlax has 150 HP — survives 80 damage (40 + 2×20)
            PlayedCard::from_id(CardId::A1211Snorlax),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );

    let action = Action {
        actor: 0,
        action: attack_action(CardId::B3b032MegaDiancieEx, 0),
        is_stack: false,
    };
    game.apply_action(&action);

    let state = game.get_state_clone();
    // 40 base + 2 * 20 = 80 damage; Snorlax has 150 HP and survives
    let opponent_active = state.get_active(1);
    assert_eq!(
        opponent_active.get_remaining_hp(),
        150 - 80,
        "Brilliant Storm should deal 80 damage with 2 Psychic energy"
    );
}
