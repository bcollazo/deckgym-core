use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

fn played_card_with_base_hp(card_id: CardId, base_hp: u32) -> PlayedCard {
    let card = get_card_by_enum(card_id);
    PlayedCard::new(card, 0, base_hp, vec![], false, vec![])
}

/// Test Gouging Fire B3a 054 - Scorching Interruption: discard 2 Energy from itself, then take
/// -30 damage from attacks during the opponent's next turn.
#[test]
fn test_gouging_fire_scorching_interruption() {
    // Give the opposing Charmander extra HP so it survives Scorching Interruption's 100 damage.
    let charmander =
        played_card_with_base_hp(CardId::A1033Charmander, 150).with_energy(vec![EnergyType::Fire]);

    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B3a054GougingFire).with_energy(vec![
                EnergyType::Fire,
                EnergyType::Lightning,
                EnergyType::Colorless,
            ]),
        ],
        vec![charmander],
    );

    // Player 0: Gouging Fire attacks with Scorching Interruption.
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3a054GougingFire, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    // 100 fixed damage to the opponent's Active.
    assert_eq!(state.get_active(1).get_remaining_hp(), 150 - 100);
    // 2 of the 3 attached Energy were discarded.
    assert_eq!(state.get_active(0).attached_energy.len(), 1);

    // End turn so it becomes the opponent's next turn.
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });

    let gouging_fire_hp_before = game.get_state_clone().get_active(0).get_remaining_hp();

    // Player 1: Charmander attacks Gouging Fire with Ember (30 damage, no weakness vs [N]).
    game.apply_action(&Action {
        actor: 1,
        action: attack_action(CardId::A1033Charmander, 0),
        is_stack: false,
    });

    let gouging_fire_hp_after = game.get_state_clone().get_active(0).get_remaining_hp();

    // -30 damage reduction should fully negate Ember's 30 damage.
    assert_eq!(gouging_fire_hp_after, gouging_fire_hp_before);
}
