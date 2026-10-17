use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, PlayedCard},
    test_support::get_initialized_game,
};

/// Sets up a board with Team Rocket's Master Plan in hand and plays it.
fn play_master_plan(seed: u64) -> deckgym::State {
    let master_plan = get_card_by_enum(CardId::B4a070TeamRocketsMasterPlan).as_trainer();

    let mut game = get_initialized_game(seed);
    let mut state = game.get_state_clone();
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    state.hands[0] = vec![Card::Trainer(master_plan.clone())];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: master_plan,
        },
        is_stack: false,
    });

    game.get_state_clone()
}

#[test]
fn test_master_plan_always_confuses_opponent_active() {
    for seed in 0..30 {
        let state = play_master_plan(seed);
        assert!(
            state.get_active(1).is_confused(),
            "Seed {seed}: opponent's Active Pokémon should always be Confused"
        );
    }
}

#[test]
fn test_master_plan_sometimes_confuses_own_active_on_tails() {
    let mut own_confused_seen = 0;
    let mut own_not_confused_seen = 0;

    for seed in 0..30 {
        let state = play_master_plan(seed);
        if state.get_active(0).is_confused() {
            own_confused_seen += 1;
        } else {
            own_not_confused_seen += 1;
        }
    }

    assert!(
        own_confused_seen > 0,
        "Master Plan should confuse own Active on tails for some seed"
    );
    assert!(
        own_not_confused_seen > 0,
        "Master Plan should not confuse own Active on heads for some seed"
    );
}
