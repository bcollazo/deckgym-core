use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

fn played_with_hp(card_id: CardId, hp: u32) -> PlayedCard {
    let card = deckgym::database::get_card_by_enum(card_id);
    PlayedCard::new(card, 0, hp, vec![], false, vec![])
}

/// Binary Thunder deals base 40 damage against a non-ex Pokémon.
#[test]
fn test_iron_thorns_binary_thunder_base_damage_vs_non_ex() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B3a018IronThorns)
            .with_energy(vec![EnergyType::Lightning, EnergyType::Lightning])],
        vec![played_with_hp(CardId::A1001Bulbasaur, 200)],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3a018IronThorns, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let damage = 200 - game.get_state_clone().get_active(1).get_remaining_hp();
    assert_eq!(
        damage, 40,
        "Binary Thunder should deal 40 damage against a non-ex Pokémon, got {damage}"
    );
}

/// Binary Thunder deals 80 damage (40 + 40 bonus) against a Pokémon ex.
#[test]
fn test_iron_thorns_binary_thunder_bonus_damage_vs_ex() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B3a018IronThorns)
            .with_energy(vec![EnergyType::Lightning, EnergyType::Lightning])],
        vec![played_with_hp(CardId::A1096PikachuEx, 300)],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3a018IronThorns, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let damage = 300 - game.get_state_clone().get_active(1).get_remaining_hp();
    assert_eq!(
        damage, 80,
        "Binary Thunder should deal 80 damage against a Pokémon ex (40 + 40 bonus), got {damage}"
    );
}
