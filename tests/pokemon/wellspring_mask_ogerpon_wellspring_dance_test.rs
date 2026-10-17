use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{Card, EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board, play_trainer, trainer_from_id},
};

const VENUSAUR_EX_MAX_HP: u32 = 190;

/// Wellspring Mask Ogerpon's Wellspring Dance: "Flip a coin. If heads, this attack ALSO does 40
/// damage to 1 of your opponent's Benched Pokémon." On heads (forced via Will) with no Benched
/// Pokémon to target, the bonus fizzles but the printed 40 to the Defending Pokémon still lands.
#[test]
fn test_wellspring_dance_heads_still_damages_active_without_a_bench() {
    let will = trainer_from_id(CardId::A4156Will);

    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2048WellspringMaskOgerpon)
            .with_energy(vec![EnergyType::Water, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1004VenusaurEx)],
    );
    let mut state = game.get_state_clone();
    state.hands[0] = vec![Card::Trainer(will.clone())];
    game.set_state(state);

    // Force the next coin flip to be heads so the bench-damage branch is the one exercised.
    play_trainer(&mut game, 0, will);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2048WellspringMaskOgerpon, 0),
        is_stack: false,
    });
    game.play_until_stable();

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        VENUSAUR_EX_MAX_HP - 40,
        "Wellspring Dance's printed 40 damage must land on heads even with no bench target"
    );
}
