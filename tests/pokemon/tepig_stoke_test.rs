use crate::deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard, TrainerCard},
    test_support::get_initialized_game,
};

fn trainer_from_id(card_id: CardId) -> TrainerCard {
    match get_card_by_enum(card_id) {
        Card::Trainer(t) => t,
        _ => panic!("Expected trainer card"),
    }
}

fn play_will(game: &mut crate::deckgym::Game<'static>, actor: usize) {
    let will = trainer_from_id(CardId::A4156Will);
    game.apply_action(&Action {
        actor,
        action: SimpleAction::Play { trainer_card: will },
        is_stack: false,
    });
}

#[test]
fn test_tepig_stoke_heads_attaches_two_fire_energy_to_self() {
    let will = trainer_from_id(CardId::A4156Will);

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.set_board(
        vec![PlayedCard::from_id(CardId::B3026Tepig).with_energy(vec![EnergyType::Fire])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 2;
    state.hands[0] = vec![Card::Trainer(will)];
    game.set_state(state);

    play_will(&mut game, 0);
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let active = state.get_active(0);
    assert_eq!(active.attached_energy.len(), 3);
    assert_eq!(
        active.attached_energy,
        vec![EnergyType::Fire, EnergyType::Fire, EnergyType::Fire]
    );
}

#[test]
fn test_tepig_stoke_has_tails_branch_without_will() {
    let mut saw_tails = false;

    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.set_board(
            vec![PlayedCard::from_id(CardId::B3026Tepig).with_energy(vec![EnergyType::Fire])],
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        );
        state.current_player = 0;
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Attack(0),
            is_stack: false,
        });

        let state = game.get_state_clone();
        let active = state.get_active(0);
        if active.attached_energy == vec![EnergyType::Fire] {
            saw_tails = true;
            break;
        }
    }

    assert!(
        saw_tails,
        "Expected at least one fixed seed where Tepig's Stoke flips tails without Will"
    );
}
