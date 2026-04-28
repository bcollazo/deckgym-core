use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard, StatusCondition, TrainerCard},
    test_support::get_initialized_game,
};

fn played_card_with_base_hp(card_id: CardId, base_hp: u32) -> PlayedCard {
    let card = get_card_by_enum(card_id);
    PlayedCard::new(card, 0, base_hp, vec![], false, vec![])
}

fn trainer_from_id(card_id: CardId) -> TrainerCard {
    match get_card_by_enum(card_id) {
        Card::Trainer(trainer) => trainer,
        _ => panic!("Expected trainer card"),
    }
}

fn play_will(game: &mut deckgym::Game<'static>, actor: usize) {
    game.apply_action(&Action {
        actor,
        action: SimpleAction::Play {
            trainer_card: trainer_from_id(CardId::A4156Will),
        },
        is_stack: false,
    });
}

#[test]
fn test_chansey_bind_wound_heals_one_of_your_pokemon() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B3127Chansey)
                .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless]),
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_damage(40),
        ],
        vec![PlayedCard::from_id(CardId::A1053Squirtle)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    let heal_bench = choices
        .iter()
        .find(|choice| {
            matches!(
                choice.action,
                SimpleAction::Heal {
                    in_play_idx: 1,
                    amount: 30,
                    cure_status: false
                }
            )
        })
        .expect("Bind Wound should offer a heal choice for damaged benched Pokemon")
        .clone();

    game.apply_action(&heal_bench);

    assert_eq!(game.get_state_clone().get_remaining_hp(0, 1), 60);
}

#[test]
fn test_chansey_scrunch_heads_prevents_next_attack_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A4131Chansey)
            .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
    );
    state.current_player = 0;
    state.turn_count = 3;
    state.hands[0] = vec![Card::Trainer(trainer_from_id(CardId::A4156Will))];
    game.set_state(state);

    play_will(&mut game, 0);
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let mut state = game.get_state_clone();
    state.current_player = 1;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    assert_eq!(game.get_state_clone().get_active(0).get_remaining_hp(), 100);
}

#[test]
fn test_blissey_happiness_supplement_removes_active_special_condition() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::B3128Blissey),
        ],
        vec![PlayedCard::from_id(CardId::A1053Squirtle)],
    );
    state.apply_status_condition(0, 0, StatusCondition::Poisoned);
    state.current_player = 0;
    state.turn_count = 3;
    game.set_state(state);

    let (_actor, choices) = game.get_state_clone().generate_possible_actions();
    let ability = choices
        .iter()
        .find(|choice| matches!(choice.action, SimpleAction::UseAbility { in_play_idx: 1 }))
        .expect("Blissey should be able to cure a condition from the Active Pokemon")
        .clone();

    game.apply_action(&ability);

    assert!(!game.get_state_clone().get_active(0).is_poisoned());
}

#[test]
fn test_blissey_ex_happy_punch_heads_heals_self_after_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::PA098BlisseyEx)
            .with_energy(vec![
                EnergyType::Colorless,
                EnergyType::Colorless,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ])
            .with_damage(80)],
        vec![played_card_with_base_hp(CardId::A1001Bulbasaur, 200)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    state.hands[0] = vec![Card::Trainer(trainer_from_id(CardId::A4156Will))];
    game.set_state(state);

    play_will(&mut game, 0);
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 100);
    assert_eq!(state.get_active(0).get_remaining_hp(), 160);
}
