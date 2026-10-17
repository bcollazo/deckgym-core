use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Kubfu's "Training": "Take a [C] Energy from your Energy Zone and attach it to
/// this Pokémon." It deals no damage and attaches one Energy to Kubfu itself.
/// The `[C]` (any-type) attach is modeled with `EnergyType::Colorless`, mirroring
/// the already-merged benched `[C]` mapping used by Delcatty's "Energy Assist" and
/// Regigigas's "Giga Turbo".
fn assert_training_attaches_one_energy(kubfu_id: CardId) {
    // Kubfu starts with one Fighting Energy so it can pay Training's [C] cost.
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(kubfu_id).with_energy(vec![EnergyType::Fighting])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    // Seed the Energy Zone so we can prove Training pulls a *new* Energy rather than
    // consuming this turn's manual attachment.
    let mut state = game.get_state_clone();
    state.energy_zone[0].current = Some(EnergyType::Fighting);
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(kubfu_id, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();

    // The [C] cost Energy stays attached; Training appends exactly one more Energy.
    assert_eq!(
        state.get_active(0).attached_energy,
        vec![EnergyType::Fighting, EnergyType::Colorless]
    );
    // Training does not consume the turn's Energy Zone (it generates its own Energy).
    assert_eq!(state.energy_zone[0].current, Some(EnergyType::Fighting));
}

#[test]
fn test_kubfu_b3_097_training_attaches_one_energy_to_self() {
    assert_training_attaches_one_energy(CardId::B3097Kubfu);
}

#[test]
fn test_kubfu_b3_172_reprint_training_attaches_one_energy_to_self() {
    assert_training_attaches_one_energy(CardId::B3172Kubfu);
}
