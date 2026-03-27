use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard, TrainerCard},
    test_support::get_initialized_game,
};

fn trainer_from_id(card_id: CardId) -> TrainerCard {
    match get_card_by_enum(card_id) {
        Card::Trainer(tc) => tc,
        _ => panic!("Expected trainer card"),
    }
}

fn fat_bulbasaur() -> PlayedCard {
    let card = get_card_by_enum(CardId::A1001Bulbasaur);
    PlayedCard::new(card, 0, 200, vec![], false, vec![])
}

fn mega_scizor_ex_with_energy() -> PlayedCard {
    PlayedCard::from_id(CardId::B2b047MegaScizorEx)
        .with_energy(vec![EnergyType::Metal, EnergyType::Metal, EnergyType::Colorless])
}

/// Mega Scizor ex that was already in the active spot deals base 100 damage.
#[test]
fn test_bullet_slugger_no_bonus_when_not_moved_from_bench() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![mega_scizor_ex_with_energy()],
        vec![fat_bulbasaur()],
    );
    state.current_player = 0;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let final_state = game.get_state_clone();
    let opponent_hp = final_state.get_active(1).get_remaining_hp();
    assert_eq!(
        opponent_hp, 100,
        "Bullet Slugger should deal 100 base damage when not moved from bench (200 - 100 = 100)"
    );
}

/// Retreating to bring Mega Scizor ex to active triggers the 50 extra damage.
#[test]
fn test_bullet_slugger_bonus_after_regular_retreat() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Varoom (Metal, 2-energy retreat cost) is active; Mega Scizor ex is on bench.
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B2b049Varoom)
                .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless]),
            mega_scizor_ex_with_energy(),
        ],
        vec![fat_bulbasaur()],
    );
    state.current_player = 0;
    game.set_state(state);

    // Retreat Varoom and promote Mega Scizor ex (bench index 1).
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Retreat(1),
        is_stack: false,
    });

    // Mega Scizor ex is now active and moved from bench this turn.
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let final_state = game.get_state_clone();
    let opponent_hp = final_state.get_active(1).get_remaining_hp();
    assert_eq!(
        opponent_hp, 50,
        "Bullet Slugger should deal 150 damage after retreating to active (200 - 150 = 50)"
    );
}

/// Revavroom's Metal Transport ability moves Mega Scizor ex to active, triggering the 50 extra damage.
#[test]
fn test_bullet_slugger_bonus_after_revavroom_metal_transport() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Varoom (Metal type) is active; Revavroom is on bench 1 with its ability unused;
    // Mega Scizor ex is on bench 2 with full energy.
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B2b049Varoom),
            PlayedCard::from_id(CardId::B2b050Revavroom),
            mega_scizor_ex_with_energy(),
        ],
        vec![fat_bulbasaur()],
    );
    state.current_player = 0;
    state.turn_count = 3;
    game.set_state(state);

    // Use Revavroom's Metal Transport ability (bench index 1).
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 1 },
        is_stack: false,
    });

    // Choose to activate Mega Scizor ex (bench index 2).
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Activate {
            player: 0,
            in_play_idx: 2,
        },
        is_stack: false,
    });

    // Mega Scizor ex is now active and moved from bench this turn.
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let final_state = game.get_state_clone();
    let opponent_hp = final_state.get_active(1).get_remaining_hp();
    assert_eq!(
        opponent_hp, 50,
        "Bullet Slugger should deal 150 damage after Metal Transport move (200 - 150 = 50)"
    );
}

/// Playing Lyra to switch Mega Scizor ex to active also triggers the 50 extra damage.
#[test]
fn test_bullet_slugger_bonus_after_lyra_switch() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Bulbasaur is active with some damage so Lyra can be played;
    // Mega Scizor ex is on bench with full energy.
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_damage(10),
            mega_scizor_ex_with_energy(),
        ],
        vec![fat_bulbasaur()],
    );
    state.current_player = 0;
    state.turn_count = 3;

    let lyra = trainer_from_id(CardId::A4157Lyra);
    state.hands[0].push(Card::Trainer(lyra.clone()));

    game.set_state(state);

    // Play Lyra — it will queue an Activate choice.
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card: lyra },
        is_stack: false,
    });

    // Activate Mega Scizor ex (bench index 1).
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Activate {
            player: 0,
            in_play_idx: 1,
        },
        is_stack: false,
    });

    // Mega Scizor ex is now active and moved from bench this turn.
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let final_state = game.get_state_clone();
    let opponent_hp = final_state.get_active(1).get_remaining_hp();
    assert_eq!(
        opponent_hp, 50,
        "Bullet Slugger should deal 150 damage after Lyra switch (200 - 150 = 50)"
    );
}
