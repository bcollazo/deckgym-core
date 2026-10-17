use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game, get_test_game_with_board},
};

#[test]
fn test_drampa_berserk_extra_damage_when_bench_damaged() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A3124Drampa)
                .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless]),
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_damage(10),
        ],
        vec![PlayedCard::from_id(CardId::B4197WailordEx)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A3124Drampa, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    // Berserk: 20 base + 50 extra since the Bulbasaur on the bench has damage on it.
    assert_eq!(state.get_active(1).get_remaining_hp(), 250 - 70);
}

#[test]
fn test_drampa_berserk_no_extra_damage_when_bench_undamaged() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A3124Drampa)
                .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::B4197WailordEx)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A3124Drampa, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    // No Benched Pokémon has damage, so only the base 20 damage applies.
    assert_eq!(state.get_active(1).get_remaining_hp(), 250 - 20);
}

#[test]
fn test_drampa_dragon_breath_heads_paralyzes_tails_does_nothing() {
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![PlayedCard::from_id(CardId::A3b054Drampa)
                .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless])],
            vec![PlayedCard::from_id(CardId::B4197WailordEx)],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A3b054Drampa, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        let remaining = state.get_active(1).get_remaining_hp();
        let paralyzed = state.get_active(1).is_paralyzed();

        if paralyzed {
            assert_eq!(
                remaining,
                250 - 70,
                "seed {seed}: heads should deal 70 damage and paralyze"
            );
        } else {
            assert_eq!(
                remaining, 250,
                "seed {seed}: tails should do nothing at all"
            );
        }
    }
}
