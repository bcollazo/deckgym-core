use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

#[test]
fn test_single_lunge_full_damage_when_undamaged() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![PlayedCard::from_id(CardId::A4022Heracross)
            .with_energy(vec![EnergyType::Grass, EnergyType::Grass])],
        // Use Snorlax (150 HP) so it survives the 80 damage hit
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });
    game.play_until_stable();

    // Single Lunge: 40 base + 40 extra when undamaged = 80
    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        150 - 80
    );
}

#[test]
fn test_single_lunge_base_damage_when_damaged() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![PlayedCard::from_id(CardId::A4022Heracross)
            .with_energy(vec![EnergyType::Grass, EnergyType::Grass])
            .with_damage(10)],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });
    game.play_until_stable();

    // Single Lunge: 40 base only (Heracross is damaged)
    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        150 - 40
    );
}

#[test]
fn test_powerful_friends_extra_damage_with_stage2_on_bench() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B3003Heracross)
                .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless]),
            PlayedCard::from_id(CardId::A1003Venusaur), // Stage 2 on bench
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });
    game.play_until_stable();

    // Powerful Friends: 30 base + 50 extra = 80 (Stage 2 on bench)
    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        150 - 80
    );
}

#[test]
fn test_powerful_friends_base_damage_without_stage2_on_bench() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B3003Heracross)
                .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless]),
            PlayedCard::from_id(CardId::A1001Bulbasaur), // Stage 0 on bench — not Stage 2
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });
    game.play_until_stable();

    // Powerful Friends: 30 base only (no Stage 2 on bench)
    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        150 - 30
    );
}

#[test]
fn test_dynamic_horn_tails_damages_itself() {
    let mut saw_self_damage = false;

    for seed in 0..200 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![
                PlayedCard::from_id(CardId::PB056MegaHeracrossEx).with_energy(vec![
                    EnergyType::Grass,
                    EnergyType::Grass,
                    EnergyType::Grass,
                    EnergyType::Colorless,
                ]),
            ],
            vec![PlayedCard::from_id(CardId::A1211Snorlax)],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Attack(0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        let attacker_hp = state.get_active(0).get_remaining_hp();

        // Dynamic Horn: Heads = no self-damage (180 HP). Tails = 60 self-damage (120 HP).
        if attacker_hp == 180 - 60 {
            saw_self_damage = true;
        } else {
            assert_eq!(
                attacker_hp, 180,
                "Seed {seed}: heads should leave Mega Heracross at full HP"
            );
        }
    }

    assert!(
        saw_self_damage,
        "Expected tails outcome where Dynamic Horn does 60 self-damage to Mega Heracross"
    );
}
