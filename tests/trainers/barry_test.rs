use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard, TrainerCard},
    test_support::get_initialized_game,
};

fn barry_trainer_card() -> TrainerCard {
    match get_card_by_enum(CardId::A2a074Barry) {
        Card::Trainer(tc) => tc,
        _ => panic!("Expected trainer card"),
    }
}

#[test]
fn test_barry_reduces_heracross_attack_cost_by_2_colorless() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;

    // Heracross A2a 001 "Single-Horn Throw": costs [G][C][C] — with Barry it should cost [G] only
    state.set_board(
        vec![PlayedCard::from_id(CardId::A2a001Heracross).with_energy(vec![EnergyType::Grass])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let barry = barry_trainer_card();
    state.hands[0] = vec![Card::Trainer(barry.clone())];
    game.set_state(state);

    // Play Barry to reduce attack cost
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: barry,
        },
        is_stack: false,
    });

    // Heracross should now be able to attack with just 1 Grass energy (cost reduced by 2 C)
    let (_, choices) = game.get_state_clone().generate_possible_actions();
    assert!(
        choices
            .iter()
            .any(|c| matches!(c.action, SimpleAction::Attack(0))),
        "Heracross should be able to attack after Barry reduces the energy cost"
    );
}

#[test]
fn test_barry_does_not_affect_other_pokemon() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;

    // Bulbasaur "Vine Whip" costs [G][C] — Barry should not reduce this
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur).with_energy(vec![EnergyType::Grass])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let barry = barry_trainer_card();
    state.hands[0] = vec![Card::Trainer(barry.clone())];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: barry,
        },
        is_stack: false,
    });

    // Bulbasaur still needs [G][C] to attack — 1 Grass energy is not enough
    let (_, choices) = game.get_state_clone().generate_possible_actions();
    assert!(
        !choices.iter().any(|c| matches!(c.action, SimpleAction::Attack(_))),
        "Bulbasaur should NOT be able to attack — Barry only helps Snorlax, Heracross, and Staraptor"
    );
}
