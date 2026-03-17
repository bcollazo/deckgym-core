use deckgym::{
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

/// Test Mega Steelix ex B1a 052 - Adamantine Rolling
/// Should apply NoWeakness and ReducedDamage effects, negating Fire weakness on next turn
#[test]
fn test_mega_steelix_adamantine_rolling_no_weakness() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    // Set up opponent (player 1) with Charmander (Fire type attacker)
    // Give it extra HP to survive Mega Steelix's 120 damage attack
    let charmander_played = played_card_with_base_hp(CardId::A1033Charmander, 150)
        .with_energy(vec![EnergyType::Fire, EnergyType::Fire]);

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B1a052MegaSteelixEx).with_energy(vec![
                EnergyType::Metal,
                EnergyType::Metal,
                EnergyType::Metal,
                EnergyType::Colorless,
            ]),
        ],
        vec![charmander_played],
    );

    game.set_state(state);

    // Player 0: Mega Steelix attacks with Adamantine Rolling
    // This should apply NoWeakness and ReducedDamage effects to Mega Steelix
    let steelix_attack = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };

    game.apply_action(&steelix_attack);

    // End turn to switch to player 1
    let end_turn = Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    };
    game.apply_action(&end_turn);

    let state = game.get_state_clone();
    let steelix_hp_before = state.get_active(0).get_remaining_hp();

    // Player 1: Charmander attacks with Ember (30 damage, Fire type)
    // Normally this would do 30 + 20 = 50 damage (base + Fire weakness)
    // But NoWeakness effect should prevent the +20 from weakness
    // And ReducedDamage should reduce damage by 20
    // So: 30 damage (base) - 20 (ReducedDamage) = 10 damage
    let charmander_attack = Action {
        actor: 1,
        action: SimpleAction::Attack(0), // Ember
        is_stack: false,
    };

    game.apply_action(&charmander_attack);
    let state = game.get_state_clone();

    let steelix_hp_after = state.get_active(0).get_remaining_hp();
    let damage_taken = steelix_hp_before - steelix_hp_after;

    // Verify NoWeakness worked: should take only 10 damage (30 - 20), not 50 (30+20) or 30 (30+20-20)
    assert_eq!(
        damage_taken, 10,
        "Mega Steelix should take 10 damage (30 base - 20 reduction), NoWeakness should negate +20 weakness bonus"
    );
    assert_eq!(
        steelix_hp_after, 210,
        "Mega Steelix should have 210 HP (220 - 10)"
    );
}
