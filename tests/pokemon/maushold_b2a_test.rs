use deckgym::{
    actions::Action,
    card_ids::CardId::{self},
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game, play_trainer, trainer_from_id},
};

#[test]
fn test_gnawing_flips_discard_opponent_active_energy() {
    // Maushold's Triple Gnawing skill deals 60 dmg and flips 3 coins and discards opponenet active Pokémon energies
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();

        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![PlayedCard::from_id(CardId::B2a082Maushold)
                .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless])],
            vec![PlayedCard::from_id(CardId::A1211Snorlax).with_energy(vec![
                EnergyType::Grass,
                EnergyType::Grass,
                EnergyType::Grass,
            ])],
        );
        let energy_before = state.get_active(1).attached_energy.len();
        let discarded_before = state.discard_energies[1].len();
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::B2a082Maushold, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();

        // Triple Gnawing: 60 damage dealt
        assert_eq!(state.get_active(1).get_remaining_hp(), 90, "seed {seed}");

        // Assert whether opponent's active Pokémon gets discarded
        let energy_after = state.get_active(1).attached_energy.len();
        assert!(
            energy_before.saturating_sub(3) <= energy_after && energy_after <= energy_before,
            "seed {seed}"
        );
        assert_eq!(
            state.discard_energies[1].len(),
            discarded_before + (energy_before - energy_after),
            "seed {seed}"
        );
    }
}

#[test]
fn test_triple_gnawing_interaction_trainer_will() {
    // Trainer Will will force the first next coin flip to be heads
    let will = trainer_from_id(CardId::A4156Will);

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.current_player = 0;
    state.turn_count = 1;

    state.set_board(
        vec![PlayedCard::from_id(CardId::B2a082Maushold)
            .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax).with_energy(vec![
            EnergyType::Grass,
            EnergyType::Grass,
            EnergyType::Grass,
        ])],
    );
    state.hands[0] = vec![get_card_by_enum(CardId::A4156Will).clone()];
    let energy_before = state.get_active(1).attached_energy.len();
    let discarded_before = state.discard_energies[1].len();
    game.set_state(state);

    play_trainer(&mut game, 0, will.clone());
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2a082Maushold, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();

    // Triple Gnawing: 60 damage dealt
    assert_eq!(state.get_active(1).get_remaining_hp(), 90);

    // Will forces heads, thus forces Triple Gnawing to trigger once
    let energy_after = state.get_active(1).attached_energy.len();
    assert!(energy_before.saturating_sub(3) <= energy_after && energy_after <= energy_before,);
    assert_eq!(
        state.discard_energies[1].len(),
        discarded_before + (energy_before - energy_after),
    );
}
