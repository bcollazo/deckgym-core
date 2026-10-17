use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

/// Politoed's Raid deals 50 damage, or 50 + 50 = 100 damage when Politoed evolved
/// from Poliwhirl during this turn.
#[test]
fn test_raid_extra_damage_when_evolved_from_poliwhirl_this_turn() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Snorlax has 150 HP and is weak to Fighting, so a [W] attack gets no weakness bonus.
    state.set_board(
        vec![PlayedCard::from_id(CardId::B3034Poliwhirl).with_energy(vec![EnergyType::Water])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    state.current_player = 0;
    state.hands[0].clear();
    state.hands[0].push(get_card_by_enum(CardId::B3035Politoed));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Evolve {
            evolution: get_card_by_enum(CardId::B3035Politoed),
            in_play_idx: 0,
            from_deck: false,
        },
        is_stack: false,
    });

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3035Politoed, 0),
        is_stack: false,
    });

    // Snorlax 150 HP - 100 (50 base + 50 evolution bonus) = 50.
    let hp = game.get_state_clone().get_active(1).get_remaining_hp();
    assert_eq!(
        hp, 50,
        "Raid should deal 100 damage the turn Politoed evolves from Poliwhirl"
    );
}

/// A Politoed that has been in play since a previous turn deals only base damage.
#[test]
fn test_raid_base_damage_when_not_evolved_this_turn() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::B3035Politoed).with_energy(vec![EnergyType::Water])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    state.current_player = 0;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3035Politoed, 0),
        is_stack: false,
    });

    // Snorlax 150 HP - 50 (base damage only) = 100.
    let hp = game.get_state_clone().get_active(1).get_remaining_hp();
    assert_eq!(
        hp, 100,
        "Raid should deal only base 50 damage when Politoed did not evolve this turn"
    );
}
